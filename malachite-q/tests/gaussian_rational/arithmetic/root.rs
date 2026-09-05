// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CanonicalizeUnit, CheckedRoot, CheckedSqrt, MulIPow, Parity, Pow, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_nz::test_util::generators::gaussian_integer_unsigned_pair_gen_var_2;
use malachite_q::Rational;
use malachite_q::gaussian_rational::{ComparableGaussianRationalRef, GaussianRational};
use malachite_q::test_util::gaussian_rational::arithmetic::root::*;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_unsigned_pair_gen_var_2,
    rational_unsigned_pair_gen_var_1,
};
use std::str::FromStr;

fn sorted_strings(roots: &[GaussianRational]) -> Vec<String> {
    let mut out = roots.iter().map(ToString::to_string).collect::<Vec<_>>();
    out.sort();
    out
}

#[test]
fn test_checked_root() {
    let test = |s, exp, out: Option<&str>, all: Vec<&str>| {
        let x = GaussianRational::from_str(s).unwrap();
        let out = out.map(ToString::to_string);
        let root = x.clone().checked_root(exp);
        if let Some(root) = &root {
            assert!(root.real.is_valid());
            assert!(root.imaginary.is_valid());
        }
        assert_eq!(root.map(|r| r.to_string()), out);
        assert_eq!((&x).checked_root(exp).map(|r| r.to_string()), out);

        let mut all = all.iter().map(ToString::to_string).collect::<Vec<_>>();
        all.sort();
        assert_eq!(sorted_strings(&x.checked_roots(exp)), all);
        assert_eq!(
            sorted_strings(&gaussian_rational_checked_roots_naive(&x, exp)),
            all
        );
    };
    test("0", 3, Some("0"), vec!["0"]);
    test("1", 5, Some("1"), vec!["1"]);
    test("1", 4, Some("1"), vec!["-1", "-i", "i", "1"]);
    test("-1", 3, Some("-1"), vec!["-1"]);
    test("-1", 2, Some("i"), vec!["-i", "i"]);
    test("1/8", 3, Some("1/2"), vec!["1/2"]);
    test("-1/8", 3, Some("-1/2"), vec!["-1/2"]);
    test("-i/8", 3, Some("i/2"), vec!["i/2"]);
    test("1/16", 4, Some("1/2"), vec!["-1/2", "-i/2", "i/2", "1/2"]);
    test(
        "-1/4",
        4,
        Some("1/2+i/2"),
        vec!["-1/2-i/2", "-1/2+i/2", "1/2-i/2", "1/2+i/2"],
    );
    test("-1/4", 2, Some("i/2"), vec!["-i/2", "i/2"]);
    test("-38/3125+41i/3125", 5, Some("2/5+i/5"), vec!["2/5+i/5"]);
    test(
        "3/25+4i/25",
        2,
        Some("2/5+i/5"),
        vec!["-2/5-i/5", "2/5+i/5"],
    );
    test("-7/4+6i", 2, Some("3/2+2i"), vec!["-3/2-2i", "3/2+2i"]);
    test("2/27+11i/27", 3, Some("2/3+i/3"), vec!["2/3+i/3"]);
    test("1/2", 3, None, vec![]);
    test("1/2+i/3", 2, None, vec![]);
    test("-1", 4, None, vec![]);
    test("i", 3, Some("-i"), vec!["-i"]);
    test("1/81", 4, Some("1/3"), vec!["-1/3", "-i/3", "i/3", "1/3"]);
}

#[test]
#[should_panic]
fn checked_root_fail() {
    GaussianRational::from(4).checked_root(0);
}

#[test]
#[should_panic]
fn checked_root_ref_fail() {
    (&GaussianRational::from(4)).checked_root(0);
}

#[test]
#[should_panic]
fn checked_roots_fail() {
    GaussianRational::from(4).checked_roots(0);
}

// The root with argument in (-pi/g, pi/g], g = gcd(exp, 4), among the rotations of `x`.
fn principal(x: GaussianRational, exp: u64) -> GaussianRational {
    match TrailingZeros::trailing_zeros(exp) {
        0 => x,
        1 => {
            if (&x.real, &x.imaginary) > (&Rational::ZERO, &Rational::ZERO) {
                x
            } else {
                -x
            }
        }
        _ => x.canonicalize_unit(),
    }
}

#[test]
fn checked_root_properties() {
    gaussian_rational_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, exp)| {
        let root = (&x).checked_root(exp);
        assert_eq!(x.clone().checked_root(exp), root);
        if let Some(root) = &root {
            assert!(root.real.is_valid());
            assert!(root.imaginary.is_valid());
            assert_eq!(root.pow(exp), x);
            assert_eq!(principal(root.clone(), exp), *root);
        }

        let roots = x.checked_roots(exp);
        assert!(roots.is_sorted_by(|a, b| {
            ComparableGaussianRationalRef(a) <= ComparableGaussianRationalRef(b)
        }));
        assert_eq!(
            roots.contains(root.as_ref().unwrap_or(&GaussianRational::ZERO)),
            root.is_some()
        );
        if x == 0u32 {
            assert_eq!(roots, vec![GaussianRational::ZERO]);
        } else if let Some(principal) = &root {
            let g = u64::power_of_2(TrailingZeros::trailing_zeros(exp).min(2));
            assert_eq!(roots.len(), usize::exact_from(g));
            for r in &roots {
                assert_eq!(r.pow(exp), x);
            }
            for j in 0..g {
                assert!(roots.contains(&principal.mul_i_pow(j * (4 / g))));
            }
        } else {
            assert!(roots.is_empty());
        }

        // a power has its base among the roots, and the principal rotation of it as the root
        let power = (&x).pow(exp);
        assert_eq!((&power).checked_root(exp), Some(principal(x.clone(), exp)));
        assert!(power.checked_roots(exp).contains(&x));
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!((&x).checked_root(1), Some(x.clone()));
        assert_eq!((&x).checked_root(2), (&x).checked_sqrt());
        assert_eq!(x.checked_roots(2), x.checked_sqrts());
    });

    gaussian_integer_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, exp)| {
        assert_eq!(
            GaussianRational::from(x.clone()).checked_root(exp),
            (&x).checked_root(exp).map(GaussianRational::from)
        );
        assert_eq!(
            GaussianRational::from(x.clone()).checked_roots(exp),
            x.checked_roots(exp)
                .into_iter()
                .map(GaussianRational::from)
                .collect::<Vec<_>>()
        );
    });

    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(q, exp)| {
        if exp == 0 {
            return;
        }
        // A real number can have a non-real Gaussian root, so the comparison only runs one way.
        let root = GaussianRational::from(q.clone()).checked_root(exp);
        if q < 0u32 && exp.even() {
            // `Rational::checked_root` refuses this case; the Gaussian root, if any, is non-real
            if let Some(root) = root {
                assert_ne!(root.imaginary, 0);
            }
        } else if let Some(r) = (&q).checked_root(exp) {
            assert_eq!(root, Some(GaussianRational::from(r)));
        } else if let Some(root) = root {
            assert_ne!(root.imaginary, 0);
        }
    });
}
