// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedDiv, DivRem};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::gaussian_integer::arithmetic::div_rem::*;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_gen_var_3, gaussian_integer_pair_gen,
    gaussian_integer_pair_gen_var_1, gaussian_integer_pair_gen_var_3,
};
use std::str::FromStr;

#[test]
fn test_div() {
    let test = |s, t, out| {
        let u = GaussianInteger::from_str(s).unwrap();
        let v = GaussianInteger::from_str(t).unwrap();

        let mut x = u.clone();
        x /= v.clone();
        assert_eq!(x.to_string(), out);
        assert!(x.real.is_valid());
        assert!(x.imaginary.is_valid());

        let mut x = u.clone();
        x /= &v;
        assert_eq!(x.to_string(), out);

        assert_eq!((u.clone() / v.clone()).to_string(), out);
        assert_eq!((u.clone() / &v).to_string(), out);
        assert_eq!((&u / v.clone()).to_string(), out);
        assert_eq!((&u / &v).to_string(), out);

        assert_eq!(gaussian_integer_div_rem_naive(&u, &v).0.to_string(), out);
    };
    test("0", "1", "0");
    test("0", "3+4i", "0");
    test("1", "100+100i", "0");
    test("-3i", "1000", "0");
    test("23+14i", "5-2i", "3+4i");
    test("6+9i", "3", "2+3i");
    test("6+9i", "3i", "3-2i");
    test("1+i", "1+i", "1");
    test("5+3i", "2+i", "3");
    test("1", "1+i", "1");
    test("i", "1+i", "1+i");
    test("7", "2", "4");
    test("-7", "2", "-3");
    test("3i", "2", "2i");
    test("1", "2", "1");
    test("-1", "2", "0");
    test("1+i", "2", "1+i");
    test("-1-i", "2", "0");
    test("1000000000001", "7", "142857142857");
    test(
        "123456789012345678901234567890+98765432109876543210i",
        "12345678901234567890-9876543210i",
        "10000000000+16i",
    );
}

#[test]
#[should_panic]
fn div_assign_fail() {
    let mut x = GaussianInteger::ONE;
    x /= GaussianInteger::ZERO;
}

#[test]
#[should_panic]
fn div_assign_ref_fail() {
    let mut x = GaussianInteger::ONE;
    x /= &GaussianInteger::ZERO;
}

#[test]
#[should_panic]
fn div_fail() {
    let _ = GaussianInteger::ONE / GaussianInteger::ZERO;
}

#[test]
#[should_panic]
fn div_val_ref_fail() {
    let _ = GaussianInteger::ONE / &GaussianInteger::ZERO;
}

#[test]
#[should_panic]
fn div_ref_val_fail() {
    let _ = &GaussianInteger::ONE / GaussianInteger::ZERO;
}

#[test]
#[should_panic]
fn div_ref_ref_fail() {
    let _ = &GaussianInteger::ONE / &GaussianInteger::ZERO;
}

#[test]
fn test_checked_div() {
    let test = |s, t, out: Option<&str>| {
        let u = GaussianInteger::from_str(s).unwrap();
        let v = GaussianInteger::from_str(t).unwrap();
        let out = out.map(ToString::to_string);

        assert_eq!(u.clone().checked_div(v.clone()).map(|x| x.to_string()), out);
        assert_eq!(u.clone().checked_div(&v).map(|x| x.to_string()), out);
        assert_eq!((&u).checked_div(v.clone()).map(|x| x.to_string()), out);
        assert_eq!((&u).checked_div(&v).map(|x| x.to_string()), out);
    };
    test("0", "0", None);
    test("1", "0", None);
    test("3+4i", "0", None);
    test("0", "1", Some("0"));
    test("23+14i", "5-2i", Some("3+4i"));
    test("5+3i", "2+i", Some("3"));
    test("-7", "2", Some("-3"));
}

#[allow(clippy::needless_pass_by_value)]
fn div_properties_helper(x: GaussianInteger, y: GaussianInteger) {
    let mut mut_x = x.clone();
    mut_x /= &y;
    assert!(mut_x.real.is_valid());
    assert!(mut_x.imaginary.is_valid());
    let q = mut_x;

    let mut mut_x = x.clone();
    mut_x /= y.clone();
    assert_eq!(mut_x, q);

    let q_alt = &x / &y;
    assert!(q_alt.real.is_valid());
    assert!(q_alt.imaginary.is_valid());
    assert_eq!(q_alt, q);
    assert_eq!(&x / y.clone(), q);
    assert_eq!(x.clone() / &y, q);
    assert_eq!(x.clone() / y.clone(), q);

    assert_eq!((&x).checked_div(&y), Some(q.clone()));
    assert_eq!((&x).div_rem(&y).0, q);
    assert_eq!(gaussian_integer_div_rem_naive(&x, &y).0, q);
}

#[test]
fn div_properties() {
    gaussian_integer_pair_gen_var_3().test_properties(|(x, y)| {
        div_properties_helper(x, y);
    });

    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 512);
    gaussian_integer_pair_gen_var_3().test_properties_with_config(&config, |(x, y)| {
        div_properties_helper(x, y);
    });

    gaussian_integer_pair_gen_var_1().test_properties(|(x, y)| {
        let q = &x / &y;
        assert_eq!(&q * &y, x);
    });

    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let q = (&x).checked_div(&y);
        assert_eq!(q.is_none(), y == 0u32);
        if let Some(q) = q {
            assert_eq!(q, x / y);
        }
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(&x / GaussianInteger::ONE, x);
        assert_eq!((&x).checked_div(GaussianInteger::ZERO), None);
    });

    gaussian_integer_gen_var_3().test_properties(|x| {
        assert_eq!(GaussianInteger::ZERO / &x, GaussianInteger::ZERO);
        assert_eq!(&x / &x, GaussianInteger::ONE);
    });
}
