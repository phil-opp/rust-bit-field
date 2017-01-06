//! Provides the abstraction of a bit field, which allows for bit-level update and retrieval
//! operations.

#![no_std]

#[cfg(test)]
mod tests;

use core::ops::{Range, Shl, Shr, BitAndAssign, BitOrAssign, Not, BitAnd, BitOr};
use core::mem::size_of;

/// A generic trait which provides methods for extracting and setting specific bits or ranges of
/// bits.
pub trait BitField: Copy + Eq + Not<Output = Self> +
    Shl<u8, Output = Self> + Shr<u8, Output = Self> +
    BitAnd<Self, Output=Self> + BitOr<Self, Output=Self> + BitAndAssign + BitOrAssign
{
    /// Returns the zero-value of the bit field; for any integral type, this will be the literal
    /// constant '0'.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// assert_eq!(<u32 as BitField>::zero(), 0u32);
    /// ```
    fn zero() -> Self;

    /// Returns the one-value of the bit field; for any integral type, this will be the literal
    /// constant `1`.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// assert_eq!(<u32 as BitField>::one(), 1u32);
    /// ```
    fn one() -> Self;

    /// Returns the length, eg number of bits, in this bit field.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// assert_eq!(0u32.length(), 32);
    /// assert_eq!(0u64.length(), 64);
    /// ```
    fn length(&self) -> u8;

    /// Obtains the bit at the index `bit`; note that index 0 is the least significant bit, while
    /// index `length() - 1` is the most significant bit.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// let value: u32 = 0b110101;
    ///
    /// assert_eq!(value.get_bit(1), false);
    /// assert_eq!(value.get_bit(2), true);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the bit index is out of bounds of the bit field.
    fn get_bit(&self, bit: u8) -> bool {
        assert!(bit < self.length());
        self.get_range(bit..(bit + 1)) == Self::one()
    }

    /// Obtains the range of bits specified by `range`; note that index 0 is the least significant
    /// bit, while index `length() - 1` is the most significant bit.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// let value: u32 = 0b110101;
    ///
    /// assert_eq!(value.get_range(0..3), 0b101);
    /// assert_eq!(value.get_range(2..6), 0b1101);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the start or end indexes of the range are out of bounds of the
    /// bit field.
    fn get_range(&self, range: Range<u8>) -> Self {
        assert!(range.start < self.length());
        assert!(range.end <= self.length());
        assert!(range.start < range.end);

        // shift away high bits
        let bits = *self << (self.length() - range.end) >> (self.length() - range.end);

        // shift away low bits
        bits >> range.start
    }

    /// Sets the bit at the index `bit` to the value `value` (where true means a value of '1' and
    /// false means a value of '0'); note that index 0 is the least significant bit, while index
    /// `length() - 1` is the most significant bit.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// let mut value = 0u32;
    ///
    /// value.set_bit(1, true);
    /// assert_eq!(value, 2u32);
    ///
    /// value.set_bit(3, true);
    /// assert_eq!(value, 10u32);
    ///
    /// value.set_bit(1, false);
    /// assert_eq!(value, 8u32);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the bit index is out of the bounds of the bit field.
    fn set_bit(&mut self, bit: u8, value: bool) -> &mut Self {
        assert!(bit < self.length());
        if value {
            *self |= Self::one() << bit;
        } else {
            *self &= !(Self::one() << bit);
        }
        self
    }

    /// Sets the range of bits defined by the range `range` to the lower bits of `value`; to be
    /// specific, if the range is N bits long, the N lower bits of `value` will be used; if any of
    /// the other bits in `value` are set to 1, this function will panic.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// let mut value = 0u32;
    ///
    /// value.set_range(0..2, 0b11);
    /// assert_eq!(value, 0b11);
    ///
    /// value.set_range(0..4, 0b1010);
    /// assert_eq!(value, 0b1010);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the range is out of bounds of the bit field, or if there are `1`s 
    /// not in the lower N bits of `value`.
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

/// An internal macro used for implementing BitField on the standard unsigned integral types.
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
