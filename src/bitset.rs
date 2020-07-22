//! There is no From<u8> or any other types.
//! The problem is that it's possible that the conversion is not doable
//! and according to the documentation, the From trait cannot fail.
//!
//! Use TryFrom<> inst

extern crate num;
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, IndexMut, Not, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use num::traits::Unsigned;
use std::cmp::max;
use std::cmp::PartialEq;
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::default::Default;
use std::fmt;
use std::mem::size_of;
use std::ops::Add;
use std::string::ToString;

#[cfg(test)]
extern crate quickcheck;

/// A simple placeholder for calculating the place where a bit is stored.
///
struct BitPosition {
    block_number: usize,
    block_position: usize,
}

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

    /// Panic if the passed position argument is outside the range [0; self.size)
    fn assert_position(&self, position: usize) {
        if position >= self.size {
            panic!(format!(
                "Bit position [{}] is outside available range: [0, {}]",
                position,
                self.size - 1
            ));
        }
    }

    /// Finds the bit position and block number
    fn get_bit_position(position: usize) -> BitPosition {
        BitPosition {
            block_number: position / Self::block_size(),
            block_position: position % Self::block_size(),
        }
    }

    /// Calculates the bitmask with just the one bit set.
    fn make_bitmask(position: usize) -> usize {
        1 << position
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
    /// Panics:
    ///    - when size=0
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
    /// Panics:
    ///    - if the position is larger than the max bit number (which is size-1)
    ///
    pub fn get(&self, position: usize) -> bool {
        self.assert_position(position);

        let bit_position = Self::get_bit_position(position);
        let bitmask = Self::make_bitmask(bit_position.block_position);

        self.blocks[bit_position.block_number] & bitmask != 0
    }

    /// Sets the bit value at the position.Add
    ///
    /// Panics:
    ///    - if the position is larger than the max bit number (which is size-1)
    ///
    pub fn set(&mut self, position: usize, value: bool) {
        self.assert_position(position);

        let bit_position = Self::get_bit_position(position);
        let bitmask = Self::make_bitmask(bit_position.block_position);

        if value == true {
            self.blocks[bit_position.block_number] =
                self.blocks[bit_position.block_number] | bitmask;
        } else {
            self.blocks[bit_position.block_number] =
                self.blocks[bit_position.block_number] & !bitmask;
        }
    }
}

// utility functions
impl BitSet {
    /// Returns true if all bits are set. False if any is not set.
    fn all(&self) -> bool {
        for block in &self.blocks {
            if *block != usize::MAX {
                return false;
            }
        }
        return true;
    }

    /// Returns true if all none bit is set. False if all are not set.
    fn any(&self) -> bool {
        for block in &self.blocks {
            if *block != 0 {
                return true;
            }
        }
        return false;
    }

    /// Returns number of bits set to true.
    fn count(&self) -> u32 {
        let mut res = 0;
        for block in &self.blocks {
            res += block.count_ones();
        }
        res
    }
}

macro_rules! add_from_uint_trait {
    ($t:ty) => {
        impl From<$t> for BitSet {
            fn from(value: $t) -> Self {
                // number of bytes we need in memory for the value
                let required_size = size_of::<$t>();
                // number of blocks needed for the values
                let blocks_number = Self::blocks_number(required_size);

                // now we need to slice the blocks in groups as every block
                // contains a couple of bytes (depending on the machine)
                let bytes_per_block = size_of::<usize>();

                // if we need more blocks, then we need to convert the bits
                // we store Little Endian in the list of blocks
                let value_bytes = value.to_be_bytes();

                let mut blocks: Vec<usize> = Vec::with_capacity(blocks_number);

                // it is possible that we convert e.g. u16 -> usize(u64)
                // in this case, there are only 2 bytes and the compiler is not happy about
                // so, let's add some more bytes

                println! {"value {}", value}
                for chunk in value_bytes.chunks(bytes_per_block) {
                    let mut block: usize = 0;
                    for byte in chunk {
                        block = block << 8;
                        block = block | usize::from(*byte);
                    }
                    blocks.push(block);
                }

                blocks.reverse();

                Self {
                    blocks: blocks,
                    size: size_of::<$t>() * 8,
                }
            }
        }
    };
}

impl From<u8> for BitSet {
    fn from(value: u8) -> Self {
        // This implementation is simple. I don't care about the number of blocks here,
        // as it's impossible to have something smaller than u8.
        Self {
            blocks: vec![usize::from(value)],
            size: size_of::<u8>() * 8,
        }
    }
}

add_from_uint_trait! {u16}
add_from_uint_trait! {u32}
add_from_uint_trait! {u64}
add_from_uint_trait! {u128}
add_from_uint_trait! {usize}

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

macro_rules! add_try_from_uint_trait {
    ($t:ty) => {
        impl TryFrom<BitSet> for $t {
            type Error = &'static str;

            fn try_from(value: BitSet) -> Result<Self, Self::Error> {
                let block_size = size_of::<usize>();
                let output_bytes = size_of::<$t>();
                let blocks_needed: usize = max(output_bytes / block_size, 1);

                // When the type is smaller than usize
                if blocks_needed == 1 {
                    match <$t>::try_from(value.blocks[0]) {
                        Ok(number) => return Ok(number),
                        Err(_) => return Err("Value stored in BitSet cannot be converted to u8."),
                    };
                }

                // The type is bigger than usize, e.g. u128 on 64bit machine,
                // this way we need to have at least two blocks, and conversion
                // should be fine.

                // if any of the not used blocks contains any bit set, then the conversion is not doable
                for block in &value.blocks[blocks_needed - 1..] {
                    if *block != 0 {
                        return Err("Value stored in BitSet cannot be converted to u8.");
                    }
                }

                let mut output: $t = 0;
                for block in &value.blocks[0..blocks_needed - 1] {
                    output = output << BitSet::block_size();
                    output = output | (*block) as $t;
                }
                return Ok(output);
            }
        }
    };
}

add_try_from_uint_trait! {u8}
add_try_from_uint_trait! {u16}
add_try_from_uint_trait! {u32}
add_try_from_uint_trait! {u64}
add_try_from_uint_trait! {u128}
add_try_from_uint_trait! {usize}

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
#[macro_use]
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
        //                          60        50        40        30        20        10         0
        //                      543210987654321098765432109876543210987654321098765432109876543210
        let expected_d = "010000000000000000000000000000000000000000000000000000000000100001";
        assert_eq!(d.to_string(), expected_d);
    }

    /// Checks conversion from different values
    macro_rules! check_type_conversion {
        ($func:ident, $from:ty, $to:ty) => {
            #[quickcheck]
            fn $func(value: $from) -> bool {
                let bitset = BitSet::from(value);

                let new_value = <$to>::try_from(bitset);
                if value as $from <= <$to>::MAX as $from {
                    // conversion should be fine
                    assert_eq!(value as $to, new_value.unwrap());
                } else {
                    // in this case the value is too large
                    assert_eq!(
                        new_value,
                        Err("Value stored in BitSet cannot be converted to u8.")
                    );
                }
                true
            }
        };
    }

    check_type_conversion! {check_conversion_from_u8_to_u8, u8, u8}
    check_type_conversion! {check_conversion_from_u16_to_u8, u16, u8}
    check_type_conversion! {check_conversion_from_u32_to_u8, u32, u8}
    check_type_conversion! {check_conversion_from_u64_to_u8, u64, u8}
    check_type_conversion! {check_conversion_from_u128_to_u8, u128, u8}
    check_type_conversion! {check_conversion_from_usize_to_u8, usize, u8}

    check_type_conversion! {check_conversion_from_u8_to_u16, u8, u16}
    check_type_conversion! {check_conversion_from_u16_to_u16, u16, u16}
    check_type_conversion! {check_conversion_from_u32_to_u16, u32, u16}
    check_type_conversion! {check_conversion_from_u64_to_u16, u64, u16}
    check_type_conversion! {check_conversion_from_u128_to_u16, u128, u16}
    check_type_conversion! {check_conversion_from_usize_to_u16, usize, u16}

    check_type_conversion! {check_conversion_from_u8_to_u32, u8, u32}
    check_type_conversion! {check_conversion_from_u16_to_u32, u16, u32}
    check_type_conversion! {check_conversion_from_u32_to_u32, u32, u32}
    check_type_conversion! {check_conversion_from_u64_to_u32, u64, u32}
    check_type_conversion! {check_conversion_from_u128_to_u32, u128, u32}
    check_type_conversion! {check_conversion_from_usize_to_u32, usize, u32}

    check_type_conversion! {check_conversion_from_u8_to_u64, u8, u64}
    check_type_conversion! {check_conversion_from_u16_to_u64, u16, u64}
    check_type_conversion! {check_conversion_from_u32_to_u64, u32, u64}
    check_type_conversion! {check_conversion_from_u64_to_u64, u64, u64}
    check_type_conversion! {check_conversion_from_u128_to_u64, u128, u64}
    check_type_conversion! {check_conversion_from_usize_to_u64, usize, u64}

    check_type_conversion! {check_conversion_from_u8_to_u128, u8, u128}
    check_type_conversion! {check_conversion_from_u16_to_u128, u16, u128}
    check_type_conversion! {check_conversion_from_u32_to_u128, u32, u128}
    check_type_conversion! {check_conversion_from_u64_to_u128, u64, u128}
    check_type_conversion! {check_conversion_from_u128_to_u128, u128, u128}
    check_type_conversion! {check_conversion_from_usize_to_u128, usize, u128}

    check_type_conversion! {check_conversion_from_u8_to_usize, u8, usize}
    check_type_conversion! {check_conversion_from_u16_to_usize, u16, usize}
    check_type_conversion! {check_conversion_from_u32_to_usize, u32, usize}
    check_type_conversion! {check_conversion_from_u64_to_usize, u64, usize}
    check_type_conversion! {check_conversion_from_u128_to_usize, u128, usize}
    check_type_conversion! {check_conversion_from_usize_to_usize, usize, usize}

    #[quickcheck]
    fn check_conversion_to_sssu8(value: u16) -> bool {
        let bitset = BitSet::from(value);

        let new_value = u8::try_from(bitset);
        if value as u16 <= u8::MAX as u16 {
            // conversion should be fine
            assert_eq!(value as u8, new_value.unwrap());
        } else {
            // in this case the value is too large
            assert_eq!(
                new_value,
                Err("Value stored in BitSet cannot be converted to u8.")
            );
        }
        true
    }
}

#[cfg(test)]
#[macro_use]
mod test_conversions_from_types {

    use super::*;

    /// Checks conversion from different values
    macro_rules! check_type_conversion {
        ($func:ident, $t:ty) => {
            #[quickcheck]
            fn $func(value: $t) -> bool {
                let bitset = BitSet::from(value);
                let value_bits_count = size_of::<$t>() * 8;

                // the bitset should have the same number of bits as the initial value
                assert_eq!(bitset.size, value_bits_count);

                // converting both to a string should give the same result
                let value_bits = format!("{:0width$b}", value, width = value_bits_count);
                let bitset_bits = bitset.to_string();
                assert_eq! {value_bits, bitset_bits}

                // also, check bit by bit that everything is the same
                for bit in 0..value_bits_count - 1 {
                    let bit_from_value: bool = (value & (1 << bit) != 0);
                    assert_eq! {bit_from_value, bitset.get(bit)}
                }
                true
            }
        };
    }

    #[test]
    fn check_conversion_from_u8_value() {
        let b = BitSet::from(0 as u8);
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

    // // Test converting from different values;
    check_type_conversion! {check_conversion_from_u8, u8}
    check_type_conversion! {check_conversion_from_u16, u16}
    check_type_conversion! {check_conversion_from_u32, u32}
    check_type_conversion! {check_conversion_from_u64, u64}
    check_type_conversion! {check_conversion_from_u128, u128}
    check_type_conversion! {check_conversion_from_usize, usize}
}

#[cfg(test)]
mod test_basic_getter_and_setter {
    use super::*;

    #[test]
    #[should_panic(expected = "Bit position [256] is outside available range: [0, 255]")]
    fn check_using_setter_for_too_large_position() {
        let mut b = BitSet::new(256);
        b.set(256, false);
    }

    #[test]
    #[should_panic(expected = "Bit position [256] is outside available range: [0, 255]")]
    fn check_using_getter_for_too_large_position() {
        let b = BitSet::new(256);
        b.get(256);
    }

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

#[cfg(test)]
mod test_utitily_functions {

    use super::*;

    #[test]
    fn check_all_function() {
        let mut b = BitSet::new(300);
        assert_eq! {b.all(), false}
        b.set(10, true);
        assert_eq! {b.all(), false}

        let mut b = BitSet::from(u128::MAX);
        assert_eq! {b.all(), true}
        b.set(10, false);
        assert_eq! {b.all(), false}
    }

    #[test]
    fn check_any_function() {
        let mut b = BitSet::new(300);
        assert_eq! {b.any(), false}
        b.set(10, true);
        assert_eq! {b.any(), true}

        let mut b = BitSet::from(u128::MAX);
        assert_eq! {b.any(), true}
        b.set(10, false);
        assert_eq! {b.any(), true}
    }

    #[test]
    fn check_count_function() {
        let mut b = BitSet::new(300);
        assert_eq! {b.count(), 0}
        b.set(10, true);
        assert_eq! {b.count(), 1}

        let mut b = BitSet::from(u128::MAX);
        assert_eq! {b.count(), 128}
        b.set(10, false);
        assert_eq! {b.count(), 127}
    }
}
