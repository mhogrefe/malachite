// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::max;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Height, HeightRef};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;

#[test]
fn test_height() {
    let test = |s, height: u32, bits| {
        let g = GaussianRational::from_str(s).unwrap();
        assert_eq!(g.to_height(), height);
        assert_eq!(*g.height_ref(), height);
        assert_eq!(g.clone().into_height(), height);
        assert_eq!(g.height_significant_bits(), bits);
    };
    // Zero's height is 1, as it is for a `Rational`: zero is 0/1, and its denominator is 1.
    test("0", 1, 1);
    test("1", 1, 1);
    test("i", 1, 1);
    // A denominator can be the largest of the four magnitudes.
    test("1/3+i/2", 3, 2);
    test("22/7", 22, 5);
    test("-1/101", 101, 7);
}

#[test]
fn height_properties() {
    gaussian_rational_gen().test_properties(|g| {
        let height = g.to_height();
        // The four ways of asking agree.
        assert_eq!(*g.height_ref(), height);
        assert_eq!(g.clone().into_height(), height);
        assert_eq!(g.height_significant_bits(), height.significant_bits());

        // The height is the larger of the two parts' heights.
        assert_eq!(height, max(g.real.to_height(), g.imaginary.to_height()));

        // It is never zero: every part is a `Rational`, whose denominator is at least 1.
        assert_ne!(height, 0u32);

        // Negating either part, or swapping them, leaves the height alone.
        let negated = GaussianRational {
            real: -&g.real,
            imaginary: -&g.imaginary,
        };
        assert_eq!(negated.to_height(), height);
        let swapped = GaussianRational {
            real: g.imaginary.clone(),
            imaginary: g.real.clone(),
        };
        assert_eq!(swapped.to_height(), height);
    });
}
