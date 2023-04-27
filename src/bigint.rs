use derive_more::{Binary, LowerHex, Octal, UpperHex, From};
use malachite::Integer;
use num_integer::Roots;
use num_traits::{Num, One, Signed, Zero};
use std::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub},
};

use crate::ParseBigIntError;

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
            Sign::Minus => Sign::Plus,
            Sign::NoSign => Sign::NoSign,
            Sign::Plus => Sign::Minus,
        }
    }
}

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug, Binary, Octal, LowerHex, UpperHex,From
)]
#[from(forward)]
pub struct BigInt(Integer);

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

impl Zero for BigInt {
    #[inline]
    fn zero() -> Self {
        Self(<Integer as malachite::num::basic::traits::Zero>::ZERO)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        <Integer as malachite::num::arithmetic::traits::Sign>::sign(&self.0) == Ordering::Equal
    }
}

impl One for BigInt {
    fn one() -> Self {
        Self(<Integer as malachite::num::basic::traits::One>::ONE)
    }
}

impl Signed for BigInt {
    fn abs(&self) -> Self {
        Self(Integer::from_sign_and_abs_ref(
            true,
            self.0.unsigned_abs_ref(),
        ))
    }

    fn abs_sub(&self, other: &Self) -> Self {
        todo!()
    }

    fn signum(&self) -> Self {
        todo!()
    }

    fn is_positive(&self) -> bool {
        todo!()
    }

    fn is_negative(&self) -> bool {
        todo!()
    }
}

impl Num for BigInt {
    type FromStrRadixErr = ParseBigIntError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl num_integer::Integer for BigInt {
    fn div_floor(&self, other: &Self) -> Self {
        todo!()
    }

    fn mod_floor(&self, other: &Self) -> Self {
        todo!()
    }

    fn gcd(&self, other: &Self) -> Self {
        todo!()
    }

    fn lcm(&self, other: &Self) -> Self {
        todo!()
    }

    fn divides(&self, other: &Self) -> bool {
        todo!()
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        todo!()
    }

    fn is_even(&self) -> bool {
        todo!()
    }

    fn is_odd(&self) -> bool {
        todo!()
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        todo!()
    }
}

impl Roots for BigInt {
    fn nth_root(&self, n: u32) -> Self {
        todo!()
    }
}

pub trait ToBigInt {
    fn to_bigint(&self) -> Option<BigInt>;
}
