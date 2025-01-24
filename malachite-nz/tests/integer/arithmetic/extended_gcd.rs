// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivExact, ExtendedGcd, Gcd};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::signed_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{integer_gen, integer_pair_gen};
use num::BigInt;
use num::Integer as NumInteger;
use std::cmp::min;
use std::str::FromStr;

#[test]
fn test_extended_gcd() {
    let test = |s, t, gcd, x, y| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let result = u.clone().extended_gcd(v.clone());
        assert!(result.0.is_valid());
        assert!(result.1.is_valid());
        assert!(result.2.is_valid());
        assert_eq!(result.0.to_string(), gcd);
        assert_eq!(result.1.to_string(), x);
        assert_eq!(result.2.to_string(), y);

        let result = (&u).extended_gcd(v.clone());
        assert!(result.0.is_valid());
        assert!(result.1.is_valid());
        assert!(result.2.is_valid());
        assert_eq!(result.0.to_string(), gcd);
        assert_eq!(result.1.to_string(), x);
        assert_eq!(result.2.to_string(), y);

        let result = u.clone().extended_gcd(&v);
        assert!(result.0.is_valid());
        assert!(result.1.is_valid());
        assert!(result.2.is_valid());
        assert_eq!(result.0.to_string(), gcd);
        assert_eq!(result.1.to_string(), x);
        assert_eq!(result.2.to_string(), y);

        let result = (&u).extended_gcd(&v);
        assert!(result.0.is_valid());
        assert!(result.1.is_valid());
        assert!(result.2.is_valid());
        assert_eq!(result.0.to_string(), gcd);
        assert_eq!(result.1.to_string(), x);
        assert_eq!(result.2.to_string(), y);

        if u != 0u32 || v != 0u32 {
            let result = BigInt::from_str(s)
                .unwrap()
                .extended_gcd(&BigInt::from_str(t).unwrap());
            assert_eq!(result.gcd.to_string(), gcd);
            assert_eq!(result.x.to_string(), x);
            assert_eq!(result.y.to_string(), y);
        }

        let result = rug::Integer::from_str(s)
            .unwrap()
            .extended_gcd(rug::Integer::from_str(t).unwrap(), rug::Integer::new());
        assert_eq!(result.0.to_string(), gcd);
        assert_eq!(result.1.to_string(), x);
        assert_eq!(result.2.to_string(), y);
    };
    test("0", "0", "0", "0", "0");
    test("0", "1", "1", "0", "1");
    test("0", "-1", "1", "0", "-1");
    test("1", "0", "1", "1", "0");
    test("-1", "0", "1", "-1", "0");
    test("1", "1", "1", "0", "1");
    test("1", "-1", "1", "0", "-1");
    test("-1", "1", "1", "0", "1");
    test("-1", "-1", "1", "0", "-1");
    test("0", "6", "6", "0", "1");
    test("0", "-6", "6", "0", "-1");
    test("6", "0", "6", "1", "0");
    test("-6", "0", "6", "-1", "0");
    test("1", "6", "1", "1", "0");
    test("1", "-6", "1", "1", "0");
    test("-1", "6", "1", "-1", "0");
    test("-1", "-6", "1", "-1", "0");
    test("6", "1", "1", "0", "1");
    test("6", "-1", "1", "0", "-1");
    test("-6", "1", "1", "0", "1");
    test("-6", "-1", "1", "0", "-1");
    test("6", "6", "6", "0", "1");
    test("6", "-6", "6", "0", "-1");
    test("-6", "6", "6", "0", "1");
    test("-6", "-6", "6", "0", "-1");
    test("8", "12", "4", "-1", "1");
    test("54", "24", "6", "1", "-2");
    test("42", "56", "14", "-1", "1");
    test("48", "18", "6", "-1", "3");
    test("3", "5", "1", "2", "-1");
    test("12", "90", "6", "-7", "1");
    test("240", "46", "2", "-9", "47");
    test("240", "-46", "2", "-9", "-47");
    test("-240", "46", "2", "9", "47");
    test("-240", "-46", "2", "9", "-47");
    test("-128", "-128", "128", "0", "-1");
    test("0", "-128", "128", "0", "-1");
    test("-128", "0", "128", "-1", "0");
    test("12", "60", "12", "1", "0");
    test("-12", "60", "12", "-1", "0");
    test("12", "-60", "12", "1", "0");
    test("-12", "-60", "12", "-1", "0");
    test("60", "12", "12", "0", "1");
    test("-60", "12", "12", "0", "1");
    test("60", "-12", "12", "0", "-1");
    test("-60", "-12", "12", "0", "-1");
    test(
        "12345678987654321",
        "98765432123456789",
        "1",
        "1777777788",
        "-222222223",
    );
    test(
        "12345678987654321",
        "-98765432123456789",
        "1",
        "1777777788",
        "222222223",
    );
    test(
        "-12345678987654321",
        "98765432123456789",
        "1",
        "-1777777788",
        "-222222223",
    );
    test(
        "-12345678987654321",
        "-98765432123456789",
        "1",
        "-1777777788",
        "222222223",
    );
    test(
        "12345678987654321",
        "98765432123456827",
        "37",
        "-577153682403132",
        "72144210138067",
    );
    test(
        "12345678987654321",
        "-98765432123456827",
        "37",
        "-577153682403132",
        "-72144210138067",
    );
    test(
        "-12345678987654321",
        "98765432123456827",
        "37",
        "577153682403132",
        "72144210138067",
    );
    test(
        "-12345678987654321",
        "-98765432123456827",
        "37",
        "577153682403132",
        "-72144210138067",
    );
}

#[test]
fn extended_gcd_properties() {
    integer_pair_gen().test_properties(|(a, b): (Integer, Integer)| {
        let result_val_val = a.clone().extended_gcd(b.clone());
        let result_val_ref = a.clone().extended_gcd(&b);
        let result_ref_val = (&a).extended_gcd(b.clone());
        let result = (&a).extended_gcd(&b);
        assert!(result_val_val.0.is_valid());
        assert!(result_val_val.1.is_valid());
        assert!(result_val_val.2.is_valid());
        assert!(result_val_ref.0.is_valid());
        assert!(result_val_ref.1.is_valid());
        assert!(result_val_ref.2.is_valid());
        assert!(result_ref_val.0.is_valid());
        assert!(result_ref_val.1.is_valid());
        assert!(result_ref_val.2.is_valid());
        assert!(result.0.is_valid());
        assert!(result.1.is_valid());
        assert!(result.2.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let (gcd, x, y) = result;

        if a != 0u32 || b != 0u32 {
            let num_result = BigInt::from(&a).extended_gcd(&BigInt::from(&b));
            assert_eq!(Integer::from(&num_result.gcd), gcd);
            assert_eq!(Integer::from(&num_result.x), x);
            assert_eq!(Integer::from(&num_result.y), y);
        }

        let (rug_gcd, rug_x, rug_y) =
            rug::Integer::from(&a).extended_gcd(rug::Integer::from(&b), rug::Integer::new());
        assert_eq!(Natural::exact_from(&rug_gcd), gcd);
        assert_eq!(Integer::from(&rug_x), x);
        assert_eq!(Integer::from(&rug_y), y);

        assert_eq!(a.unsigned_abs_ref().gcd(b.unsigned_abs_ref()), gcd);
        assert_eq!(&a * &x + &b * &y, Integer::from(&gcd));

        // uniqueness
        if a != 0u32 && b != 0u32 && &gcd != min(a.unsigned_abs_ref(), b.unsigned_abs_ref()) {
            assert!(x.le_abs(&((&b).div_exact(Integer::from(&gcd)) >> 1u32)));
            assert!(y.le_abs(&((&a).div_exact(Integer::from(&gcd)) >> 1u32)));
        }

        let reverse = (&b).extended_gcd(&a);
        if a == b {
            assert_eq!(reverse, (gcd, x, y));
        } else if a == -b {
            assert_eq!(reverse, (gcd, x, -y));
        } else {
            assert_eq!(reverse, (gcd, y, x));
        }
    });

    integer_gen().test_properties(|x| {
        if x != 0u32 {
            let result = (&x).extended_gcd(&x);
            assert_eq!(result.0, *x.unsigned_abs_ref());
            assert_eq!(result.1, 0u32);
            assert_eq!(result.2, if x >= 0u32 { 1i32 } else { -1i32 });
            let result = (&x).extended_gcd(-&x);
            assert_eq!(result.0, *x.unsigned_abs_ref());
            assert_eq!(result.1, 0u32);
            assert_eq!(result.2, if x < 0u32 { 1i32 } else { -1i32 });
            let result = (&x).extended_gcd(Integer::ZERO);
            assert_eq!(result.0, *x.unsigned_abs_ref());
            assert_eq!(result.1, if x >= 0u32 { 1i32 } else { -1i32 });
            assert_eq!(result.2, 0u32);
            let result = Integer::ZERO.extended_gcd(&x);
            assert_eq!(result.0, *x.unsigned_abs_ref());
            assert_eq!(result.1, 0u32);
            assert_eq!(result.2, if x >= 0u32 { 1i32 } else { -1i32 });
        }
        if *x.unsigned_abs_ref() != 1u32 {
            assert_eq!(
                Integer::ONE.extended_gcd(&x),
                (Natural::ONE, Integer::ONE, Integer::ZERO)
            );
        }
        assert_eq!(
            x.extended_gcd(Integer::ONE),
            (Natural::ONE, Integer::ZERO, Integer::ONE)
        );
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(a, b)| {
        let (u_gcd, u_x, u_y) = a.extended_gcd(b);
        let (gcd, x, y) = Integer::from(a).extended_gcd(Integer::from(b));
        assert_eq!(gcd, u_gcd);
        assert_eq!(x, u_x);
        assert_eq!(y, u_y);
    });
}
