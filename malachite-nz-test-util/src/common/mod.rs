use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::Sign as SignTrait;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use num::bigint::Sign;
use num::{BigInt, BigUint};
use rug::integer::Order;

#[cfg(feature = "32_bit_limbs")]
#[inline]
pub fn biguint_to_natural(n: &BigUint) -> Natural {
    Natural::from_owned_limbs_asc(n.to_u32_digits())
}

#[cfg(not(feature = "32_bit_limbs"))]
#[inline]
pub fn biguint_to_natural(n: &BigUint) -> Natural {
    Natural::from_owned_limbs_asc(Limb::vec_from_other_type_slice(&n.to_u32_digits()))
}

#[cfg(feature = "32_bit_limbs")]
#[inline]
pub fn natural_to_biguint(n: &Natural) -> BigUint {
    BigUint::new(n.to_limbs_asc())
}

#[cfg(not(feature = "32_bit_limbs"))]
#[inline]
pub fn natural_to_biguint(n: &Natural) -> BigUint {
    BigUint::new(u32::vec_from_other_type_slice(&n.to_limbs_asc()))
}

pub fn rug_integer_to_natural(n: &rug::Integer) -> Natural {
    assert!(*n >= 0);
    Natural::from_owned_limbs_asc(n.to_digits(Order::Lsf))
}

#[inline]
pub fn natural_to_rug_integer(n: &Natural) -> rug::Integer {
    rug::Integer::from_digits(&n.to_limbs_asc(), Order::Lsf)
}

pub fn bigint_to_integer(n: &BigInt) -> Integer {
    Integer::from_sign_and_abs(n.sign() != Sign::Minus, biguint_to_natural(n.magnitude()))
}

pub fn integer_to_bigint(n: &Integer) -> BigInt {
    let sign = match n.sign() {
        Ordering::Less => Sign::Minus,
        Ordering::Equal => Sign::NoSign,
        Ordering::Greater => Sign::Plus,
    };
    BigInt::from_biguint(sign, natural_to_biguint(n.unsigned_abs_ref()))
}

pub fn rug_integer_to_integer(n: &rug::Integer) -> Integer {
    Integer::from_sign_and_abs(
        *n >= 0,
        Natural::from_owned_limbs_asc(n.to_digits(Order::Lsf)),
    )
}

pub fn integer_to_rug_integer(n: &Integer) -> rug::Integer {
    let out = natural_to_rug_integer(n.unsigned_abs_ref());
    if *n >= 0 {
        out
    } else {
        -out
    }
}
