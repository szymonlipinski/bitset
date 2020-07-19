use std::convert::{From, TryFrom};

use core::cmp::{Eq, Ord};
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, IndexMut, Not, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use std::fmt::{Debug, Display};
use std::iter::FromIterator;

use core::default::Default;
use core::hash::Hash;

trait TBitSet:
    Clone
    + Iterator
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Index<usize>
    + IndexMut<usize>
    + Not
    + Shl
    + ShlAssign
    + Shr
    + ShrAssign
    + Sub
    + SubAssign
    + From<String>
    + TryFrom<String>
    + From<u32>
    + TryFrom<u32>
    + From<u64>
    + TryFrom<u64>
    + From<u32>
    + TryFrom<u32>
    + From<Vec<u8>>
    + TryFrom<Vec<u8>>
    + Display
    + Debug
    + Default
    + Hash
    + FromIterator<bool>
    + Eq
    + Ord
{
    fn new() -> Self;

    fn get(&self, i: usize) -> Option<bool>;
    fn set(&mut self, i: usize, value: bool) -> Option<bool>;
    fn set_all(&mut self, value: bool);
    fn set_all_range(&mut self, from: usize, to: usize, value: bool);

    fn negate(&mut self);

    fn union(&mut self, other: &Self);
    fn intersect(&mut self, other: &Self);
    fn difference(&mut self, other: &Self);

    fn intersects(&self, other: &Self) -> bool;
    fn contains(&self, other: &Self) -> bool;
    fn is_disjoint(&self, other: &Self) -> bool;
    fn is_subset(&self, other: &Self) -> bool;
    fn is_superset(&self, other: &Self) -> bool;

    fn find_first_set(&self) -> usize;
    fn find_last_set(&self) -> usize;
    fn count(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn all(&self) -> bool;
    fn any(&self) -> bool;

    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
}

trait Resizeable {
    fn append(&mut self, other: &Self);
    fn truncate(&mut self, to_size: usize);
    fn resize(&mut self, to_size: usize);
    fn capacity(&self) -> usize;
    fn shrink_to_fit(&mut self);
}
