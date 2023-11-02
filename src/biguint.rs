use derive_more::{Binary, Display, From, Into, LowerHex, Octal, UpperHex};
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
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, FromPrimitive, Num, One, Pow, ToPrimitive,
    Unsigned, Zero,
};
use paste::paste;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    str::FromStr,
};

use crate::{ParseBigIntError, ToBigInt, TryFromBigIntError, U32Digits, U64Digits};

pub trait ToBigUint {
    fn to_biguint(&self) -> Option<BigUint>;
}

apply_to_primitives!(impl_primitive_convert{BigUint, _});
impl_primitive_convert!(BigUint, f32);
impl_primitive_convert!(BigUint, f64);

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
#[display(fmt = "{}", "self.0")]
#[into(owned, ref, ref_mut)]
pub struct BigUint(pub(crate) Natural);

apply_to_unsigneds!(forward_from{BigUint, _});
apply_to_signeds!(forward_try_from{BigUint, _});
apply_to_primitives!(forward_try_into{BigUint, _});

forward_binary_self!(BigUint, Add, add);
forward_binary_self!(BigUint, Sub, sub);
forward_binary_self!(BigUint, Mul, mul);
forward_binary_self!(BigUint, Div, div);
forward_binary_self!(BigUint, Rem, rem);
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

forward_pow_biguint!(BigUint);

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

impl_product_iter_type!(BigUint);
impl_sum_iter_type!(BigUint);

impl std::fmt::Debug for BigUint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl CheckedAdd for BigUint {
    #[inline]
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(self.add(v))
    }
}

impl CheckedSub for BigUint {
    #[inline]
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        match self.cmp(v) {
            Less => None,
            Equal => Some(Self::zero()),
            Greater => Some(self.sub(v)),
        }
    }
}

impl CheckedMul for BigUint {
    #[inline]
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Some(self.mul(v))
    }
}

impl CheckedDiv for BigUint {
    #[inline]
    fn checked_div(&self, v: &Self) -> Option<Self> {
        (!v.is_zero()).then(|| self.div(v))
    }
}

impl ToBigUint for BigUint {
    #[inline]
    fn to_biguint(&self) -> Option<BigUint> {
        Some(self.clone())
    }
}

impl ToBigInt for BigUint {
    #[inline]
    fn to_bigint(&self) -> Option<crate::BigInt> {
        Some(malachite::Integer::from(&self.0).into())
    }
}

impl ToPrimitive for BigUint {
    apply_to_primitives!(impl_to_primitive_fn_try_into{_});
    impl_to_primitive_fn_float!(f32);
    impl_to_primitive_fn_float!(f64);
}

impl FromPrimitive for BigUint {
    apply_to_signeds!(impl_from_primitive_fn_try_from{_});
    apply_to_unsigneds!(impl_from_primitive_fn_infallible{_});
    impl_from_primitive_fn_float!(f32);
    impl_from_primitive_fn_float!(f64);
}

impl Zero for BigUint {
    #[inline]
    fn zero() -> Self {
        Self(<Natural as malachite::num::basic::traits::Zero>::ZERO)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl One for BigUint {
    #[inline]
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

        // FIXME: workaround, remove the check if malachite issue fixed
        // https://github.com/mhogrefe/malachite/issues/20
        if radix == 16
            && s.bytes().any(|x| {
                !matches!(x,
                    b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b'_'
                )
            })
        {
            return Err(ParseBigIntError::invalid());
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
            .ok_or_else(ParseBigIntError::invalid)
    }
}

impl num_integer::Integer for BigUint {
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
        (&self.0).gcd(&other.0).into()
    }

    #[inline]
    fn lcm(&self, other: &Self) -> Self {
        (&self.0).lcm(&other.0).into()
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

impl Roots for BigUint {
    #[inline]
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
    #[inline]
    pub fn new(digits: Vec<u32>) -> Self {
        Self::from_slice(digits.as_slice())
    }

    #[inline]
    pub fn from_slice(slice: &[u32]) -> Self {
        let mut uint = BigUint::zero();
        uint.assign_from_slice(slice);
        uint
    }

    #[inline]
    pub fn assign_from_slice(&mut self, slice: &[u32]) {
        // SAFETY: &[u32] cannot have any digit greater than 2^32
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
    pub fn parse_bytes(bytes: &[u8], radix: u32) -> Option<Self> {
        let s = std::str::from_utf8(bytes).ok()?;
        Self::from_str_radix(s, radix).ok()
    }

    #[inline]
    pub fn from_radix_be(bytes: &[u8], radix: u32) -> Option<Self> {
        if radix == 256 {
            Some(Self::from_bytes_be(bytes))
        } else {
            Natural::from_digits_desc(&(radix as u8), bytes.iter().cloned()).map(Self)
        }
    }

    #[inline]
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

    #[inline]
    pub fn iter_u32_digits(&self) -> U32Digits {
        U32Digits::new(self.0.limbs())
    }

    #[inline]
    pub fn iter_u64_digits(&self) -> U64Digits {
        U64Digits::new(self.0.limbs())
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

    #[inline]
    pub fn pow(&self, exponent: u32) -> Self {
        Pow::pow(self, exponent)
    }

    #[inline]
    pub fn modpow(&self, exponent: &Self, modulus: &Self) -> Self {
        if self >= modulus {
            let x = self % modulus;
            x.0.mod_pow(&exponent.0, &modulus.0).into()
        } else {
            (&self.0).mod_pow(&exponent.0, &modulus.0).into()
        }
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

#[test]
fn test_from_string_base() {
    assert!(BigUint::from_str_radix("1000000000000000111111100112abcdefg", 16).is_err());
}
