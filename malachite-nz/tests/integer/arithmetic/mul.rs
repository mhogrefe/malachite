// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, DivMod, PowerOf2, Square, UnsignedAbs,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::test_util::generators::signed_pair_gen;
use malachite_base::vecs::vec_from_str;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, integer_vec_gen, natural_pair_gen,
    natural_vec_gen,
};
use malachite_nz::test_util::integer::arithmetic::mul::integer_product_naive;
use num::BigInt;
use std::iter::{once, Product};
use std::str::FromStr;

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut n = u.clone();
        n *= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n *= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() * v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u * v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() * &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u * &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigInt::from_str(s).unwrap() * BigInt::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(s).unwrap() * rug::Integer::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("0", "-123", "0");
    test("123", "0", "0");
    test("-123", "0", "0");
    test("1", "123", "123");
    test("1", "-123", "-123");
    test("-1", "123", "-123");
    test("-1", "-123", "123");
    test("123", "1", "123");
    test("123", "-1", "-123");
    test("-123", "1", "-123");
    test("-123", "-1", "123");
    test("123", "456", "56088");
    test("123", "-456", "-56088");
    test("-123", "456", "-56088");
    test("-123", "-456", "56088");
    test("0", "1000000000000", "0");
    test("0", "-1000000000000", "0");
    test("1000000000000", "0", "0");
    test("-1000000000000", "0", "0");
    test("1", "1000000000000", "1000000000000");
    test("1", "-1000000000000", "-1000000000000");
    test("-1", "1000000000000", "-1000000000000");
    test("-1", "-1000000000000", "1000000000000");
    test("1000000000000", "1", "1000000000000");
    test("1000000000000", "-1", "-1000000000000");
    test("-1000000000000", "1", "-1000000000000");
    test("-1000000000000", "-1", "1000000000000");
    test("1000000000000", "123", "123000000000000");
    test("1000000000000", "-123", "-123000000000000");
    test("-1000000000000", "123", "-123000000000000");
    test("-1000000000000", "-123", "123000000000000");
    test("123", "1000000000000", "123000000000000");
    test("123", "-1000000000000", "-123000000000000");
    test("-123", "1000000000000", "-123000000000000");
    test("-123", "-1000000000000", "123000000000000");
    test("123456789000", "987654321000", "121932631112635269000000");
    test("123456789000", "-987654321000", "-121932631112635269000000");
    test("-123456789000", "987654321000", "-121932631112635269000000");
    test("-123456789000", "-987654321000", "121932631112635269000000");
    test("4294967295", "2", "8589934590");
    test("4294967295", "-2", "-8589934590");
    test("-4294967295", "2", "-8589934590");
    test("-4294967295", "-2", "8589934590");
    test("4294967295", "4294967295", "18446744065119617025");
    test("4294967295", "-4294967295", "-18446744065119617025");
    test("-4294967295", "4294967295", "-18446744065119617025");
    test("-4294967295", "-4294967295", "18446744065119617025");
    test("18446744073709551615", "2", "36893488147419103230");
    test("18446744073709551615", "-2", "-36893488147419103230");
    test("-18446744073709551615", "2", "-36893488147419103230");
    test("-18446744073709551615", "-2", "36893488147419103230");
    let large_power_of_2 = Integer::power_of_2(100_000) * Integer::power_of_2(100_000);
    assert!(large_power_of_2.is_valid());
    assert_eq!(
        large_power_of_2.unsigned_abs().checked_log_base_2(),
        Some(200_000)
    );
}

#[test]
fn test_product() {
    let test = |xs, out| {
        let xs = vec_from_str(xs).unwrap();
        let product = Integer::product(xs.iter().cloned());
        assert!(product.is_valid());
        assert_eq!(product.to_string(), out);

        let product_alt = Integer::product(xs.iter());
        assert!(product_alt.is_valid());
        assert_eq!(product_alt, product);

        let product_alt = integer_product_naive(xs.into_iter());
        assert!(product_alt.is_valid());
        assert_eq!(product_alt, product);
    };
    test("[]", "1");
    test("[10]", "10");
    test("[6, -2]", "-12");
    test("[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]", "0");
    test("[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]", "3628800");
    test(
        "[123456, -789012, 345678, -9012345]",
        "303462729062737285547520",
    );
}

#[test]
fn mul_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * &y;
        let product_ref_val = &x * y.clone();
        let product = &x * &y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x *= &y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let mut mut_x = rug::Integer::from(&x);
        mut_x *= rug::Integer::from(&y);
        assert_eq!(Integer::from(&mut_x), product);

        assert_eq!(
            Integer::from(&(BigInt::from(&x) * BigInt::from(&y))),
            product
        );
        assert_eq!(
            Integer::from(&(rug::Integer::from(&x) * rug::Integer::from(&y))),
            product
        );
        assert_eq!(&y * &x, product);
        if x != 0 {
            let (q, r) = (&product).div_mod(&x);
            assert_eq!(q, y);
            assert_eq!(r, 0);
        }
        if y != 0 {
            let (q, r) = (&product).div_mod(&y);
            assert_eq!(q, x);
            assert_eq!(r, 0);
        }

        assert_eq!(-&x * &y, -&product);
        assert_eq!(x * -y, -product);
    });

    integer_gen().test_properties(|ref x| {
        assert_eq!(x * Integer::ZERO, 0);
        assert_eq!(Integer::ZERO * x, 0);
        assert_eq!(x * Integer::ONE, *x);
        assert_eq!(Integer::ONE * x, *x);
        assert_eq!(x * Integer::NEGATIVE_ONE, -x);
        assert_eq!(Integer::NEGATIVE_ONE * x, -x);
        assert_eq!(x * x, x.square());
    });

    integer_triple_gen().test_properties(|(ref x, ref y, ref z)| {
        assert_eq!((x * y) * z, x * (y * z));
        assert_eq!(x * (y + z), x * y + x * z);
        assert_eq!((x + y) * z, x * z + y * z);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(&x * &y, Integer::from(x) * Integer::from(y));
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(
            Integer::from(SignedDoubleLimb::from(x) * SignedDoubleLimb::from(y)),
            Integer::from(x) * Integer::from(y)
        );
    });
}

#[test]
fn product_properties() {
    integer_vec_gen().test_properties(|xs| {
        let product = Integer::product(xs.iter().cloned());
        assert!(product.is_valid());

        let product_alt = Integer::product(xs.iter());
        assert!(product_alt.is_valid());
        assert_eq!(product_alt, product);

        let product_alt = integer_product_naive(xs.into_iter());
        assert!(product_alt.is_valid());
        assert_eq!(product_alt, product);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(Integer::product(once(&x)), x);
        assert_eq!(Integer::product(once(x.clone())), x);
    });

    integer_pair_gen().test_properties(|(x, y)| {
        let product = &x * &y;
        assert_eq!(Integer::product([&x, &y].into_iter()), product);
        assert_eq!(Integer::product([x, y].into_iter()), product);
    });

    natural_vec_gen().test_properties(|xs| {
        assert_eq!(
            Integer::product(xs.iter().map(Integer::from)),
            Integer::from(Natural::product(xs.into_iter()))
        );
    });
}
