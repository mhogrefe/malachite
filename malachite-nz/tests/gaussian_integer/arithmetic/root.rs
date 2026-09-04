// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CanonicalizeUnit, CheckedRoot, CheckedSqrt, MulIPow, Pow, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{SignificantBits, TrailingZeros};
use malachite_nz::gaussian_integer::{ComparableGaussianIntegerRef, GaussianInteger};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::gaussian_integer::arithmetic::root::*;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_unsigned_pair_gen_var_2, integer_unsigned_pair_gen_var_3,
};
use std::str::FromStr;

fn sorted_strings(roots: &[GaussianInteger]) -> Vec<String> {
    let mut out = roots.iter().map(ToString::to_string).collect::<Vec<_>>();
    out.sort();
    out
}

#[test]
fn test_checked_root() {
    let test = |s, exp, out: Option<&str>, all: Vec<&str>| {
        let x = GaussianInteger::from_str(s).unwrap();
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
            sorted_strings(&gaussian_integer_checked_roots_naive(&x, exp)),
            all
        );
    };
    test("0", 1, Some("0"), vec!["0"]);
    test("0", 5, Some("0"), vec!["0"]);
    // units
    test("1", 1, Some("1"), vec!["1"]);
    test("1", 3, Some("1"), vec!["1"]);
    test("1", 4, Some("1"), vec!["-1", "-i", "i", "1"]);
    test("-1", 3, Some("-1"), vec!["-1"]);
    test("-1", 2, Some("i"), vec!["-i", "i"]);
    test("-1", 4, None, vec![]);
    test("i", 1, Some("i"), vec!["i"]);
    test("i", 3, Some("-i"), vec!["-i"]);
    test("i", 5, Some("i"), vec!["i"]);
    test("-i", 3, Some("i"), vec!["i"]);
    // odd exponents: a unique root
    test("-2+2i", 3, Some("1+i"), vec!["1+i"]);
    test("2+2i", 3, Some("-1+i"), vec!["-1+i"]);
    test("8", 3, Some("2"), vec!["2"]);
    test("-8", 3, Some("-2"), vec!["-2"]);
    test("8i", 3, Some("-2i"), vec!["-2i"]);
    test("2+11i", 3, Some("2+i"), vec!["2+i"]);
    test("-38+41i", 5, Some("2+i"), vec!["2+i"]);
    test("-9+46i", 3, Some("3+2i"), vec!["3+2i"]);
    test("-117+44i", 3, Some("3+4i"), vec!["3+4i"]);
    test("-341525-145668i", 5, Some("-5+12i"), vec!["-5+12i"]);
    test("3+4i", 3, None, vec![]);
    // exponents 2 mod 4: two roots
    test("-4", 2, Some("2i"), vec!["-2i", "2i"]);
    test("3+4i", 2, Some("2+i"), vec!["-2-i", "2+i"]);
    test("-7+24i", 2, Some("3+4i"), vec!["-3-4i", "3+4i"]);
    test("-341525-145668i", 10, Some("2+3i"), vec!["-2-3i", "2+3i"]);
    test(
        "-341525-145668i",
        2,
        Some("122-597i"),
        vec!["-122+597i", "122-597i"],
    );
    test(
        "164833+354144i",
        2,
        Some("527+336i"),
        vec!["-527-336i", "527+336i"],
    );
    test("5+12i", 6, None, vec![]);
    // exponents divisible by 4: four roots, the canonical one first
    test("-4", 4, Some("1+i"), vec!["-1-i", "-1+i", "1-i", "1+i"]);
    test("16", 4, Some("2"), vec!["-2", "-2i", "2i", "2"]);
    test("16", 8, Some("1+i"), vec!["-1-i", "-1+i", "1-i", "1+i"]);
    test(
        "-64",
        4,
        Some("2+2i"),
        vec!["-2-2i", "-2+2i", "2-2i", "2+2i"],
    );
    test(
        "-7+24i",
        4,
        Some("2+i"),
        vec!["-2-i", "-1+2i", "1-2i", "2+i"],
    );
    test(
        "164833+354144i",
        4,
        Some("24+7i"),
        vec!["-24-7i", "-7+24i", "7-24i", "24+7i"],
    );
    test(
        "164833+354144i",
        8,
        Some("4-3i"),
        vec!["-4+3i", "-3-4i", "3+4i", "4-3i"],
    );
}

#[test]
#[should_panic]
fn checked_root_fail() {
    GaussianInteger::from(4).checked_root(0);
}

#[test]
#[should_panic]
fn checked_root_ref_fail() {
    (&GaussianInteger::from(4)).checked_root(0);
}

#[test]
#[should_panic]
fn checked_roots_fail() {
    GaussianInteger::from(4).checked_roots(0);
}

// The root with argument in (-pi/g, pi/g], g = gcd(exp, 4), among the rotations of `x`.
fn principal(x: GaussianInteger, exp: u64) -> GaussianInteger {
    match TrailingZeros::trailing_zeros(exp) {
        0 => x,
        1 => {
            if (&x.real, &x.imaginary) > (&Integer::ZERO, &Integer::ZERO) {
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
    gaussian_integer_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, exp)| {
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
            ComparableGaussianIntegerRef(a) <= ComparableGaussianIntegerRef(b)
        }));
        assert_eq!(
            roots.contains(root.as_ref().unwrap_or(&GaussianInteger::ZERO)),
            root.is_some()
        );
        if x == GaussianInteger::ZERO {
            assert_eq!(roots, vec![GaussianInteger::ZERO]);
        } else if root.is_some() {
            let g = u64::power_of_2(TrailingZeros::trailing_zeros(exp).min(2));
            assert_eq!(roots.len(), usize::exact_from(g));
            // the roots are the rotations of the principal one by multiples of 2 pi / g
            let principal = root.as_ref().unwrap();
            for r in &roots {
                assert_eq!(r.pow(exp), x);
            }
            for j in 0..g {
                assert!(roots.contains(&principal.mul_i_pow(j * (4 / g))));
            }
        } else {
            assert!(roots.is_empty());
        }

        // brute force over Gaussian integers of the right norm, where that is cheap
        let norm_bits = x
            .real
            .significant_bits()
            .max(x.imaginary.significant_bits())
            << 1;
        if norm_bits <= 40 {
            assert_eq!(
                sorted_strings(&roots),
                sorted_strings(&gaussian_integer_checked_roots_naive(&x, exp))
            );
        }

        // a power has its base among the roots, and the principal rotation of it as the root
        let power = (&x).pow(exp);
        assert_eq!((&power).checked_root(exp), Some(principal(x.clone(), exp)));
        assert!(power.checked_roots(exp).contains(&x));
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!((&x).checked_root(1), Some(x.clone()));
        assert_eq!((&x).checked_root(2), (&x).checked_sqrt());
        assert_eq!(x.checked_roots(2), x.checked_sqrts());
    });

    integer_unsigned_pair_gen_var_3::<u64>().test_properties(|(n, exp)| {
        // A real number can have a non-real Gaussian root (16 = (1+i)^8), so the comparison only
        // runs one way: an integer root is the principal Gaussian root, and a real Gaussian root is
        // the integer root.
        let root = GaussianInteger::from(n.clone()).checked_root(exp);
        if let Some(r) = (&n).checked_root(exp) {
            assert_eq!(root, Some(GaussianInteger::from(r)));
        } else if let Some(root) = root {
            assert_ne!(root.imaginary, 0);
        }
    });
}
