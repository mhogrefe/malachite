use std::str::FromStr;

use malachite_base::num::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedDoubleLimb, SignedLimb};
use num::BigInt;
use rug;

use common::test_properties;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_signed, pairs_of_integer_and_unsigned, pairs_of_integers,
    triples_of_integers,
};
use malachite_test::inputs::natural::pairs_of_naturals;

#[test]
fn test_mul() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n *= Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n *= &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() * Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() * Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() * &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() * &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigInt::from_str(u).unwrap() * BigInt::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() * rug::Integer::from_str(v).unwrap();
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
}

#[test]
fn mul_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * y;
        let product_ref_val = x * y.clone();
        let product = x * y;
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
        mut_x *= y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x *= integer_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), product);

        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(x) * integer_to_bigint(y))),
            product
        );
        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) * integer_to_rug_integer(y))),
            product
        );
        assert_eq!(y * x, product);
        //TODO assert_eq!((product / x).unwrap(), y);
        //TODO assert_eq!((product / y).unwrap(), x);

        assert_eq!(-x * y, -&product);
        assert_eq!(x * -y, -product);
    });

    #[allow(unknown_lints, erasing_op)]
    test_properties(integers, |x| {
        assert_eq!(x * Integer::ZERO, 0 as Limb);
        assert_eq!(Integer::ZERO * x, 0 as Limb);
        assert_eq!(x * Integer::ONE, *x);
        assert_eq!(Integer::ONE * x, *x);
        assert_eq!(x * Integer::NEGATIVE_ONE, -x);
        assert_eq!(Integer::NEGATIVE_ONE * x, -x);
        //TODO assert_eq!(x * x, x.pow(2));
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        assert_eq!((x * y) * z, x * (y * z));
        assert_eq!(x * (y + z), x * y + x * z);
        assert_eq!((x + y) * z, x * z + y * z);
    });

    test_properties(pairs_of_integer_and_unsigned::<Limb>, |&(ref x, y)| {
        let product = x * Integer::from(y);
        assert_eq!(x * y, product);
        assert_eq!(y * x, product);
    });

    test_properties(pairs_of_integer_and_signed::<SignedLimb>, |&(ref x, y)| {
        let product = x * Integer::from(y);
        assert_eq!(x * y, product);
        assert_eq!(y * x, product);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(x * y, Integer::from(x) * Integer::from(y));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(
            Integer::from(SignedDoubleLimb::from(x) * SignedDoubleLimb::from(y)),
            Integer::from(x) * Integer::from(y)
        );
    });
}
