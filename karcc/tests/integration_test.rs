use karcc::*;

#[test]
fn test_n8_from_u8_and_back() {
    for i in 0..=255u8 {
        let n = N8::from(i);
        assert_eq!(u8::from(n), i);
    }
}

#[test]
fn test_n8_add() {
    let a = N8::from(10);
    let b = N8::from(20);
    let result = a + b;
    assert_eq!(u8::from(result), 30);
}

#[test]
fn test_n8_sub() {
    let a = N8::from(20);
    let b = N8::from(10);
    let result = a - b;
    assert_eq!(u8::from(result), 10);
}

#[test]
fn test_n8_mul() {
    let a = N8::from(10);
    let b = N8::from(5);
    let result = a * b;
    assert_eq!(u8::from(result), 50);
}

#[test]
fn test_n8_div() {
    let a = N8::from(20);
    let b = N8::from(10);
    let result = a / b;
    assert_eq!(u8::from(result), 2);
}

#[test]
fn test_n8_count_ones() {
    let n = N8::from(0b1101_0101);
    assert_eq!(n.count_ones(), 5);
}

#[test]
fn test_n8_reverse_bits() {
    let mut n = N8::from(0b1100_1010);
    n.reverse_bits();
    assert_eq!(u8::from(n), 0b01010011);
}

#[test]
fn test_n8_rotate_left() {
    let mut n = N8::from(0b11001010);
    n.rotate_left(2);
    assert_eq!(u8::from(n), 0b00101011);
}

#[test]
fn test_n16_from_u16_and_back() {
    for i in 0..=u16::MAX {
        let n = N16::from(i);
        assert_eq!(u16::from(n), i);
    }
}

#[test]
fn test_n16_add() {
    let a = N16::from(1000);
    let b = N16::from(2000);
    let result = a + b;
    assert_eq!(u16::from(result), 3000);
}

#[test]
fn test_n16_sub() {
    let a = N16::from(2000);
    let b = N16::from(1000);
    let result = a - b;
    assert_eq!(u16::from(result), 1000);
}

#[test]
fn test_n16_mul() {
    let a = N16::from(100);
    let b = N16::from(50);
    let result = a * b;
    assert_eq!(u32::from(result), 5000);
}

#[test]
fn test_n16_div() {
    let a = N16::from(200);
    let b = N16::from(10);
    let result = a / b;
    assert_eq!(u16::from(result), 20);
}

#[test]
fn test_n16_count_ones() {
    let n = N16::from(0b1101_0101_1101_0101);
    assert_eq!(n.count_ones(), 10);
}

#[test]
fn test_n16_reverse_bits() {
    let mut n = N16::from(0b1100_1010_1100_1010);
    n.reverse_bits();
    assert_eq!(u16::from(n), 0b0101001101010011);
}

#[test]
fn test_n32_from_u32_and_back() {
    let n = N32::from(123456789);
    assert_eq!(u32::from(n), 123456789);
}

#[test]
fn test_n32_add() {
    let a = N32::from(100000);
    let b = N32::from(200000);
    let result = a + b;
    assert_eq!(u32::from(result), 300000);
}

#[test]
fn test_n32_sub() {
    let a = N32::from(200000);
    let b = N32::from(100000);
    let result = a - b;
    assert_eq!(u32::from(result), 100000);
}

#[test]
fn test_n32_mul() {
    let a = N32::from(1000);
    let b = N32::from(500);
    let result = a * b;
    assert_eq!(u32::from(result), 500000);
}

#[test]
fn test_n32_div() {
    let a = N32::from(2000);
    let b = N32::from(10);
    let result = a / b;
    assert_eq!(u32::from(result), 200);
}

#[test]
fn test_n32_count_ones() {
    let n = N32::from(0b1101_0101_1101_0101_1101_0101_1101_0101);
    assert_eq!(n.count_ones(), 20);
}

#[test]
fn test_n64_from_u64_and_back() {
    let n = N64::from(123456789123456789);
    assert_eq!(u64::from(n), 123456789123456789);
}

#[test]
fn test_n64_add() {
    let a = N64::from(1000000000);
    let b = N64::from(2000000000);
    let result = a + b;
    assert_eq!(u64::from(result), 3000000000);
}

#[test]
fn test_n64_sub() {
    let a = N64::from(2000000000);
    let b = N64::from(1000000000);
    let result = a - b;
    assert_eq!(u64::from(result), 1000000000);
}

#[test]
fn test_n64_mul() {
    let a = N64::from(100000);
    let b = N64::from(50000);
    let result = a * b;
    assert_eq!(u64::from(result), 5000000000);
}

#[test]
fn test_n64_div() {
    let a = N64::from(200000);
    let b = N64::from(10);
    let result = a / b;
    assert_eq!(u64::from(result), 20000);
}

#[test]
fn test_z8_from_i8_and_back() {
    for i in -128..=127 {
        let n = Z8::from(i);
        assert_eq!(i8::from(n), i);
    }
}

#[test]
fn test_z8_add() {
    let a = Z8::from(10);
    let b = Z8::from(20);
    let result = a + b;
    assert_eq!(i8::from(result), 30);
}

#[test]
fn test_z8_sub() {
    let a = Z8::from(20);
    let b = Z8::from(10);
    let result = a - b;
    assert_eq!(i8::from(result), 10);
}

#[test]
fn test_z8_mul() {
    let a = Z8::from(10);
    let b = Z8::from(5);
    let result = a * b;
    assert_eq!(i8::from(result), 50);
}

#[test]
fn test_z8_div() {
    let a = Z8::from(20);
    let b = Z8::from(10);
    let result = a / b;
    assert_eq!(i8::from(result), 2);
}

#[test]
fn test_z8_rem() {
    let a = Z8::from(20);
    let b = Z8::from(3);
    let result = a % b;
    assert_eq!(i8::from(result), 2);
}

#[test]
fn test_z8_not() {
    let a = Z8::from(10);
    let result = !a;
    assert_eq!(i8::from(result), -11);
}

#[test]
fn test_z16_from_i16_and_back() {
    for i in i16::MIN..=i16::MAX {
        let n = Z16::from(i);
        assert_eq!(i16::from(n), i);
    }
}

#[test]
fn test_z16_add() {
    let a = Z16::from(1000);
    let b = Z16::from(2000);
    let result = a + b;
    assert_eq!(i16::from(result), 3000);
}

#[test]
fn test_z16_sub() {
    let a = Z16::from(2000);
    let b = Z16::from(1000);
    let result = a - b;
    assert_eq!(i16::from(result), 1000);
}

#[test]
fn test_z16_mul() {
    let a = Z16::from(100);
    let b = Z16::from(50);
    let result = a * b;
    assert_eq!(i16::from(result), 5000);
}

#[test]
fn test_z16_div() {
    let a = Z16::from(200);
    let b = Z16::from(10);
    let result = a / b;
    assert_eq!(i16::from(result), 20);
}

#[test]
fn test_z16_rem() {
    let a = Z16::from(200);
    let b = Z16::from(30);
    let result = a % b;
    assert_eq!(i16::from(result), 20);
}

#[test]
fn test_z16_not() {
    let a = Z16::from(1000);
    let result = !a;
    assert_eq!(i16::from(result), -1001);
}

#[test]
fn test_z32_from_i32_and_back() {
    let n = Z32::from(123456789);
    assert_eq!(i32::from(n), 123456789);
}

#[test]
fn test_z32_add() {
    let a = Z32::from(100000);
    let b = Z32::from(200000);
    let result = a + b;
    assert_eq!(i32::from(result), 300000);
}

#[test]
fn test_z32_sub() {
    let a = Z32::from(200000);
    let b = Z32::from(100000);
    let result = a - b;
    assert_eq!(i32::from(result), 100000);
}

#[test]
fn test_z32_mul() {
    let a = Z32::from(1000);
    let b = Z32::from(500);
    let result = a * b;
    assert_eq!(i32::from(result), 500000);
}

#[test]
fn test_z32_div() {
    let a = Z32::from(2000);
    let b = Z32::from(10);
    let result = a / b;
    assert_eq!(i32::from(result), 200);
}

#[test]
fn test_z32_rem() {
    let a = Z32::from(2000);
    let b = Z32::from(300);
    let result = a % b;
    assert_eq!(i32::from(result), 200);
}

#[test]
fn test_z32_not() {
    let a = Z32::from(100000);
    let result = !a;
    assert_eq!(i32::from(result), -100001);
}

#[test]
fn test_z64_from_i64_and_back() {
    let n = Z64::from(123456789123456789);
    assert_eq!(i64::from(n), 123456789123456789);
}

#[test]
fn test_z64_add() {
    let a = Z64::from(1000000000);
    let b = Z64::from(2000000000);
    let result = a + b;
    assert_eq!(i64::from(result), 3000000000);
}

#[test]
fn test_z64_sub() {
    let a = Z64::from(2000000000);
    let b = Z64::from(1000000000);
    let result = a - b;
    assert_eq!(i64::from(result), 1000000000);
}

#[test]
fn test_z64_mul() {
    let a = Z64::from(100000);
    let b = Z64::from(50000);
    let result = a * b;
    assert_eq!(i64::from(result), 5000000000);
}

#[test]
fn test_z64_div() {
    let a = Z64::from(200000);
    let b = Z64::from(10);
    let result = a / b;
    assert_eq!(i64::from(result), 20000);
}

#[test]
fn test_z64_rem() {
    let a = Z64::from(200000);
    let b = Z64::from(30000);
    let result = a % b;
    assert_eq!(i64::from(result), 20000);
}

#[test]
fn test_z64_not() {
    let a = Z64::from(1000000000);
    let result = !a;
    assert_eq!(i64::from(result), -1000000001);
}

#[test]
fn test_r32_from_f32_and_back() {
    let n = R32::from(123.456);
    assert_eq!(f32::from(n), 123.456);
}

#[test]
fn test_r32_add() {
    let a = R32::from(10.5);
    let b = R32::from(20.5);
    let result = a + b;
    assert_eq!(f32::from(result), 31.0);
}

#[test]
fn test_r32_sub() {
    let a = R32::from(20.5);
    let b = R32::from(10.5);
    let result = a - b;
    assert_eq!(f32::from(result), 10.0);
}

#[test]
fn test_r32_mul() {
    let a = R32::from(10.5);
    let b = R32::from(2.0);
    let result = a * b;
    assert_eq!(f32::from(result), 21.0);
}

#[test]
fn test_r32_div() {
    let a = R32::from(21.0);
    let b = R32::from(2.0);
    let result = a / b;
    assert_eq!(f32::from(result), 10.5);
}

#[test]
fn test_r32_rem() {
    let a = R32::from(21.0);
    let b = R32::from(2.0);
    let result = a % b;
    assert_eq!(f32::from(result), 1.0);
}

#[test]
fn test_r64_from_f64_and_back() {
    let n = R64::from(123.456789);
    assert_eq!(f64::from(n), 123.456789);
}

#[test]
fn test_r64_add() {
    let a = R64::from(10.5);
    let b = R64::from(20.5);
    let result = a + b;
    assert_eq!(f64::from(result), 31.0);
}

#[test]
fn test_r64_sub() {
    let a = R64::from(20.5);
    let b = R64::from(10.5);
    let result = a - b;
    assert_eq!(f64::from(result), 10.0);
}

#[test]
fn test_r64_mul() {
    let a = R64::from(10.5);
    let b = R64::from(2.0);
    let result = a * b;
    assert_eq!(f64::from(result), 21.0);
}

#[test]
fn test_r64_div() {
    let a = R64::from(21.0);
    let b = R64::from(2.0);
    let result = a / b;
    assert_eq!(f64::from(result), 10.5);
}

#[test]
fn test_r64_rem() {
    let a = R64::from(21.0);
    let b = R64::from(2.0);
    let result = a % b;
    assert_eq!(f64::from(result), 1.0);
}