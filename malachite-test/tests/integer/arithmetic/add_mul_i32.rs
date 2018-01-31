use common::LARGE_LIMIT;
use malachite_base::num::{AddMul, AddMulAssign, NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed, pairs_of_integers,
                                      triples_of_integer_integer_and_signed};
use std::str::FromStr;

#[test]
fn test_add_mul_i32() {
    let test = |u, v, c: i32, out| {
        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(u).unwrap();
        a.add_mul_assign(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u)
            .unwrap()
            .add_mul(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u)
            .unwrap()
            .add_mul(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(u).unwrap()).add_mul(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(u).unwrap()).add_mul(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 123, "0");
    test("123", "0", 5, "123");
    test("-123", "0", 5, "-123");
    test("123", "5", 1, "128");
    test("-123", "5", 1, "-118");
    test("123", "5", 100, "623");
    test("-123", "5", 100, "377");
    test("10", "3", 4, "22");
    test("10", "-3", 4, "-2");
    test("1000000000000", "0", 123, "1000000000000");
    test("1000000000000", "1", 123, "1000000000123");
    test("1000000000000", "123", 1, "1000000000123");
    test("1000000000000", "123", 100, "1000000012300");
    test("1000000000000", "100", 123, "1000000012300");
    test("1000000000000", "65536", 0x1_0000, "1004294967296");
    test("-1000000000000", "-65536", 0x1_0000, "-1004294967296");
    test("-1000000000000", "65536", 0x1_0000, "-995705032704");
    test("1000000000000", "-65536", 0x1_0000, "995705032704");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", 1, "2000000000000");
    test("1000000000000", "1000000000000", 100, "101000000000000");
    test("0", "1000000000000", 100, "100000000000000");
    test("-1", "1000000000000", 100, "99999999999999");
    test("0", "-1000000000000", 100, "-100000000000000");
    test("1", "-1000000000000", 100, "-99999999999999");
    test("0", "0", -123, "0");
    test("-123", "0", -5, "-123");
    test("123", "0", -5, "123");
    test("-123", "5", -1, "-128");
    test("123", "5", -1, "118");
    test("-123", "5", -100, "-623");
    test("123", "5", -100, "-377");
    test("-10", "3", -4, "-22");
    test("-10", "-3", -4, "2");
    test("-1000000000000", "0", -123, "-1000000000000");
    test("-1000000000000", "1", -123, "-1000000000123");
    test("-1000000000000", "123", -1, "-1000000000123");
    test("-1000000000000", "123", -100, "-1000000012300");
    test("-1000000000000", "100", -123, "-1000000012300");
    test("-1000000000000", "65536", -0x1_0000, "-1004294967296");
    test("1000000000000", "-65536", -0x1_0000, "1004294967296");
    test("1000000000000", "65536", -0x1_0000, "995705032704");
    test("-1000000000000", "-65536", -0x1_0000, "-995705032704");
    test("-1000000000000", "1000000000000", -1, "-2000000000000");
    test("-1000000000000", "1000000000000", -100, "-101000000000000");
    test("0", "1000000000000", -100, "-100000000000000");
    test("1", "1000000000000", -100, "-99999999999999");
    test("0", "-1000000000000", -100, "100000000000000");
    test("-1", "-1000000000000", -100, "99999999999999");
    test("1000000000000", "-1000000000000", 1, "0");
    test("1000000000000", "1000000000000", -1, "0");
    test("-1000000000000", "1000000000000", 1, "0");
    test("1000000000000", "1000000000000", -1, "0");
    test(
        "1000000000000000000000",
        "-1000000000000",
        1_000_000_000,
        "0",
    );
    test(
        "1000000000000000000000",
        "1000000000000",
        -1_000_000_000,
        "0",
    );
    test(
        "-1000000000000000000000",
        "1000000000000",
        1_000_000_000,
        "0",
    );
    test(
        "1000000000000000000000",
        "1000000000000",
        -1_000_000_000,
        "0",
    );
}

#[test]
fn add_mul_i32_properties() {
    // a.add_mul_assign(b, c); a is valid.
    // a.add_mul_assign(&b, c); a is valid.
    // a.add_mul(b, c) is valid.
    // a.add_mul(&b, c) is valid.
    // (&a).add_mul(b, c) is valid.
    // (&a).add_mul(&b, c) is valid.
    // a.add_mul_assign(b, c), a.add_mul_assign(&b, c), a.add_mul(b, c), a.add_mul(&b, c),
    //      (&a).add_mul(b, c), and (&a).add_mul(&b, c) give the same result.
    // a.add_mul(b, c) is equivalent to a + b * c.
    // a.add_mul(b, c) = a.add_mul(-b, -c)
    // -(a.add_mul(b, c)) = (-a).add_mul(-b, c) = (-a).add_mul(b, -c)
    // a.add_mul(&b, c) is equivalent to a.add_mul(&b, Integer::from(c))
    let integer_integer_and_i32 = |mut a: Integer, b: Integer, c: i32| {
        let old_a = a.clone();
        a.add_mul_assign(b.clone(), c);
        assert!(a.is_valid());

        let mut a2 = old_a.clone();
        a2.add_mul_assign(&b, c);
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let a2 = old_a.clone();
        let result = (&a2).add_mul(b.clone(), c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let a2 = old_a.clone();
        let result = (&a2).add_mul(&b, c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().add_mul(b.clone(), c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.add_mul(&b, c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        assert_eq!(&old_a + &b * c, result);
        assert_eq!((&old_a).add_mul(-&b, -c), result);
        assert_eq!((-&old_a).add_mul(-&b, c), -&result);
        assert_eq!((-&old_a).add_mul(&b, -c), -&result);
        assert_eq!(old_a.add_mul(b, Integer::from(c)), result);
    };

    // (n * c).add_mul(-n, c) == 0
    // (n * c).add_mul(n, -c) == 0
    let single_integer = |n: &Integer| {
        assert_eq!(n.add_mul(n, -1), 0);
        assert_eq!(n.add_mul(-n, 1), 0);
    };

    // n.add_mul(0, c) == n
    // n.add_mul(1, c) == n + c
    // n.add_mul(-1, c) == n - c
    // 0.add_mul(n, c) == n * c
    // n.add_mul(n, -1) == 0
    // n.add_mul(-n, 1) == 0
    let integer_and_i32 = |n: &Integer, c: i32| {
        assert_eq!(n.add_mul(&Integer::ZERO, c), *n);
        assert_eq!(n.add_mul(&Integer::ONE, c), n + c);
        assert_eq!(n.add_mul(&Integer::NEGATIVE_ONE, c), n - c);
        assert_eq!(Integer::ZERO.add_mul(n, c), n * c);
        assert_eq!((n * c).add_mul(-n, c), 0);
        assert_eq!((n * c).add_mul(n, -c), 0);
    };

    // a.add_mul(b, 0) == a
    // a.add_mul(b, 1) == a + b
    // a.add_mul(b, -1) == a - b
    let two_integers = |a: &Integer, b: &Integer| {
        assert_eq!(a.add_mul(b, 0), *a);
        assert_eq!(a.add_mul(b, 1), a + b);
        assert_eq!(a.add_mul(b, -1), a - b);
    };

    for (a, b, c) in
        triples_of_integer_integer_and_signed(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_integer_and_i32(a, b, c);
    }

    for (a, b, c) in
        triples_of_integer_integer_and_signed(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_integer_and_i32(a, b, c);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        single_integer(&n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        single_integer(&n);
    }

    for (n, c) in pairs_of_integer_and_signed(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_i32(&n, c);
    }

    for (n, c) in pairs_of_integer_and_signed(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_i32(&n, c);
    }

    for (a, b) in pairs_of_integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_integers(&a, &b);
    }

    for (a, b) in pairs_of_integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_integers(&a, &b);
    }
}
