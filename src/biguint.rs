use derive_more::{Binary, From, LowerHex, Octal, UpperHex};
use malachite::{
    num::{
        arithmetic::traits::{
            DivRem, DivRound, DivisibleBy, FloorRoot, Gcd, Lcm, Mod, ModPow, Parity,
        },
        conversion::traits::{Digits, FromStringBase, PowerOf2Digits, RoundingInto, ToStringBase},
        logic::traits::{BitAccess, BitIterable, CountOnes, SignificantBits},
    },
    rounding_modes::RoundingMode,
    Natural,
};
use num_integer::Roots;
use num_traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Num, One, Pow, ToPrimitive, Unsigned, Zero,
};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    str::FromStr,
};

use crate::{ParseBigIntError, TryFromBigIntError};

pub trait ToBigUint {
    fn to_biguint(&self) -> Option<BigUint>;
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Debug,
    Binary,
    Octal,
    LowerHex,
    UpperHex,
    From,
)]
pub struct BigUint(Natural);

apply_to_unsigneds!(forward_from{BigUint, _});
apply_to_signeds!(forward_try_from{BigUint, _});

forward_binary_self!(BigUint, Add, add);
forward_binary_self!(BigUint, Sub, sub);
forward_binary_self!(BigUint, Mul, mul);
forward_binary_self!(BigUint, Div, div);
forward_binary_self!(BigUint, Rem, rem);
// // forward_binary_self!(BigUint, Pow, pow, malachite::num::arithmetic::traits::Pow::pow);
forward_binary_self!(BigUint, BitAnd, bitand);
forward_binary_self!(BigUint, BitOr, bitor);
forward_binary_self!(BigUint, BitXor, bitxor);

forward_assign_self!(BigUint, AddAssign, add_assign);
forward_assign_self!(BigUint, SubAssign, sub_assign);
forward_assign_self!(BigUint, MulAssign, mul_assign);
forward_assign_self!(BigUint, DivAssign, div_assign);
forward_assign_self!(BigUint, RemAssign, rem_assign);
forward_assign_self!(BigUint, BitAndAssign, bitand_assign);
forward_assign_self!(BigUint, BitOrAssign, bitor_assign);
forward_assign_self!(BigUint, BitXorAssign, bitxor_assign);

apply_to_unsigneds!(forward_binary_right_primitive_into{BigUint, _, Add, add});
apply_to_unsigneds!(forward_binary_right_primitive_into{BigUint, _, Sub, sub});
apply_to_unsigneds!(forward_binary_right_primitive_into{BigUint, _, Mul, mul});
apply_to_unsigneds!(forward_binary_right_primitive_into{BigUint, _, Div, div});
apply_to_unsigneds!(forward_binary_right_primitive_into{BigUint, _, Rem, rem});

apply_to_unsigneds!(forward_binary_left_primitive_into{_, BigUint, Add, add});
apply_to_unsigneds!(forward_binary_left_primitive_into{_, BigUint, Sub, sub});
apply_to_unsigneds!(forward_binary_left_primitive_into{_, BigUint, Mul, mul});
apply_to_unsigneds!(forward_binary_left_primitive_into{_, BigUint, Div, div});
apply_to_unsigneds!(forward_binary_left_primitive_into{_, BigUint, Rem, rem});

apply_to_primitives!(forward_binary_right_primitive{BigUint, _, Shl, shl});
apply_to_primitives!(forward_binary_right_primitive{BigUint, _, Shr, shr});

apply_to_unsigneds!(forward_assign_primitive_into{BigUint, _, AddAssign, add_assign});
apply_to_unsigneds!(forward_assign_primitive_into{BigUint, _, SubAssign, sub_assign});
apply_to_unsigneds!(forward_assign_primitive_into{BigUint, _, MulAssign, mul_assign});
apply_to_unsigneds!(forward_assign_primitive_into{BigUint, _, DivAssign, div_assign});
apply_to_unsigneds!(forward_assign_primitive_into{BigUint, _, RemAssign, rem_assign});

apply_to_primitives!(forward_assign_primitive{BigUint, _, ShlAssign, shl_assign});
apply_to_primitives!(forward_assign_primitive{BigUint, _, ShrAssign, shr_assign});

apply_to_unsigneds!(forward_pow_primitive{BigUint, _});
// TODO: pow self

impl CheckedAdd for BigUint {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(self.add(v))
    }
}

impl CheckedSub for BigUint {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        match self.cmp(v) {
            Less => None,
            Equal => Some(Self::zero()),
            Greater => Some(self.sub(v)),
        }
    }
}

impl CheckedMul for BigUint {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Some(self.mul(v))
    }
}

impl CheckedDiv for BigUint {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        (!v.is_zero()).then(|| self.div(v))
    }
}

impl ToBigUint for BigUint {
    fn to_biguint(&self) -> Option<BigUint> {
        Some(self.clone())
    }
}

impl ToPrimitive for BigUint {
    fn to_i64(&self) -> Option<i64> {
        (&self.0).try_into().ok()
    }

    fn to_u64(&self) -> Option<u64> {
        (&self.0).try_into().ok()
    }

    fn to_isize(&self) -> Option<isize> {
        (&self.0).try_into().ok()
    }

    fn to_i8(&self) -> Option<i8> {
        (&self.0).try_into().ok()
    }

    fn to_i16(&self) -> Option<i16> {
        (&self.0).try_into().ok()
    }

    fn to_i32(&self) -> Option<i32> {
        (&self.0).try_into().ok()
    }

    fn to_i128(&self) -> Option<i128> {
        (&self.0).try_into().ok()
    }

    fn to_usize(&self) -> Option<usize> {
        (&self.0).try_into().ok()
    }

    fn to_u8(&self) -> Option<u8> {
        (&self.0).try_into().ok()
    }

    fn to_u16(&self) -> Option<u16> {
        (&self.0).try_into().ok()
    }

    fn to_u32(&self) -> Option<u32> {
        (&self.0).try_into().ok()
    }

    fn to_u128(&self) -> Option<u128> {
        (&self.0).try_into().ok()
    }

    fn to_f32(&self) -> Option<f32> {
        // FIXME: correctness?
        let val: f32 = (&self.0).rounding_into(RoundingMode::Down);
        if val == f32::MAX || val == f32::MIN {
            (self.0 == val).then_some(val)
        } else {
            Some(val)
        }
    }

    fn to_f64(&self) -> Option<f64> {
        // FIXME: correctness?
        let val: f64 = (&self.0).rounding_into(RoundingMode::Down);
        if val == f64::MAX || val == f64::MIN {
            (self.0 == val).then_some(val)
        } else {
            Some(val)
        }
    }
}

impl Zero for BigUint {
    fn zero() -> Self {
        Self(<Natural as malachite::num::basic::traits::Zero>::ZERO)
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl One for BigUint {
    fn one() -> Self {
        Self(<Natural as malachite::num::basic::traits::One>::ONE)
    }
}

impl Unsigned for BigUint {}

impl Num for BigUint {
    type FromStrRadixErr = ParseBigIntError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let mut s = s;
        if s.starts_with('+') {
            let tail = &s[1..];
            if !tail.starts_with('+') {
                s = tail
            }
        }

        // fast path
        if let Some(val) = Natural::from_string_base(radix as u8, s) {
            return Ok(val.into());
        }

        if s.is_empty() {
            return Err(ParseBigIntError::empty());
        }

        if s.starts_with('_') {
            // Must lead with a real digit!
            return Err(ParseBigIntError::invalid());
        }

        let v: Vec<u8> = s.bytes().filter(|&x| x != b'_').collect();
        let s = std::str::from_utf8(v.as_slice()).map_err(|_| ParseBigIntError::invalid())?;
        Natural::from_string_base(radix as u8, s)
            .map(Self)
            .ok_or_else(|| ParseBigIntError::invalid())
    }
}

impl num_integer::Integer for BigUint {
    fn div_floor(&self, other: &Self) -> Self {
        (&self.0).div_round(&other.0, RoundingMode::Floor).into()
    }

    fn mod_floor(&self, other: &Self) -> Self {
        (&self.0).mod_op(&other.0).into()
    }

    fn gcd(&self, other: &Self) -> Self {
        (&self.0).gcd(&other.0).into()
    }

    fn lcm(&self, other: &Self) -> Self {
        (&self.0).lcm(&other.0).into()
    }

    fn divides(&self, other: &Self) -> bool {
        Self::is_multiple_of(self, other)
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        (&self.0).divisible_by(&other.0)
    }

    fn is_even(&self) -> bool {
        self.0.even()
    }

    fn is_odd(&self) -> bool {
        self.0.odd()
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (div, rem) = (&self.0).div_rem(&other.0);
        (div.into(), rem.into())
    }
}

impl Roots for BigUint {
    fn nth_root(&self, n: u32) -> Self {
        (&self.0).floor_root(n as u64).into()
    }
}

impl FromStr for BigUint {
    type Err = ParseBigIntError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(s, 10)
    }
}

impl BigUint {
    pub fn new(digits: Vec<u32>) -> Self {
        Self::from_slice(digits.as_slice())
    }

    #[inline]
    pub fn from_slice(slice: &[u32]) -> Self {
        let mut uint = BigUint::zero();
        uint.assign_from_slice(slice);
        uint
    }

    pub fn assign_from_slice(&mut self, slice: &[u32]) {
        self.0 = unsafe {
            Natural::from_power_of_2_digits_asc(32, slice.iter().cloned()).unwrap_unchecked()
        };
    }

    #[inline]
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        // SAFETY: &[u8] cannot have any digit greater than 2^8
        Self(unsafe {
            Natural::from_power_of_2_digits_desc(8, bytes.iter().cloned()).unwrap_unchecked()
        })
    }

    #[inline]
    pub fn from_bytes_le(bytes: &[u8]) -> Self {
        // SAFETY: &[u8] cannot have any digit greater than 2^8
        Self(unsafe {
            Natural::from_power_of_2_digits_asc(8, bytes.iter().cloned()).unwrap_unchecked()
        })
    }

    #[inline]
    pub fn parse_bytes(bytes: &[u8], radix: u32) -> Option<BigUint> {
        let s = std::str::from_utf8(bytes).ok()?;
        Self::from_str_radix(s, radix).ok()
    }

    pub fn from_radix_be(bytes: &[u8], radix: u32) -> Option<Self> {
        if radix == 256 {
            Some(Self::from_bytes_be(bytes))
        } else {
            Natural::from_digits_desc(&(radix as u8), bytes.iter().cloned()).map(Self)
        }
    }

    pub fn from_radix_le(bytes: &[u8], radix: u32) -> Option<Self> {
        if radix == 256 {
            Some(Self::from_bytes_le(bytes))
        } else {
            Natural::from_digits_asc(&(radix as u8), bytes.iter().cloned()).map(Self)
        }
    }

    #[inline]
    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.0.to_power_of_2_digits_desc(8)
    }

    #[inline]
    pub fn to_bytes_le(&self) -> Vec<u8> {
        self.0.to_power_of_2_digits_asc(8)
    }

    #[inline]
    pub fn to_u32_digits(&self) -> Vec<u32> {
        self.0.to_power_of_2_digits_asc(32)
    }

    #[inline]
    pub fn to_u64_digits(&self) -> Vec<u64> {
        self.0.to_limbs_asc()
    }

    pub fn iter_u32_digits(&self) {
        todo!()
    }

    pub fn iter_u64_digits(&self) {
        todo!()
    }

    #[inline]
    pub fn to_str_radix(&self, radix: u32) -> String {
        self.0.to_string_base(radix as u8)
    }

    #[inline]
    pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
        debug_assert!(radix <= 256);
        if radix == 256 {
            self.to_bytes_be()
        } else {
            self.0.to_digits_desc(&(radix as u8))
        }
    }

    #[inline]
    pub fn to_radix_le(&self, radix: u32) -> Vec<u8> {
        debug_assert!(radix <= 256);
        if radix == 256 {
            self.to_bytes_le()
        } else {
            self.0.to_digits_asc(&(radix as u8))
        }
    }

    #[inline]
    pub fn bits(&self) -> u64 {
        self.0.significant_bits()
    }

    pub fn pow(&self, exponent: u32) -> Self {
        Pow::pow(self, exponent)
    }

    pub fn modpow(&self, exponent: &Self, modulus: &Self) -> Self {
        (&self.0).mod_pow(&exponent.0, &modulus.0).into()
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
        self.0.trailing_zeros()
    }

    #[inline]
    pub fn trailing_ones(&self) -> u64 {
        self.0.bits().take_while(|&x| x).count() as u64
    }

    #[inline]
    pub fn count_ones(&self) -> u64 {
        self.0.count_ones()
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
