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
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;

#[test]
fn test_height() {
    let test = |s, height: u32, bits| {
        let g = GaussianInteger::from_str(s).unwrap();
        assert_eq!(g.to_height(), height);
        assert_eq!(*g.height_ref(), height);
        assert_eq!(g.clone().into_height(), height);
        assert_eq!(g.height_significant_bits(), bits);
    };
    test("0", 0, 0);
    test("1", 1, 1);
    test("i", 1, 1);
    // The height is a magnitude, so the sign of either part does not matter.
    test("3-5i", 5, 3);
    test("-7", 7, 3);
    test("-3+5i", 5, 3);
}

#[test]
fn height_properties() {
    gaussian_integer_gen().test_properties(|g| {
        let height = g.to_height();
        // The four ways of asking agree.
        assert_eq!(*g.height_ref(), height);
        assert_eq!(g.clone().into_height(), height);
        assert_eq!(g.height_significant_bits(), height.significant_bits());

        // The height is the larger of the two parts' magnitudes.
        assert_eq!(
            height,
            max(
                g.real.unsigned_abs_ref().clone(),
                g.imaginary.unsigned_abs_ref().clone()
            )
        );

        // Negating either part, or swapping them, leaves the height alone.
        let negated = GaussianInteger {
            real: -&g.real,
            imaginary: -&g.imaginary,
        };
        assert_eq!(negated.to_height(), height);
        let swapped = GaussianInteger {
            real: g.imaginary.clone(),
            imaginary: g.real.clone(),
        };
        assert_eq!(swapped.to_height(), height);
    });
}
