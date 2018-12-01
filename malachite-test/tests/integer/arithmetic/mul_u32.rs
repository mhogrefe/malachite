use common::test_properties;
use malachite_base::num::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use malachite_test::integer::arithmetic::mul_u32::num_mul_u32;
use num::BigInt;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_add_u32() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_mul_u32(BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v * &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from(0);
        n.assign(v * &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "0");
    test("123", 0, "0");
    test("1", 123, "123");
    test("123", 1, "123");
    test("123", 456, "56088");
    test("1000000000000", 0, "0");
    test("1000000000000", 1, "1000000000000");
    test("1000000000000", 123, "123000000000000");
    test("4294967295", 2, "8589934590");
    test("18446744073709551615", 2, "36893488147419103230");
    test("-1", 123, "-123");
    test("-123", 1, "-123");
    test("-123", 456, "-56088");
    test("-1000000000000", 0, "0");
    test("-1000000000000", 1, "-1000000000000");
    test("-1000000000000", 123, "-123000000000000");
    test("-4294967295", 2, "-8589934590");
    test("-4294967296", 2, "-8589934592");
    test("-18446744073709551615", 2, "-36893488147419103230");
}

#[test]
fn mul_u32_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            let mut mut_n = n.clone();
            mut_n *= u;
            assert!(mut_n.is_valid());
            let product = mut_n;

            let mut rug_n = integer_to_rug_integer(n);
            rug_n *= u;
            assert_eq!(rug_integer_to_integer(&rug_n), product);

            let product_alt = n * u;
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);
            let product_alt = n.clone() * u;
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);

            let product_alt = u * n;
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);
            let product_alt = u * n.clone();
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);

            let product_alt = n * Integer::from(u);
            assert_eq!(product_alt, product);
            let product_alt = Integer::from(u) * n.clone();
            assert_eq!(product_alt, product);

            assert_eq!(
                bigint_to_integer(&num_mul_u32(integer_to_bigint(n), u)),
                product
            );
            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) * u)),
                product
            );

            assert_eq!((-n) * u, -(n * u));
            if u != 0 {
                assert_eq!(product / u, *n);
            }
        },
    );

    #[allow(unknown_lints, erasing_op, identity_op)]
    test_properties(integers, |n| {
        assert_eq!(n * 0u32, 0);
        assert_eq!(0u32 * n, 0);
        assert_eq!(n * 1u32, *n);
        assert_eq!(1u32 * n, *n);
        assert_eq!(n * 2u32, n << 1);
        assert_eq!(2u32 * n, n << 1);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Integer::ZERO * u, 0);
        assert_eq!(u * Integer::ZERO, 0);
        assert_eq!(Integer::ONE * u, u);
        assert_eq!(u * Integer::ONE, u);
    });
}
