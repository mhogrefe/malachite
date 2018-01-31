use common::LARGE_LIMIT;
use malachite_base::num::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rugint_integer,
                             rugint_integer_to_integer, GenerationMode};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned, pairs_of_integers,
                                      triples_of_integers};
use num::BigInt;
use rugint;
use std::str::FromStr;

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

        let n = rugint::Integer::from_str(u).unwrap() * rugint::Integer::from_str(v).unwrap();
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
    // x * y is valid.
    // x * &y is valid.
    // &x * y is valid.
    // &x * &y is valid.
    // x * y is equivalent for malachite, num, and rugint.
    // x *= y, x *= &y, x * y, x * &y, &x * y, and &x * &y give the same result.
    // x * y == y * x
    //TODO x * y / y == x and x * y / x == y
    // (-x) * y == -(x * y)
    // x * (-y) == -(x * y)
    let two_integers = |x: Integer, y: Integer| {
        let num_product = bigint_to_integer(&(integer_to_bigint(&x) * integer_to_bigint(&y)));
        let rugint_product = rugint_integer_to_integer(
            &(integer_to_rugint_integer(&x) * integer_to_rugint_integer(&y)),
        );

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

        let mut mut_x = integer_to_rugint_integer(&x);
        mut_x *= integer_to_rugint_integer(&y);
        assert_eq!(rugint_integer_to_integer(&mut_x), product);

        let reverse_product = &y * &x;
        //TODO let inv_1 = (&product / &x).unwrap();
        //TODO let inv_2 = (&product / &y).unwrap();
        assert_eq!(num_product, product);
        assert_eq!(rugint_product, product);
        assert_eq!(reverse_product, product);
        //TODO assert_eq!(inv_1, y);
        //TODO assert_eq!(inv_2, x);

        assert_eq!(-&x * &y, -&product);
        assert_eq!(x * -y, -product);
    };

    // x * (y: u32) == x * from(y)
    // (y: u32) * x == x * from(y)
    let integer_and_u32 = |x: Integer, y: u32| {
        let primitive_product_1 = &x * y;
        let primitive_product_2 = y * &x;
        let product = x * Integer::from(y);
        assert_eq!(primitive_product_1, product);
        assert_eq!(primitive_product_2, product);
    };

    // x * 0 == 0
    // 0 * x == 0
    // x * 1 == x
    // 1 * x == x
    //TODO x * x == x ^ 2
    #[allow(unknown_lints, erasing_op)]
    let one_integer = |x: Integer| {
        let x_old = x.clone();
        assert_eq!(&x * Integer::ZERO, 0);
        assert_eq!(Integer::ZERO * 0, 0);
        let id_1 = &x * Integer::ONE;
        let id_2 = Integer::ONE * &x;
        //TODO let square = &x * &x;
        assert_eq!(id_1, x_old);
        assert_eq!(id_2, x_old);
        //TODO assert_eq!(square, x_old.pow(2));
    };

    // (x * y) * z == x * (y * z)
    // x * (y + z) == x * y + x * z
    // (x + y) * z == x * z + y * z
    let three_integers = |x: Integer, y: Integer, z: Integer| {
        assert_eq!((&x * &y) * &z, &x * (&y * &z));
        assert_eq!(&x * (&y + &z), &x * &y + &x * &z);
        assert_eq!((&x + &y) * &z, x * &z + y * z);
    };

    for (x, y) in pairs_of_integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in pairs_of_integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_integers(x, y);
    }

    for (x, y) in pairs_of_integer_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(x, y);
    }

    for (x, y) in pairs_of_integer_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(x, y);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for (x, y, z) in triples_of_integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        three_integers(x, y, z);
    }

    for (x, y, z) in triples_of_integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        three_integers(x, y, z);
    }
}
