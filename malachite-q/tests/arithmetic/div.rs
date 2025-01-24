// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedDiv, Reciprocal};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_q::test_util::arithmetic::div::div_naive;
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_1, rational_pair_gen, rational_pair_gen_var_1,
    rational_triple_gen_var_1,
};
use malachite_q::Rational;
use num::{BigRational, CheckedDiv as NumCheckedDiv};
use std::str::FromStr;

#[test]
fn test_div() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut n = u.clone();
        n /= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n /= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() / v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u / v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() / &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u / &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigRational::from_str(s).unwrap() / BigRational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Rational::from_str(s).unwrap() / rug::Rational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "1/123", "0");
    test("0", "-1/123", "0");
    test("1", "1/123", "123");
    test("1", "-1/123", "-123");
    test("-1", "1/123", "-123");
    test("-1", "-1/123", "123");
    test("123", "1", "123");
    test("123", "-1", "-123");
    test("-123", "1", "-123");
    test("-123", "-1", "123");
    test("123", "1/456", "56088");
    test("123", "-1/456", "-56088");
    test("-123", "1/456", "-56088");
    test("-123", "-1/456", "56088");
    test("22/7", "2/3", "33/7");
    test("22/7", "-2/3", "-33/7");
    test("-22/7", "2/3", "-33/7");
    test("-22/7", "-2/3", "33/7");
    test("4/5", "4/5", "1");
    test("4/5", "-4/5", "-1");
    test("-4/5", "4/5", "-1");
    test("-4/5", "-4/5", "1");
}

#[test]
fn test_checked_div() {
    let test = |s, t, out: Option<&'static str>| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let n = u.clone().checked_div(v.clone());
        assert_eq!(n.as_ref().map(ToString::to_string).as_deref(), out);
        if out.is_some() {
            assert!(n.unwrap().is_valid());
        }

        let n = (&u).checked_div(v.clone());
        assert_eq!(n.as_ref().map(ToString::to_string).as_deref(), out);
        if out.is_some() {
            assert!(n.unwrap().is_valid());
        }

        let n = u.clone().checked_div(&v);
        assert_eq!(n.as_ref().map(ToString::to_string).as_deref(), out);
        if out.is_some() {
            assert!(n.unwrap().is_valid());
        }

        let n = (&u).checked_div(&v);
        assert_eq!(n.as_ref().map(ToString::to_string).as_deref(), out);
        if out.is_some() {
            assert!(n.unwrap().is_valid());
        }

        let n = BigRational::from_str(s)
            .unwrap()
            .checked_div(&BigRational::from_str(t).unwrap());
        assert_eq!(n.as_ref().map(ToString::to_string).as_deref(), out);
    };
    test("0", "1/123", Some("0"));
    test("0", "-1/123", Some("0"));
    test("0", "0", None);
    test("1", "1/123", Some("123"));
    test("1", "-1/123", Some("-123"));
    test("1", "0", None);
    test("-1", "1/123", Some("-123"));
    test("-1", "-1/123", Some("123"));
    test("-1", "0", None);
    test("123", "1", Some("123"));
    test("123", "-1", Some("-123"));
    test("123", "0", None);
    test("-123", "1", Some("-123"));
    test("-123", "-1", Some("123"));
    test("-123", "0", None);
    test("123", "1/456", Some("56088"));
    test("123", "-1/456", Some("-56088"));
    test("-123", "1/456", Some("-56088"));
    test("-123", "-1/456", Some("56088"));
    test("22/7", "2/3", Some("33/7"));
    test("22/7", "-2/3", Some("-33/7"));
    test("22/7", "0", None);
    test("-22/7", "2/3", Some("-33/7"));
    test("-22/7", "-2/3", Some("33/7"));
    test("-22/7", "0", None);
    test("4/5", "4/5", Some("1"));
    test("4/5", "-4/5", Some("-1"));
    test("4/5", "0", None);
    test("-4/5", "4/5", Some("-1"));
    test("-4/5", "-4/5", Some("1"));
    test("-4/5", "0", None);
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_fail_1() {
    Rational::ONE / Rational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_fail_2() {
    Rational::ZERO / Rational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_val_ref_fail_1() {
    Rational::ONE / &Rational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_val_ref_fail_2() {
    Rational::ZERO / &Rational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_ref_val_fail_1() {
    &Rational::ONE / Rational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_ref_val_fail_2() {
    &Rational::ZERO / Rational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_ref_ref_fail_1() {
    &Rational::ONE / &Rational::ZERO;
}

#[allow(clippy::no_effect, unused_must_use)]
#[test]
#[should_panic]
fn div_ref_ref_fail_2() {
    &Rational::ZERO / &Rational::ZERO;
}

#[test]
#[should_panic]
fn div_assign_fail_1() {
    let mut x = Rational::ONE;
    x /= Rational::ZERO;
}

#[test]
#[should_panic]
fn div_assign_fail_2() {
    let mut x = Rational::ZERO;
    x /= Rational::ZERO;
}

#[test]
#[should_panic]
fn div_assign_ref_fail_1() {
    let mut x = Rational::ONE;
    x /= &Rational::ZERO;
}

#[test]
#[should_panic]
fn div_assign_ref_fail_2() {
    let mut x = Rational::ZERO;
    x /= &Rational::ZERO;
}

#[allow(clippy::eq_op)]
#[test]
fn div_properties() {
    rational_pair_gen_var_1().test_properties(|(x, y)| {
        let quotient_val_val = x.clone() / y.clone();
        let quotient_val_ref = x.clone() / &y;
        let quotient_ref_val = &x / y.clone();
        let quotient = &x / &y;
        assert!(quotient_val_val.is_valid());
        assert!(quotient_val_ref.is_valid());
        assert!(quotient_ref_val.is_valid());
        assert!(quotient.is_valid());
        assert_eq!(quotient_val_val, quotient);
        assert_eq!(quotient_val_ref, quotient);
        assert_eq!(quotient_ref_val, quotient);

        let mut mut_x = x.clone();
        mut_x /= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, quotient);
        let mut mut_x = x.clone();
        mut_x /= &y;
        assert_eq!(mut_x, quotient);
        assert!(mut_x.is_valid());

        let mut mut_x = rug::Rational::from(&x);
        mut_x /= rug::Rational::from(&y);
        assert_eq!(Rational::from(&mut_x), quotient);

        assert_eq!(
            Rational::from(&(BigRational::from(&x) / BigRational::from(&y))),
            quotient
        );
        assert_eq!(
            Rational::from(&(rug::Rational::from(&x) / rug::Rational::from(&y))),
            quotient
        );
        assert_eq!(div_naive(x.clone(), y.clone()), quotient);
        assert_eq!((&x).checked_div(y.clone()).unwrap(), quotient);
        assert_eq!(&x * (&y).reciprocal(), quotient);
        assert_eq!(&quotient * &y, x);
        if quotient != 0u32 {
            assert_eq!(&y / &x, (&quotient).reciprocal());
            assert_eq!(&x / &quotient, y);
        }
        assert_eq!(-&x / &y, -&quotient);
        assert_eq!(x / -y, -quotient);
    });

    rational_gen().test_properties(|ref x| {
        assert_eq!(x / Rational::ONE, *x);
        assert_eq!(x / Rational::NEGATIVE_ONE, -x);
    });

    rational_gen_var_1().test_properties(|ref x| {
        assert_eq!(Rational::ZERO / x, 0);
        assert_eq!(Rational::ONE / x, x.reciprocal());
        assert_eq!(Rational::NEGATIVE_ONE / x, -x.reciprocal());
        assert_eq!(x / x, 1);
    });

    rational_triple_gen_var_1().test_properties(|(ref x, ref y, ref z)| {
        assert_eq!((x + y) / z, x / z + y / z);
        assert_eq!((x - y) / z, x / z - y / z);
    });
}

#[test]
fn checked_div_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let quotient_val_val = x.clone().checked_div(y.clone());
        let quotient_val_ref = x.clone().checked_div(&y);
        let quotient_ref_val = (&x).checked_div(y.clone());
        let quotient = (&x).checked_div(&y);
        assert!(quotient_val_val.as_ref().map_or(true, Rational::is_valid));
        assert!(quotient_val_ref.as_ref().map_or(true, Rational::is_valid));
        assert!(quotient_ref_val.as_ref().map_or(true, Rational::is_valid));
        assert!(quotient.as_ref().map_or(true, Rational::is_valid));
        assert_eq!(quotient_val_val, quotient);
        assert_eq!(quotient_val_ref, quotient);
        assert_eq!(quotient_ref_val, quotient);

        if y != 0u32 {
            assert_eq!(quotient, Some(&x / &y));
        }

        assert_eq!(
            BigRational::from(&x)
                .checked_div(&BigRational::from(&y))
                .map(|q| Rational::from(&q)),
            quotient
        );
    });

    rational_gen().test_properties(|ref x| {
        assert_eq!(x.checked_div(Rational::ZERO), None);
        assert_eq!(x.checked_div(Rational::ONE), Some(x.clone()));
        assert_eq!(x.checked_div(Rational::NEGATIVE_ONE), Some(-x));
    });

    rational_gen_var_1().test_properties(|ref x| {
        assert_eq!(Rational::ZERO.checked_div(x), Some(Rational::ZERO));
        assert_eq!(Rational::ONE.checked_div(x), Some(x.reciprocal()));
        assert_eq!(Rational::NEGATIVE_ONE.checked_div(x), Some(-x.reciprocal()));
        assert_eq!(x.checked_div(x), Some(Rational::ONE));
    });
}
