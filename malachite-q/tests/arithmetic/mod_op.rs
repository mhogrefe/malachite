// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CeilingMod, CeilingModAssign, Mod, ModAssign, NegMod,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::test_util::generators::{integer_pair_gen_var_1, natural_pair_gen_var_5};
use malachite_q::Rational;
use malachite_q::test_util::arithmetic::mod_op::{ceiling_mod_naive, mod_op_naive, rem_naive};
use malachite_q::test_util::generators::{rational_gen_var_1, rational_pair_gen_var_1};
use num::BigRational;
use std::str::FromStr;

#[test]
fn test_mod() {
    let test = |s, t, remainder| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut x = u.clone();
        x.mod_assign(v.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = u.clone();
        x.mod_assign(&v);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = u.clone().mod_op(v.clone());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = u.clone().mod_op(&v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&u).mod_op(v.clone());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&u).mod_op(&v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = mod_op_naive(u, v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
    };
    test("0", "1", "0");
    test("0", "123", "0");
    test("1", "1", "0");
    test("123", "1", "0");
    test("123", "123", "0");
    test("123", "456", "123");
    test("456", "123", "87");
    test("4/3", "1", "1/3");
    test("22/7", "1/2", "1/7");
    test("10", "22/7", "4/7");
    test("0", "-1", "0");
    test("0", "-123", "0");
    test("1", "-1", "0");
    test("123", "-1", "0");
    test("123", "-123", "0");
    test("123", "-456", "-333");
    test("456", "-123", "-36");
    test("4/3", "-1", "-2/3");
    test("22/7", "-1/2", "-5/14");
    test("10", "-22/7", "-18/7");
    test("-1", "1", "0");
    test("-123", "1", "0");
    test("-123", "123", "0");
    test("-123", "456", "333");
    test("-456", "123", "36");
    test("-4/3", "1", "2/3");
    test("-22/7", "1/2", "5/14");
    test("-10", "22/7", "18/7");
    test("-1", "-1", "0");
    test("-123", "-1", "0");
    test("-123", "-123", "0");
    test("-123", "-456", "-123");
    test("-456", "-123", "-87");
    test("-4/3", "-1", "-1/3");
    test("-22/7", "-1/2", "-1/7");
    test("-10", "-22/7", "-4/7");
}

#[test]
#[should_panic]
fn mod_assign_fail() {
    Rational::from(10).mod_assign(Rational::ZERO);
}

#[test]
#[should_panic]
fn mod_assign_ref_fail() {
    Rational::from(10).mod_assign(&Rational::ZERO);
}

#[test]
#[should_panic]
fn mod_fail() {
    Rational::from(10).mod_op(Rational::ZERO);
}

#[test]
#[should_panic]
fn mod_val_ref_fail() {
    Rational::from(10).mod_op(&Rational::ZERO);
}

#[test]
#[should_panic]
fn mod_ref_val_fail() {
    (&Rational::from(10)).mod_op(Rational::ZERO);
}

#[test]
#[should_panic]
fn mod_ref_ref_fail() {
    (&Rational::from(10)).mod_op(&Rational::ZERO);
}

#[test]
fn test_rem() {
    let test = |s, t, remainder| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut x = u.clone();
        x %= v.clone();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = u.clone();
        x %= &v;
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = u.clone() % v.clone();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = u.clone() % &v;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = &u % v.clone();
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = &u % &v;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = rem_naive(u, v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = BigRational::from_str(s).unwrap() % &BigRational::from_str(t).unwrap();
        assert_eq!(r.to_string(), remainder);
    };
    test("0", "1", "0");
    test("0", "123", "0");
    test("1", "1", "0");
    test("123", "1", "0");
    test("123", "123", "0");
    test("123", "456", "123");
    test("456", "123", "87");
    test("4/3", "1", "1/3");
    test("22/7", "1/2", "1/7");
    test("10", "22/7", "4/7");
    test("0", "-1", "0");
    test("0", "-123", "0");
    test("1", "-1", "0");
    test("123", "-1", "0");
    test("123", "-123", "0");
    test("123", "-456", "123");
    test("456", "-123", "87");
    test("4/3", "-1", "1/3");
    test("22/7", "-1/2", "1/7");
    test("10", "-22/7", "4/7");
    test("-1", "1", "0");
    test("-123", "1", "0");
    test("-123", "123", "0");
    test("-123", "456", "-123");
    test("-456", "123", "-87");
    test("-4/3", "1", "-1/3");
    test("-22/7", "1/2", "-1/7");
    test("-10", "22/7", "-4/7");
    test("-1", "-1", "0");
    test("-123", "-1", "0");
    test("-123", "-123", "0");
    test("-123", "-456", "-123");
    test("-456", "-123", "-87");
    test("-4/3", "-1", "-1/3");
    test("-22/7", "-1/2", "-1/7");
    test("-10", "-22/7", "-4/7");
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut x = Rational::from(10);
    x %= Rational::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_ref_fail() {
    let mut x = Rational::from(10);
    x %= &Rational::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn rem_fail() {
    Rational::from(10) % Rational::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn rem_val_ref_fail() {
    Rational::from(10) % &Rational::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn rem_ref_val_fail() {
    &Rational::from(10) % Rational::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn rem_ref_ref_fail() {
    &Rational::from(10) % &Rational::ZERO;
}

#[test]
fn test_ceiling_mod() {
    let test = |s, t, remainder| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut x = u.clone();
        x.ceiling_mod_assign(v.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let mut x = u.clone();
        x.ceiling_mod_assign(&v);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), remainder);

        let r = u.clone().ceiling_mod(v.clone());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = u.clone().ceiling_mod(&v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&u).ceiling_mod(v.clone());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = (&u).ceiling_mod(&v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let r = ceiling_mod_naive(u, v);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
    };
    test("0", "1", "0");
    test("0", "123", "0");
    test("1", "1", "0");
    test("123", "1", "0");
    test("123", "123", "0");
    test("123", "456", "-333");
    test("456", "123", "-36");
    test("4/3", "1", "-2/3");
    test("22/7", "1/2", "-5/14");
    test("10", "22/7", "-18/7");
    test("0", "-1", "0");
    test("0", "-123", "0");
    test("1", "-1", "0");
    test("123", "-1", "0");
    test("123", "-123", "0");
    test("123", "-456", "123");
    test("456", "-123", "87");
    test("4/3", "-1", "1/3");
    test("22/7", "-1/2", "1/7");
    test("10", "-22/7", "4/7");
    test("-1", "1", "0");
    test("-123", "1", "0");
    test("-123", "123", "0");
    test("-123", "456", "-123");
    test("-456", "123", "-87");
    test("-4/3", "1", "-1/3");
    test("-22/7", "1/2", "-1/7");
    test("-10", "22/7", "-4/7");
    test("-1", "-1", "0");
    test("-123", "-1", "0");
    test("-123", "-123", "0");
    test("-123", "-456", "333");
    test("-456", "-123", "36");
    test("-4/3", "-1", "2/3");
    test("-22/7", "-1/2", "5/14");
    test("-10", "-22/7", "18/7");
}

#[test]
#[should_panic]
fn ceiling_mod_assign_fail() {
    Rational::from(10).ceiling_mod_assign(Rational::ZERO);
}

#[test]
#[should_panic]
fn ceiling_mod_assign_ref_fail() {
    Rational::from(10).ceiling_mod_assign(&Rational::ZERO);
}

#[test]
#[should_panic]
fn ceiling_mod_fail() {
    Rational::from(10).ceiling_mod(Rational::ZERO);
}

#[test]
#[should_panic]
fn ceiling_mod_val_ref_fail() {
    Rational::from(10).ceiling_mod(&Rational::ZERO);
}

#[test]
#[should_panic]
fn ceiling_mod_ref_val_fail() {
    (&Rational::from(10)).ceiling_mod(Rational::ZERO);
}

#[test]
#[should_panic]
fn ceiling_mod_ref_ref_fail() {
    (&Rational::from(10)).ceiling_mod(&Rational::ZERO);
}

fn mod_properties_helper(x: Rational, y: Rational) {
    let mut mut_x = x.clone();
    mut_x.mod_assign(&y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x.mod_assign(y.clone());
    let remainder_alt = mut_x;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = (&x).mod_op(&y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = (&x).mod_op(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().mod_op(&y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().mod_op(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = mod_op_naive(x.clone(), y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert!(remainder.lt_abs(&y));
    assert!(remainder == 0 || (remainder > 0) == (y > 0));

    assert_eq!((-&x).mod_op(&y), -(&x).ceiling_mod(&y));
    assert_eq!((&x).mod_op(-&y), x.ceiling_mod(y));
}

#[test]
fn mod_properties() {
    rational_pair_gen_var_1().test_properties(|(x, y)| mod_properties_helper(x, y));

    rational_gen_var_1().test_properties(|ref x| {
        assert_eq!(x.mod_op(x), 0);
        assert_eq!(x.mod_op(-x), 0);
        assert_eq!(Rational::ZERO.mod_op(x), 0);
        if *x > 1 {
            assert_eq!(Rational::ONE.mod_op(x), 1);
            assert_eq!(Rational::NEGATIVE_ONE.mod_op(x), x - Rational::ONE);
        }
    });

    natural_pair_gen_var_5().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x).mod_op(Rational::from(&y)), x.mod_op(y));
    });

    integer_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x).mod_op(Rational::from(&y)), x.mod_op(y));
    });
}

fn rem_properties_helper(x: Rational, y: Rational) {
    let mut mut_x = x.clone();
    mut_x %= &y;
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x %= y.clone();
    assert!(mut_x.is_valid());
    assert_eq!(mut_x, remainder);

    let remainder_alt = &x % &y;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = &x % y.clone();
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone() % &y;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone() % y.clone();
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = rem_naive(x.clone(), y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let num_remainder = BigRational::from(&x) % &BigRational::from(&y);
    assert_eq!(Rational::from(&num_remainder), remainder);

    assert!(remainder.lt_abs(&y));
    assert!(remainder == 0 || (remainder > 0) == (x > 0));

    assert_eq!((-&x) % &y, -&remainder);
    assert_eq!(x % (-y), remainder);
}

#[test]
fn rem_properties() {
    rational_pair_gen_var_1().test_properties(|(x, y)| rem_properties_helper(x, y));

    rational_gen_var_1().test_properties(|ref x| {
        assert_eq!(x % x, 0);
        assert_eq!(x % -x, 0);
        assert_eq!(Rational::ZERO % x, 0);
        if *x > 1 {
            assert_eq!(Rational::ONE % x, 1);
            assert_eq!(Rational::NEGATIVE_ONE % x, -1);
        }
    });

    natural_pair_gen_var_5().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x) % Rational::from(&y), x % y);
    });

    integer_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x) % Rational::from(&y), x % y);
    });
}

fn ceiling_mod_properties_helper(x: Rational, y: Rational) {
    let mut mut_x = x.clone();
    mut_x.ceiling_mod_assign(&y);
    assert!(mut_x.is_valid());
    let remainder = mut_x;

    let mut mut_x = x.clone();
    mut_x.ceiling_mod_assign(y.clone());
    let remainder_alt = mut_x;
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = (&x).ceiling_mod(&y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = (&x).ceiling_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().ceiling_mod(&y);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = x.clone().ceiling_mod(y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let remainder_alt = ceiling_mod_naive(x.clone(), y.clone());
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    assert!(remainder.lt_abs(&y));
    assert!(remainder == 0 || (remainder >= 0) != (y > 0));

    assert_eq!((-&x).ceiling_mod(&y), -(&x).mod_op(&y));
    assert_eq!((&x).ceiling_mod(-&y), x.mod_op(y));
}

#[test]
fn ceiling_mod_properties() {
    rational_pair_gen_var_1().test_properties(|(x, y)| ceiling_mod_properties_helper(x, y));

    rational_gen_var_1().test_properties(|ref x| {
        assert_eq!(x.ceiling_mod(x), 0);
        assert_eq!(x.ceiling_mod(-x), 0);
        assert_eq!(Rational::ZERO.ceiling_mod(x), 0);
    });

    natural_pair_gen_var_5().test_properties(|(x, y)| {
        assert_eq!(
            Rational::from(&x).ceiling_mod(Rational::from(&y)),
            -x.neg_mod(y)
        );
    });

    integer_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(
            Rational::from(&x).ceiling_mod(Rational::from(&y)),
            x.ceiling_mod(y)
        );
    });
}
