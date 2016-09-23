use BitField;

#[test]
fn test_set_reset_u8() {
    let mut field = 0b11110010u8;
    let mut bit_i = |i| {
        field.set_bit(i, true);
        assert_eq!(field.get_bit(i), true);
        field.set_bit(i, false);
        assert_eq!(field.get_bit(i), false);
        field.set_bit(i, true);
        assert_eq!(field.get_bit(i), true);
    };
    for i in 0..8 {
        bit_i(i);
    }
}

#[test]
fn test_set_reset_u16() {
    let mut field = 0b1111001010010110u16;
    let mut bit_i = |i| {
        field.set_bit(i, true);
        assert_eq!(field.get_bit(i), true);
        field.set_bit(i, false);
        assert_eq!(field.get_bit(i), false);
        field.set_bit(i, true);
        assert_eq!(field.get_bit(i), true);
    };
    for i in 0..16 {
        bit_i(i);
    }
}

#[test]
fn test_read_u32() {
    let field = 0b1111111111010110u32;
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
fn test_set_reset_u32() {
    let mut field = 0b1111111111010110u32;
    let mut bit_i = |i| {
        field.set_bit(i, true);
        assert_eq!(field.get_bit(i), true);
        field.set_bit(i, false);
        assert_eq!(field.get_bit(i), false);
        field.set_bit(i, true);
        assert_eq!(field.get_bit(i), true);
    };
    for i in 0..32 {
        bit_i(i);
    }
}

#[test]
fn test_set_range_u32() {
    let mut field = 0b1111111111010110u32;
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

#[test]
fn test_read_u64() {
    let field = 0b1111111111010110u64 << 32;
    for i in 0..32 {
        assert_eq!(field.get_bit(i), false);
    }
    assert_eq!(field.get_bit(32), false);
    assert_eq!(field.get_bit(33), true);
    assert_eq!(field.get_bit(34), true);
    assert_eq!(field.get_bit(35), false);
    assert_eq!(field.get_bit(36), true);
    assert_eq!(field.get_bit(37), false);
    for i in 38..48 {
        assert_eq!(field.get_bit(i), true);
    }
    for i in 48..64 {
        assert_eq!(field.get_bit(i), false);
    }

    assert_eq!(field.get_range(0..32), 0);
    assert_eq!(field.get_range(48..64), 0);
    assert_eq!(field.get_range(38..48), 0b1111111111);
    assert_eq!(field.get_range(32..38), 0b010110);
    assert_eq!(field.get_range(32..42), 0b1111010110);
    assert_eq!(field.get_range(37..44), 0b1111110);
}

#[test]
fn test_set_reset_u64() {
    let mut field = 0b1111111111010110u64 << 32;
    let mut bit_i = |i| {
        field.set_bit(i, true);
        assert_eq!(field.get_bit(i), true);
        field.set_bit(i, false);
        assert_eq!(field.get_bit(i), false);
        field.set_bit(i, true);
        assert_eq!(field.get_bit(i), true);
    };
    for i in 0..64 {
        bit_i(i);
    }
}

#[test]
fn test_set_range_u64() {
    let mut field = 0b1111111111010110u64 << 32;
    field.set_range(42..47, 0b00000);
    assert_eq!(field.get_range(42..47), 0b00000);
    field.set_range(10..15, 0b10101);
    assert_eq!(field.get_range(10..15), 0b10101);
    field.set_range(40..45, 0b01010);
    assert_eq!(field.get_range(40..45), 0b01010);
    field.set_range(40..45, 0b11111);
    assert_eq!(field.get_range(40..45), 0b11111);

    field.set_range(0..16, 0xdead);
    field.set_range(14..32, 0xbeaf);
    field.set_range(32..64, 0xcafebabe);
    assert_eq!(field.get_range(0..16), 0xdead);
    assert_eq!(field.get_range(14..32), 0xbeaf);
    assert_eq!(field.get_range(32..64), 0xcafebabe);
}
