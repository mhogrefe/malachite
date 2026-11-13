// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2};
use malachite_base::num::basic::traits::One;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;

#[test]
fn test_power_of_2() {
    let test = |n: i64, expected: BigExponentFloat| {
        let x = BigExponentFloat::power_of_2(n);
        assert!(x.is_valid());
        assert_eq!(x, expected);
        if let Ok(u) = u64::try_from(n) {
            assert_eq!(BigExponentFloat::power_of_2(u), expected);
        }
    };
    test(0, BigExponentFloat::ONE);
    test(1, BigExponentFloat::from(2));
    test(10, BigExponentFloat::from(1024));
    test(-1, BigExponentFloat::from(0.5));
    test(-10, BigExponentFloat::from(1.0 / 1024.0));
    // The extremes of the exponent types are exact: 2^n has exponent n + 1.
    assert_eq!(
        *BigExponentFloat::power_of_2(u64::MAX)
            .get_exponent()
            .unwrap(),
        Integer::from(u64::MAX) + Integer::ONE
    );
    assert_eq!(
        *BigExponentFloat::power_of_2(i64::MIN)
            .get_exponent()
            .unwrap(),
        Integer::from(i64::MIN) + Integer::ONE
    );
}

#[test]
fn power_of_2_properties() {
    unsigned_gen::<u64>().test_properties(|n| {
        let x = BigExponentFloat::power_of_2(n);
        assert!(x.is_valid());
        assert!(x.is_power_of_2());
        assert_eq!(x.get_prec(), Some(1));
        assert_eq!(*x.get_exponent().unwrap(), Integer::from(n) + Integer::ONE);
        if let Ok(s) = i64::try_from(n) {
            assert_eq!(BigExponentFloat::power_of_2(s), x);
        }
        if n < 1 << 20 {
            assert_eq!(float_to_big_exponent_float(&Float::power_of_2(n)), x);
        }
    });

    signed_gen::<i64>().test_properties(|n| {
        let x = BigExponentFloat::power_of_2(n);
        assert!(x.is_valid());
        assert!(x.is_power_of_2());
        assert_eq!(*x.get_exponent().unwrap(), Integer::from(n) + Integer::ONE);
        if let Some(m) = n.checked_neg() {
            assert_eq!(&x * BigExponentFloat::power_of_2(m), BigExponentFloat::ONE);
        }
        if n.unsigned_abs() < 1 << 20 {
            assert_eq!(float_to_big_exponent_float(&Float::power_of_2(n)), x);
        }
    });
}
