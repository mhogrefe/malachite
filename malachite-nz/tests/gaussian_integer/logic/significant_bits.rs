// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Conjugate;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::{gaussian_integer_gen, integer_gen};
use std::str::FromStr;

#[test]
fn test_significant_bits() {
    let test = |s, sum, max| {
        let x = GaussianInteger::from_str(s).unwrap();
        assert_eq!(x.significant_bits(), sum);
        assert_eq!(x.max_significant_bits(), max);
    };
    test("0", 0, 0);
    test("1", 1, 1);
    test("i", 1, 1);
    test("-1", 1, 1);
    test("100", 7, 7);
    test("-100", 7, 7);
    test("100i", 7, 7);
    test("1+i", 2, 1);
    test("3+4i", 5, 3);
    test("-3-4i", 5, 3);
    test("1000000000000", 40, 40);
    test("1000000000000+i", 41, 40);
    test("1000000000000+1000000000000i", 80, 40);
}

#[test]
fn significant_bits_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let sum = x.significant_bits();
        let max = x.max_significant_bits();
        assert_eq!(
            sum,
            x.real.significant_bits() + x.imaginary.significant_bits()
        );
        assert_eq!(
            max,
            x.real
                .significant_bits()
                .max(x.imaginary.significant_bits())
        );
        assert!(max <= sum);
        assert!(sum <= max << 1);
        assert_eq!((-&x).significant_bits(), sum);
        assert_eq!((&x).conjugate().significant_bits(), sum);
        assert_eq!((-&x).max_significant_bits(), max);
        assert_eq!((&x).conjugate().max_significant_bits(), max);
        assert_eq!(sum == 0, x == 0u32);
    });

    integer_gen().test_properties(|x| {
        let y = GaussianInteger::from(x.clone());
        assert_eq!(y.significant_bits(), x.significant_bits());
        assert_eq!(y.max_significant_bits(), x.significant_bits());
    });
}
