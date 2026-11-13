// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::big_exponent_float_power_of_2;
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1_float,
};
use malachite_nz::integer::Integer;

#[test]
fn test_is_power_of_2() {
    for (x, out) in [
        (1.0, true),
        (2.0, true),
        (0.5, true),
        (1024.0, true),
        (3.0, false),
        (0.75, false),
        (-2.0, false),
    ] {
        assert_eq!(BigExponentFloat::from(x).is_power_of_2(), out);
    }
    assert!(!BigExponentFloat::ZERO.is_power_of_2());
    assert!(!BigExponentFloat::NAN.is_power_of_2());
    assert!(!BigExponentFloat::INFINITY.is_power_of_2());
    assert!(!BigExponentFloat::NEGATIVE_INFINITY.is_power_of_2());
    let big = Integer::from_str("100000000000000000000").unwrap();
    assert!(big_exponent_float_power_of_2(big.clone()).is_power_of_2());
    assert!(big_exponent_float_power_of_2(-big.clone()).is_power_of_2());
    assert!(!(-big_exponent_float_power_of_2(big)).is_power_of_2());
}

#[test]
fn is_power_of_2_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let p = x.is_power_of_2();
        assert_eq!(p, x > 0u32 && x.is_finite() && x.get_prec() == Some(1));
        if p && let Some(e) = x
            .get_exponent()
            .and_then(|e| i64::try_from(e).ok())
            .and_then(|e| e.checked_sub(1))
        {
            assert_eq!(x, BigExponentFloat::power_of_2(e));
        }
    });

    big_exponent_float_gen_var_1_float().test_properties(|(f, x)| {
        assert_eq!(x.is_power_of_2(), f.is_power_of_2());
    });
}
