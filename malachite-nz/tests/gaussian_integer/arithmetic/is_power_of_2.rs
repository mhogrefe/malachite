// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, IsPowerOf2};
use malachite_base::num::conversion::traits::IsReal;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_gen_var_1, gaussian_integer_gen_var_2, integer_gen,
    natural_gen,
};
use std::str::FromStr;

#[test]
fn test_is_power_of_2() {
    let test = |s, out| {
        assert_eq!(GaussianInteger::from_str(s).unwrap().is_power_of_2(), out);
    };
    test("0", false);
    test("1", true);
    test("2", true);
    test("3", false);
    test("4", true);
    test("5", false);
    test("8", true);
    test("-8", false);
    test("1024", true);
    test("1025", false);
    test("1000000000000", false);
    test("1099511627776", true);
    test("i", false);
    test("2i", false);
    test("-i", false);
    test("1+i", false);
    test("8+i", false);
    test("4+4i", false);
    test("1099511627776+i", false);
}

#[test]
fn is_power_of_2_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let is_power = x.is_power_of_2();
        if is_power {
            assert!(x.is_real());
            assert!(x.real > 0u32);
        }
        assert_eq!((&x).conjugate().is_power_of_2(), is_power);
        assert!(!(-&x).is_power_of_2() || !is_power);
    });

    gaussian_integer_gen_var_1().test_properties(|x| {
        assert_eq!(
            x.is_power_of_2(),
            x.real > 0u32 && x.real.unsigned_abs_ref().is_power_of_2()
        );
    });

    gaussian_integer_gen_var_2().test_properties(|x| {
        assert!(!x.is_power_of_2());
    });

    natural_gen().test_properties(|x| {
        assert_eq!(GaussianInteger::from(&x).is_power_of_2(), x.is_power_of_2());
    });

    integer_gen().test_properties(|x| {
        assert_eq!(
            GaussianInteger::from(x.clone()).is_power_of_2(),
            x > 0u32 && x.unsigned_abs_ref().is_power_of_2()
        );
    });
}
