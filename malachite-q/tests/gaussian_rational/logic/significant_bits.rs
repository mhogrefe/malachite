// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Conjugate;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{gaussian_rational_gen, rational_gen};
use std::str::FromStr;

#[test]
fn test_significant_bits() {
    let test = |s, out| {
        assert_eq!(
            GaussianRational::from_str(s).unwrap().significant_bits(),
            out
        );
    };
    test("0", 2);
    test("1", 3);
    test("i", 3);
    test("-1", 3);
    test("100", 9);
    test("1/2", 4);
    test("i/2", 4);
    test("1+i", 4);
    test("1/2+i/3", 6);
    test("-100/101+i", 16);
    test("1000000000000", 42);
}

#[test]
fn significant_bits_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let bits = x.significant_bits();
        assert_eq!(
            bits,
            x.real.significant_bits() + x.imaginary.significant_bits()
        );
        assert!(bits >= 2);
        assert_eq!((-&x).significant_bits(), bits);
        assert_eq!((&x).conjugate().significant_bits(), bits);
    });

    rational_gen().test_properties(|x| {
        assert_eq!(
            GaussianRational::from(x.clone()).significant_bits(),
            x.significant_bits() + 1
        );
    });
}
