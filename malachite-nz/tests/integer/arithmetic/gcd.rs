// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Abs, DivisibleBy, ExtendedGcd, Gcd, GcdAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{integer_gen, integer_pair_gen, integer_triple_gen};

#[test]
fn test_gcd() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let n = u.clone().gcd(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().gcd(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).gcd(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).gcd(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.gcd_assign(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u;
        n.gcd_assign(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0");
    test("0", "6", "6");
    test("6", "0", "6");
    test("0", "-6", "6");
    test("-6", "0", "6");
    test("1", "6", "1");
    test("6", "6", "6");
    test("-6", "6", "6");
    test("-6", "-6", "6");
    test("8", "12", "4");
    test("-8", "12", "4");
    test("8", "-12", "4");
    test("-8", "-12", "4");
    test("54", "24", "6");
    test("42", "56", "14");
    test("-48", "18", "6");
    test("3", "5", "1");
    test("12", "60", "12");
    test("-12", "90", "6");
    test("12345678987654321", "98765432123456789", "1");
    test("-12345678987654321", "98765432123456789", "1");
    test("1000000000000", "-2000000000000", "1000000000000");
    test(
        "123456789000000000000",
        "-987654321000000000000",
        "9000000000000",
    );
}

#[test]
fn gcd_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let gcd_val_val = x.clone().gcd(y.clone());
        let gcd_val_ref = x.clone().gcd(&y);
        let gcd_ref_val = (&x).gcd(y.clone());
        let gcd = (&x).gcd(&y);
        assert!(gcd_val_val.is_valid());
        assert!(gcd_val_ref.is_valid());
        assert!(gcd_ref_val.is_valid());
        assert!(gcd.is_valid());
        assert_eq!(gcd_val_val, gcd);
        assert_eq!(gcd_val_ref, gcd);
        assert_eq!(gcd_ref_val, gcd);

        let mut mut_x = x.clone();
        mut_x.gcd_assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, gcd);

        let mut mut_x = x.clone();
        mut_x.gcd_assign(&y);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, gcd);

        // It is the GCD of the absolute values, and agrees with the extended GCD.
        assert_eq!((&x).unsigned_abs().gcd((&y).unsigned_abs()), gcd);
        assert_eq!((&x).abs().gcd((&y).abs()), gcd);
        assert_eq!((&x).extended_gcd(&y).0, gcd);
        assert_eq!((&y).gcd(&x), gcd);
        assert_eq!((-&x).gcd(&y), gcd);
        assert_eq!((&x).gcd(-&y), gcd);

        if gcd != 0u32 {
            assert!((&x).divisible_by(Integer::from(&gcd)));
            assert!((&y).divisible_by(Integer::from(&gcd)));
        }
        assert_eq!(gcd == 0u32, x == 0u32 && y == 0u32);
    });

    integer_gen().test_properties(|x| {
        assert_eq!((&x).gcd(&x), (&x).unsigned_abs());
        assert_eq!((&x).gcd(Integer::ZERO), (&x).unsigned_abs());
        assert_eq!(Integer::ZERO.gcd(&x), (&x).unsigned_abs());
        assert_eq!((&x).gcd(Integer::ONE), 1u32);
        assert_eq!((&x).gcd(Integer::NEGATIVE_ONE), 1u32);
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        let gcd_x_y = Integer::from((&x).gcd(&y));
        let gcd_y_z = Integer::from((&y).gcd(&z));
        assert_eq!(gcd_x_y.gcd(&z), x.gcd(gcd_y_z));
    });

    assert_eq!(Integer::ZERO.gcd(Integer::ZERO), Natural::ZERO);
}
