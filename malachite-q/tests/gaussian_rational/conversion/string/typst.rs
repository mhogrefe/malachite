// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ImaginaryFrom;
use malachite_base::strings::typst::ToTypst;
use malachite_q::Rational;
use malachite_q::gaussian_rational::{
    ComparableGaussianRational, ComparableGaussianRationalRef, GaussianRational,
};
use malachite_q::test_util::generators::gaussian_rational_gen;

// Turns each fraction back into the `a/b` that `Display` writes, so that a fragment can be compared
// with it directly. A numerator and a denominator hold only digits and possibly an `i`, so there is
// nothing nested to worry about.
fn undo_fractions(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("frac(") {
        out.push_str(&rest[..i]);
        rest = &rest[i + 5..];
        let mid = rest.find(", ").unwrap();
        let numerator = &rest[..mid];
        rest = &rest[mid + 2..];
        let end = rest.find(')').unwrap();
        out.push_str(numerator);
        out.push('/');
        out.push_str(&rest[..end]);
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    out
}

#[test]
fn test_gaussian_rational_to_typst() {
    let test = |x: GaussianRational, out: &str| assert_eq!(x.to_typst_string(), out);
    test(GaussianRational::default(), "0");
    test(GaussianRational::from(Rational::from(2)), "2");
    test(
        GaussianRational::from(Rational::from_signeds(-2, 3)),
        "-frac(2, 3)",
    );
    // the imaginary unit goes in the numerator, and a coefficient of 1 is elided
    test(GaussianRational::imaginary_from(1), "i");
    test(GaussianRational::imaginary_from(-1), "-i");
    test(GaussianRational::imaginary_from(2), "2i");
    test(
        GaussianRational::imaginary_from(Rational::from_signeds(1, 2)),
        "frac(i, 2)",
    );
    test(
        GaussianRational::imaginary_from(Rational::from_signeds(-5, 6)),
        "-frac(5i, 6)",
    );
    // a real term, then the imaginary one with a joining sign
    test(
        GaussianRational {
            real: Rational::from_signeds(2, 3),
            imaginary: Rational::from_signeds(-5, 6),
        },
        "frac(2, 3)-frac(5i, 6)",
    );
    test(
        GaussianRational {
            real: Rational::from_signeds(2, 3),
            imaginary: Rational::from_signeds(5, 6),
        },
        "frac(2, 3)+frac(5i, 6)",
    );
}

#[test]
fn gaussian_rational_to_typst_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let s = x.to_typst_string();
        // Undoing the fractions gives back exactly what `Display` writes.
        assert_eq!(undo_fractions(&s), x.to_string());
        // A minus sign never appears inside a fraction.
        assert!(!s.contains("frac(-"));
        // The imaginary unit is always inside a numerator, never hung off a fraction.
        assert!(!s.contains(")i"));
        // The wrappers exist to give an ordering, and do not change what the value is.
        assert_eq!(ComparableGaussianRational(x.clone()).to_typst_string(), s);
        assert_eq!(ComparableGaussianRationalRef(&x).to_typst_string(), s);
    });
}
