
/// Type used for indexing.
pub type Index = u32;

/// Base two log of the number of bits in a usize.
#[cfg(target_pointer_width= "64")]
pub const BITS: usize = 6;
#[cfg(target_pointer_width= "32")]
pub const BITS: usize = 5;
/// Amount of layers in the hierarchical bitset.
pub const LAYERS: usize = 4;
pub const MAX: usize = BITS * LAYERS;
/// Maximum amount of bits per bitset.
pub const MAX_EID: usize = 2 << MAX - 1;

/// Layer0 shift (bottom layer, true bitset).
pub const SHIFT0: usize = 0;
/// Layer1 shift (third layer).
pub const SHIFT1: usize = SHIFT0 + BITS;
/// Layer2 shift (second layer).
pub const SHIFT2: usize = SHIFT1 + BITS;
/// Top layer shift.
pub const SHIFT3: usize = SHIFT2 + BITS;

pub trait Row: Sized + Copy {
    /// Location of the bit in the row.
    fn row(self, shift: usize) -> usize;

    /// Index of the row that the bit is in.
    fn offset(self, shift: usize) -> usize;

    /// Bitmask of the row the bit is in.
    #[inline(always)]
    fn mask(self, shift: usize) -> usize {
        1usize << self.row(shift)
    }
}

impl Row for Index {
    #[inline(always)]
    fn row(self, shift: usize) -> usize {
        ((self >> shift) as usize) & ((1 << BITS) - 1)
    }

    #[inline(always)]
    fn offset(self, shift: usize) -> usize {
        self as usize / (1 << shift)
    }
}

/// Helper method for getting parent offsets of 3 layers at once.
///
/// Returns them in (Layer0, Layer1, Layer2) order.
#[inline]
pub fn offsets(bit: Index) -> (usize, usize, usize) {
    (bit.offset(SHIFT1), bit.offset(SHIFT2), bit.offset(SHIFT3))
}
