// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_pair_gen, float_signed_pair_gen,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_shr() {
    let one = BigExponentFloat::ONE;
    let test = |x: &BigExponentFloat, k: i64, out: &BigExponentFloat| {
        let r = x.clone() >> k;
        assert!(r.is_valid());
        assert!(same(&r, out));
        assert!(same(&(x >> k), out));
        let mut m = x.clone();
        m >>= k;
        assert!(same(&m, out));
    };
    test(&BigExponentFloat::NAN, 10, &BigExponentFloat::NAN);
    test(
        &BigExponentFloat::INFINITY,
        -10,
        &BigExponentFloat::INFINITY,
    );
    test(&BigExponentFloat::ZERO, 10, &BigExponentFloat::ZERO);
    test(&one, 10, &BigExponentFloat::from(1.0 / 1024.0));
    test(&one, -10, &BigExponentFloat::from(1024u32));
    test(
        &BigExponentFloat::from(-12i32),
        3,
        &BigExponentFloat::from(-1.5),
    );
    test(
        &one,
        i64::MAX,
        &big_exponent_float_power_of_2(-Integer::from(i64::MAX)),
    );
    test(
        &one,
        i64::MIN,
        &big_exponent_float_power_of_2(-Integer::from(i64::MIN)),
    );
}

#[test]
fn shr_properties() {
    big_exponent_float_unsigned_pair_gen::<u8>().test_properties(|(x, k)| {
        let r = &x >> k;
        assert!(r.is_valid());
        assert!(same(&(&r << k), &x));
        assert!(same(&(x.clone() >> k), &r));
        assert!(same(&(&x << -i16::from(k)), &r));
        assert_eq!(r.get_prec(), x.get_prec());
        if x.is_finite() && big_exponent_float_exponent_is_small(&x) {
            assert_eq!(Rational::exact_from(&r), Rational::exact_from(&x) >> k);
        }
    });

    float_signed_pair_gen::<i8>().test_properties(|(x, k)| {
        let f = &x >> k;
        if f.is_normal()
            && f.get_exponent().unwrap() < Float::MAX_EXPONENT
            && f.get_exponent().unwrap() > Float::MIN_EXPONENT
        {
            assert!(same(
                &(float_to_big_exponent_float(&x) >> k),
                &float_to_big_exponent_float(&f)
            ));
        }
    });
}
