use common::LARGE_LIMIT;
use malachite_base::num::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rug_integer,
                             rug_integer_to_integer, GenerationMode};
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
    // n *= u is equivalent for malachite and rug.
    // n * u is equivalent for malachite, num, and rug.
    // &n * u is equivalent for malachite and num.
    // n *= u; n is valid.
    // n * u and u * n are valid.
    // &n * u and u * &n are valid.
    // n *= u, n * u, u * n, &n * u, and u * &n give the same result.
    // n * u == n * from(u)
    // (-n) * u == -(n * u)
    // TODO n * u / u == n
    let integer_and_u32 = |mut n: Integer, u: u32| {
        let old_n = n.clone();
        n *= u;
        assert!(n.is_valid());

        let mut rug_n = integer_to_rug_integer(&old_n);
        rug_n *= u;
        assert_eq!(rug_integer_to_integer(&rug_n), n);

        let n2 = old_n.clone();
        let result = &n2 * u;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = n2 * u;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let n2 = old_n.clone();
        let result = u * &n2;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = u * n2;
        assert_eq!(result, n);
        assert!(result.is_valid());

        let n2 = old_n.clone();
        let result = n2 * Integer::from(u);
        assert_eq!(result, n);
        let n2 = old_n.clone();
        let result = Integer::from(u) * n2;
        assert_eq!(result, n);

        let num_n2 = integer_to_bigint(&old_n);
        assert_eq!(bigint_to_integer(&num_mul_u32(num_n2, u)), n);

        let rug_n2 = integer_to_rug_integer(&old_n);
        assert_eq!(rug_integer_to_integer(&(rug_n2 * u)), n);

        assert_eq!((-&n) * u, -(n * u));
        //TODO assert_eq!(n / u, Some(old_n));
    };

    // n * 0 == 0
    // 0 * n == 0
    // n * 1 == n
    // 1 * n == n
    // n * 2 == n << 1
    // 2 * n == n << 1
    #[allow(unknown_lints, erasing_op, identity_op)]
    let one_integer = |n: Integer| {
        assert_eq!(&n * 0u32, 0);
        assert_eq!(0u32 * &n, 0);
        assert_eq!(&n * 1u32, n);
        assert_eq!(1u32 * &n, n);
        assert_eq!(&n * 2u32, &n << 1);
        assert_eq!(2u32 * &n, &n << 1);
    };

    // 0 * u == 0
    // u * 0 == 0
    // 1 * u == u
    // u * 1 == u
    let one_u32 = |u: u32| {
        assert_eq!(Integer::ZERO * u, 0);
        assert_eq!(u * Integer::ZERO, 0);
        assert_eq!(Integer::ONE * u, u);
        assert_eq!(u * Integer::ONE, u);
    };

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for u in unsigneds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(u);
    }

    for u in unsigneds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(u);
    }
}
