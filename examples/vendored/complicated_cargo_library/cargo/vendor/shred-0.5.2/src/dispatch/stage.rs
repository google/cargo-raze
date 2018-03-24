//! Stages module. To explain the rough functionality, here some information in words:
//!
//! 1) A *stage* is a part of the dispatching which contains work that can be done
//!    in parallel
//!
//! 2) In each stage, there's a *group*. A group is a list of systems, which are executed
//!    in order. Thus, systems of a group may conflict with each other, but groups of
//!    a stage may not.
//!
//! So the actual dispatching works like this (pseudo code):
//!
//! for stage in stages {
//!     stage.for_each_group(|group| for system in group {
//!         system.run(res);
//!     });
//! }
//!
//! As you can see, we execute stages sequentially, fork the stage to execute
//! multiple groups at once, but execute the systems of each group sequentially again.
//! Here's why:
//!
//! Imagine we have like a really heavy system, like a collision detection.
//! And we also have a really light system. Now, given both systems don't have any
//! conflicts, thus can run in parallel, all the other systems had to wait until
//! the collision detection finished. That's not what we want. Instead, we say:
//!
//! > If a system only conflicts with one group of a stage, it gets executed after
//!   all the other systems of this group, but only if by doing this, the running
//!   times of the groups of this stage get closer to each other (called balanced
//!   in code).
//!

use arrayvec::ArrayVec;
use smallvec::SmallVec;

use dispatch::dispatcher::{SystemExecSend, SystemId};
use dispatch::util::check_intersection;
use res::{ResourceId, Resources};
use system::{RunningTime, System};

const MAX_SYSTEMS_PER_GROUP: usize = 5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Conflict {
    None,
    Single(usize),
    Multiple,
}

impl Conflict {
    fn add(conflict: Self, group: usize) -> Self {
        match conflict {
            Conflict::None => Conflict::Single(group),
            Conflict::Single(_) => Conflict::Multiple,
            Conflict::Multiple => Conflict::Multiple,
        }
    }
}

type GroupVec<T> = SmallVec<[T; 6]>;

#[derive(Debug)]
enum InsertionTarget {
    Stage(usize),
    Group(usize, usize),
    NewStage,
}

#[derive(Default)]
pub struct Stage<'a> {
    groups: GroupVec<ArrayVec<[SystemExecSend<'a>; MAX_SYSTEMS_PER_GROUP]>>,
}

impl<'a> Stage<'a> {
    fn new() -> Self {
        Default::default()
    }

    #[cfg(not(target_os = "emscripten"))]
    pub fn execute(&mut self, res: &Resources) {
        use rayon::prelude::*;

        self.groups
            .par_iter_mut()
            .for_each(|group| for system in group {
                system.run_now(res);
            });
    }

    pub fn execute_seq(&mut self, res: &Resources) {
        for group in &mut self.groups {
            for system in group {
                system.run_now(res);
            }
        }
    }
}

#[derive(Default)]
pub struct StagesBuilder<'a> {
    barrier: usize,
    ids: Vec<GroupVec<ArrayVec<[SystemId; MAX_SYSTEMS_PER_GROUP]>>>,
    reads: Vec<GroupVec<SmallVec<[ResourceId; 12]>>>,
    running_time: Vec<GroupVec<u8>>,
    stages: Vec<Stage<'a>>,
    writes: Vec<GroupVec<SmallVec<[ResourceId; 10]>>>,
}

impl<'a> StagesBuilder<'a> {
    pub fn add_barrier(&mut self) {
        self.barrier = self.stages.len();
    }

    pub fn insert<T>(&mut self, mut dep: SmallVec<[SystemId; 4]>, id: SystemId, system: T)
    where
        T: for<'b> System<'b> + Send + 'a,
    {
        use system::SystemData;

        let mut reads = T::SystemData::reads(0);
        let writes = T::SystemData::writes(0);

        reads.sort();
        reads.dedup();

        let new_time = system.running_time();

        let target = self.insertion_target(&reads, &writes, &mut dep, new_time);

        let (stage, group) = match target {
            InsertionTarget::Stage(stage) => {
                let group = self.ids[stage].len();
                self.add_group(stage);

                (stage, group)
            }
            InsertionTarget::Group(stage, group) => (stage, group),
            InsertionTarget::NewStage => {
                let stage = self.stages.len();

                self.add_stage();
                self.add_group(stage);

                (stage, 0)
            }
        };

        self.ids[stage][group].push(id);
        self.reads[stage][group].extend(reads);
        self.running_time[stage][group] += new_time as u8;
        self.stages[stage].groups[group].push(Box::new(system));
        self.writes[stage][group].extend(writes);
    }

    pub fn build(self) -> Vec<Stage<'a>> {
        self.stages
    }

    fn add_stage(&mut self) {
        self.ids.push(GroupVec::new());
        self.reads.push(GroupVec::new());
        self.running_time.push(GroupVec::new());
        self.stages.push(Stage::new());
        self.writes.push(GroupVec::new());
    }

    fn add_group(&mut self, stage: usize) {
        self.ids[stage].push(ArrayVec::new());
        self.reads[stage].push(SmallVec::new());
        self.running_time[stage].push(0);
        self.stages[stage].groups.push(ArrayVec::new());
        self.writes[stage].push(SmallVec::new());
    }

    fn insertion_target<'rw, R, W>(
        &self,
        new_reads: R,
        new_writes: W,
        new_dep: &mut SmallVec<[SystemId; 4]>,
        new_time: RunningTime,
    ) -> InsertionTarget
    where
        R: IntoIterator<Item = &'rw ResourceId>,
        R::IntoIter: Clone,
        W: IntoIterator<Item = &'rw ResourceId>,
        W::IntoIter: Clone,
    {
        let new_reads = new_reads.into_iter();
        let new_writes = new_writes.into_iter();

        (self.barrier..self.stages.len())
            .map(|stage| {
                let conflict = Self::find_conflict(
                    &*self.ids,
                    &*self.reads,
                    &*self.writes,
                    stage,
                    new_reads.clone(),
                    new_writes.clone(),
                    new_dep,
                );
                self.remove_ids(stage, new_dep);
                (stage, conflict)
            })
            .find(|&(stage, conflict)| match conflict {
                Conflict::None => true,
                Conflict::Single(group) => {
                    self.stages[stage].groups[group].len() < MAX_SYSTEMS_PER_GROUP - 1 &&
                        self.improves_balance(stage, group, new_time as u8)
                }
                Conflict::Multiple => false,
            })
            .map(|(stage, conflict)| match conflict {
                Conflict::None => InsertionTarget::Stage(stage),
                Conflict::Single(group) => InsertionTarget::Group(stage, group),
                Conflict::Multiple => unreachable!(),
            })
            .unwrap_or(InsertionTarget::NewStage)
    }

    fn improves_balance(&self, stage: usize, group: usize, new_time: u8) -> bool {
        let max = *self.running_time[stage].iter().max().unwrap() as i8;
        let old_time = self.running_time[stage][group];
        let new_time = (old_time + new_time) as i8;

        // Check if adding the system to the group would
        // balance the stage better.

        (max - new_time).abs() < (max - old_time as i8).abs()
    }

    /// Returns an enum indicating which kind of conflict a system has
    /// with a stage.
    fn find_conflict<'rw, R, W>(
        ids: &[GroupVec<ArrayVec<[SystemId; MAX_SYSTEMS_PER_GROUP]>>],
        reads: &[GroupVec<SmallVec<[ResourceId; 12]>>],
        writes: &[GroupVec<SmallVec<[ResourceId; 10]>>],
        stage: usize,
        new_reads: R,
        new_writes: W,
        new_dep: &SmallVec<[SystemId; 4]>,
    ) -> Conflict
    where
        R: IntoIterator<Item = &'rw ResourceId>,
        R::IntoIter: Clone,
        W: IntoIterator<Item = &'rw ResourceId>,
        W::IntoIter: Clone,
    {
        let new_reads = new_reads.into_iter();
        let new_writes = new_writes.into_iter();

        let num_groups = ids[stage].len();
        let mut dep_conflict = false;

        let conflict = (0..num_groups)
            .filter(|&group| {
                let reads_and_writes = writes[stage][group]
                    .iter()
                    .chain(reads[stage][group].iter());

                let inters = check_intersection(new_writes.clone(), reads_and_writes) ||
                    check_intersection(new_reads.clone(), writes[stage][group].iter());

                if inters {
                    true
                } else {
                    if check_intersection(new_dep.iter(), ids[stage][group].iter()) {
                        dep_conflict = true;

                        true
                    } else {
                        false
                    }
                }
            })
            .fold(Conflict::None, Conflict::add);

        // If there is a dependency in the dependency list
        // which was not in a previous or this stage,
        // return `Multiple` conflict.

        if (dep_conflict && new_dep.len() > 1) || (!dep_conflict && !new_dep.is_empty()) {
            Conflict::Multiple
        } else {
            conflict
        }
    }

    /// Removes the ids of a given stage from the passed dependency list.
    fn remove_ids(&self, stage: usize, new_dep: &mut SmallVec<[SystemId; 4]>) {
        if !new_dep.is_empty() {
            for id in self.ids[stage].iter().flat_map(|id_group| id_group) {
                if let Some(index) = new_dep.iter().position(|x| *x == *id) {
                    new_dep.remove(index);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_ids(
        ids: &[&[&[usize]]],
    ) -> Vec<GroupVec<ArrayVec<[SystemId; MAX_SYSTEMS_PER_GROUP]>>> {
        ids.into_iter()
            .map(|groups| {
                groups
                    .into_iter()
                    .map(|systems| {
                        systems.into_iter().map(|id| SystemId(*id)).collect()
                    })
                    .collect()
            })
            .collect()
    }

    fn create_reads(reads: &[&[&[ResourceId]]]) -> Vec<GroupVec<SmallVec<[ResourceId; 12]>>> {
        reads
            .into_iter()
            .map(|groups| {
                groups
                    .into_iter()
                    .map(|reads| reads.into_iter().map(|id| *id).collect())
                    .collect()
            })
            .collect()
    }

    fn create_writes(writes: &[&[&[ResourceId]]]) -> Vec<GroupVec<SmallVec<[ResourceId; 10]>>> {
        writes
            .into_iter()
            .map(|groups| {
                groups
                    .into_iter()
                    .map(|writes| writes.into_iter().map(|id| *id).collect())
                    .collect()
            })
            .collect()
    }

    struct ResA;
    struct ResB;
    struct ResC;

    #[test]
    fn check_intersection_basic() {
        assert!(check_intersection((&[1, 5]).iter(), (&[2, 5]).iter()));
    }

    #[test]
    fn conflict_add() {
        assert_eq!(Conflict::add(Conflict::None, 45), Conflict::Single(45));
        assert_eq!(Conflict::add(Conflict::Single(3), 5), Conflict::Multiple);
    }

    #[test]
    fn conflict_rw() {
        let ids = create_ids(&[&[&[0], &[1]]]);
        let reads = create_reads(&[
            &[&[ResourceId::new::<ResA>()], &[ResourceId::new::<ResB>()]],
        ]);
        let writes = create_writes(&[&[&[], &[]]]);

        let conflict = StagesBuilder::find_conflict(
            &ids,
            &reads,
            &writes,
            0,
            &[],
            &[ResourceId::new::<ResB>()],
            &SmallVec::new(),
        );
        assert_eq!(conflict, Conflict::Single(1));
    }

    #[test]
    fn conflict_ww() {
        let ids = create_ids(&[&[&[0]]]);
        let reads = create_reads(&[&[&[ResourceId::new::<ResA>()]]]);
        let writes = create_writes(&[&[&[ResourceId::new::<ResB>()]]]);

        let conflict = StagesBuilder::find_conflict(
            &ids,
            &reads,
            &writes,
            0,
            &[],
            &[ResourceId::new::<ResB>()],
            &SmallVec::new(),
        );
        assert_eq!(conflict, Conflict::Single(0));
    }

    #[test]
    fn conflict_ww_multi() {
        let ids = create_ids(&[&[&[0], &[1]]]);
        let reads = create_reads(&[
            &[&[ResourceId::new::<ResA>(), ResourceId::new::<ResC>()], &[]],
        ]);
        let writes = create_writes(&[&[&[], &[ResourceId::new::<ResB>()]]]);

        let conflict = StagesBuilder::find_conflict(
            &ids,
            &reads,
            &writes,
            0,
            &[],
            &[ResourceId::new::<ResB>(), ResourceId::new::<ResC>()],
            &SmallVec::new(),
        );
        assert_eq!(conflict, Conflict::Multiple);
    }

    #[test]
    fn uses_group() {
        use res::{Fetch, FetchMut};

        struct SysA;

        impl<'a> System<'a> for SysA {
            type SystemData = Fetch<'a, ResA>;

            fn run(&mut self, _: Self::SystemData) {}
        }

        struct SysB;

        impl<'a> System<'a> for SysB {
            type SystemData = FetchMut<'a, ResB>;

            fn run(&mut self, _: Self::SystemData) {}

            fn running_time(&self) -> RunningTime {
                RunningTime::VeryShort
            }
        }

        struct SysC;

        impl<'a> System<'a> for SysC {
            type SystemData = Fetch<'a, ResB>;

            fn run(&mut self, _: Self::SystemData) {}

            fn running_time(&self) -> RunningTime {
                RunningTime::Short
            }
        }

        // SysA needs average time, SysB very short and SysC short.
        // To balance the stage SysA and SysB are in, we execute SysC
        // *after* SysB, so in the same group.

        let mut builder: StagesBuilder = Default::default();

        builder.insert(SmallVec::new(), SystemId(0), SysA);
        builder.insert(SmallVec::new(), SystemId(1), SysB);
        builder.insert(SmallVec::new(), SystemId(2), SysC);

        let ref ids = builder.ids[0];

        assert_eq!(ids[0][0], SystemId(0));
        assert_eq!(ids[1][0], SystemId(1));
        assert_eq!(ids[1][1], SystemId(2));
    }

    #[test]
    fn test_chained_dependency() {
        let mut builder: StagesBuilder = Default::default();

        struct Sys;

        impl<'a> System<'a> for Sys {
            type SystemData = ();

            fn run(&mut self, _: Self::SystemData) {}
        }

        builder.insert(SmallVec::from(&[][..]), SystemId(0), Sys);
        builder.insert(SmallVec::from(&[SystemId(0)][..]), SystemId(1), Sys);
        builder.insert(SmallVec::from(&[SystemId(1)][..]), SystemId(2), Sys);

        assert_eq!(builder.ids[0][0][0], SystemId(0));
        assert_eq!(builder.ids[1][0][0], SystemId(1));
        assert_eq!(builder.ids[2][0][0], SystemId(2));
    }
}
