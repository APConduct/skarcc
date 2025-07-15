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
