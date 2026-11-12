// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{big_exponent_float_gen, float_gen};
use malachite_nz::integer::Integer;
use std::cmp::Ordering::*;

#[test]
fn test_sign() {
    assert_eq!(BigExponentFloat::INFINITY.sign(), Greater);
    assert_eq!(BigExponentFloat::NEGATIVE_INFINITY.sign(), Less);
    assert_eq!(BigExponentFloat::ZERO.sign(), Equal);
    assert_eq!(BigExponentFloat::ONE.sign(), Greater);
    assert_eq!(BigExponentFloat::from(-1.5).sign(), Less);
    let huge = big_exponent_float_power_of_2(Integer::from_str("100000000000000000000").unwrap());
    assert_eq!(huge.sign(), Greater);
    assert_eq!((-huge).sign(), Less);
}

#[test]
#[should_panic]
fn sign_fail() {
    BigExponentFloat::NAN.sign();
}

#[test]
fn sign_properties() {
    big_exponent_float_gen().test_properties(|x| {
        if !x.is_nan() {
            let s = x.sign();
            assert_eq!(s, x.partial_cmp(&BigExponentFloat::ZERO).unwrap());
            assert_eq!((-&x).sign(), s.reverse());
        }
    });

    float_gen().test_properties(|x| {
        if !x.is_nan() && !x.is_zero() {
            assert_eq!(float_to_big_exponent_float(&x).sign(), x.sign());
        }
    });
}
