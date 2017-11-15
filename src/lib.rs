//! Provides the abstraction of a bit field, which allows for bit-level update and retrieval
//! operations.

#![feature(const_size_of)]
#![no_std]

#[cfg(test)]
mod tests;

use core::ops::Range;

/// A generic trait which provides methods for extracting and setting specific bits or ranges of
/// bits.
pub trait BitField {
    /// The number of bits in this bit field.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// assert_eq!(u32::BIT_LENGTH, 32);
    /// assert_eq!(u64::BIT_LENGTH, 64);
    /// ```
    const BIT_LENGTH: usize;

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
    fn get_bit(&self, bit: usize) -> bool;

    /// Obtains the range of bits specified by `range`; note that index 0 is the least significant
    /// bit, while index `length() - 1` is the most significant bit.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// let value: u32 = 0b110101;
    ///
    /// assert_eq!(value.get_bits(0..3), 0b101);
    /// assert_eq!(value.get_bits(2..6), 0b1101);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the start or end indexes of the range are out of bounds of the
    /// bit field.
    fn get_bits(&self, range: Range<usize>) -> Self;

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
    fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self;

    /// Sets the range of bits defined by the range `range` to the lower bits of `value`; to be
    /// specific, if the range is N bits long, the N lower bits of `value` will be used; if any of
    /// the other bits in `value` are set to 1, this function will panic.
    ///
    /// ```rust
    /// use bit_field::BitField;
    ///
    /// let mut value = 0u32;
    ///
    /// value.set_bits(0..2, 0b11);
    /// assert_eq!(value, 0b11);
    ///
    /// value.set_bits(0..4, 0b1010);
    /// assert_eq!(value, 0b1010);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the range is out of bounds of the bit field, or if there are `1`s 
    /// not in the lower N bits of `value`.
    fn set_bits(&mut self, range: Range<usize>, value: Self) -> &mut Self;
}


pub trait BitArray<T: BitField> {
    /// Returns the length, eg number of bits, in this bit array.
    ///
    /// ```rust
    /// use bit_field::BitArray;
    ///
    /// assert_eq!([0u8, 4u8, 8u8].bit_length(), 24);
    /// assert_eq!([0u32, 5u32].bit_length(), 64);
    /// ```
    fn bit_length(&self) -> usize;

    /// Obtains the bit at the index `bit`; note that index 0 is the least significant bit, while
    /// index `length() - 1` is the most significant bit.
    ///
    /// ```rust
    /// use bit_field::BitArray;
    ///
    /// let value: [u32; 1] = [0b110101];
    ///
    /// assert_eq!(value.get_bit(1), false);
    /// assert_eq!(value.get_bit(2), true);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the bit index is out of bounds of the bit array.
    fn get_bit(&self, bit: usize) -> bool;

    /// Obtains the range of bits specified by `range`; note that index 0 is the least significant
    /// bit, while index `length() - 1` is the most significant bit.
    ///
    /// ```rust
    /// use bit_field::BitArray;
    ///
    /// let value: [u32; 2] = [0b110101, 0b11];
    ///
    /// assert_eq!(value.get_bits(0..3), 0b101);
    /// assert_eq!(value.get_bits(31..33), 0b10);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the start or end indexes of the range are out of bounds of the
    /// bit array, or if the range can't be contained by the bit field T.
    fn get_bits(&self, range: Range<usize>) -> T;

    /// Sets the bit at the index `bit` to the value `value` (where true means a value of '1' and
    /// false means a value of '0'); note that index 0 is the least significant bit, while index
    /// `length() - 1` is the most significant bit.
    ///
    /// ```rust
    /// use bit_field::BitArray;
    ///
    /// let mut value = [0u32];
    ///
    /// value.set_bit(1, true);
    /// assert_eq!(value, [2u32]);
    ///
    /// value.set_bit(3, true);
    /// assert_eq!(value, [10u32]);
    ///
    /// value.set_bit(1, false);
    /// assert_eq!(value, [8u32]);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the bit index is out of the bounds of the bit array.
    fn set_bit(&mut self, bit: usize, value: bool);

    /// Sets the range of bits defined by the range `range` to the lower bits of `value`; to be
    /// specific, if the range is N bits long, the N lower bits of `value` will be used; if any of
    /// the other bits in `value` are set to 1, this function will panic.
    ///
    /// ```rust
    /// use bit_field::BitArray;
    ///
    /// let mut value = [0u32, 0u32];
    ///
    /// value.set_bits(0..2, 0b11);
    /// assert_eq!(value, [0b11, 0u32]);
    ///
    /// value.set_bits(31..35, 0b1010);
    /// assert_eq!(value, [0x0003, 0b101]);
    /// ```
    ///
    /// ## Panics
    ///
    /// This method will panic if the range is out of bounds of the bit array,
    /// if the range can't be contained by the bit field T, or if there are `1`s 
    /// not in the lower N bits of `value`.
    fn set_bits(&mut self, range: Range<usize>, value: T);
}


/// An internal macro used for implementing BitField on the standard integral types.
macro_rules! bitfield_numeric_impl {
    ($($t:ty)*) => ($(
        impl BitField for $t {
            const BIT_LENGTH: usize = ::core::mem::size_of::<Self>() as usize * 8;

            fn get_bit(&self, bit: usize) -> bool {
                assert!(bit < Self::BIT_LENGTH);

                (*self & (1 << bit)) != 0
            }

            fn get_bits(&self, range: Range<usize>) -> Self {
                assert!(range.start < Self::BIT_LENGTH);
                assert!(range.end <= Self::BIT_LENGTH);
                assert!(range.start < range.end);

                // shift away high bits
                let bits = *self << (Self::BIT_LENGTH - range.end) >> (Self::BIT_LENGTH - range.end);

                // shift away low bits
                bits >> range.start
            }

            fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self {
                assert!(bit < Self::BIT_LENGTH);

                if value {
                    *self |= 1 << bit;
                } else {
                    *self &= !(1 << bit);
                }

                self
            }

            fn set_bits(&mut self, range: Range<usize>, value: Self) -> &mut Self {
                assert!(range.start < Self::BIT_LENGTH);
                assert!(range.end <= Self::BIT_LENGTH);
                assert!(range.start < range.end);
                assert!(value << (Self::BIT_LENGTH - (range.end - range.start)) >>
                        (Self::BIT_LENGTH - (range.end - range.start)) == value,
                        "value does not fit into bit range");

                let bitmask: Self = !(!0 << (Self::BIT_LENGTH - range.end) >>
                                    (Self::BIT_LENGTH - range.end) >>
                                    range.start << range.start);

                // set bits
                *self = (*self & bitmask) | (value << range.start);

                self
            }
        }
    )*)
}

bitfield_numeric_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

impl<T: BitField> BitArray<T> for [T] {
    fn bit_length(&self) -> usize {
        self.len() * T::BIT_LENGTH
    }

    fn get_bit(&self, bit: usize) -> bool {
        let slice_index = bit / T::BIT_LENGTH;
        let bit_index = bit % T::BIT_LENGTH;
        self[slice_index].get_bit(bit_index)
    }

    fn get_bits(&self, range: Range<usize>) -> T {
        assert!(range.len() <= T::BIT_LENGTH);
        
        let slice_start = range.start/T::BIT_LENGTH;
        let slice_end = range.end / T::BIT_LENGTH;
        let bit_start = range.start % T::BIT_LENGTH;
        let bit_end = range.end % T::BIT_LENGTH;
        let len = range.len();

        assert!(slice_end - slice_start<= 1);
        
        if slice_start == slice_end {
            self[slice_start].get_bits(bit_start..bit_end)
        } else if bit_end == 0 {
            self[slice_start].get_bits(bit_start..T::BIT_LENGTH)
        } else {
            let mut ret = self[slice_start].get_bits(bit_start..T::BIT_LENGTH);
            ret.set_bits((T::BIT_LENGTH - bit_start)..len, self[slice_end].get_bits(0..bit_end));
            ret
        }
    }

    fn set_bit(&mut self, bit: usize, value: bool) {
        let slice_index = bit / T::BIT_LENGTH;
        let bit_index = bit % T::BIT_LENGTH;
        self[slice_index].set_bit(bit_index, value);
    }

    fn set_bits(&mut self, range: Range<usize>, value: T) {
        assert!(range.len() <= T::BIT_LENGTH);

        let slice_start = range.start/T::BIT_LENGTH;
        let slice_end = range.end / T::BIT_LENGTH;
        let bit_start = range.start % T::BIT_LENGTH;
        let bit_end = range.end % T::BIT_LENGTH;
        
        assert!(slice_end - slice_start<= 1);
        
        if slice_start == slice_end {
            self[slice_start].set_bits(bit_start..bit_end, value);
        } else if bit_end == 0 {
            self[slice_start].set_bits(bit_start..T::BIT_LENGTH, value);
        } else {
            self[slice_start].set_bits(bit_start..T::BIT_LENGTH, value.get_bits(0..T::BIT_LENGTH-bit_start));
            self[slice_end].set_bits(0..bit_end, value.get_bits(T::BIT_LENGTH-bit_start..T::BIT_LENGTH));
        }
    }
    
}

