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
use std::string::ToString;

// type usize = u32;

pub struct BitSet {
    /// list of blocks with data
    blocks: Vec<usize>,
    /// number of bits allowed to use
    size: usize,
}

// A couple of private functions
impl BitSet {
    /// Returns the number of blocks needed for the specified number of bits.
    /// There is always at least one bit, so at least one block is needed.
    fn blocks_number(size: usize) -> usize {
        let div = size / Self::block_size();
        let rem = size % Self::block_size();
        if rem == 0 {
            div
        } else {
            div + 1
        }
    }

    /// Returns the size of one block in bits.
    fn block_size() -> usize {
        size_of::<usize>() * 8
    }
}

// Constructors
impl BitSet {
    /// Creates a new BitSet with the given amount of allowed bits.
    ///
    /// Plenty of BitSet implementations allow for a zero length set.
    /// This is fine as long as this bitset is able to enlarge.
    /// I this case, the number of bits is set at the creation time,
    /// so a BitSet of zero length is useless, as you won't be able to
    /// do anything with that.
    ///
    /// This is the reason why there is no `default()` or `with_capacity()` functions
    /// and you should use `new()` instead.
    ///
    /// For the same reason, this function panics when you would try to create
    /// a BitSet with zero bits. This simiplifies the code in other places.
    ///
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Creating BitSet with zero bits is not allowed.");
        }
        let blocks_number = Self::blocks_number(size);
        let mut blocks = Vec::with_capacity(blocks_number);
        for _ in 0..blocks_number {
            blocks.push(0);
        }
        BitSet { blocks, size }
    }
}

// Basic functions
impl BitSet {
    /// Gets the bit from the position.
    ///
    pub fn get(&self, position: usize) -> bool {
        let bitmask: usize = 1 << position;
        let res: usize = self.blocks[0] & bitmask;
        res != 0
    }

    pub fn set(&mut self, position: usize, value: bool) {
        let block_number = position / Self::block_size();
        let bitmask: usize = 1 << (position % Self::block_size());
        if value == true {
            self.blocks[block_number] = self.blocks[block_number] | bitmask;
        } else {
            self.blocks[block_number] = self.blocks[block_number] & !bitmask;
        }
    }
}

impl From<u8> for BitSet {
    fn from(value: u8) -> Self {
        Self {
            blocks: vec![usize::from(value)],
            size: size_of::<u8>() * 8,
        }
    }
}

impl ToString for BitSet {
    fn to_string(&self) -> String {
        let mut res = String::with_capacity(Self::block_size() * self.blocks.len());
        let block_size = Self::block_size();
        for block in (&self.blocks).iter().rev() {
            res += &format!("{:0width$b}", block, width = block_size);
        }
        res[(res.len() - self.size)..].to_string()
    }
}

#[cfg(test)]
mod test_private_functions {

    use super::*;

    #[test]
    fn check_getting_number_of_blocks() {
        let block_size = size_of::<usize>() * 8;

        assert_eq!(BitSet::blocks_number(1), 1);
        assert_eq!(BitSet::blocks_number(10), 1);

        assert_eq!(BitSet::blocks_number(block_size), 1);
        assert_eq!(BitSet::blocks_number(block_size + 1), 2);

        assert_eq!(BitSet::blocks_number(2 * block_size), 2);
        assert_eq!(BitSet::blocks_number(2 * block_size + 1), 3);
    }

    #[test]
    fn check_getting_number_of_bits_in_block() {
        assert_eq!(BitSet::block_size(), size_of::<usize>() * 8);
    }
}

#[cfg(test)]
mod test_constructors {

    use super::*;

    #[test]
    #[should_panic(expected = "Creating BitSet with zero bits is not allowed.")]
    fn check_creating_bitset_with_zero_bits() {
        BitSet::new(0);
    }

    #[test]
    fn check_creating_new_bitset() {
        let block_size = size_of::<usize>() * 8;

        // for 1 bit we should have 1 block
        let a = BitSet::new(1);
        assert_eq!(a.blocks.len(), 1);
        assert_eq!(a.size, 1);

        // for max bits in one block, we should have one block
        let b = BitSet::new(block_size);
        assert_eq!(b.blocks.len(), 1);
        assert_eq!(b.size, block_size);

        // for 1 + max bits in one block, we should have two block
        let c = BitSet::new(block_size + 1);
        assert_eq!(c.blocks.len(), 2);
        assert_eq!(c.size, block_size + 1);
    }
}

#[cfg(test)]
mod test_conversions_to_types {

    use super::*;
    #[test]
    fn check_string_conversion() {
        let b = BitSet::new(1);
        assert_eq!(b.to_string(), "0");

        let mut c = BitSet::new(1);
        c.set(0, true);
        assert_eq!(c.to_string(), "1");

        let mut d = BitSet::new(66);
        d.set(0, true);
        d.set(5, true);
        d.set(64, true);
        println!("d |{}|", d.to_string());
        //                          60        50        40        30        20        10         0
        //                      543210987654321098765432109876543210987654321098765432109876543210
        let expected_d = "010000000000000000000000000000000000000000000000000000000000100001";
        assert_eq!(d.to_string(), expected_d);
    }
}

#[cfg(test)]
mod test_conversions_from_types {

    use super::*;
    #[test]
    fn check_conversion_from_u8_value() {
        let b = BitSet::from(0);
        assert_eq!(b.size, 8);
        assert_eq!(b.blocks.len(), 1);
        assert_eq!(b.to_string(), "00000000");

        let b = BitSet::from(u8::MAX);
        assert_eq!(b.size, 8);
        assert_eq!(b.blocks.len(), 1);
        assert_eq!(b.to_string(), "11111111");

        let b = BitSet::from(u8::from(170));
        assert_eq!(b.size, 8);
        assert_eq!(b.blocks.len(), 1);
        assert_eq!(b.to_string(), "10101010");
    }
}

#[cfg(test)]
mod test_basic_getter_and_setter {
    use super::*;

    #[test]
    fn check_simple_operations() {
        let mut b = BitSet::new(4);
        assert_eq!(b.get(0), false);
        assert_eq!(b.get(1), false);
        assert_eq!(b.get(2), false);
        assert_eq!(b.get(3), false);

        b.set(0, true);
        assert_eq!(b.get(0), true);
        assert_eq!(b.get(1), false);
        assert_eq!(b.get(2), false);
        assert_eq!(b.get(3), false);

        b.set(3, true);
        assert_eq!(b.get(0), true);
        assert_eq!(b.get(1), false);
        assert_eq!(b.get(2), false);
        assert_eq!(b.get(3), true);

        b.set(1, false);
        b.set(3, false);
        assert_eq!(b.get(0), true);
        assert_eq!(b.get(1), false);
        assert_eq!(b.get(2), false);
        assert_eq!(b.get(3), false);
        
    }
}
