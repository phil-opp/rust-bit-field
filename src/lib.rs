#![feature(zero_one)]
#![feature(const_fn)]
#![no_std]

#[cfg(test)]
mod tests;

use core::mem::size_of;

#[derive(Debug, Clone, Copy)]
pub struct BitField<T: Number>(T);

impl<T> BitField<T>
    where T: Number
{
    pub const fn new(value: T) -> BitField<T> {
        BitField(value)
    }

    pub fn get_bit<N>(&self, bit: N) -> bool where N: Into<u8> + Ord {
        let bit = bit.into();
        assert!(bit < self.length());
        self.get_range(bit..(bit + 1)) == T::one()
    }

    pub fn get_range<N>(&self, range: Range<N>) -> T where N: Into<u8> + Ord {
        let range = Range {
            start: range.start.into(),
            end: range.end.into(),
        };
        
        assert!(range.start < self.length());
        assert!(range.end <= self.length());
        assert!(range.start < range.end);

        // shift away high bits
        let bits = self.0 << (self.length() - range.end) >> (self.length() - range.end);

        // shift away low bits
        bits >> range.start
    }

    pub fn set_bit<N>(&mut self, bit: N) where N: Into<u8> + Ord {
        let bit = bit.into();
        assert!(bit < self.length());
        self.0 |= T::one() << bit;
    }

    pub fn reset_bit<N>(&mut self, bit: N) where N: Into<u8> + Ord {
        let bit = bit.into();
        assert!(bit < self.length());
        self.0 &= !(T::one() << bit);
    }

    pub fn set_range<N>(&mut self, range: Range<N>, value: T) where N: Into<u8> + Ord {
        let range = Range {
            start: range.start.into(),
            end: range.end.into(),
        };
        
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

    fn length(&self) -> u8 {
        size_of::<T>() as u8 * 8
    }
}

use core::ops::{Range, Shl, Shr, BitAnd, BitOr, BitOrAssign, BitAndAssign, Not};
use core::num::{Zero, One};
use core::fmt::Debug;
use core::cmp::Ord;

pub trait Number: Debug + Copy + Eq + Zero + One + Ord + 
    Not<Output=Self> + Shl<u8, Output=Self> + Shr<u8, Output=Self> +
    BitAnd<Self, Output=Self> + BitOr<Self, Output=Self>  + BitAndAssign + BitOrAssign {}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for usize {}
