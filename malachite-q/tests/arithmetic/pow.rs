// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CheckedRoot, Parity, Pow, PowAssign, PowerOf2, Reciprocal, Square,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_gen_var_5, unsigned_gen_var_5};
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_2;
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_1, rational_rational_signed_triple_gen_var_1,
    rational_rational_unsigned_triple_gen_var_1, rational_signed_pair_gen_var_2,
    rational_signed_signed_triple_gen_var_1, rational_unsigned_pair_gen_var_1,
    rational_unsigned_unsigned_triple_gen_var_1,
};
use malachite_q::Rational;
use num::traits::Pow as NumPow;
use num::BigRational;
use rug::ops::Pow as RugPow;
use std::str::FromStr;

#[test]
fn test_pow() {
    let test = |s, exp: u64, out| {
        let u = Rational::from_str(s).unwrap();

        let mut x = u.clone();
        x.pow_assign(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = u.clone().pow(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = (&u).pow(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = BigRational::from_str(s).unwrap().pow(exp);
        assert_eq!(x.to_string(), out);

        let x = rug::Rational::from_str(s)
            .unwrap()
            .pow(u32::exact_from(exp));
        assert_eq!(x.to_string(), out);
    };
    test("0", 0, "1");
    test("1", 0, "1");
    test("2", 0, "1");
    test("1000", 0, "1");
    test("1000000000000", 0, "1");
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("1000", 1, "1000");
    test("1000000000000", 1, "1000000000000");
    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "4");
    test("3", 2, "9");
    test("1000", 2, "1000000");
    test("1000000000000", 2, "1000000000000000000000000");
    test("1/2", 0, "1");
    test("1/2", 1, "1/2");
    test("1/2", 2, "1/4");
    test("1/2", 3, "1/8");
    test("22/7", 0, "1");
    test("22/7", 1, "22/7");
    test("22/7", 2, "484/49");
    test("22/7", 3, "10648/343");

    test("-1", 0, "1");
    test("-2", 0, "1");
    test("-1000", 0, "1");
    test("-1000000000000", 0, "1");
    test("-1", 1, "-1");
    test("-2", 1, "-2");
    test("-1000", 1, "-1000");
    test("-1000000000000", 1, "-1000000000000");
    test("-1", 2, "1");
    test("-2", 2, "4");
    test("-3", 2, "9");
    test("-1000", 2, "1000000");
    test("-1000000000000", 2, "1000000000000000000000000");
    test("-1/2", 0, "1");
    test("-1/2", 1, "-1/2");
    test("-1/2", 2, "1/4");
    test("-1/2", 3, "-1/8");
    test("-22/7", 0, "1");
    test("-22/7", 1, "-22/7");
    test("-22/7", 2, "484/49");
    test("-22/7", 3, "-10648/343");

    let test = |s, exp: i64, out| {
        let u = Rational::from_str(s).unwrap();

        let mut x = u.clone();
        x.pow_assign(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = u.clone().pow(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = (&u).pow(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = BigRational::from_str(s).unwrap().pow(exp);
        assert_eq!(x.to_string(), out);

        let x = rug::Rational::from_str(s)
            .unwrap()
            .pow(i32::exact_from(exp));
        assert_eq!(x.to_string(), out);
    };
    test("0", 0, "1");
    test("1", 0, "1");
    test("2", 0, "1");
    test("1000", 0, "1");
    test("1000000000000", 0, "1");
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("1000", 1, "1000");
    test("1000000000000", 1, "1000000000000");
    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "4");
    test("3", 2, "9");
    test("1000", 2, "1000000");
    test("1000000000000", 2, "1000000000000000000000000");
    test("1/2", 0, "1");
    test("1/2", 1, "1/2");
    test("1/2", 2, "1/4");
    test("1/2", 3, "1/8");
    test("22/7", 0, "1");
    test("22/7", 1, "22/7");
    test("22/7", 2, "484/49");
    test("22/7", 3, "10648/343");

    test("1", -1, "1");
    test("2", -1, "1/2");
    test("1000", -1, "1/1000");
    test("1000000000000", -1, "1/1000000000000");
    test("1", -2, "1");
    test("2", -2, "1/4");
    test("3", -2, "1/9");
    test("1000", -2, "1/1000000");
    test("1000000000000", -2, "1/1000000000000000000000000");
    test("1/2", -1, "2");
    test("1/2", -2, "4");
    test("1/2", -3, "8");
    test("22/7", -1, "7/22");
    test("22/7", -2, "49/484");
    test("22/7", -3, "343/10648");

    test("-1", 0, "1");
    test("-2", 0, "1");
    test("-1000", 0, "1");
    test("-1000000000000", 0, "1");
    test("-1", 1, "-1");
    test("-2", 1, "-2");
    test("-1000", 1, "-1000");
    test("-1000000000000", 1, "-1000000000000");
    test("-1", 2, "1");
    test("-2", 2, "4");
    test("-3", 2, "9");
    test("-1000", 2, "1000000");
    test("-1000000000000", 2, "1000000000000000000000000");
    test("-1/2", 0, "1");
    test("-1/2", 1, "-1/2");
    test("-1/2", 2, "1/4");
    test("-1/2", 3, "-1/8");
    test("-22/7", 0, "1");
    test("-22/7", 1, "-22/7");
    test("-22/7", 2, "484/49");
    test("-22/7", 3, "-10648/343");

    test("-1", -1, "-1");
    test("-2", -1, "-1/2");
    test("-1000", -1, "-1/1000");
    test("-1000000000000", -1, "-1/1000000000000");
    test("-1", -2, "1");
    test("-2", -2, "1/4");
    test("-3", -2, "1/9");
    test("-1000", -2, "1/1000000");
    test("-1000000000000", -2, "1/1000000000000000000000000");
    test("-1/2", -1, "-2");
    test("-1/2", -2, "4");
    test("-1/2", -3, "-8");
    test("-22/7", -1, "-7/22");
    test("-22/7", -2, "49/484");
    test("-22/7", -3, "-343/10648");
}

#[test]
fn pow_properties() {
    // exponent is u64

    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, exp)| {
        let power = (&x).pow(exp);
        assert!(power.is_valid());

        let power_alt = x.clone().pow(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        let mut power_alt = x.clone();
        power_alt.pow_assign(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        let power_of_neg = (-&x).pow(exp);
        if exp.even() {
            assert_eq!(power_of_neg, power);
        } else {
            assert_eq!(power_of_neg, -&power);
        }
        if exp > 0 && (x >= 0 || exp.odd()) {
            assert_eq!((&power).checked_root(exp).as_ref(), Some(&x));
        }

        assert_eq!((&x).pow(i64::exact_from(exp)), power);

        assert_eq!(Rational::from(&BigRational::from(&x).pow(exp)), power);
        assert_eq!(
            Rational::from(&rug::Rational::from(&x).pow(u32::exact_from(exp))),
            power
        );
    });

    rational_gen().test_properties(|x| {
        assert_eq!((&x).pow(0u64), 1);
        assert_eq!((&x).pow(1u64), x);
        assert_eq!((&x).pow(2u64), x.square());
    });

    unsigned_gen_var_5::<u64>().test_properties(|exp| {
        assert_eq!(Rational::ZERO.pow(exp), u64::from(exp == 0));
        assert_eq!(Rational::ONE.pow(exp), 1);
        assert_eq!(Rational::TWO.pow(exp), Rational::power_of_2(exp));

        assert_eq!(
            Rational::NEGATIVE_ONE.pow(exp),
            if exp.even() { 1 } else { -1 }
        );
    });

    rational_rational_unsigned_triple_gen_var_1::<u64>().test_properties(|(x, y, exp)| {
        assert_eq!((&x * &y).pow(exp), x.pow(exp) * y.pow(exp));
    });

    rational_unsigned_unsigned_triple_gen_var_1::<u64>().test_properties(|(x, e, f)| {
        assert_eq!((&x).pow(e + f), (&x).pow(e) * (&x).pow(f));
        assert_eq!((&x).pow(e * f), x.pow(e).pow(f));
    });

    integer_unsigned_pair_gen_var_2().test_properties(|(x, exp)| {
        assert_eq!((&x).pow(exp), Rational::from(x).pow(exp));
    });

    // exponent is i64

    rational_signed_pair_gen_var_2::<i64>().test_properties(|(x, exp)| {
        let power = (&x).pow(exp);
        assert!(power.is_valid());

        let power_alt = x.clone().pow(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        let mut power_alt = x.clone();
        power_alt.pow_assign(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        let power_of_neg = (-&x).pow(exp);
        if exp.even() {
            assert_eq!(power_of_neg, power);
        } else {
            assert_eq!(power_of_neg, -&power);
        }

        if x != 0 {
            assert_eq!((&x).pow(-exp), (&power).reciprocal());
        }

        if exp > 0 && (x >= 0 || exp.odd()) {
            assert_eq!((&power).checked_root(exp).as_ref(), Some(&x));
        }

        assert_eq!(Rational::from(&BigRational::from(&x).pow(exp)), power);
        assert_eq!(
            Rational::from(&rug::Rational::from(&x).pow(i32::exact_from(exp))),
            power
        );
    });

    rational_gen().test_properties(|x| {
        assert_eq!((&x).pow(0i64), 1);
        assert_eq!((&x).pow(1i64), x);
        assert_eq!((&x).pow(2i64), x.square());
    });

    rational_gen_var_1().test_properties(|x| {
        assert_eq!((&x).pow(-1i64), x.reciprocal());
    });

    signed_gen_var_5::<i64>().test_properties(|exp| {
        if exp >= 0 {
            assert_eq!(Rational::ZERO.pow(exp), u64::from(exp == 0));
        }
        assert_eq!(Rational::ONE.pow(exp), 1);
        assert_eq!(Rational::TWO.pow(exp), Rational::power_of_2(exp));

        assert_eq!(
            Rational::NEGATIVE_ONE.pow(exp),
            if exp.even() { 1 } else { -1 }
        );
    });

    rational_rational_signed_triple_gen_var_1::<i64>().test_properties(|(x, y, exp)| {
        assert_eq!((&x * &y).pow(exp), x.pow(exp) * y.pow(exp));
    });

    rational_signed_signed_triple_gen_var_1::<i64>().test_properties(|(x, e, f)| {
        assert_eq!((&x).pow(e + f), (&x).pow(e) * (&x).pow(f));
        assert_eq!((&x).pow(e * f), x.pow(e).pow(f));
    });
}
