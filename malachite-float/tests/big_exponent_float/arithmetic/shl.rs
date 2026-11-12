// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_signed_pair_gen, float_signed_pair_gen,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_shl() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let one = BigExponentFloat::ONE;
    let test = |x: &BigExponentFloat, k: i64, out: &BigExponentFloat| {
        let l = x.clone() << k;
        assert!(l.is_valid());
        assert!(same(&l, out));
        assert!(same(&(x << k), out));
        let mut m = x.clone();
        m <<= k;
        assert!(same(&m, out));
        let neg_k = -i128::from(k);
        assert!(same(&(x.clone() >> neg_k), out));
        assert!(same(&(x >> neg_k), out));
        let mut m = x.clone();
        m >>= neg_k;
        assert!(same(&m, out));
    };
    test(&BigExponentFloat::NAN, 10, &BigExponentFloat::NAN);
    test(
        &BigExponentFloat::INFINITY,
        -10,
        &BigExponentFloat::INFINITY,
    );
    test(&BigExponentFloat::ZERO, 10, &BigExponentFloat::ZERO);
    test(&one, 0, &one);
    test(&one, 10, &BigExponentFloat::from(1024u32));
    test(&one, -10, &BigExponentFloat::from(1.0 / 1024.0));
    test(
        &BigExponentFloat::from(-1.5),
        3,
        &BigExponentFloat::from(-12i32),
    );
    // Far beyond Float's exponent range, exactly.
    test(
        &one,
        i64::MAX,
        &big_exponent_float_power_of_2(Integer::from(i64::MAX)),
    );
    test(
        &one,
        i64::MIN,
        &big_exponent_float_power_of_2(Integer::from(i64::MIN)),
    );
    assert!(same(
        &(&huge << u128::MAX),
        &big_exponent_float_power_of_2(big + Integer::from(u128::MAX))
    ));
}

#[test]
fn shl_properties() {
    big_exponent_float_signed_pair_gen::<i64>().test_properties(|(x, k)| {
        let l = &x << k;
        assert!(l.is_valid());
        assert!(same(&(x.clone() << k), &l));
        assert!(same(&(&x >> -i128::from(k)), &l));
        assert!(same(&(&l >> k), &x));
        assert_eq!(l.get_prec(), x.get_prec());
        if x.is_finite() && !x.is_zero() {
            assert_eq!(
                l.get_exponent().unwrap(),
                &(x.get_exponent().unwrap() + Integer::from(k))
            );
            if big_exponent_float_exponent_is_small(&x) && k.unsigned_abs() < 1 << 16 {
                assert_eq!(Rational::exact_from(&l), Rational::exact_from(&x) << k);
            }
        }
    });

    float_signed_pair_gen::<i8>().test_properties(|(x, k)| {
        let f = &x << k;
        if f.is_normal()
            && f.get_exponent().unwrap() < Float::MAX_EXPONENT
            && f.get_exponent().unwrap() > Float::MIN_EXPONENT
        {
            assert!(same(
                &(float_to_big_exponent_float(&x) << k),
                &float_to_big_exponent_float(&f)
            ));
        }
    });
}
