
use std::ops::Range;

pub struct BitField(u32);

impl BitField {
    pub fn get_bit(&self, bit: u32) -> bool {
        assert!(bit < 32);
        self.get_range(bit..(bit+1)) == 1
    }

    pub fn get_range(&self, range: Range<u32>) -> u32 {
        assert!(range.start < 32);
        assert!(range.end <= 32);
        assert!(range.start < range.end);

        // shift away high bits
        let bits = self.0 << (32 - range.end) >> (32 - range.end);
        // shift away low bits
        bits >> range.start
    }

    pub fn set_bit(&mut self, bit: u32) {
        assert!(bit < 32);
        self.0 |= 1 << bit;
    }

    pub fn reset_bit(&mut self, bit: u32) {
        assert!(bit < 32);
        self.0 &= !(1 << bit);
    }

    pub fn set_range(&mut self, range: Range<u32>, value: u32) {
        assert!(range.start < 32);
        assert!(range.end <= 32);
        assert!(range.start < range.end);
        assert!(value << (32 - (range.end - range.start)) >> (32 - (range.end - range.start)) == value, "value too big");

        let bitmask = !(!0 << (32 - range.end) >> (32 - range.end) >> range.start << range.start);

        let bits = self.0 & bitmask;
        // set bits
        self.0 = bits | (value << range.start);
    }
}

#[cfg(test)]
mod tests {
    use BitField;

    #[test]
    fn test_read() {
        let field = BitField(0b1111111111010110);
        assert_eq!(field.get_bit(0), false);
        assert_eq!(field.get_bit(1), true);
        assert_eq!(field.get_bit(2), true);
        assert_eq!(field.get_bit(3), false);
        assert_eq!(field.get_bit(4), true);
        assert_eq!(field.get_bit(5), false);
        for i in 6..16 {
            assert_eq!(field.get_bit(i), true);
        }
        for i in 16..32 {
            assert_eq!(field.get_bit(i), false);
        }

        assert_eq!(field.get_range(16..32), 0);
        assert_eq!(field.get_range(6..16), 0b1111111111);
        assert_eq!(field.get_range(0..6), 0b010110);
        assert_eq!(field.get_range(0..10), 0b1111010110);
        assert_eq!(field.get_range(5..12), 0b1111110);
    }

    #[test]
    fn test_set_reset() {
        let mut field = BitField(0b1111111111010110);
        let mut bit_i = |i| {
            field.set_bit(i);
            assert_eq!(field.get_bit(i), true);
            field.reset_bit(i);
            assert_eq!(field.get_bit(i), false);
            field.set_bit(i);
            assert_eq!(field.get_bit(i), true);
        };
        for i in 0..32 {
            bit_i(i);
        }
    }

    #[test]
    fn test_set_range() {
        let mut field = BitField(0b1111111111010110);
        field.set_range(10..15, 0b00000);
        assert_eq!(field.get_range(10..15), 0b00000);
        field.set_range(10..15, 0b10101);
        assert_eq!(field.get_range(10..15), 0b10101);
        field.set_range(10..15, 0b01010);
        assert_eq!(field.get_range(10..15), 0b01010);
        field.set_range(10..15, 0b11111);
        assert_eq!(field.get_range(10..15), 0b11111);

        field.set_range(0..16, 0xdead);
        field.set_range(14..32, 0xbeaf);
        assert_eq!(field.get_range(0..16), 0xdead);
        assert_eq!(field.get_range(14..32), 0xbeaf);
    }
}
