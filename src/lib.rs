#![feature(zero_one)]
#![no_std]

#[cfg(test)]
mod tests;

use core::mem::size_of;

#[derive(Debug, Clone, Copy)]
pub struct BitField<T: Number>(T);

impl<T> BitField<T>
    where T: Number
{
    pub fn new(value: T) -> BitField<T> {
        BitField(value)
    }

    pub fn get_bit(&self, bit: usize) -> bool {
        assert!(bit < self.length());
        self.get_range(bit..(bit + 1)) == T::one()
    }

    pub fn get_range(&self, range: Range<usize>) -> T {
        assert!(range.start < self.length());
        assert!(range.end <= self.length());
        assert!(range.start < range.end);

        // shift away high bits
        let bits = self.0 << (self.length() - range.end) >> (self.length() - range.end);

        // shift away low bits
        bits >> range.start
    }

    pub fn set_bit(&mut self, bit: usize) {
        assert!(bit < self.length());
        self.0 |= T::one() << bit;
    }

    pub fn reset_bit(&mut self, bit: usize) {
        assert!(bit < self.length());
        self.0 &= !(T::one() << bit);
    }

    pub fn set_range(&mut self, range: Range<usize>, value: T) {
        assert!(range.start < self.length());
        assert!(range.end <= self.length());
        assert!(range.start < range.end);
        assert!(value << (self.length() - (range.end - range.start)) >> (self.length() - (range.end - range.start)) ==
                value,
                "value too big");

        let bitmask: T = !(!T::zero() << (self.length() - range.end) >> (self.length() - range.end) >> range.start <<
                           range.start);

        let bits = self.0 & bitmask;
        // set bits
        self.0 = bits | (value << range.start);
    }

    fn length(&self) -> usize {
        size_of::<T>() * 8
    }
}

use core::ops::{Range, Shl, Shr, BitAnd, BitOr, BitOrAssign, BitAndAssign, Not};
use core::num::{Zero, One};
use core::fmt::Debug;

pub trait Number: Debug + Copy + Eq + Zero + One +
    Not<Output=Self> + Shl<usize, Output=Self> + Shr<usize, Output=Self> +
    BitAnd<Self, Output=Self> + BitOr<Self, Output=Self>  + BitAndAssign + BitOrAssign {}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for usize {}
