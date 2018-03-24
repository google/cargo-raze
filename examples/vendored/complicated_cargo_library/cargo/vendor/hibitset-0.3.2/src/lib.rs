//! # hibitset
//!
//! Provides hierarchical bit sets,
//! which allow very fast iteration
//! on sparse data structures.

#![deny(missing_docs)]

extern crate atom;
#[cfg(feature="parallel")]
extern crate rayon;
#[cfg(test)]
extern crate rand;

mod atomic;
mod iter;
mod ops;
mod util;

pub use atomic::AtomicBitSet;
pub use iter::BitIter;
#[cfg(feature="parallel")]
pub use iter::{BitParIter, BitProducer};
pub use ops::{BitSetAnd, BitSetNot, BitSetOr, BitSetXor};

use util::*;

/// A `BitSet` is a simple set designed to track entity indices for which
/// a certain component exists. It does not track the `Generation` of the
/// entities that it contains.
///
/// Note, a `BitSet` is limited by design to only `1,048,576` indices.
/// Adding beyond this limit will cause the `BitSet` to panic.
#[derive(Clone, Debug, Default)]
pub struct BitSet {
    layer3: usize,
    layer2: Vec<usize>,
    layer1: Vec<usize>,
    layer0: Vec<usize>,
}

impl BitSet {
    /// Creates an empty `BitSet`.
    pub fn new() -> BitSet {
        Default::default()
    }

    #[inline]
    fn valid_range(max: Index) {
        if (MAX_EID as u32) < max {
            panic!("Expected index to be less then {}, found {}", MAX_EID, max);
        }
    }

    /// Creates an empty `BitSet`, preallocated for up to `max` indices.
    pub fn with_capacity(max: Index) -> BitSet {
        Self::valid_range(max);
        let mut value = BitSet::new();
        value.extend(max);
        value
    }

    #[inline(never)]
    fn extend(&mut self, id: Index) {
        Self::valid_range(id);
        let (p0, p1, p2) = offsets(id);

        Self::fill_up(&mut self.layer2, p2);
        Self::fill_up(&mut self.layer1, p1);
        Self::fill_up(&mut self.layer0, p0);
    }

    fn fill_up(vec: &mut Vec<usize>, upper_index: usize) {
        if vec.len() <= upper_index {
            vec.resize(upper_index + 1, 0);
        }
    }

    /// This is used to set the levels in the hierarchy
    /// when the lowest layer was set from 0.
    #[inline(never)]
    fn add_slow(&mut self, id: Index) {
        let (_, p1, p2) = offsets(id);
        self.layer1[p1] |= id.mask(SHIFT1);
        self.layer2[p2] |= id.mask(SHIFT2);
        self.layer3 |= id.mask(SHIFT3);
    }

    /// Adds `id` to the `BitSet`. Returns `true` if the value was
    /// already in the set.
    #[inline]
    pub fn add(&mut self, id: Index) -> bool {
        let (p0, mask) = (id.offset(SHIFT1), id.mask(SHIFT0));

        if p0 >= self.layer0.len() {
            self.extend(id);
        }

        if self.layer0[p0] & mask != 0 {
            return true;
        }

        // we need to set the bit on every layer to indicate
        // that the value can be found here.
        let old = self.layer0[p0];
        self.layer0[p0] |= mask;
        if old == 0 {
            self.add_slow(id);
        } else {
            self.layer0[p0] |= mask;
        }
        false
    }

    /// Removes `id` from the set, returns `true` if the value
    /// was removed, and `false` if the value was not set
    /// to begin with.
    #[inline]
    pub fn remove(&mut self, id: Index) -> bool {
        let (p0, p1, p2) = offsets(id);

        if p0 >= self.layer0.len() {
            return false;
        }

        if self.layer0[p0] & id.mask(SHIFT0) == 0 {
            return false;
        }

        // if the bitmask was set we need to clear
        // its bit from layer0 to 3. the layers abover only
        // should be cleared if the bit cleared was the last bit
        // in its set
        self.layer0[p0] &= !id.mask(SHIFT0);
        if self.layer0[p0] != 0 {
            return true;
        }

        self.layer1[p1] &= !id.mask(SHIFT1);
        if self.layer1[p1] != 0 {
            return true;
        }

        self.layer2[p2] &= !id.mask(SHIFT2);
        if self.layer2[p2] != 0 {
            return true;
        }

        self.layer3 &= !id.mask(SHIFT3);
        return true;
    }

    /// Returns `true` if `id` is in the set.
    #[inline]
    pub fn contains(&self, id: Index) -> bool {
        let p0 = id.offset(SHIFT1);
        p0 < self.layer0.len() && (self.layer0[p0] & id.mask(SHIFT0)) != 0
    }

    /// Completely wipes out the bit set.
    pub fn clear(&mut self) {
        self.layer0.clear();
        self.layer1.clear();
        self.layer2.clear();
        self.layer3 = 0;
    }
}

/// A generic interface for [`BitSetLike`]-like types.
///
/// Every `BitSetLike` is hierarchical, meaning that there
/// are multiple levels that branch out in a tree like structure.
///
/// Layer0 each bit represents one Index of the set
/// Layer1 each bit represents one `usize` of Layer0, and will be
/// set only if the word below it is not zero.
/// Layer2 has the same arrangement but with Layer1, and Layer3 with Layer2.
///
/// This arrangement allows for rapid jumps across the key-space.
///
/// [`BitSetLike`]: ../trait.BitSetLike.html
pub trait BitSetLike {
    /// Return a usize where each bit represents if any word in layer2
    /// has been set.
    fn layer3(&self) -> usize;

    /// Return the usize from the array of usizes that indicates if any
    /// bit has been set in layer1
    fn layer2(&self, i: usize) -> usize;

    /// Return the usize from the array of usizes that indicates if any
    /// bit has been set in layer0
    fn layer1(&self, i: usize) -> usize;

    /// Return a usize that maps to the direct 1:1 association with
    /// each index of the set
    fn layer0(&self, i: usize) -> usize;

    /// Create an iterator that will scan over the keyspace
    fn iter(self) -> BitIter<Self>
        where Self: Sized
    {
        let layer3 = self.layer3();

        BitIter::new(self, [0, 0, 0, layer3], [0; 3])
    }

    /// Create a parallel iterator that will scan over the keyspace
    #[cfg(feature="parallel")]
    fn par_iter(self) -> BitParIter<Self>
        where Self: Sized
    {
        BitParIter::new(self)
    }
}

impl<'a, T> BitSetLike for &'a T
    where T: BitSetLike
{
    #[inline]
    fn layer3(&self) -> usize {
        (*self).layer3()
    }

    #[inline]
    fn layer2(&self, i: usize) -> usize {
        (*self).layer2(i)
    }

    #[inline]
    fn layer1(&self, i: usize) -> usize {
        (*self).layer1(i)
    }

    #[inline]
    fn layer0(&self, i: usize) -> usize {
        (*self).layer0(i)
    }
}

impl BitSetLike for BitSet {
    #[inline]
    fn layer3(&self) -> usize {
        self.layer3
    }

    #[inline]
    fn layer2(&self, i: usize) -> usize {
        self.layer2.get(i).map(|&x| x).unwrap_or(0)
    }

    #[inline]
    fn layer1(&self, i: usize) -> usize {
        self.layer1.get(i).map(|&x| x).unwrap_or(0)
    }

    #[inline]
    fn layer0(&self, i: usize) -> usize {
        self.layer0.get(i).map(|&x| x).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::{BitSet, BitSetAnd, BitSetNot, BitSetLike};

    #[test]
    fn insert() {
        let mut c = BitSet::new();
        for i in 0..1_000 {
            assert!(!c.add(i));
            assert!(c.add(i));
        }

        for i in 0..1_000 {
            assert!(c.contains(i));
        }
    }

    #[test]
    fn insert_100k() {
        let mut c = BitSet::new();
        for i in 0..100_000 {
            assert!(!c.add(i));
            assert!(c.add(i));
        }

        for i in 0..100_000 {
            assert!(c.contains(i));
        }
    }
    #[test]
    fn remove() {
        let mut c = BitSet::new();
        for i in 0..1_000 {
            assert!(!c.add(i));
        }

        for i in 0..1_000 {
            assert!(c.contains(i));
            assert!(c.remove(i));
            assert!(!c.contains(i));
            assert!(!c.remove(i));
        }
    }

    #[test]
    fn iter() {
        let mut c = BitSet::new();
        for i in 0..100_000 {
            c.add(i);
        }

        let mut count = 0;
        for (idx, i) in c.iter().enumerate() {
            count += 1;
            assert_eq!(idx, i as usize);
        }
        assert_eq!(count, 100_000);
    }

    #[test]
    fn iter_odd_even() {
        let mut odd = BitSet::new();
        let mut even = BitSet::new();
        for i in 0..100_000 {
            if i % 2 == 1 {
                odd.add(i);
            } else {
                even.add(i);
            }
        }

        assert_eq!((&odd).iter().count(), 50_000);
        assert_eq!((&even).iter().count(), 50_000);
        assert_eq!(BitSetAnd(&odd, &even).iter().count(), 0);
    }

    #[test]
    fn iter_random_add() {
        use rand::{Rng, weak_rng};
        let mut set = BitSet::new();
        let mut rng = weak_rng();
        let max_added = 1_048_576 / 10;
        let mut added = 0;
        for _ in 0..max_added {
            let index = rng.gen_range(0, max_added);
            if !set.add(index) {
                added += 1;
            }
        }
        assert_eq!(set.iter().count(), added as usize);
    }

    #[test]
    fn iter_clusters() {
        let mut set = BitSet::new();
        for x in 0..8 {
            let x = (x * 3) << (::BITS * 2); // scale to the last slot
            for y in 0..8 {
                let y = (y * 3) << (::BITS);
                for z in 0..8 {
                    let z = z * 2;
                    set.add(x + y + z);
                }
            }
        }
        assert_eq!(set.iter().count(), 8usize.pow(3));
    }

    #[test]
    fn not() {
        let mut c = BitSet::new();
        for i in 0..10_000 {
            if i % 2 == 1 {
                c.add(i);
            }
        }
        let d = BitSetNot(c);
        for (idx, i) in d.iter().take(5_000).enumerate() {
            assert_eq!(idx * 2, i as usize);
        }
    }
}

#[cfg(all(test, feature="parallel"))]
mod test_parallel {
    use super::{BitSet, BitSetAnd, BitSetLike};
    use rayon::iter::ParallelIterator;

    #[test]
    fn par_iter_one() {
        let step = 5000;
        let tests = 1_048_576 / step;
        for n in 0..tests {
            let n = n * step;
            let mut set = BitSet::new();
            set.add(n);
            assert_eq!(set.par_iter().count(), 1);
        }
        let mut set = BitSet::new();
        set.add(1_048_576 - 1);
        assert_eq!(set.par_iter().count(), 1);
    }

    #[test]
    fn par_iter_random_add() {
        use rand::{Rng, weak_rng};
        use std::collections::HashSet;
        use std::sync::{Arc, Mutex};
        let mut set = BitSet::new();
        let mut check_set = HashSet::new();
        let mut rng = weak_rng();
        let max_added = 1_048_576 / 10;
        for _ in 0..max_added {
            let index = rng.gen_range(0, max_added);
            set.add(index);
            check_set.insert(index);
        }
        let check_set = Arc::new(Mutex::new(check_set));
        let missing_set = Arc::new(Mutex::new(HashSet::new()));
        set.par_iter()
            .for_each(|n| {
                let check_set = check_set.clone();
                let missing_set = missing_set.clone();
                let mut check = check_set.lock().unwrap();
                if !check.remove(&n) {
                    let mut missing = missing_set.lock().unwrap();
                    missing.insert(n);
                }
            });
        let check_set = check_set.lock().unwrap();
        let missing_set = missing_set.lock().unwrap();
        if !check_set.is_empty() && !missing_set.is_empty() {
            panic!("There were values that didn't get iterated: {:?}
            There were values that got iterated, but that shouldn't be: {:?}", *check_set, *missing_set);
        }
        if !check_set.is_empty() {
            panic!("There were values that didn't get iterated: {:?}", *check_set);
        }
        if !missing_set.is_empty() {
            panic!("There were values that got iterated, but that shouldn't be: {:?}", *missing_set);
        }
    }

    #[test]
    fn par_iter_odd_even() {
        let mut odd = BitSet::new();
        let mut even = BitSet::new();
        for i in 0..100_000 {
            if i % 2 == 1 {
                odd.add(i);
            } else {
                even.add(i);
            }
        }

        assert_eq!((&odd).par_iter().count(), 50_000);
        assert_eq!((&even).par_iter().count(), 50_000);
        assert_eq!(BitSetAnd(&odd, &even).par_iter().count(), 0);
    }

    #[test]
    fn par_iter_clusters() {
        use std::collections::HashSet;
        use std::sync::{Arc, Mutex};
        let mut set = BitSet::new();
        let mut check_set = HashSet::new();
        for x in 0..8 {
            let x = (x * 3) << (::BITS * 2); // scale to the last slot
            for y in 0..8 {
                let y = (y * 3) << (::BITS);
                for z in 0..8 {
                    let z = z * 2;
                    let index = x + y + z;
                    set.add(index);
                    check_set.insert(index);
                }
            }
        }
        let check_set = Arc::new(Mutex::new(check_set));
        let missing_set = Arc::new(Mutex::new(HashSet::new()));
        set.par_iter()
            .for_each(|n| {
                let check_set = check_set.clone();
                let missing_set = missing_set.clone();
                let mut check = check_set.lock().unwrap();
                if !check.remove(&n) {
                    let mut missing = missing_set.lock().unwrap();
                    missing.insert(n);
                }
            });
        let check_set = check_set.lock().unwrap();
        let missing_set = missing_set.lock().unwrap();
        if !check_set.is_empty() && !missing_set.is_empty() {
            panic!("There were values that didn't get iterated: {:?}
            There were values that got iterated, but that shouldn't be: {:?}", *check_set, *missing_set);
        }
        if !check_set.is_empty() {
            panic!("There were values that didn't get iterated: {:?}", *check_set);
        }
        if !missing_set.is_empty() {
            panic!("There were values that got iterated, but that shouldn't be: {:?}", *missing_set);
        }
    }
}
