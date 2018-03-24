#![cfg(test)]

use rand::{thread_rng, Rng};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::mem;
use super::ParallelSliceMut;

macro_rules! sort {
    ($f:ident, $name:ident) => {
        #[test]
        fn $name() {
            let mut rng = thread_rng();

            for len in (0..25).chain(500..501) {
                for &modulus in &[5, 10, 100] {
                    for _ in 0..100 {
                        let v: Vec<_> = rng.gen_iter::<i32>()
                            .map(|x| x % modulus)
                            .take(len)
                            .collect();

                        // Test sort using `<` operator.
                        let mut tmp = v.clone();
                        tmp.$f(|a, b| a.cmp(b));
                        assert!(tmp.windows(2).all(|w| w[0] <= w[1]));

                        // Test sort using `>` operator.
                        let mut tmp = v.clone();
                        tmp.$f(|a, b| b.cmp(a));
                        assert!(tmp.windows(2).all(|w| w[0] >= w[1]));
                    }
                }
            }

            // Test sort with many duplicates.
            for &len in &[1_000, 10_000, 100_000] {
                for &modulus in &[5, 10, 100, 10_000] {
                    let mut v: Vec<_> = rng.gen_iter::<i32>()
                        .map(|x| x % modulus)
                        .take(len)
                        .collect();

                    v.$f(|a, b| a.cmp(b));
                    assert!(v.windows(2).all(|w| w[0] <= w[1]));
                }
            }

            // Test sort with many pre-sorted runs.
            for &len in &[1_000, 10_000, 100_000] {
                for &modulus in &[5, 10, 1000, 50_000] {
                    let mut v: Vec<_> = rng.gen_iter::<i32>()
                        .map(|x| x % modulus)
                        .take(len)
                        .collect();

                    v.sort();
                    v.reverse();

                    for _ in 0..5 {
                        let a = rng.gen::<usize>() % len;
                        let b = rng.gen::<usize>() % len;
                        if a < b {
                            v[a..b].reverse();
                        } else {
                            v.swap(a, b);
                        }
                    }

                    v.$f(|a, b| a.cmp(b));
                    assert!(v.windows(2).all(|w| w[0] <= w[1]));
                }
            }

            // Sort using a completely random comparison function.
            // This will reorder the elements *somehow*, but won't panic.
            let mut v: Vec<_> = (0..100).collect();
            v.$f(|_, _| *thread_rng().choose(&[Less, Equal, Greater]).unwrap());
            v.$f(|a, b| a.cmp(b));
            for i in 0..v.len() {
                assert_eq!(v[i], i);
            }

            // Should not panic.
            [0i32; 0].$f(|a, b| a.cmp(b));
            [(); 10].$f(|a, b| a.cmp(b));
            [(); 100].$f(|a, b| a.cmp(b));

            let mut v = [0xDEADBEEFu64];
            v.$f(|a, b| a.cmp(b));
            assert!(v == [0xDEADBEEF]);
        }
    }
}

sort!(par_sort_by, test_par_sort);
sort!(par_sort_unstable_by, test_par_sort_unstable);

#[test]
fn test_par_sort_stability() {
    for len in (2..25).chain(500..510).chain(50_000..50_010) {
        for _ in 0..10 {
            let mut counts = [0; 10];

            // Create a vector like [(6, 1), (5, 1), (6, 2), ...],
            // where the first item of each tuple is random, but
            // the second item represents which occurrence of that
            // number this element is, i.e. the second elements
            // will occur in sorted order.
            let mut v: Vec<_> = (0..len)
                .map(|_| {
                    let n = thread_rng().gen::<usize>() % 10;
                    counts[n] += 1;
                    (n, counts[n])
                })
                .collect();

            // Only sort on the first element, so an unstable sort
            // may mix up the counts.
            v.par_sort_by(|&(a, _), &(b, _)| a.cmp(&b));

            // This comparison includes the count (the second item
            // of the tuple), so elements with equal first items
            // will need to be ordered with increasing
            // counts... i.e. exactly asserting that this sort is
            // stable.
            assert!(v.windows(2).all(|w| w[0] <= w[1]));
        }
    }
}
