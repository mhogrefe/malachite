// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::IsReal;
use malachite_base::test_util::generators::{signed_gen_var_5, unsigned_gen_var_5};
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

#[test]
fn test_power_of_2() {
    let test = |pow: u64, out| assert_eq!(GaussianRational::power_of_2(pow).to_string(), out);
    test(0, "1");
    test(1, "2");
    test(2, "4");
    test(3, "8");
    test(32, "4294967296");
    test(100, "1267650600228229401496703205376");

    let test = |pow: i64, out| assert_eq!(GaussianRational::power_of_2(pow).to_string(), out);
    test(0, "1");
    test(1, "2");
    test(3, "8");
    test(100, "1267650600228229401496703205376");
    test(-1, "1/2");
    test(-2, "1/4");
    test(-3, "1/8");
    test(-32, "1/4294967296");
    test(-100, "1/1267650600228229401496703205376");
}

#[test]
fn power_of_2_properties() {
    unsigned_gen_var_5::<u64>().test_properties(|pow| {
        let x = GaussianRational::power_of_2(pow);
        assert!(x.real.is_valid());
        assert!(x.imaginary.is_valid());

        assert!(x.is_real());
        assert_eq!(x.imaginary, Rational::ZERO);
        assert_eq!(x.real, Rational::power_of_2(pow));
        assert_eq!(x, GaussianRational::from(Rational::power_of_2(pow)));
        assert!(x.is_power_of_2());
    });

    signed_gen_var_5::<i64>().test_properties(|pow| {
        let x = GaussianRational::power_of_2(pow);
        assert!(x.real.is_valid());
        assert!(x.imaginary.is_valid());

        assert!(x.is_real());
        assert_eq!(x.imaginary, Rational::ZERO);
        assert_eq!(x.real, Rational::power_of_2(pow));
        assert_eq!(x, GaussianRational::from(Rational::power_of_2(pow)));
        assert!(x.is_power_of_2());
        if pow >= 0 {
            assert_eq!(x, GaussianRational::power_of_2(pow.unsigned_abs()));
        }
    });
}
