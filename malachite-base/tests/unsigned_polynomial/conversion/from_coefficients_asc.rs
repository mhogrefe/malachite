// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn u64s(xs: &[u32]) -> Vec<u64> {
    xs.iter().copied().map(u64::from).collect()
}

#[test]
fn test_from_coefficients_asc() {
    let test = |xs: &[u32], out| {
        let p = UnsignedPolynomial::<u64>::from_coefficients_asc(u64s(xs));
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[5], "5");
    test(&[2, 3, 1], "x^2+3*x+2");
    // Trailing zeros are dropped, however many there are.
    test(&[2, 3, 1, 0], "x^2+3*x+2");
    test(&[2, 3, 1, 0, 0, 0], "x^2+3*x+2");
    // Zeros that are not trailing are kept.
    test(&[0, 0, 1], "x^2");
}

#[test]
fn from_coefficients_asc_properties() {
    unsigned_vec_gen::<u64>().test_properties(|xs| {
        let p = UnsignedPolynomial::<u64>::from_coefficients_asc(xs.clone());
        assert!(p.is_valid());

        // The result holds the same coefficients, minus the trailing zeros.
        let trimmed = xs.len() - xs.iter().rev().take_while(|&c| *c == 0).count();
        assert_eq!(p.coefficients_asc(), &xs[..trimmed]);
        assert_eq!(
            p.degree(),
            u64::try_from(trimmed).ok().and_then(|l| l.checked_sub(1))
        );

        // Appending zeros does not change the polynomial, and the round trip is the identity.
        let mut padded = xs.clone();
        padded.push(0);
        assert_eq!(UnsignedPolynomial::<u64>::from_coefficients_asc(padded), p);
        assert_eq!(
            UnsignedPolynomial::<u64>::from_coefficients_asc(p.coefficients_asc().to_vec()),
            p
        );
    });
}
