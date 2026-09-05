// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CanonicalizeUnit, Conjugate, DivExact, Gcd, GcdAssign, MulI,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::gaussian_integer::arithmetic::gcd::*;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_pair_gen, gaussian_integer_triple_gen, integer_pair_gen,
};
use std::str::FromStr;

#[test]
fn test_gcd() {
    let test = |s, t, out| {
        let u = GaussianInteger::from_str(s).unwrap();
        let v = GaussianInteger::from_str(t).unwrap();

        let mut n = u.clone();
        n.gcd_assign(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let mut n = u.clone();
        n.gcd_assign(&v);
        assert_eq!(n.to_string(), out);

        assert_eq!(u.clone().gcd(v.clone()).to_string(), out);
        assert_eq!(u.clone().gcd(&v).to_string(), out);
        assert_eq!((&u).gcd(v.clone()).to_string(), out);
        assert_eq!((&u).gcd(&v).to_string(), out);

        assert_eq!(gaussian_integer_gcd_euclidean(&u, &v).to_string(), out);
        assert_eq!(gaussian_integer_gcd_binary(&u, &v).to_string(), out);
    };
    // zeros: the other argument in canonical unit form
    test("0", "0", "0");
    test("0", "3+4i", "4-3i");
    test("3+4i", "0", "4-3i");
    test("0", "-2", "2");
    test("0", "-2i", "2");
    // units
    test("1", "1+i", "1");
    test("3+4i", "4+3i", "1");
    // the prime above 2
    test("2", "1+i", "1+i");
    test("2", "2", "2");
    test("-2", "2", "2");
    test("2i", "2", "2");
    // split primes
    test("5", "2+i", "2+i");
    test("5", "2-i", "2-i");
    test("13", "3+2i", "3+2i");
    test("3+4i", "5", "2+i");
    // composite
    test("6+8i", "4+2i", "4+2i");
    test("-7+24i", "3+4i", "4-3i");
    test("10+15i", "6+9i", "3-2i");
    test("12", "18", "6");
    test("1000000000000", "999999999999+i", "1+i");
    test("123456789+987654321i", "111111111+222222222i", "9");
    // beyond the double-precision kernel
    test("1180591620717411303424+3i", "34359738369+7i", "1");
    test(
        "1000000000000000000000000000001+999999999999999999999999999999i",
        "1000000000000000+1000000000000003i",
        "1",
    );
    test(
        "-358024691735802469170+345679011934567901190i",
        "-1123456790812345679070+641975307364197530730i",
        "98765432109876543210-12345678901234567890i",
    );
}

#[allow(clippy::needless_pass_by_value)]
// The oracles are far slower than the port on large operands, so the large config runs only the
// Euclidean oracle, once.
fn gcd_properties_helper(x: GaussianInteger, y: GaussianInteger, thorough: bool) {
    let mut mut_x = x.clone();
    mut_x.gcd_assign(&y);
    assert!(mut_x.real.is_valid());
    assert!(mut_x.imaginary.is_valid());
    let g = mut_x;

    let mut mut_x = x.clone();
    mut_x.gcd_assign(y.clone());
    assert_eq!(mut_x, g);

    let g_alt = (&x).gcd(&y);
    assert!(g_alt.real.is_valid());
    assert!(g_alt.imaginary.is_valid());
    assert_eq!(g_alt, g);
    assert_eq!((&x).gcd(y.clone()), g);
    assert_eq!(x.clone().gcd(&y), g);
    assert_eq!(x.clone().gcd(y.clone()), g);

    assert_eq!(gaussian_integer_gcd_euclidean(&x, &y), g);
    if thorough {
        assert_eq!(gaussian_integer_gcd_binary(&x, &y), g);
    }

    // canonical unit form
    assert_eq!((&g).canonicalize_unit(), g);
    // symmetry and invariance under units
    assert_eq!((&y).gcd(&x), g);
    assert_eq!((-&x).gcd(&y), g);
    assert_eq!((&x).mul_i().gcd(&y), g);
    assert_eq!(
        (&x).conjugate().gcd((&y).conjugate()),
        (&g).conjugate().canonicalize_unit()
    );
    if g != 0u32 {
        // a common divisor, and the cofactors are coprime
        assert_eq!(&x % &g, GaussianInteger::ZERO);
        assert_eq!(&y % &g, GaussianInteger::ZERO);
        assert_eq!(
            (&x).div_exact(&g).gcd((&y).div_exact(&g)),
            GaussianInteger::ONE
        );
    }
    // an unbalanced pair takes the exact division tier of the approximate division
    if thorough {
        let x_big = &x << 100u32;
        assert_eq!((&x_big).gcd(&y), gaussian_integer_gcd_euclidean(&x_big, &y));
    }
}

#[test]
fn gcd_properties() {
    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        gcd_properties_helper(x, y, true);
    });

    // parts straddling the 50-bit limit of the double-precision kernel
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 60);
    gaussian_integer_pair_gen().test_properties_with_config(&config, |(x, y)| {
        gcd_properties_helper(x, y, true);
    });

    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 256);
    gaussian_integer_pair_gen().test_properties_with_config(&config, |(x, y)| {
        gcd_properties_helper(x, y, false);
    });

    gaussian_integer_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!(
            (&x * &z).gcd(&y * &z),
            ((&x).gcd(&y) * z).canonicalize_unit()
        );
    });

    gaussian_integer_gen().test_properties(|x| {
        let canonical = (&x).canonicalize_unit();
        assert_eq!((&x).gcd(GaussianInteger::ZERO), canonical);
        assert_eq!(GaussianInteger::ZERO.gcd(&x), canonical);
        assert_eq!((&x).gcd(&x), canonical);
        assert_eq!((&x).gcd(GaussianInteger::ONE), GaussianInteger::ONE);
    });

    integer_pair_gen().test_properties(|(a, b)| {
        let g: Natural = a.unsigned_abs_ref().gcd(b.unsigned_abs_ref());
        assert_eq!(
            GaussianInteger::from(a).gcd(GaussianInteger::from(b)),
            GaussianInteger::from(g)
        );
    });
}
