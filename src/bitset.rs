extern crate num;
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, IndexMut, Not, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use num::traits::Unsigned;
use std::cmp::PartialEq;
use std::default::Default;
use std::mem::size_of;
use std::ops::Add;

trait UnsignedInt:
    Add<Output = Self>
    + Sized
    + From<u8>
    + BitAnd<Output = Self>
    + Shl<usize, Output = Self>
    + PartialEq
    + Copy
    + Not<Output = Self>
    + BitOr<Output = Self>
{
}

impl UnsignedInt for u8 {}
impl UnsignedInt for u16 {}
impl UnsignedInt for u32 {}
impl UnsignedInt for u64 {}
impl UnsignedInt for u128 {}

struct BitSet {
    data: Vec<usize>,
    size: usize,
}

/// A couple of private functions
impl BitSet {
    fn blocks_number(for_capacity: usize) -> usize {
        for_capacity % size_of::<usize>()
    }
}

/// Constructors
impl BitSet {
    /// Creates a new `BitSet` with all bits set to false.
    ///
    /// The size of the bitset depends on the size of the
    pub fn new() -> Self {
        Self::default()
    }

    fn default() -> Self {
        BitSet {
            data: Vec::new(),
            size: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        BitSet {
            data: Vec::with_capacity(Self::blocks_number(capacity)),
            size: 0,
        }
    }
}

impl BitSet {
    pub fn capacity(&self) -> usize {
        self.data.len() * size_of::<usize>() * 8
    }

    pub fn get(&self, i: usize) -> bool {
        let bitmask: usize = 1 << i;
        let res: usize = self.data[0] & bitmask;
        res != 0
    }

    pub fn set(&mut self, i: usize, value: bool) {
        let bitmask: usize = 1 << i;
        if value == true {
            self.data[0] = self.data[0] | bitmask;
        } else {
            self.data[0] = self.data[0] & !bitmask;
        }
    }
}

impl From<u8> for BitSet {
    fn from(value: u8) -> Self {
        Self {
            data: vec![usize::from(value)],
            size: size_of::<usize>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_creating_empty_bitset() {
        let b = BitSet::new();
        assert_eq!(b.capacity(), 0);
        assert_eq!(b.size, 0);
    }

    #[test]
    fn check_creating_from_u8_value() {
        let b = BitSet::from(u8::MIN);
        assert_eq!(b.capacity(), size_of::<usize>() * 8);
        assert_eq!(b.size, 8);

        let b = BitSet::from(u8::MAX);
        assert_eq!(b.capacity(), size_of::<usize>() * 8);
        assert_eq!(b.size, 8);
    }

    #[test]
    fn check_getting_bits_for_small_set() {
        // 12 = 001100
        let b = BitSet::from(u8::from(12));
        assert_eq!(b.get(0), false);
        assert_eq!(b.get(1), false);
        assert_eq!(b.get(2), true);
        assert_eq!(b.get(3), true);
        assert_eq!(b.get(4), false);
    }

    #[test]
    fn check_setting_bits_for_small_set() {
        // 12 = 001100
        let mut b = BitSet::from(u8::from(12));
        assert_eq!(b.get(0), false);
        b.set(0, true);
        assert_eq!(b.get(0), true);

        b.set(1, true);
        assert_eq!(b.get(1), true);
        b.set(1, false);
        assert_eq!(b.get(1), false);
    }
}
