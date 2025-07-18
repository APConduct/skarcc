
//! karcc - Bitwise and numeric types for custom arithmetic and logic operations.
//!
//! This module provides custom types for bits, bytes, nibbles, words, and various
//! signed/unsigned/floating-point numbers, along with bitwise and arithmetic operations.
//! The types are implemented using arrays of `Bit` and provide operator overloading
//! for arithmetic and logic, as well as conversion to/from primitive Rust types.

use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
    str::FromStr,
};

/// Trait for counting ones and zeros in a bit sequence.
pub trait BitCount {
    /// Returns the number of ones in the bit sequence.
    fn count_ones(&self) -> u32;
    /// Returns the number of zeros in the bit sequence.
    fn count_zeros(&self) -> u32;
}

/// Trait for rotating bits left and right.
pub trait BitwiseRotate {
    /// Rotates bits to the left by `n` positions.
    fn rotate_left(&mut self, n: u32);
    /// Rotates bits to the right by `n` positions.
    fn rotate_right(&mut self, n: u32);
}

/// Trait for reversing the order of bits.
pub trait BitwiseReverse {
    /// Reverses the order of bits in the sequence.
    fn reverse_bits(&mut self);
}

/// Represents a single bit (0 or 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One,
}

impl From<Bit> for u8 {
    fn from(val: Bit) -> Self {
        match val {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

impl PartialOrd for Bit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bit {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Bit::Zero, Bit::Zero) | (Bit::One, Bit::One) => Ordering::Equal,
            (Bit::Zero, Bit::One) => Ordering::Less,
            (Bit::One, Bit::Zero) => Ordering::Greater,
        }
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Bit::Zero => write!(f, "0"),
            Bit::One => write!(f, "1"),
        }
    }
}

impl Bit {
    /// Converts the bit to a `Bool`.
    pub fn as_bool(&self) -> Bool {
        match self {
            Bit::Zero => Bool::False,
            Bit::One => Bool::True,
        }
    }

    /// Creates a `Bit` from a `Bool`.
    pub fn from_bool(value: Bool) -> Bit {
        match value {
            Bool::False => Bit::Zero,
            Bool::True => Bit::One,
        }
    }

    /// Returns `Bool::True` if the bit is zero.
    pub fn is_zero(&self) -> Bool {
        match self {
            Bit::Zero => Bool::True,
            Bit::One => Bool::False,
        }
    }

    /// Returns `Bool::True` if the bit is one.
    pub fn is_one(&self) -> Bool {
        match self {
            Bit::Zero => Bool::False,
            Bit::One => Bool::True,
        }
    }

    /// Compares two bits for equality, returning a `Bool`.
    pub fn is(&self, other: &Bit) -> Bool {
        match (self, other) {
            (Bit::Zero, Bit::Zero) => Bool::True,
            (Bit::One, Bit::One) => Bool::True,
            _ => Bool::False,
        }
    }

    /// Bitwise OR operation.
    pub fn or(&self, other: &Bit) -> Bit {
        match (self, other) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::One,
            (Bit::One, Bit::Zero) => Bit::One,
            (Bit::One, Bit::One) => Bit::One,
        }
    }

    /// Bitwise AND operation.
    pub fn and(&self, other: &Bit) -> Bit {
        match (self, other) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::Zero,
            (Bit::One, Bit::Zero) => Bit::Zero,
            (Bit::One, Bit::One) => Bit::One,
        }
    }

    /// Bitwise XOR operation.
    pub fn xor(&self, other: &Bit) -> Bit {
        match (self, other) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::One,
            (Bit::One, Bit::Zero) => Bit::One,
            (Bit::One, Bit::One) => Bit::Zero,
        }
    }
}

impl Rem for Bit {
    type Output = Bit;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Bit::One, Bit::One) => Bit::Zero,
            _ => self,
        }
    }
}

impl Add for Bit {
    type Output = Bit;

    fn add(self, other: Self) -> Self::Output {
        self.xor(&other)
    }
}

impl AddAssign for Bit {
    fn add_assign(&mut self, other: Self) {
        *self = self.xor(&other);
    }
}

impl std::ops::BitXor for Bit {
    type Output = Bit;

    fn bitxor(self, other: Self) -> Self::Output {
        self.xor(&other)
    }
}

impl std::ops::BitAndAssign for Bit {
    fn bitand_assign(&mut self, other: Self) {
        *self = self.and(&other);
    }
}

impl std::ops::BitOrAssign for Bit {
    fn bitor_assign(&mut self, other: Self) {
        *self = self.or(&other);
    }
}

impl std::ops::BitXorAssign for Bit {
    fn bitxor_assign(&mut self, other: Self) {
        *self = self.xor(&other);
    }
}

impl Not for Bit {
    type Output = Bit;

    fn not(self) -> Self::Output {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

impl std::ops::BitOr for Bit {
    type Output = Bit;

    fn bitor(self, other: Self) -> Self {
        self.or(&other)
    }
}

impl Bit {
    /// Bitwise NAND operation.
    pub fn nand(&self, other: &Bit) -> Bit {
        self.and(other).not()
    }

    /// Bitwise NOR operation.
    pub fn nor(&self, other: &Bit) -> Bit {
        self.or(other).not()
    }

    /// Bitwise XNOR operation.
    pub fn xnor(&self, other: &Bit) -> Bit {
        self.xor(other).not()
    }
}

/// Represents a byte (8 bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Byte {
    bits: [Bit; 8],
}

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for bit in self.bits.iter().rev() {
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

impl fmt::Binary for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for bit in self.bits.iter().rev() {
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

impl Byte {
    /// Creates a new `Byte` from an array of 8 bits.
    pub fn new(bits: [Bit; 8]) -> Self {
        Byte { bits }
    }

    /// Gets the bit at the specified index.
    pub fn get_bit(&self, index: usize) -> Bit {
        self.bits[index]
    }

    /// Sets the bit at the specified index.
    pub fn set_bit(&mut self, index: usize, bit: Bit) {
        self.bits[index] = bit;
    }

    /// Returns a reference to the underlying bits.
    pub fn get_bits(&self) -> &[Bit; 8] {
        &self.bits
    }

    /// Inverts all bits in the byte.
    pub fn invert(&mut self) {
        for bit in &mut self.bits {
            *bit = bit.not()
        }
    }
}

impl std::ops::BitAnd for Bit {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        self.and(&other)
    }
}

impl std::ops::BitAnd for Byte {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        let mut result = [Bit::Zero; 8];
        for i in 0..8 {
            result[i] = self.bits[i].bitand(other.bits[i]);
        }
        Byte { bits: result }
    }
}

impl FromIterator<Bit> for Byte {
    fn from_iter<I: IntoIterator<Item = Bit>>(iter: I) -> Self {
        let mut bits = [Bit::Zero; 8];
        for (i, bit) in iter.into_iter().enumerate().take(8) {
            bits[i] = bit;
        }
        Byte { bits }
    }
}

impl Byte {
    /// Bitwise AND with another byte.
    pub fn and(&self, other: &Byte) -> Byte {
        let mut result = [Bit::Zero; 8];
        for i in 0..8 {
            result[i] = self.bits[i].and(&other.bits[i]);
        }
        Byte { bits: result }
    }

    /// Bitwise OR with another byte.
    pub fn or(&self, other: &Byte) -> Byte {
        let mut result = [Bit::Zero; 8];
        for i in 0..8 {
            result[i] = self.bits[i].or(&other.bits[i]);
        }
        Byte { bits: result }
    }

    /// Bitwise XOR with another byte.
    pub fn xor(&self, other: &Byte) -> Byte {
        let mut result = [Bit::Zero; 8];
        for i in 0..8 {
            result[i] = self.bits[i].xor(&other.bits[i]);
        }
        Byte { bits: result }
    }
}

impl Not for Byte {
    type Output = Byte;

    fn not(self) -> Self::Output {
        let mut result = [Bit::Zero; 8];
        for i in 0..8 {
            result[i] = self.bits[i].not();
        }
        Byte { bits: result }
    }
}

impl Shl<u8> for Byte {
    type Output = Byte;

    fn shl(self, shift: u8) -> Self::Output {
        let mut result = Byte::new([Bit::Zero; 8]);
        let shift = shift as usize;
        if shift >= 8 {
            return result;
        }
        for i in shift..8 {
            result.bits[i] = self.bits[i - shift];
        }
        result
    }
}

impl Shr<u8> for Byte {
    type Output = Byte;

    fn shr(self, shift: u8) -> Self::Output {
        let mut result = Byte::new([Bit::Zero; 8]);
        let shift = shift as usize;
        if shift >= 8 {
            return result;
        }
        for i in 0..(8 - shift) {
            result.bits[i] = self.bits[i + shift];
        }
        result
    }
}

impl Byte {
    /// Rotates bits to the left by `n` positions.
    pub fn rotate_left(&self, n: usize) -> Byte {
        let mut result = [Bit::Zero; 8];
        let n = n % 8;
        for i in 0..8 {
            result[(i + n) % 8] = self.bits[i];
        }
        Byte { bits: result }
    }

    /// Rotates bits to the right by `n` positions.
    pub fn rotate_right(&self, n: usize) -> Byte {
        let mut result = [Bit::Zero; 8];
        let n = n % 8;
        for i in 0..8 {
            result[(i + 8 - n) % 8] = self.bits[i];
        }
        Byte { bits: result }
    }
}

impl std::ops::BitOr for Byte {
    type Output = Byte;

    fn bitor(self, other: Self) -> Self {
        let mut result = [Bit::Zero; 8];
        for i in 0..8 {
            result[i] = self.bits[i].bitor(other.bits[i]);
        }
        Byte { bits: result }
    }
}

/// Boolean type for logic operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bool {
    True,
    False,
}

impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Bool::True => write!(f, "true"),
            Bool::False => write!(f, "false"),
        }
    }
}

impl Bool {
    /// Creates a `Bool` from a primitive `bool`.
    pub fn new(value: bool) -> Bool {
        if value { Bool::True } else { Bool::False }
    }

    /// Logical NOT.
    pub fn not(&self) -> Bool {
        match self {
            Bool::True => Bool::False,
            Bool::False => Bool::True,
        }
    }

    /// Logical AND.
    pub fn and(&self, other: &Bool) -> Bool {
        match (self, other) {
            (Bool::True, Bool::True) => Bool::True,
            _ => Bool::False,
        }
    }

    /// Logical OR.
    pub fn or(&self, other: &Bool) -> Bool {
        match (self, other) {
            (Bool::True, _) | (_, Bool::True) => Bool::True,
            _ => Bool::False,
        }
    }

    /// Logical XOR.
    pub fn xor(&self, other: &Bool) -> Bool {
        match (self, other) {
            (Bool::True, Bool::False) | (Bool::False, Bool::True) => Bool::True,
            _ => Bool::False,
        }
    }
}

impl Not for Bool {
    type Output = Bool;

    fn not(self) -> Self::Output {
        match self {
            Bool::True => Bool::False,
            Bool::False => Bool::True,
        }
    }
}

impl std::ops::BitAnd for Bool {
    type Output = Bool;

    fn bitand(self, other: Self) -> Self::Output {
        match (self, other) {
            (Bool::True, Bool::True) => Bool::True,
            _ => Bool::False,
        }
    }
}

impl std::ops::BitOr for Bool {
    type Output = Bool;

    fn bitor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Bool::True, _) | (_, Bool::True) => Bool::True,
            _ => Bool::False,
        }
    }
}

impl std::ops::BitXor for Bool {
    type Output = Bool;

    fn bitxor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Bool::True, Bool::False) | (Bool::False, Bool::True) => Bool::True,
            _ => Bool::False,
        }
    }
}

/// Represents a nibble (4 bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nibble {
    bits: [Bit; 4],
}

impl Nibble {
    /// Creates a new `Nibble` from an array of 4 bits.
    pub fn new(bits: [Bit; 4]) -> Self {
        Nibble { bits }
    }

    /// Maximum value for a nibble (0b1111).
    pub const MAX: Nibble = Nibble {
        bits: [Bit::One, Bit::One, Bit::One, Bit::One],
    };

    /// Zero value for a nibble (0b0000).
    pub const ZERO: Nibble = Nibble {
        bits: [Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero],
    };

    /// One value for a nibble (0b0001).
    pub const ONE: Nibble = Nibble {
        bits: [Bit::Zero, Bit::Zero, Bit::Zero, Bit::One],
    };
}

impl Not for Nibble {
    type Output = Nibble;

    fn not(self) -> Self::Output {
        Nibble {
            bits: self.bits.map(|bit| !bit),
        }
    }
}

impl std::ops::BitAnd for Nibble {
    type Output = Nibble;

    fn bitand(self, other: Self) -> Self::Output {
        let mut bits = [Bit::Zero; 4];
        for i in 0..4 {
            bits[i] = self.bits[i] & other.bits[i];
        }
        Nibble { bits }
    }
}

impl std::ops::BitOr for Nibble {
    type Output = Nibble;

    fn bitor(self, other: Self) -> Self::Output {
        let mut bits = [Bit::Zero; 4];
        for i in 0..4 {
            bits[i] = self.bits[i] | other.bits[i];
        }
        Nibble { bits }
    }
}

impl std::ops::BitXor for Nibble {
    type Output = Nibble;

    fn bitxor(self, other: Self) -> Self::Output {
        let mut bits = [Bit::Zero; 4];
        for i in 0..4 {
            bits[i] = self.bits[i] ^ other.bits[i];
        }
        Nibble { bits }
    }
}

impl std::ops::BitXorAssign for Nibble {
    fn bitxor_assign(&mut self, other: Self) {
        for i in 0..4 {
            self.bits[i] ^= other.bits[i];
        }
    }
}

impl std::ops::BitAndAssign for Nibble {
    fn bitand_assign(&mut self, other: Self) {
        for i in 0..4 {
            self.bits[i] &= other.bits[i];
        }
    }
}

impl std::ops::BitOrAssign for Nibble {
    fn bitor_assign(&mut self, other: Self) {
        for i in 0..4 {
            self.bits[i] |= other.bits[i];
        }
    }
}

impl Display for Nibble {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for bit in self.bits.iter().rev() {
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

impl FromIterator<Bit> for Nibble {
    fn from_iter<I: IntoIterator<Item = Bit>>(iter: I) -> Self {
        let mut bits = [Bit::Zero; 4];
        for (i, bit) in iter.into_iter().enumerate().take(4) {
            bits[i] = bit;
        }
        Nibble { bits }
    }
}

impl Shl<u8> for Nibble {
    type Output = Nibble;

    fn shl(self, shift: u8) -> Self::Output {
        let mut result = Nibble::ZERO;
        let shift = shift as usize;
        if shift >= 4 {
            return result;
        }
        for i in shift..4 {
            result.bits[i] = self.bits[i - shift];
        }
        result
    }
}

impl Shr<u8> for Nibble {
    type Output = Nibble;

    fn shr(self, shift: u8) -> Self::Output {
        let mut result = Nibble::ZERO;
        let shift = shift as usize;
        if shift >= 4 {
            return result;
        }
        for i in 0..(4 - shift) {
            result.bits[i] = self.bits[i + shift];
        }
        result
    }
}

impl ShlAssign<u8> for Nibble {
    fn shl_assign(&mut self, shift: u8) {
        *self = *self << shift;
    }
}

impl ShrAssign<u8> for Nibble {
    fn shr_assign(&mut self, shift: u8) {
        *self = *self >> shift;
    }
}

/// Represents a word (16 bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    bits: [Bit; 16],
}

impl ShlAssign<u8> for Byte {
    fn shl_assign(&mut self, shift: u8) {
        *self = *self << shift;
    }
}

impl ShrAssign<u8> for Byte {
    fn shr_assign(&mut self, shift: u8) {
        *self = *self >> shift;
    }
}

/// Either type for holding one of two possible values.
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Maps the left value using the provided function.
    pub fn map_left<F, T>(self, f: F) -> Either<T, R>
    where
        F: FnOnce(L) -> T,
    {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }

    /// Maps the right value using the provided function.
    pub fn map_right<F, T>(self, f: F) -> Either<L, T>
    where
        F: FnOnce(R) -> T,
    {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(f(r)),
        }
    }
}

impl Mul for Bit {
    type Output = Bit;

    fn mul(self, rhs: Self) -> Self::Output {
        self.and(&rhs)
    }
}

// -------------------- N8 --------------------

/// Unsigned 8-bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct N8 {
    bits: [Bit; 8],
}

impl N8 {
    /// Creates a new `N8` from an array of 8 bits.
    pub fn new(bits: [Bit; 8]) -> Self {
        N8 { bits }
    }

    /// Maximum value for N8 (0xFF).
    pub const MAX: N8 = N8 {
        bits: [
            Bit::One,
            Bit::One,
            Bit::One,
            Bit::One,
            Bit::One,
            Bit::One,
            Bit::One,
            Bit::One,
        ],
    };

    /// Minimum value for N8 (0).
    pub const MIN: N8 = Self::ZERO;

    /// Zero value for N8.
    pub const ZERO: N8 = N8 {
        bits: [Bit::Zero; 8],
    };

    /// One value for N8.
    pub const ONE: N8 = N8::from_u8(1);
    pub const TWO: N8 = N8::from_u8(2);
    pub const THREE: N8 = N8::from_u8(3);
    pub const FOUR: N8 = N8::from_u8(4);
    pub const FIVE: N8 = N8::from_u8(5);
    pub const SIX: N8 = N8::from_u8(6);
    pub const SEVEN: N8 = N8::from_u8(7);
    pub const EIGHT: N8 = N8::from_u8(8);
    pub const NINE: N8 = N8::from_u8(9);
    pub const TEN: N8 = N8::from_u8(10);
}

impl Shl<u8> for N8 {
    type Output = N8;

    fn shl(self, shift: u8) -> Self::Output {
        (u8::from(self) << shift).into()
    }
}

impl Shr<u8> for N8 {
    type Output = N8;

    fn shr(self, shift: u8) -> Self::Output {
        (u8::from(self) >> shift).into()
    }
}

impl ShlAssign<u8> for N8 {
    fn shl_assign(&mut self, shift: u8) {
        *self = *self << shift;
    }
}

impl ShrAssign<u8> for N8 {
    fn shr_assign(&mut self, shift: u8) {
        *self = *self >> shift;
    }
}

/// Full adder for single bits.
fn full_adder(a: Bit, b: Bit, carry: Bit) -> (Bit, Bit) {
    let sum = a ^ b ^ carry;
    let new_carry = (a & b) | (a & carry) | (b & carry);
    (sum, new_carry)
}

impl Add for N8 {
    type Output = N8;

    fn add(self, other: N8) -> N8 {
        let mut carry = Bit::Zero;
        let mut result_bits = [Bit::Zero; 8];
        for i in 0..8 {
            let (sum, new_carry) = full_adder(self.bits[i], other.bits[i], carry);
            result_bits[i] = sum;
            carry = new_carry;
        }
        N8 { bits: result_bits }
    }
}

impl AddAssign for N8 {
    fn add_assign(&mut self, other: N8) {
        *self = *self + other;
    }
}

impl From<u8> for N8 {
    fn from(value: u8) -> Self {
        let mut bits = [Bit::Zero; 8];
        for i in 0..8 {
            bits[i] = if (value & (1 << i)) != 0 {
                Bit::One
            } else {
                Bit::Zero
            };
        }
        N8 { bits }
    }
}

impl From<N8> for u8 {
    fn from(value: N8) -> Self {
        let mut result = 0u8;
        for i in 0..8 {
            if value.bits[i] == Bit::One {
                result |= 1 << i;
            }
        }
        result
    }
}

impl N8 {
    /// Creates a `N8` from a primitive `u8` (const).
    pub const fn from_u8(value: u8) -> Self {
        let mut bits = [Bit::Zero; 8];
        let mut i = 0;
        while i < 8 {
            bits[i] = if (value & (1 << i)) != 0 {
                Bit::One
            } else {
                Bit::Zero
            };
            i += 1;
        }
        N8 { bits }
    }

    /// Creates a new `N8` from an array of 8 bits.
    pub fn new_from_bits(bits: [Bit; 8]) -> Self {
        N8 { bits }
    }

    /// Returns the value as a `Byte`.
    pub fn as_byte(&self) -> Byte {
        Byte { bits: self.bits }
    }
}

impl BitCount for N8 {
    fn count_ones(&self) -> u32 {
        self.bits.iter().filter(|&&b| b == Bit::One).count() as u32
    }
    fn count_zeros(&self) -> u32 {
        self.bits.iter().filter(|&&b| b == Bit::Zero).count() as u32
    }
}

impl BitwiseReverse for N8 {
    fn reverse_bits(&mut self) {
        self.bits.reverse();
    }
}

impl BitwiseRotate for N8 {
    fn rotate_left(&mut self, n: u32) {
        let val = u8::from(*self);
        let rotated = val.rotate_left(n);
        *self = rotated.into();
    }
    fn rotate_right(&mut self, n: u32) {
        let val = u8::from(*self);
        let rotated = val.rotate_right(n);
        *self = rotated.into();
    }
}

impl FromStr for N8 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 8 {
            return Err(());
        }
        let mut bits = [Bit::Zero; 8];
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '0' => bits[i] = Bit::Zero,
                '1' => bits[i] = Bit::One,
                _ => return Err(()),
            }
        }
        Ok(N8 { bits })
    }
}

/// Full subtractor for single bits.
fn full_subtractor(a: Bit, b: Bit, borrow: Bit) -> (Bit, Bit) {
    let diff = a ^ b ^ borrow;
    let new_borrow = (b & !a) | (borrow & !(a ^ b));
    (diff, new_borrow)
}

impl Sub for N8 {
    type Output = N8;

    fn sub(self, other: N8) -> N8 {
        let mut borrow = Bit::Zero;
        let mut result_bits = [Bit::Zero; 8];
        for i in 0..8 {
            let (diff, new_borrow) = full_subtractor(self.bits[i], other.bits[i], borrow);
            result_bits[i] = diff;
            borrow = new_borrow;
        }
        N8 { bits: result_bits }
    }
}

impl PartialOrd for N8 {
    fn partial_cmp(&self, other: &N8) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for N8 {
    fn cmp(&self, other: &N8) -> Ordering {
        u8::from(*self).cmp(&u8::from(*other))
    }
}

impl SubAssign for N8 {
    fn sub_assign(&mut self, other: N8) {
        *self = *self - other;
    }
}

impl Mul for N8 {
    type Output = N8;

    fn mul(self, other: N8) -> N8 {
        (u8::from(self) * u8::from(other)).into()
    }
}

impl Div for N8 {
    type Output = N8;

    fn div(self, other: N8) -> N8 {
        (u8::from(self) / u8::from(other)).into()
    }
}

impl Display for N8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

// -------------------- N16 --------------------

/// Unsigned 16-bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct N16 {
    bits: [Bit; 16],
}

impl From<u16> for N16 {
    fn from(value: u16) -> Self {
        let mut bits = [Bit::Zero; 16];
        for i in 0..16 {
            bits[i] = if (value & (1 << i)) != 0 {
                Bit::One
            } else {
                Bit::Zero
            };
        }
        N16 { bits }
    }
}

impl From<N16> for u16 {
    fn from(value: N16) -> Self {
        let mut result = 0u16;
        for i in 0..16 {
            if value.bits[i] == Bit::One {
                result |= 1 << i;
            }
        }
        result
    }
}

impl Sub for N16 {
    type Output = N16;

    fn sub(self, other: N16) -> N16 {
        u16::from(self).wrapping_sub(u16::from(other)).into()
    }
}

impl SubAssign for N16 {
    fn sub_assign(&mut self, other: N16) {
        *self = *self - other;
    }
}

impl Add for N16 {
    type Output = N16;

    fn add(self, other: N16) -> N16 {
        u16::from(self).wrapping_add(u16::from(other)).into()
    }
}

impl AddAssign for N16 {
    fn add_assign(&mut self, other: N16) {
        *self = *self + other;
    }
}

impl Mul for N16 {
    type Output = N32;

    fn mul(self, other: N16) -> N32 {
        (u16::from(self) as u32 * u16::from(other) as u32).into()
    }
}

impl Shr<u8> for N16 {
    type Output = N16;

    fn shr(self, shift: u8) -> N16 {
        (u16::from(self) >> shift).into()
    }
}

impl Shl<u8> for N16 {
    type Output = N16;

    fn shl(self, shift: u8) -> N16 {
        (u16::from(self) << shift).into()
    }
}

impl ShlAssign<u8> for N16 {
    fn shl_assign(&mut self, shift: u8) {
        *self = *self << shift;
    }
}

impl ShrAssign<u8> for N16 {
    fn shr_assign(&mut self, shift: u8) {
        *self = *self >> shift;
    }
}

impl Div for N16 {
    type Output = N16;

    fn div(self, other: N16) -> N16 {
        (u16::from(self) / u16::from(other)).into()
    }
}

impl PartialOrd for N16 {
    fn partial_cmp(&self, other: &N16) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for N16 {
    fn cmp(&self, other: &N16) -> Ordering {
        u16::from(*self).cmp(&u16::from(*other))
    }
}

impl N16 {
    /// Creates a new `N16` from an array of 16 bits.
    pub fn new(bits: [Bit; 16]) -> Self {
        N16 { bits }
    }
    /// Returns a zero value for N16.
    pub fn zero() -> Self {
        N16 {
            bits: [Bit::Zero; 16],
        }
    }
}

impl BitCount for N16 {
    fn count_ones(&self) -> u32 {
        self.bits.iter().filter(|&bit| *bit == Bit::One).count() as u32
    }
    fn count_zeros(&self) -> u32 {
        self.bits.iter().filter(|&bit| *bit == Bit::Zero).count() as u32
    }
}

impl BitwiseReverse for N16 {
    fn reverse_bits(&mut self) {
        self.bits.reverse();
    }
}

impl Display for N16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u16::from(*self))
    }
}

// ---------------- N32 --------------------

/// Unsigned 32-bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct N32 {
    bits: [Bit; 32],
}

impl N32 {
    /// Creates a new `N32` from an array of 32 bits.
    pub fn new(bits: [Bit; 32]) -> Self {
        N32 { bits }
    }
    /// Returns a zero value for N32.
    pub fn zero() -> Self {
        N32 {
            bits: [Bit::Zero; 32],
        }
    }
}

impl BitCount for N32 {
    fn count_ones(&self) -> u32 {
        self.bits.iter().filter(|&bit| *bit == Bit::One).count() as u32
    }
    fn count_zeros(&self) -> u32 {
        self.bits.iter().filter(|&bit| *bit == Bit::Zero).count() as u32
    }
}

impl Display for N32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u32::from(*self))
    }
}

impl From<u32> for N32 {
    fn from(value: u32) -> Self {
        let mut bits = [Bit::Zero; 32];
        for i in 0..32 {
            bits[i] = if (value & (1 << i)) != 0 {
                Bit::One
            } else {
                Bit::Zero
            };
        }
        N32 { bits }
    }
}

impl From<N32> for u32 {
    fn from(value: N32) -> Self {
        value
            .bits
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &bit)| acc | (u8::from(bit) as u32) << i)
    }
}

impl Add for N32 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        u32::from(self).wrapping_add(u32::from(other)).into()
    }
}

impl Sub for N32 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        u32::from(self).wrapping_sub(u32::from(other)).into()
    }
}

impl SubAssign for N32 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl AddAssign for N32 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Mul for N32 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        u32::from(self).wrapping_mul(u32::from(other)).into()
    }
}

impl MulAssign for N32 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Div for N32 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        (u32::from(self) / u32::from(other)).into()
    }
}

impl DivAssign for N32 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl Rem for N32 {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        (u32::from(self) % u32::from(other)).into()
    }
}

impl RemAssign for N32 {
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}

// ---------------- N64 --------------------

/// Unsigned 64-bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct N64 {
    bits: [Bit; 64],
}

impl From<u64> for N64 {
    fn from(value: u64) -> Self {
        let mut bits = [Bit::Zero; 64];
        for i in 0..64 {
            bits[i] = if (value & (1 << i)) != 0 {
                Bit::One
            } else {
                Bit::Zero
            };
        }
        N64 { bits }
    }
}

impl From<N64> for u64 {
    fn from(value: N64) -> Self {
        value
            .bits
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &bit)| acc | (u8::from(bit) as u64) << i)
    }
}

impl Add for N64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        u64::from(self).wrapping_add(u64::from(other)).into()
    }
}

impl Sub for N64 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        u64::from(self).wrapping_sub(u64::from(other)).into()
    }
}

impl Mul for N64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        u64::from(self).wrapping_mul(u64::from(other)).into()
    }
}

impl Div for N64 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        (u64::from(self) / u64::from(other)).into()
    }
}

impl AddAssign for N64 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for N64 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Display for N64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u64::from(*self))
    }
}

// --------------------- Z8 ---------------------

/// Signed 8-bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Z8 {
    bits: [Bit; 8],
}

impl From<i8> for Z8 {
    fn from(value: i8) -> Self {
        let mut bits = [Bit::Zero; 8];
        for i in 0..8 {
            if (value & (1 << i)) != 0 {
                bits[i] = Bit::One;
            }
        }
        Z8 { bits }
    }
}

impl From<Z8> for i8 {
    fn from(value: Z8) -> Self {
        let mut result: i8 = 0;
        for i in 0..8 {
            if value.bits[i] == Bit::One {
                result |= 1 << i;
            }
        }
        result
    }
}

impl Add for Z8 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        i8::from(self).wrapping_add(i8::from(rhs)).into()
    }
}

impl Sub for Z8 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        i8::from(self).wrapping_sub(i8::from(rhs)).into()
    }
}

impl Mul for Z8 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        i8::from(self).wrapping_mul(i8::from(rhs)).into()
    }
}

impl Div for Z8 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        i8::from(self).wrapping_div(i8::from(rhs)).into()
    }
}

impl Rem for Z8 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        i8::from(self).wrapping_rem(i8::from(rhs)).into()
    }
}

impl Not for Z8 {
    type Output = Self;
    fn not(self) -> Self::Output {
        (!i8::from(self)).into()
    }
}

impl Display for Z8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", i8::from(*self))
    }
}

// --------------------- Z16 ---------------------

/// Signed 16-bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Z16 {
    bits: [Bit; 16],
}

impl From<i16> for Z16 {
    fn from(value: i16) -> Self {
        let mut bits = [Bit::Zero; 16];
        for i in 0..16 {
            if (value & (1 << i)) != 0 {
                bits[i] = Bit::One;
            }
        }
        Z16 { bits }
    }
}

impl From<Z16> for i16 {
    fn from(value: Z16) -> Self {
        let mut result: i16 = 0;
        for i in 0..16 {
            if value.bits[i] == Bit::One {
                result |= 1 << i;
            }
        }
        result
    }
}

impl Add for Z16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        i16::from(self).wrapping_add(i16::from(rhs)).into()
    }
}

impl Sub for Z16 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        i16::from(self).wrapping_sub(i16::from(rhs)).into()
    }
}

impl Mul for Z16 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        i16::from(self).wrapping_mul(i16::from(rhs)).into()
    }
}

impl Div for Z16 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        i16::from(self).wrapping_div(i16::from(rhs)).into()
    }
}

impl Rem for Z16 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        i16::from(self).wrapping_rem(i16::from(rhs)).into()
    }
}

impl Not for Z16 {
    type Output = Self;
    fn not(self) -> Self::Output {
        (!i16::from(self)).into()
    }
}

impl Display for Z16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", i16::from(*self))
    }
}

// --------------------- Z32 ---------------------

/// Signed 32-bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Z32 {
    bits: [Bit; 32],
}

impl From<i32> for Z32 {
    fn from(value: i32) -> Self {
        let mut bits = [Bit::Zero; 32];
        for i in 0..32 {
            if (value & (1 << i)) != 0 {
                bits[i] = Bit::One;
            }
        }
        Z32 { bits }
    }
}

impl From<Z32> for i32 {
    fn from(value: Z32) -> Self {
        let mut result: i32 = 0;
        for i in 0..32 {
            if value.bits[i] == Bit::One {
                result |= 1 << i;
            }
        }
        result
    }
}

impl Add for Z32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        i32::from(self).wrapping_add(i32::from(rhs)).into()
    }
}

impl Sub for Z32 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        i32::from(self).wrapping_sub(i32::from(rhs)).into()
    }
}

impl Mul for Z32 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        i32::from(self).wrapping_mul(i32::from(rhs)).into()
    }
}

impl Div for Z32 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        i32::from(self).wrapping_div(i32::from(rhs)).into()
    }
}

impl Rem for Z32 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        i32::from(self).wrapping_rem(i32::from(rhs)).into()
    }
}

impl Not for Z32 {
    type Output = Self;
    fn not(self) -> Self::Output {
        (!i32::from(self)).into()
    }
}

impl Display for Z32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", i32::from(*self))
    }
}

// --------------------- Z64 ---------------------

/// Signed 64-bit integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Z64 {
    bits: [Bit; 64],
}

impl From<i64> for Z64 {
    fn from(value: i64) -> Self {
        let mut bits = [Bit::Zero; 64];
        for i in 0..64 {
            if (value & (1 << i)) != 0 {
                bits[i] = Bit::One;
            }
        }
        Z64 { bits }
    }
}

impl From<Z64> for i64 {
    fn from(value: Z64) -> Self {
        let mut result: i64 = 0;
        for i in 0..64 {
            if value.bits[i] == Bit::One {
                result |= 1 << i;
            }
        }
        result
    }
}

impl Add for Z64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        i64::from(self).wrapping_add(i64::from(rhs)).into()
    }
}

impl Sub for Z64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        i64::from(self).wrapping_sub(i64::from(rhs)).into()
    }
}

impl Mul for Z64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        i64::from(self).wrapping_mul(i64::from(rhs)).into()
    }
}

impl Div for Z64 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        i64::from(self).wrapping_div(i64::from(rhs)).into()
    }
}

impl Rem for Z64 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        i64::from(self).wrapping_rem(i64::from(rhs)).into()
    }
}

impl Not for Z64 {
    type Output = Self;
    fn not(self) -> Self::Output {
        (!i64::from(self)).into()
    }
}

impl Display for Z64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", i64::from(*self))
    }
}

// --------------------- R32 ---------------------

/// 32-bit floating-point number.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct R32 {
    bits: [Bit; 32],
}

impl From<f32> for R32 {
    fn from(value: f32) -> Self {
        let mut bits = [Bit::Zero; 32];
        let value_bits = value.to_bits();
        for i in 0..32 {
            if (value_bits & (1 << i)) != 0 {
                bits[i] = Bit::One;
            }
        }
        R32 { bits }
    }
}

impl From<R32> for f32 {
    fn from(value: R32) -> Self {
        let mut result: u32 = 0;
        for i in 0..32 {
            if value.bits[i] == Bit::One {
                result |= 1 << i;
            }
        }
        f32::from_bits(result)
    }
}

impl Add for R32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (f32::from(self) + f32::from(rhs)).into()
    }
}

impl Sub for R32 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (f32::from(self) - f32::from(rhs)).into()
    }
}

impl Mul for R32 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (f32::from(self) * f32::from(rhs)).into()
    }
}

impl Div for R32 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        (f32::from(self) / f32::from(rhs)).into()
    }
}

impl Rem for R32 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        (f32::from(self) % f32::from(rhs)).into()
    }
}

impl Display for R32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", f32::from(*self))
    }
}

// --------------------- R64 ---------------------

/// 64-bit floating-point number.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct R64 {
    bits: [Bit; 64],
}

impl From<f64> for R64 {
    fn from(value: f64) -> Self {
        let mut bits = [Bit::Zero; 64];
        let value_bits = value.to_bits();
        for i in 0..64 {
            if (value_bits & (1 << i)) != 0 {
                bits[i] = Bit::One;
            }
        }
        R64 { bits }
    }
}

impl From<R64> for f64 {
    fn from(value: R64) -> Self {
        let mut result: u64 = 0;
        for i in 0..64 {
            if value.bits[i] == Bit::One {
                result |= 1 << i;
            }
        }
        f64::from_bits(result)
    }
}

impl Add for R64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (f64::from(self) + f64::from(rhs)).into()
    }
}

impl Sub for R64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (f64::from(self) - f64::from(rhs)).into()
    }
}

impl Mul for R64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (f64::from(self) * f64::from(rhs)).into()
    }
}

impl Div for R64 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        (f64::from(self) / f64::from(rhs)).into()
    }
}

impl Rem for R64 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        (f64::from(self) % f64::from(rhs)).into()
    }
}

impl Display for R64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", f64::from(*self))
    }
}
