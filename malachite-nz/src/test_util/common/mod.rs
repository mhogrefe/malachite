// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
#[cfg(not(feature = "32_bit_limbs"))]
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::Sign as SignTrait;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
use num::bigint::Sign;
use num::{BigInt, BigUint};
use rug::integer::Order;
use std::cmp::Ordering::*;

#[cfg(feature = "32_bit_limbs")]
impl From<&BigUint> for Natural {
    #[inline]
    fn from(n: &BigUint) -> Natural {
        Natural::from_owned_limbs_asc(n.to_u32_digits())
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl From<&BigUint> for Natural {
    #[inline]
    fn from(n: &BigUint) -> Natural {
        Natural::from_owned_limbs_asc(Limb::vec_from_other_type_slice(&n.to_u32_digits()))
    }
}

#[cfg(feature = "32_bit_limbs")]
impl From<&Natural> for BigUint {
    #[inline]
    fn from(n: &Natural) -> BigUint {
        BigUint::new(n.to_limbs_asc())
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl From<&Natural> for BigUint {
    #[inline]
    fn from(n: &Natural) -> BigUint {
        BigUint::new(u32::vec_from_other_type_slice(&n.to_limbs_asc()))
    }
}

#[cfg(feature = "32_bit_limbs")]
impl From<&Natural> for BigInt {
    #[inline]
    fn from(n: &Natural) -> BigInt {
        BigInt::from_biguint(
            if *n == 0 { Sign::NoSign } else { Sign::Plus },
            BigUint::new(n.to_limbs_asc()),
        )
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl From<&Natural> for BigInt {
    #[inline]
    fn from(n: &Natural) -> BigInt {
        BigInt::from_biguint(
            if *n == 0 { Sign::NoSign } else { Sign::Plus },
            BigUint::new(u32::vec_from_other_type_slice(&n.to_limbs_asc())),
        )
    }
}

impl TryFrom<&rug::Integer> for Natural {
    type Error = ();

    #[inline]
    fn try_from(n: &rug::Integer) -> Result<Natural, ()> {
        if *n >= 0 {
            Ok(Natural::from_owned_limbs_asc(n.to_digits(Order::Lsf)))
        } else {
            Err(())
        }
    }
}

impl From<&Natural> for rug::Integer {
    #[inline]
    fn from(n: &Natural) -> rug::Integer {
        rug::Integer::from_digits(&n.to_limbs_asc(), Order::Lsf)
    }
}

impl From<&BigInt> for Integer {
    #[inline]
    fn from(n: &BigInt) -> Integer {
        Integer::from_sign_and_abs(n.sign() != Sign::Minus, Natural::from(n.magnitude()))
    }
}

impl From<&Integer> for BigInt {
    #[inline]
    fn from(n: &Integer) -> BigInt {
        let sign = match n.sign() {
            Less => Sign::Minus,
            Equal => Sign::NoSign,
            Greater => Sign::Plus,
        };
        BigInt::from_biguint(sign, BigUint::from(n.unsigned_abs_ref()))
    }
}

impl From<&rug::Integer> for Integer {
    #[inline]
    fn from(n: &rug::Integer) -> Integer {
        Integer::from_sign_and_abs(
            *n >= 0,
            Natural::from_owned_limbs_asc(n.to_digits(Order::Lsf)),
        )
    }
}

impl From<&Integer> for rug::Integer {
    #[inline]
    fn from(n: &Integer) -> rug::Integer {
        let out = rug::Integer::from(n.unsigned_abs_ref());
        if *n >= 0 {
            out
        } else {
            -out
        }
    }
}
