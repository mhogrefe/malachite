// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, DivAssignMod, UnsignedAbs};
use malachite_base::num::basic::traits::Two;
use malachite_base::test_util::generators::signed_gen;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};
use malachite_nz::test_util::generators::{integer_gen, integer_integer_natural_triple_gen};
use num::{BigInt, Signed};
use std::str::FromStr;

#[test]
fn test_abs() {
    let test = |s, out| {
        let n = Integer::from_str(s).unwrap();

        let abs = n.clone().abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let abs = (&n).abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        assert_eq!(BigInt::from_str(s).unwrap().abs().to_string(), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().abs().to_string(), out);

        let abs = n.clone().unsigned_abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let abs = (&n).unsigned_abs();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let x = n.clone();
        let abs = x.unsigned_abs_ref();
        assert!(abs.is_valid());
        assert_eq!(abs.to_string(), out);

        let mut x = n;
        x.abs_assign();
        assert!(abs.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("123", "123");
    test("-123", "123");
    test("1000000000000", "1000000000000");
    test("-1000000000000", "1000000000000");
    test("3000000000", "3000000000");
    test("-3000000000", "3000000000");
    test("-2147483648", "2147483648");

    let mut n = Integer::from(-123);
    let remainder = n.mutate_unsigned_abs(|x| x.div_assign_mod(Natural::TWO));
    assert_eq!(n, -61);
    assert_eq!(remainder, 1);

    let mut n = Integer::from(-123);
    n.mutate_unsigned_abs(|x| *x >>= 10);
    assert_eq!(n, 0);
}

#[test]
fn abs_properties() {
    integer_gen().test_properties(|x| {
        let abs = x.clone().abs();
        assert!(abs.is_valid());

        assert_eq!(Integer::from(&BigInt::from(&x).abs()), abs);

        assert_eq!(Integer::from(&rug::Integer::from(&x).abs()), abs);

        let abs_alt = (&x).abs();
        assert!(abs_alt.is_valid());
        assert_eq!(abs_alt, abs);

        let mut abs_alt = x.clone();
        abs_alt.abs_assign();
        assert!(abs_alt.is_valid());
        assert_eq!(abs_alt, abs);

        assert!(abs >= 0);
        assert_eq!(abs == x, x >= 0);
        assert_eq!((&abs).abs(), abs);

        let abs_alt = x.clone().unsigned_abs();
        assert!(abs_alt.is_valid());
        assert_eq!(Ok(abs_alt), (&abs).try_into());

        let abs_alt = (&x).unsigned_abs();
        assert!(abs_alt.is_valid());
        assert_eq!(Ok(&abs_alt), abs.try_into().as_ref());

        let internal_abs = x.unsigned_abs_ref();
        assert!(internal_abs.is_valid());
        assert_eq!(*internal_abs, abs_alt);
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(
            Integer::from(i).abs(),
            Integer::from(SignedDoubleLimb::from(i).abs())
        );
    });
}

#[test]
fn mutate_unsigned_abs_properties() {
    integer_integer_natural_triple_gen().test_properties(|(mut n, out, new_abs)| {
        let out_2 = out.clone();
        let new_abs_2 = new_abs.clone();
        assert_eq!(
            n.mutate_unsigned_abs(|x| {
                *x = new_abs;
                out
            }),
            out_2
        );
        assert!(n.is_valid());
        assert_eq!(n.abs(), new_abs_2);
    });
}
