use derive_more::{Binary, LowerHex, Octal, UpperHex, From};
use malachite::Natural;
use num_integer::Roots;
use num_traits::{Num, One, Signed, Zero, Unsigned};
use std::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub},
};

use crate::ParseBigIntError;

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug, Binary, Octal, LowerHex, UpperHex,From
)]
#[from(forward)]
pub struct BigUint(Natural);

forward_binary_op!(BigUint, Add, add);
forward_binary_op!(BigUint, Sub, sub);
forward_binary_op!(BigUint, Mul, mul);
forward_binary_op!(BigUint, Div, div);
forward_binary_op!(BigUint, Rem, rem);
forward_binary_op!(BigUint, BitAnd, bitand);
forward_binary_op!(BigUint, BitOr, bitor);
forward_binary_op!(BigUint, BitXor, bitxor);

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

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl num_integer::Integer for BigUint {
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

impl Roots for BigUint {
    fn nth_root(&self, n: u32) -> Self {
        todo!()
    }
}

pub trait ToBigUint {
    fn to_biguint(&self) -> Option<BigUint>;
}
