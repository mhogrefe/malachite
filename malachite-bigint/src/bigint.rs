use derive_more::{Binary, Display, From, Into, LowerHex, Octal, UpperHex};
use malachite::{
    base::{
        num::{
            arithmetic::traits::{
                Abs, DivRem, DivRound, DivisibleBy, FloorRoot, Mod, Parity, UnsignedAbs,
            },
            conversion::traits::{RoundingInto, ToStringBase},
            logic::traits::BitAccess,
        },
        rounding_modes::RoundingMode,
    },
    Integer,
};
use num_integer::Roots;
use num_traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Num, One, Pow, Signed,
    ToPrimitive, Zero,
};
use paste::paste;
use std::{
    cmp::Ordering,
    fmt::Debug,
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
    str::FromStr,
};

use crate::{
    BigUint, ParseBigIntError,
    Sign::{Minus, NoSign, Plus},
    ToBigUint, TryFromBigIntError, U32Digits, U64Digits,
};

pub trait ToBigInt {
    fn to_bigint(&self) -> Option<BigInt>;
}

apply_to_primitives!(impl_primitive_convert{BigInt, _});
impl_primitive_convert!(BigInt, f32);
impl_primitive_convert!(BigInt, f64);

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Debug, Hash)]
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

impl Neg for Sign {
    type Output = Sign;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Minus => Plus,
            NoSign => NoSign,
            Plus => Minus,
        }
    }
}

#[repr(transparent)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Display,
    Binary,
    Octal,
    LowerHex,
    UpperHex,
    From,
    Into,
)]
#[display("{}", self.0)]
#[into(owned, ref, ref_mut)]
pub struct BigInt(Integer);

apply_to_primitives!(forward_from{BigInt, _});
apply_to_primitives!(forward_try_into{BigInt, _});

forward_unary_op!(BigInt, Not, not);
forward_unary_op!(BigInt, Neg, neg);

forward_binary_self!(BigInt, Add, add);
forward_binary_self!(BigInt, Sub, sub);
forward_binary_self!(BigInt, Mul, mul);
forward_binary_self!(BigInt, Div, div);
forward_binary_self!(BigInt, Rem, rem);
forward_binary_self!(BigInt, BitAnd, bitand);
forward_binary_self!(BigInt, BitOr, bitor);
forward_binary_self!(BigInt, BitXor, bitxor);

forward_assign_self!(BigInt, AddAssign, add_assign);
forward_assign_self!(BigInt, SubAssign, sub_assign);
forward_assign_self!(BigInt, MulAssign, mul_assign);
forward_assign_self!(BigInt, DivAssign, div_assign);
forward_assign_self!(BigInt, RemAssign, rem_assign);
forward_assign_self!(BigInt, BitAndAssign, bitand_assign);
forward_assign_self!(BigInt, BitOrAssign, bitor_assign);
forward_assign_self!(BigInt, BitXorAssign, bitxor_assign);

forward_pow_biguint!(BigInt);

apply_to_primitives!(forward_binary_right_primitive_into{BigInt, _, Add, add});
apply_to_primitives!(forward_binary_right_primitive_into{BigInt, _, Sub, sub});
apply_to_primitives!(forward_binary_right_primitive_into{BigInt, _, Mul, mul});
apply_to_primitives!(forward_binary_right_primitive_into{BigInt, _, Div, div});
apply_to_primitives!(forward_binary_right_primitive_into{BigInt, _, Rem, rem});

apply_to_primitives!(forward_binary_left_primitive_into{_, BigInt, Add, add});
apply_to_primitives!(forward_binary_left_primitive_into{_, BigInt, Sub, sub});
apply_to_primitives!(forward_binary_left_primitive_into{_, BigInt, Mul, mul});
apply_to_primitives!(forward_binary_left_primitive_into{_, BigInt, Div, div});
apply_to_primitives!(forward_binary_left_primitive_into{_, BigInt, Rem, rem});

apply_to_primitives!(forward_binary_right_primitive{BigInt, _, Shl, shl});
apply_to_primitives!(forward_binary_right_primitive{BigInt, _, Shr, shr});

apply_to_primitives!(forward_assign_primitive_into{BigInt, _, AddAssign, add_assign});
apply_to_primitives!(forward_assign_primitive_into{BigInt, _, SubAssign, sub_assign});
apply_to_primitives!(forward_assign_primitive_into{BigInt, _, MulAssign, mul_assign});
apply_to_primitives!(forward_assign_primitive_into{BigInt, _, DivAssign, div_assign});
apply_to_primitives!(forward_assign_primitive_into{BigInt, _, RemAssign, rem_assign});

apply_to_primitives!(forward_assign_primitive{BigInt, _, ShlAssign, shl_assign});
apply_to_primitives!(forward_assign_primitive{BigInt, _, ShrAssign, shr_assign});

apply_to_unsigneds!(forward_pow_primitive{BigInt, _});

impl_product_iter_type!(BigInt);
impl_sum_iter_type!(BigInt);

impl std::fmt::Debug for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl CheckedAdd for BigInt {
    #[inline]
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(self.add(v))
    }
}

impl CheckedSub for BigInt {
    #[inline]
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        Some(self.sub(v))
    }
}

impl CheckedMul for BigInt {
    #[inline]
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Some(self.mul(v))
    }
}

impl CheckedDiv for BigInt {
    #[inline]
    fn checked_div(&self, v: &Self) -> Option<Self> {
        (!v.is_zero()).then(|| self.div(v))
    }
}

impl ToBigInt for BigInt {
    #[inline]
    fn to_bigint(&self) -> Option<BigInt> {
        Some(self.clone())
    }
}

impl ToBigUint for BigInt {
    #[inline]
    fn to_biguint(&self) -> Option<BigUint> {
        (!self.is_negative()).then(|| self.magnitude().clone())
    }
}

impl ToPrimitive for BigInt {
    apply_to_primitives!(impl_to_primitive_fn_try_into{_});
    impl_to_primitive_fn_float!(f32);
    impl_to_primitive_fn_float!(f64);
}

impl FromPrimitive for BigInt {
    apply_to_primitives!(impl_from_primitive_fn_infallible{_});
    impl_from_primitive_fn_float!(f32);
    impl_from_primitive_fn_float!(f64);
}

impl From<BigUint> for BigInt {
    #[inline]
    fn from(value: BigUint) -> Self {
        Integer::from(value.0).into()
    }
}

impl Zero for BigInt {
    #[inline]
    fn zero() -> Self {
        Self(<Integer as malachite::base::num::basic::traits::Zero>::ZERO)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.sign() == NoSign
    }
}

impl One for BigInt {
    #[inline]
    fn one() -> Self {
        Self(<Integer as malachite::base::num::basic::traits::One>::ONE)
    }
}

impl Signed for BigInt {
    #[inline]
    fn abs(&self) -> Self {
        (&self.0).abs().into()
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        if self <= other {
            Self::zero()
        } else {
            self - other
        }
    }

    #[inline]
    fn signum(&self) -> Self {
        match self.sign() {
            Minus => -Self::one(),
            NoSign => Self::zero(),
            Plus => Self::one(),
        }
    }

    #[inline]
    fn is_positive(&self) -> bool {
        self.sign() == Plus
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.sign() == Minus
    }
}

impl Num for BigInt {
    type FromStrRadixErr = ParseBigIntError;

    #[inline]
    fn from_str_radix(mut s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let sign = if s.starts_with('-') {
            let tail = &s[1..];
            if !tail.starts_with('+') {
                s = tail
            }
            Minus
        } else {
            Plus
        };
        let u = BigUint::from_str_radix(s, radix)?;
        Ok(BigInt::from_biguint(sign, u))
    }
}

impl num_integer::Integer for BigInt {
    #[inline]
    fn div_floor(&self, other: &Self) -> Self {
        (&self.0).div_round(&other.0, RoundingMode::Floor).0.into()
    }

    #[inline]
    fn mod_floor(&self, other: &Self) -> Self {
        (&self.0).mod_op(&other.0).into()
    }

    #[inline]
    fn gcd(&self, other: &Self) -> Self {
        self.magnitude().gcd(other.magnitude()).into()
    }

    #[inline]
    fn lcm(&self, other: &Self) -> Self {
        self.magnitude().lcm(other.magnitude()).into()
    }

    #[inline]
    fn divides(&self, other: &Self) -> bool {
        Self::is_multiple_of(self, other)
    }

    #[inline]
    fn is_multiple_of(&self, other: &Self) -> bool {
        (&self.0).divisible_by(&other.0)
    }

    #[inline]
    fn is_even(&self) -> bool {
        self.0.even()
    }

    #[inline]
    fn is_odd(&self) -> bool {
        self.0.odd()
    }

    #[inline]
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (div, rem) = (&self.0).div_rem(&other.0);
        (div.into(), rem.into())
    }
}

impl Roots for BigInt {
    #[inline]
    fn nth_root(&self, n: u32) -> Self {
        (&self.0).floor_root(n as u64).into()
    }
}

impl FromStr for BigInt {
    type Err = ParseBigIntError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(s, 10)
    }
}

impl BigInt {
    #[inline]
    pub fn new(sign: Sign, digits: Vec<u32>) -> Self {
        Self::from_biguint(sign, BigUint::new(digits))
    }

    #[inline]
    pub fn from_biguint(sign: Sign, mut abs: BigUint) -> Self {
        if sign == NoSign {
            abs = BigUint::zero();
        }

        Integer::from_sign_and_abs(sign != Minus, abs.0).into()
    }

    #[inline]
    pub fn from_slice(sign: Sign, slice: &[u32]) -> Self {
        Self::from_biguint(sign, BigUint::from_slice(slice))
    }

    #[inline]
    pub fn assign_from_slice(&mut self, sign: Sign, slice: &[u32]) {
        if sign == NoSign {
            self.set_zero();
        } else {
            *self = Self::from_slice(sign, slice);
        }
    }

    #[inline]
    pub fn from_bytes_be(sign: Sign, bytes: &[u8]) -> Self {
        Self::from_biguint(sign, BigUint::from_bytes_be(bytes))
    }

    #[inline]
    pub fn from_bytes_le(sign: Sign, bytes: &[u8]) -> Self {
        Self::from_biguint(sign, BigUint::from_bytes_le(bytes))
    }

    #[inline]
    pub fn from_signed_bytes_be(digits: &[u8]) -> Self {
        let is_negative = match digits.first().cloned() {
            Some(x) => x > 0x7f,
            None => return Self::zero(),
        };

        if is_negative {
            let mut v = Vec::from(digits);
            twos_complement_be(&mut v);
            let u = BigUint::from_bytes_be(v.as_slice());
            Self::from_biguint(Minus, u)
        } else {
            let u = BigUint::from_bytes_be(digits);
            Self::from_biguint(Plus, u)
        }
    }

    #[inline]
    pub fn from_signed_bytes_le(digits: &[u8]) -> Self {
        let is_negative = match digits.last().cloned() {
            Some(x) => x > 0x7f,
            None => return Self::zero(),
        };

        if is_negative {
            let mut v = Vec::from(digits);
            twos_complement_le(&mut v);
            let u = BigUint::from_bytes_le(v.as_slice());
            Self::from_biguint(Minus, u)
        } else {
            let u = BigUint::from_bytes_le(digits);
            Self::from_biguint(Plus, u)
        }
    }

    #[inline]
    pub fn parse_bytes(bytes: &[u8], radix: u32) -> Option<Self> {
        let s = std::str::from_utf8(bytes).ok()?;
        Self::from_str_radix(s, radix).ok()
    }

    #[inline]
    pub fn from_radix_be(sign: Sign, buf: &[u8], radix: u32) -> Option<Self> {
        BigUint::from_radix_be(buf, radix).map(|u| Self::from_biguint(sign, u))
    }

    #[inline]
    pub fn from_radix_le(sign: Sign, buf: &[u8], radix: u32) -> Option<Self> {
        BigUint::from_radix_le(buf, radix).map(|u| Self::from_biguint(sign, u))
    }

    #[inline]
    pub fn to_bytes_be(&self) -> (Sign, Vec<u8>) {
        (self.sign(), self.magnitude().to_bytes_be())
    }

    #[inline]
    pub fn to_bytes_le(&self) -> (Sign, Vec<u8>) {
        (self.sign(), self.magnitude().to_bytes_le())
    }

    #[inline]
    pub fn to_u32_digits(&self) -> (Sign, Vec<u32>) {
        (self.sign(), self.magnitude().to_u32_digits())
    }

    #[inline]
    pub fn to_u64_digits(&self) -> (Sign, Vec<u64>) {
        (self.sign(), self.magnitude().to_u64_digits())
    }

    #[inline]
    pub fn iter_u32_digits(&self) -> U32Digits {
        self.magnitude().iter_u32_digits()
    }

    #[inline]
    pub fn iter_u64_digits(&self) -> U64Digits {
        self.magnitude().iter_u64_digits()
    }

    #[inline]
    pub fn to_signed_bytes_be(&self) -> Vec<u8> {
        let mut bytes = self.magnitude().to_bytes_be();
        let first_byte = bytes.first().cloned().unwrap_or(0);
        let is_negative = self.is_negative();
        if first_byte > 0x7f
            && !(first_byte == 0x80 && bytes.iter().skip(1).all(Zero::is_zero) && is_negative)
        {
            // msb used by magnitude, extend by 1 byte
            bytes.insert(0, 0);
        }
        if self.is_negative() {
            twos_complement_be(&mut bytes);
        }
        bytes
    }

    #[inline]
    pub fn to_signed_bytes_le(&self) -> Vec<u8> {
        let mut bytes = self.magnitude().to_bytes_le();
        let is_negative = self.is_negative();
        let last_byte = bytes.last().cloned().unwrap_or(0);
        if last_byte > 0x7f
            && !(last_byte == 0x80 && bytes.iter().rev().skip(1).all(Zero::is_zero) && is_negative)
        {
            // msb used by magnitude, extend by 1 byte
            bytes.push(0);
        }
        if self.is_negative() {
            twos_complement_le(&mut bytes);
        }
        bytes
    }

    #[inline]
    pub fn to_str_radix(&self, radix: u32) -> String {
        self.0.to_string_base(radix as u8)
    }

    #[inline]
    pub fn to_radix_be(&self, radix: u32) -> (Sign, Vec<u8>) {
        (self.sign(), self.magnitude().to_radix_be(radix))
    }

    #[inline]
    pub fn to_radix_le(&self, radix: u32) -> (Sign, Vec<u8>) {
        (self.sign(), self.magnitude().to_radix_le(radix))
    }

    #[inline]
    pub fn sign(&self) -> Sign {
        match <_ as malachite::base::num::arithmetic::traits::Sign>::sign(&self.0) {
            Ordering::Less => Minus,
            Ordering::Equal => NoSign,
            Ordering::Greater => Plus,
        }
    }

    #[inline]
    pub fn magnitude(&self) -> &BigUint {
        unsafe { std::mem::transmute(self.0.unsigned_abs_ref()) }
    }

    #[inline]
    pub fn into_parts(self) -> (Sign, BigUint) {
        (self.sign(), self.0.unsigned_abs().into())
    }

    #[inline]
    pub fn bits(&self) -> u64 {
        self.magnitude().bits()
    }

    #[inline]
    pub fn to_biguint(&self) -> Option<BigUint> {
        match self.sign() {
            Plus => Some(self.magnitude().clone()),
            NoSign => Some(BigUint::zero()),
            Minus => None,
        }
    }

    #[inline]
    pub fn checked_add(&self, v: &BigInt) -> Option<BigInt> {
        Some(self + v)
    }

    #[inline]
    pub fn checked_sub(&self, v: &BigInt) -> Option<BigInt> {
        Some(self - v)
    }

    #[inline]
    pub fn checked_mul(&self, v: &BigInt) -> Option<BigInt> {
        Some(self * v)
    }

    #[inline]
    pub fn checked_div(&self, v: &BigInt) -> Option<BigInt> {
        if v.is_zero() {
            return None;
        }
        Some(self / v)
    }

    #[inline]
    pub fn pow(&self, exponent: u32) -> Self {
        Pow::pow(self, exponent)
    }

    #[inline]
    pub fn modpow(&self, exponent: &Self, modulus: &Self) -> Self {
        assert!(
            !exponent.is_negative(),
            "negative exponentiation is not supported!"
        );
        assert!(
            !modulus.is_zero(),
            "attempt to calculate with zero modulus!"
        );

        let mut abs = self
            .magnitude()
            .modpow(exponent.magnitude(), modulus.magnitude());

        if abs.is_zero() {
            return Self::zero();
        }

        if (self.is_negative() && exponent.0.odd()) != modulus.is_negative() {
            abs = modulus.magnitude() - abs;
        }

        Self::from_biguint(modulus.sign(), abs)
    }

    #[inline]
    pub fn sqrt(&self) -> Self {
        Roots::sqrt(self)
    }

    #[inline]
    pub fn cbrt(&self) -> Self {
        Roots::cbrt(self)
    }

    #[inline]
    pub fn nth_root(&self, n: u32) -> Self {
        Roots::nth_root(self, n)
    }

    #[inline]
    pub fn trailing_zeros(&self) -> Option<u64> {
        self.magnitude().trailing_zeros()
    }

    #[inline]
    pub fn bit(&self, bit: u64) -> bool {
        self.0.get_bit(bit)
    }

    #[inline]
    pub fn set_bit(&mut self, bit: u64, value: bool) {
        if value {
            self.0.set_bit(bit)
        } else {
            self.0.clear_bit(bit)
        }
    }
}
/// Perform in-place two's complement of the given binary representation,
/// in little-endian byte order.
#[inline]
fn twos_complement_le(digits: &mut [u8]) {
    twos_complement(digits)
}

/// Perform in-place two's complement of the given binary representation
/// in big-endian byte order.
#[inline]
fn twos_complement_be(digits: &mut [u8]) {
    twos_complement(digits.iter_mut().rev())
}

/// Perform in-place two's complement of the given digit iterator
/// starting from the least significant byte.
#[inline]
fn twos_complement<'a, I>(digits: I)
where
    I: IntoIterator<Item = &'a mut u8>,
{
    let mut carry = true;
    for d in digits {
        *d = !*d;
        if carry {
            *d = d.wrapping_add(1);
            carry = d.is_zero();
        }
    }
}

#[test]
fn test_float_convert_nearest() {
    let n25 = "10000000000000000000000000";
    let val = BigInt::from_str(n25).unwrap();
    let f = val.to_f64().unwrap();
    assert_eq!(f.to_string(), n25);
}

#[test]
fn test_to_signed_bytes() {
    let sysmax = i64::MAX;
    let i = BigInt::from(sysmax);
    let b = i.to_signed_bytes_le();
    let i2 = BigInt::from_signed_bytes_le(&b);
    assert_eq!(i, i2);
}

#[test]
fn test_display_bigint() {
    let n = BigInt::from_str("1234567890").unwrap();
    assert_eq!(format!("{}", n), "1234567890");
}
