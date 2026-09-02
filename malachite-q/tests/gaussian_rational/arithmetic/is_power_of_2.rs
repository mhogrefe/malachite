// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, IsPowerOf2};
use malachite_base::num::conversion::traits::IsReal;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_gen_var_1, gaussian_rational_gen_var_2, rational_gen,
};
use std::str::FromStr;

#[test]
fn test_is_power_of_2() {
    let test = |s, out| {
        assert_eq!(GaussianRational::from_str(s).unwrap().is_power_of_2(), out);
    };
    test("0", false);
    test("1", true);
    test("2", true);
    test("3", false);
    test("4", true);
    test("-4", false);
    test("1024", true);
    test("1025", false);
    test("1099511627776", true);
    test("1/2", true);
    test("1/3", false);
    test("1/4", true);
    test("-1/4", false);
    test("1/1024", true);
    test("1/1099511627776", true);
    test("22/7", false);
    test("i", false);
    test("i/2", false);
    test("2i", false);
    test("1/2+i", false);
    test("8+i/2", false);
    test("4+4i", false);
}

#[test]
fn is_power_of_2_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let is_power = x.is_power_of_2();
        if is_power {
            assert!(x.is_real());
            assert!(x.real > 0u32);
        }
        assert_eq!((&x).conjugate().is_power_of_2(), is_power);
        assert!(!(-&x).is_power_of_2() || !is_power);
    });

    gaussian_rational_gen_var_1().test_properties(|x| {
        assert_eq!(x.is_power_of_2(), x.real.is_power_of_2());
    });

    gaussian_rational_gen_var_2().test_properties(|x| {
        assert!(!x.is_power_of_2());
    });

    rational_gen().test_properties(|x| {
        assert_eq!(
            GaussianRational::from(x.clone()).is_power_of_2(),
            x.is_power_of_2()
        );
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(
            GaussianRational::from(&x).is_power_of_2(),
            x.is_power_of_2()
        );
    });
}
