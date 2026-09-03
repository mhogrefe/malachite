// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, DivRem};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::gaussian_integer::arithmetic::div_rem::*;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_gen_var_3, gaussian_integer_pair_gen_var_1,
    gaussian_integer_pair_gen_var_3,
};
use std::str::FromStr;

#[test]
fn test_rem() {
    let test = |s, t, out| {
        let u = GaussianInteger::from_str(s).unwrap();
        let v = GaussianInteger::from_str(t).unwrap();

        let mut x = u.clone();
        x %= v.clone();
        assert_eq!(x.to_string(), out);
        assert!(x.real.is_valid());
        assert!(x.imaginary.is_valid());

        let mut x = u.clone();
        x %= &v;
        assert_eq!(x.to_string(), out);

        assert_eq!((u.clone() % v.clone()).to_string(), out);
        assert_eq!((u.clone() % &v).to_string(), out);
        assert_eq!((&u % v.clone()).to_string(), out);
        assert_eq!((&u % &v).to_string(), out);

        assert_eq!(gaussian_integer_div_rem_naive(&u, &v).1.to_string(), out);
    };
    test("0", "1", "0");
    test("0", "3+4i", "0");
    test("1", "100+100i", "1");
    test("-3i", "1000", "-3i");
    test("23+14i", "5-2i", "0");
    test("6+9i", "3", "0");
    test("6+9i", "3i", "0");
    test("1+i", "1+i", "0");
    test("5+3i", "2+i", "-1");
    test("1", "1+i", "-i");
    test("i", "1+i", "-i");
    test("7", "2", "-1");
    test("-7", "2", "-1");
    test("3i", "2", "-i");
    test("1", "2", "-1");
    test("-1", "2", "-1");
    test("1+i", "2", "-1-i");
    test("-1-i", "2", "-1-i");
    test("1000000000001", "7", "2");
    test(
        "123456789012345678901234567890+98765432109876543210i",
        "12345678901234567890-9876543210i",
        "-156790123470+1790123456970i",
    );
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut x = GaussianInteger::ONE;
    x %= GaussianInteger::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_ref_fail() {
    let mut x = GaussianInteger::ONE;
    x %= &GaussianInteger::ZERO;
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ = GaussianInteger::ONE % GaussianInteger::ZERO;
}

#[test]
#[should_panic]
fn rem_val_ref_fail() {
    let _ = GaussianInteger::ONE % &GaussianInteger::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_val_fail() {
    let _ = &GaussianInteger::ONE % GaussianInteger::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_ref_fail() {
    let _ = &GaussianInteger::ONE % &GaussianInteger::ZERO;
}

#[allow(clippy::needless_pass_by_value)]
fn rem_properties_helper(x: GaussianInteger, y: GaussianInteger) {
    let mut mut_x = x.clone();
    mut_x %= &y;
    assert!(mut_x.real.is_valid());
    assert!(mut_x.imaginary.is_valid());
    let r = mut_x;

    let mut mut_x = x.clone();
    mut_x %= y.clone();
    assert_eq!(mut_x, r);

    let r_alt = &x % &y;
    assert!(r_alt.real.is_valid());
    assert!(r_alt.imaginary.is_valid());
    assert_eq!(r_alt, r);
    assert_eq!(&x % y.clone(), r);
    assert_eq!(x.clone() % &y, r);
    assert_eq!(x.clone() % y.clone(), r);

    assert_eq!((&x).div_rem(&y).1, r);
    assert_eq!(gaussian_integer_div_rem_naive(&x, &y).1, r);
    assert!((&r).abs_squared() << 1u64 <= (&y).abs_squared());
    assert_eq!(&r % &y, r);
}

#[test]
fn rem_properties() {
    gaussian_integer_pair_gen_var_3().test_properties(|(x, y)| {
        rem_properties_helper(x, y);
    });

    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 512);
    gaussian_integer_pair_gen_var_3().test_properties_with_config(&config, |(x, y)| {
        rem_properties_helper(x, y);
    });

    gaussian_integer_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(x % y, GaussianInteger::ZERO);
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(x % GaussianInteger::ONE, GaussianInteger::ZERO);
    });

    gaussian_integer_gen_var_3().test_properties(|x| {
        assert_eq!(GaussianInteger::ZERO % &x, GaussianInteger::ZERO);
        assert_eq!(&x % &x, GaussianInteger::ZERO);
    });
}
