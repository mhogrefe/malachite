// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, CheckedAddMul};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::generators::{signed_triple_gen, signed_triple_gen_var_1};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_triple_gen,
};
use std::str::FromStr;

#[test]
fn test_add_mul() {
    let test = |r, s, t, out| {
        let u = Integer::from_str(r).unwrap();
        let v = Integer::from_str(s).unwrap();
        let w = Integer::from_str(t).unwrap();

        let mut a = u.clone();
        a.add_mul_assign(v.clone(), w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = u.clone();
        a.add_mul_assign(v.clone(), &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = u.clone();
        a.add_mul_assign(&v, w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = u.clone();
        a.add_mul_assign(&v, &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().add_mul(v.clone(), w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().add_mul(v.clone(), &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().add_mul(&v, w.clone());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = u.clone().add_mul(&v, &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&u).add_mul(&v, &w);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "5", "1", "128");
    test("123", "5", "100", "623");
    test("10", "3", "4", "22");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "1", "123", "1000000000123");
    test("1000000000000", "123", "1", "1000000000123");
    test("1000000000000", "123", "100", "1000000012300");
    test("1000000000000", "100", "123", "1000000012300");
    test("1000000000000", "65536", "65536", "1004294967296");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "2000000000000");
    test("1000000000000", "1000000000000", "100", "101000000000000");
    test("0", "1000000000000", "100", "100000000000000");
    test(
        "1000000000000",
        "65536",
        "1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "1000000000000",
        "1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "1000000000000",
        "1000000000000",
        "1000000000000000000000000",
    );

    test("123", "-5", "-1", "128");
    test("123", "-5", "-100", "623");
    test("10", "-3", "-4", "22");
    test("1000000000000", "-1", "-123", "1000000000123");
    test("1000000000000", "-123", "-1", "1000000000123");
    test("1000000000000", "-123", "-100", "1000000012300");
    test("1000000000000", "-100", "-123", "1000000012300");
    test("1000000000000", "-65536", "-65536", "1004294967296");
    test("1000000000000", "-1000000000000", "-1", "2000000000000");
    test("1000000000000", "-1000000000000", "-100", "101000000000000");
    test("0", "-1000000000000", "-100", "100000000000000");
    test(
        "1000000000000",
        "-65536",
        "-1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "-1000000000000",
        "-1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "-1000000000000",
        "-1000000000000",
        "1000000000000000000000000",
    );

    test("0", "0", "-123", "0");
    test("123", "0", "-5", "123");
    test("123", "-5", "1", "118");
    test("123", "5", "-1", "118");
    test("123", "-5", "100", "-377");
    test("123", "5", "-100", "-377");
    test("10", "-3", "4", "-2");
    test("10", "3", "-4", "-2");
    test("15", "-3", "4", "3");
    test("15", "3", "-4", "3");
    test("1000000000000", "0", "-123", "1000000000000");
    test("1000000000000", "-1", "123", "999999999877");
    test("1000000000000", "1", "-123", "999999999877");
    test("1000000000000", "-123", "1", "999999999877");
    test("1000000000000", "123", "-1", "999999999877");
    test("1000000000000", "-123", "100", "999999987700");
    test("1000000000000", "123", "-100", "999999987700");
    test("1000000000000", "-100", "123", "999999987700");
    test("1000000000000", "100", "-123", "999999987700");
    test("1000000000000", "-65536", "65536", "995705032704");
    test("1000000000000", "65536", "-65536", "995705032704");
    test("1000000000000", "-1000000000000", "0", "1000000000000");
    test("1000000000000", "-1000000000000", "1", "0");
    test("1000000000000", "1000000000000", "-1", "0");
    test("1000000000000", "-1000000000000", "100", "-99000000000000");
    test("1000000000000", "1000000000000", "-100", "-99000000000000");
    test("0", "-1000000000000", "100", "-100000000000000");
    test("4294967296", "-1", "1", "4294967295");
    test("4294967296", "1", "-1", "4294967295");
    test("3902609153", "-88817093856604", "1", "-88813191247451");
    test("3902609153", "88817093856604", "-1", "-88813191247451");

    test("-123", "0", "5", "-123");
    test("-123", "-5", "1", "-128");
    test("-123", "-5", "100", "-623");
    test("-10", "-3", "4", "-22");
    test("-1000000000000", "0", "123", "-1000000000000");
    test("-1000000000000", "-1", "123", "-1000000000123");
    test("-1000000000000", "-123", "1", "-1000000000123");
    test("-1000000000000", "-123", "100", "-1000000012300");
    test("-1000000000000", "-100", "123", "-1000000012300");
    test("-1000000000000", "-65536", "65536", "-1004294967296");
    test("-1000000000000", "-1000000000000", "0", "-1000000000000");
    test("-1000000000000", "-1000000000000", "1", "-2000000000000");
    test(
        "-1000000000000",
        "-1000000000000",
        "100",
        "-101000000000000",
    );
    test(
        "-1000000000000",
        "-65536",
        "1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "-1000000000000",
        "1000000000000",
        "-1000000000001000000000000",
    );
    test(
        "0",
        "-1000000000000",
        "1000000000000",
        "-1000000000000000000000000",
    );

    test("-123", "5", "-1", "-128");
    test("-123", "5", "-100", "-623");
    test("-10", "3", "-4", "-22");
    test("-1000000000000", "1", "-123", "-1000000000123");
    test("-1000000000000", "123", "-1", "-1000000000123");
    test("-1000000000000", "123", "-100", "-1000000012300");
    test("-1000000000000", "100", "-123", "-1000000012300");
    test("-1000000000000", "65536", "-65536", "-1004294967296");
    test("-1000000000000", "1000000000000", "-1", "-2000000000000");
    test(
        "-1000000000000",
        "1000000000000",
        "-100",
        "-101000000000000",
    );
    test(
        "-1000000000000",
        "65536",
        "-1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "1000000000000",
        "-1000000000000",
        "-1000000000001000000000000",
    );

    test("-123", "0", "-5", "-123");
    test("-123", "5", "1", "-118");
    test("-123", "-5", "-1", "-118");
    test("-123", "5", "100", "377");
    test("-123", "-5", "-100", "377");
    test("-10", "3", "4", "2");
    test("-10", "-3", "-4", "2");
    test("-15", "3", "4", "-3");
    test("-15", "-3", "-4", "-3");
    test("-1000000000000", "0", "-123", "-1000000000000");
    test("-1000000000000", "1", "123", "-999999999877");
    test("-1000000000000", "-1", "-123", "-999999999877");
    test("-1000000000000", "123", "1", "-999999999877");
    test("-1000000000000", "-123", "-1", "-999999999877");
    test("-1000000000000", "123", "100", "-999999987700");
    test("-1000000000000", "-123", "-100", "-999999987700");
    test("-1000000000000", "100", "123", "-999999987700");
    test("-1000000000000", "-100", "-123", "-999999987700");
    test("-1000000000000", "65536", "65536", "-995705032704");
    test("-1000000000000", "-65536", "-65536", "-995705032704");
    test("-1000000000000", "1000000000000", "0", "-1000000000000");
    test("-1000000000000", "1000000000000", "1", "0");
    test("-1000000000000", "-1000000000000", "-1", "0");
    test("-1000000000000", "1000000000000", "100", "99000000000000");
    test("-1000000000000", "-1000000000000", "-100", "99000000000000");
    test("-4294967296", "1", "1", "-4294967295");
    test("-4294967296", "-1", "-1", "-4294967295");
    test("-3902609153", "88817093856604", "1", "88813191247451");
    test("-3902609153", "-88817093856604", "-1", "88813191247451");
    test(
        "1000000000000000000000000",
        "-1000000000000",
        "1000000000000",
        "0",
    );
    test(
        "-4",
        "-24227802588",
        "-14313318194700",
        "346780247600420147883596",
    );
}

#[test]
fn add_mul_properties() {
    integer_triple_gen().test_properties(|(a, b, c)| {
        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b.clone(), &c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(&b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(&b, &c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.clone().add_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b.clone(), &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(&b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(&b, &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = (&a).add_mul(&b, &c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let a = &a;
        let b = &b;
        let c = &c;
        assert_eq!(a + b * c, result);
        assert_eq!(a.add_mul(c, b), result);
        assert_eq!(a.add_mul(&(-b), &(-c)), result);
        assert_eq!((-a).add_mul(&(-b), c), -&result);
        assert_eq!((-a).add_mul(b, -c), -result);
    });

    integer_gen().test_properties(|a| {
        let a = &a;
        assert_eq!(a.add_mul(a, &Integer::NEGATIVE_ONE), 0);
        assert_eq!(a.add_mul(&(-a), &Integer::ONE), 0);
    });

    integer_pair_gen().test_properties(|(a, b)| {
        let a = &a;
        let b = &b;
        assert_eq!(a.add_mul(&Integer::ZERO, b), *a);
        assert_eq!(a.add_mul(&Integer::ONE, b), a + b);
        assert_eq!(Integer::ZERO.add_mul(a, b), a * b);
        assert_eq!(a.add_mul(b, &Integer::ZERO), *a);
        assert_eq!(a.add_mul(b, &Integer::ONE), a + b);
        assert_eq!((a * b).add_mul(-a, b), 0);
        assert_eq!((a * b).add_mul(a, -b), 0);
    });

    natural_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!(
            (&x).add_mul(&y, &z),
            Integer::from(x).add_mul(Integer::from(y), Integer::from(z))
        );
    });

    signed_triple_gen_var_1::<SignedLimb>().test_properties(|(x, y, z)| {
        assert_eq!(
            x.add_mul(y, z),
            Integer::from(x).add_mul(Integer::from(y), Integer::from(z))
        );
    });

    signed_triple_gen::<SignedLimb>().test_properties(|(x, y, z)| {
        let result = Integer::from(x).add_mul(Integer::from(y), Integer::from(z));
        assert_eq!(
            x.checked_add_mul(y, z).is_some(),
            SignedLimb::convertible_from(&result)
        );
    });
}
