use util::*;
use BitSetLike;

#[cfg(feature="parallel")]
pub use self::parallel::{BitParIter, BitProducer};

#[cfg(feature="parallel")]
mod parallel;

/// An `Iterator` over a [`BitSetLike`] structure.
///
/// [`BitSetLike`]: ../trait.BitSetLike.html
#[derive(Debug)]
pub struct BitIter<T> {
    set: T,
    masks: [usize; 4],
    prefix: [u32; 3],
}

impl<T> BitIter<T> {
    /// Creates a new `BitIter`. You usually don't call this function
    /// but just [`.iter()`] on a bit set.
    ///
    /// [`.iter()`]: ../trait.BitSetLike.html#method.iter
    pub fn new(set: T, masks: [usize; 4], prefix: [u32; 3]) -> Self {
        BitIter {
            set: set,
            masks: masks,
            prefix: prefix,
        }
    }
}

impl<T> Iterator for BitIter<T>
    where T: BitSetLike
{
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Look at first level
            if self.masks[0] != 0 {
                // Take first bit that isn't zero
                let bit = self.masks[0].trailing_zeros();
                // Remove it from masks
                self.masks[0] &= !(1 << bit);
                // and returns it's index
                return Some(self.prefix[0] | bit);
            }
            // Look at second level
            if self.masks[1] != 0 {
                // Take first bit that isn't zero
                let bit = self.masks[1].trailing_zeros();
                // Remove it from masks
                self.masks[1] &= !(1 << bit);
                // Calculate index of the bit in first level
                let idx = self.prefix[1] | bit;
                // Take corresponding usize from layer below
                self.masks[0] = self.set.layer0(idx as usize);
                // Prefix of the complete index
                self.prefix[0] = idx << BITS;
                continue;
            }
            // Look at third level
            if self.masks[2] != 0 {
                // Take first bit that isn't zero
                let bit = self.masks[2].trailing_zeros();
                // Remove it from masks
                self.masks[2] &= !(1 << bit);
                // Calculate index of the bit in second level
                let idx = self.prefix[2] | bit;
                // Take corresponding usize from layer below
                self.masks[1] = self.set.layer1(idx as usize);
                // Prefix of the index of the second level
                self.prefix[1] = idx << BITS;
                continue;
            }
            // Look at the 4th and highest level
            if self.masks[3] != 0 {
                // Take first bit that isn't zero
                let bit = self.masks[3].trailing_zeros();
                // Remove it from masks
                self.masks[3] &= !(1 << bit);
                // Take corresponding usize from layer below
                self.masks[2] = self.set.layer2(bit as usize);
                // Prefix of the index of the third level
                self.prefix[2] = bit << BITS;
                continue;
            }
            // There is no set indices left
            return None;
        }
    }
}
