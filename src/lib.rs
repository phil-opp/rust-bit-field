#![feature(const_fn)]
#![no_std]

#[cfg(test)]
mod tests;

use core::ops::{Range, Shl, Shr, BitAndAssign, BitOrAssign, Not, BitAnd, BitOr};
use core::mem::size_of;

pub trait BitField: Copy + Eq + Not<Output = Self> +
    Shl<u8, Output = Self> + Shr<u8, Output = Self> +
    BitAnd<Self, Output=Self> + BitOr<Self, Output=Self> + BitAndAssign + BitOrAssign
{
    fn zero() -> Self;
    fn one() -> Self;
    fn length(&self) -> u8;

    fn get_bit(&self, bit: u8) -> bool {
        assert!(bit < self.length());
        self.get_range(bit..(bit + 1)) == Self::one()
    }

    fn get_range(&self, range: Range<u8>) -> Self {
        assert!(range.start < self.length());
        assert!(range.end <= self.length());
        assert!(range.start < range.end);

// shift away high bits
        let bits = *self << (self.length() - range.end) >> (self.length() - range.end);

// shift away low bits
        bits >> range.start
    }

    fn set_bit(&mut self, bit: u8, value: bool) -> &mut Self {
        assert!(bit < self.length());
        if value {
            *self |= Self::one() << bit;
        } else {
            *self &= !(Self::one() << bit);
        }
        self
    }

    fn set_range(&mut self, range: Range<u8>, value: Self) -> &mut Self {
        assert!(range.start < self.length());
        assert!(range.end <= self.length());
        assert!(range.start < range.end);
        assert!(value << (self.length() - (range.end - range.start)) >>
                (self.length() - (range.end - range.start)) == value,
                "value too big");

        let bitmask: Self = !(!Self::zero() << (self.length() - range.end) >>
                              (self.length() - range.end) >>
                              range.start << range.start);

        let bits = *self & bitmask;
// set bits
        *self = bits | (value << range.start);

        self
    }
}

macro_rules! bitfield_impl {
    ($($t:ty)*) => ($(
        impl BitField for $t {
            fn zero() -> Self { 0 }
            fn one() -> Self { 1 }
            fn length(&self) -> u8 {
                size_of::<Self>() as u8 * 8
            }
        }
    )*)
}

bitfield_impl! { u8 u16 u32 u64 usize }
