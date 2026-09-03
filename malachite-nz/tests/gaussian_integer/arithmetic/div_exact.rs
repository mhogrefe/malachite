// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, DivExact, DivExactAssign, DivI, MulI};
use malachite_base::num::basic::traits::{I, One, Zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::gaussian_integer::arithmetic::div_exact::*;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_gen_var_3, gaussian_integer_pair_gen_var_2,
    integer_pair_gen_var_2,
};
use std::str::FromStr;

#[test]
fn test_div_exact() {
    let test = |s, t, out| {
        let u = GaussianInteger::from_str(s).unwrap();
        let v = GaussianInteger::from_str(t).unwrap();

        let mut n = u.clone();
        n.div_exact_assign(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let mut n = u.clone();
        n.div_exact_assign(&v);
        assert_eq!(n.to_string(), out);

        let n = u.clone().div_exact(v.clone());
        assert_eq!(n.to_string(), out);

        let n = u.clone().div_exact(&v);
        assert_eq!(n.to_string(), out);

        let n = (&u).div_exact(v.clone());
        assert_eq!(n.to_string(), out);

        let n = (&u).div_exact(&v);
        assert_eq!(n.to_string(), out);

        assert_eq!(gaussian_integer_div_exact_naive(&u, &v).to_string(), out);
    };
    // purely real divisors
    test("0", "1", "0");
    test("0", "-3", "0");
    test("6+9i", "3", "2+3i");
    test("6+9i", "-3", "-2-3i");
    test("123", "123", "1");
    // purely imaginary divisors
    test("0", "i", "0");
    test("6+9i", "3i", "3-2i");
    test("6+9i", "-3i", "-3+2i");
    test("i", "i", "1");
    test("-1", "i", "i");
    // zero dividend
    test("0", "1+i", "0");
    test("0", "3+4i", "0");
    // small quotients: the double-precision path
    test("23+14i", "5-2i", "3+4i");
    test("2", "1+i", "1-i");
    test("2i", "1+i", "1+i");
    test("3+4i", "3+4i", "1");
    test("-7+24i", "3+4i", "3+4i");
    test("1000000000000+1000000000000i", "1+i", "1000000000000");
    test("1000000000001+999999999999i", "1+i", "1000000000000-i");
    // large quotients: the general path
    test(
        "1000000000000000000000000000000000000000+1000000000000000000000000000000000000000i",
        "1+i",
        "1000000000000000000000000000000000000000",
    );
}

#[test]
#[should_panic]
fn div_exact_fail() {
    GaussianInteger::ONE.div_exact(GaussianInteger::ZERO);
}

#[test]
#[should_panic]
fn div_exact_val_ref_fail() {
    GaussianInteger::ONE.div_exact(&GaussianInteger::ZERO);
}

#[test]
#[should_panic]
fn div_exact_ref_val_fail() {
    (&GaussianInteger::ONE).div_exact(GaussianInteger::ZERO);
}

#[test]
#[should_panic]
fn div_exact_ref_ref_fail() {
    (&GaussianInteger::ONE).div_exact(&GaussianInteger::ZERO);
}

#[test]
#[should_panic]
fn div_exact_assign_fail() {
    let mut x = GaussianInteger::ONE;
    x.div_exact_assign(GaussianInteger::ZERO);
}

#[test]
#[should_panic]
fn div_exact_assign_ref_fail() {
    let mut x = GaussianInteger::ONE;
    x.div_exact_assign(&GaussianInteger::ZERO);
}

#[allow(clippy::needless_pass_by_value)]
fn div_exact_properties_helper(x: GaussianInteger, y: GaussianInteger) {
    let mut mut_x = x.clone();
    mut_x.div_exact_assign(&y);
    assert!(mut_x.real.is_valid());
    assert!(mut_x.imaginary.is_valid());
    let q = mut_x;

    let mut mut_x = x.clone();
    mut_x.div_exact_assign(y.clone());
    assert_eq!(mut_x, q);

    let q_alt = (&x).div_exact(&y);
    assert!(q_alt.real.is_valid());
    assert!(q_alt.imaginary.is_valid());
    assert_eq!(q_alt, q);
    assert_eq!((&x).div_exact(y.clone()), q);
    assert_eq!(x.clone().div_exact(&y), q);
    assert_eq!(x.clone().div_exact(y.clone()), q);

    assert_eq!(gaussian_integer_div_exact_naive(&x, &y), q);
    assert_eq!(&q * &y, x);
    assert_eq!((-&x).div_exact(&y), -&q);
    assert_eq!((&x).div_exact(-&y), -&q);
    assert_eq!(
        (&x).conjugate().div_exact((&y).conjugate()),
        (&q).conjugate()
    );
    assert_eq!((&x).mul_i().div_exact(&y), (&q).mul_i());
    assert_eq!((&x).div_exact((&y).mul_i()), (&q).div_i());
    if q != GaussianInteger::ZERO {
        assert_eq!((&x).div_exact(&q), y);
    }
}

#[test]
fn div_exact_properties() {
    gaussian_integer_pair_gen_var_2().test_properties(|(x, y)| {
        div_exact_properties_helper(x, y);
    });

    // Small quotients of small operands: the unscaled double-precision path.
    gaussian_integer_pair_gen_var_2().test_properties(|(x, y)| {
        div_exact_properties_helper(x, y);
    });

    // Small quotients of large operands: the scaled double-precision path.
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 700);
    gaussian_integer_pair_gen_var_2().test_properties_with_config(&config, |(x, y)| {
        div_exact_properties_helper(x, y);
    });

    // Large quotients of large operands: the general path.
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 512);
    gaussian_integer_pair_gen_var_2().test_properties_with_config(&config, |(x, y)| {
        div_exact_properties_helper(x, y);
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!((&x).div_exact(GaussianInteger::ONE), x);
        assert_eq!((&x).div_exact(GaussianInteger::I), (&x).div_i());
    });

    gaussian_integer_gen_var_3().test_properties(|x| {
        assert_eq!(GaussianInteger::ZERO.div_exact(&x), GaussianInteger::ZERO);
        assert_eq!((&x).div_exact(&x), GaussianInteger::ONE);
    });

    integer_pair_gen_var_2().test_properties(|(x, y)| {
        assert_eq!(
            GaussianInteger::from(x.clone()).div_exact(GaussianInteger::from(y.clone())),
            GaussianInteger::from(x.div_exact(y))
        );
    });
}
