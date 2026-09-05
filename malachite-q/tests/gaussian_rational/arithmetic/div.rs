// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedDiv, Conjugate, DivI, MulI, Reciprocal};
use malachite_base::num::basic::traits::{I, NegativeOne, One, Zero};
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::gaussian_rational::arithmetic::div::gaussian_rational_div_naive;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_gen_var_3, gaussian_rational_pair_gen,
    gaussian_rational_pair_gen_var_1, rational_pair_gen_var_1,
};
use std::str::FromStr;

#[test]
fn test_div() {
    let test = |s, t, out| {
        let u = GaussianRational::from_str(s).unwrap();
        let v = GaussianRational::from_str(t).unwrap();

        let mut n = u.clone();
        n /= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let mut n = u.clone();
        n /= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = u.clone() / v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = &u / v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = u.clone() / &v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        let n = &u / &v;
        assert_eq!(n.to_string(), out);
        assert!(n.real.is_valid());
        assert!(n.imaginary.is_valid());

        assert_eq!(gaussian_rational_div_naive(&u, &v).to_string(), out);
    };
    test("0", "1", "0");
    test("0", "i", "0");
    test("0", "1+i", "0");
    test("1", "1", "1");
    test("1", "i", "-i");
    test("i", "i", "1");
    test("1", "-i", "i");
    test("1", "1+i", "1/2-i/2");
    test("1+i", "1-i", "i");
    test("1+i", "1+i", "1");
    test("2+2i", "2", "1+i");
    test("2+2i", "2i", "1-i");
    test("2+2i", "1/2", "4+4i");
    test("3+4i", "5", "3/5+4i/5");
    test("3+4i", "3-4i", "-7/25+24i/25");
    test("22/7+i", "1/2+i/3", "480/91-138i/91");
    test("1/2+i/3", "22/7+i", "280/1599+161i/3198");
    test("-22/7+3i/5", "-22/7+3i/5", "1");
    test("1000000000000", "i", "-1000000000000i");
    test(
        "1000000000000+i",
        "1000000000000-i",
        concat!(
            "999999999999999999999999/1000000000000000000000001",
            "+2000000000000i/1000000000000000000000001"
        ),
    );
}

#[test]
fn test_checked_div() {
    let test = |s, t, out: Option<&str>| {
        let u = GaussianRational::from_str(s).unwrap();
        let v = GaussianRational::from_str(t).unwrap();
        let out = out.map(|o| GaussianRational::from_str(o).unwrap());

        assert_eq!(u.clone().checked_div(v.clone()), out);
        assert_eq!(u.clone().checked_div(&v), out);
        assert_eq!((&u).checked_div(v.clone()), out);
        assert_eq!((&u).checked_div(&v), out);
    };
    test("0", "0", None);
    test("1", "0", None);
    test("1+i", "0", None);
    test("0", "1+i", Some("0"));
    test("1", "i", Some("-i"));
    test("1+i", "1-i", Some("i"));
    test("22/7+i", "1/2+i/3", Some("480/91-138i/91"));
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_fail_1() {
    GaussianRational::ONE / GaussianRational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_fail_2() {
    GaussianRational::ZERO / GaussianRational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_val_ref_fail_1() {
    GaussianRational::ONE / &GaussianRational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_val_ref_fail_2() {
    GaussianRational::ZERO / &GaussianRational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_ref_val_fail_1() {
    &GaussianRational::ONE / GaussianRational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_ref_val_fail_2() {
    &GaussianRational::ZERO / GaussianRational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_ref_ref_fail_1() {
    &GaussianRational::ONE / &GaussianRational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_ref_ref_fail_2() {
    &GaussianRational::ZERO / &GaussianRational::ZERO;
}

#[test]
#[should_panic]
fn div_assign_fail_1() {
    let mut x = GaussianRational::ONE;
    x /= GaussianRational::ZERO;
}

#[test]
#[should_panic]
fn div_assign_fail_2() {
    let mut x = GaussianRational::ZERO;
    x /= GaussianRational::ZERO;
}

#[test]
#[should_panic]
fn div_assign_ref_fail_1() {
    let mut x = GaussianRational::ONE;
    x /= &GaussianRational::ZERO;
}

#[test]
#[should_panic]
fn div_assign_ref_fail_2() {
    let mut x = GaussianRational::ZERO;
    x /= &GaussianRational::ZERO;
}

#[test]
fn div_properties() {
    gaussian_rational_pair_gen_var_1().test_properties(|(x, y)| {
        let quotient_val_val = x.clone() / y.clone();
        let quotient_val_ref = x.clone() / &y;
        let quotient_ref_val = &x / y.clone();
        let quotient = &x / &y;
        assert!(quotient_val_val.real.is_valid());
        assert!(quotient_val_val.imaginary.is_valid());
        assert!(quotient_val_ref.real.is_valid());
        assert!(quotient_val_ref.imaginary.is_valid());
        assert!(quotient_ref_val.real.is_valid());
        assert!(quotient_ref_val.imaginary.is_valid());
        assert!(quotient.real.is_valid());
        assert!(quotient.imaginary.is_valid());
        assert_eq!(quotient_val_val, quotient);
        assert_eq!(quotient_val_ref, quotient);
        assert_eq!(quotient_ref_val, quotient);

        let mut mut_x = x.clone();
        mut_x /= y.clone();
        assert!(mut_x.real.is_valid());
        assert!(mut_x.imaginary.is_valid());
        assert_eq!(mut_x, quotient);
        let mut mut_x = x.clone();
        mut_x /= &y;
        assert!(mut_x.real.is_valid());
        assert!(mut_x.imaginary.is_valid());
        assert_eq!(mut_x, quotient);

        assert_eq!(gaussian_rational_div_naive(&x, &y), quotient);
        assert_eq!((&x).checked_div(&y).unwrap(), quotient);
        assert_eq!(&x * (&y).reciprocal(), quotient);
        assert_eq!(&quotient * &y, x);
        if quotient != 0u32 {
            assert_eq!(&y / &x, (&quotient).reciprocal());
            assert_eq!(&x / &quotient, y);
        }
        assert_eq!(-&x / &y, -&quotient);
        assert_eq!(&x / -&y, -&quotient);
        assert_eq!((&x).conjugate() / (&y).conjugate(), (&quotient).conjugate());
        assert_eq!((&x).mul_i() / &y, (&quotient).mul_i());
        assert_eq!(&x / (&y).mul_i(), (&quotient).div_i());
    });

    gaussian_rational_gen().test_properties(|ref x| {
        assert_eq!(x / GaussianRational::ONE, *x);
        assert_eq!(x / GaussianRational::NEGATIVE_ONE, -x);
        assert_eq!(x / GaussianRational::I, x.div_i());
    });

    gaussian_rational_gen_var_3().test_properties(|ref x| {
        assert_eq!(GaussianRational::ZERO / x, GaussianRational::ZERO);
        assert_eq!(GaussianRational::ONE / x, x.reciprocal());
        assert_eq!(GaussianRational::NEGATIVE_ONE / x, -x.reciprocal());
        assert_eq!(x / x, GaussianRational::ONE);
    });

    rational_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(
            GaussianRational::from(x.clone()) / GaussianRational::from(y.clone()),
            GaussianRational::from(x / y)
        );
    });
}

#[test]
fn checked_div_properties() {
    gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let quotient = (&x).checked_div(&y);
        assert_eq!(x.clone().checked_div(y.clone()), quotient);
        assert_eq!(x.clone().checked_div(&y), quotient);
        assert_eq!((&x).checked_div(y.clone()), quotient);
        assert_eq!(quotient.is_none(), y == 0u32);
        if let Some(quotient) = quotient {
            assert_eq!(&x / &y, quotient);
        }
    });
}
