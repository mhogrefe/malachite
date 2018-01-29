use common::LARGE_LIMIT;
use malachite_base::traits::{One, Zero};
use malachite_base::traits::{SubMul, SubMulAssign};
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned, pairs_of_naturals,
                                      triples_of_natural_natural_and_unsigned};
use std::str::FromStr;

#[test]
fn test_sub_mul_assign_u32() {
    let test = |u, v, c, out: &str| {
        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(&Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 123, "0");
    test("123", "0", 5, "123");
    test("123", "5", 1, "118");
    test("15", "3", 4, "3");
    test("1000000000000", "0", 123, "1000000000000");
    test("1000000000000", "1", 123, "999999999877");
    test("1000000000000", "123", 1, "999999999877");
    test("1000000000000", "123", 100, "999999987700");
    test("1000000000000", "100", 123, "999999987700");
    test("1000000000000", "65536", 0x1_0000, "995705032704");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", 1, "0");
    test("4294967296", "1", 1, "4294967295");
    test("1000000000000", "1000000000000", 1, "0");
    test(
        "1000000000000000000000",
        "1000000000000",
        1_000_000_000,
        "0",
    );
}

#[test]
#[should_panic(expected = "Natural sub_mul_assign cannot have a negative result")]
fn sub_mul_assign_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(&Natural::from_str("5").unwrap(), 100);
}

#[test]
#[should_panic(expected = "Natural sub_mul_assign cannot have a negative result")]
fn sub_mul_assign_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(&Natural::from_str("1000000000000").unwrap(), 100);
}

#[test]
fn test_sub_mul_u32() {
    let test = |u, v, c: u32, out| {
        let on = Natural::from_str(u)
            .unwrap()
            .sub_mul(&Natural::from_str(v).unwrap(), c);
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&Natural::from_str(u).unwrap()).sub_mul(&Natural::from_str(v).unwrap(), c);
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "0", 0, "Some(0)");
    test("0", "0", 123, "Some(0)");
    test("123", "0", 5, "Some(123)");
    test("123", "5", 1, "Some(118)");
    test("123", "5", 100, "None");
    test("10", "3", 4, "None");
    test("15", "3", 4, "Some(3)");
    test("1000000000000", "0", 123, "Some(1000000000000)");
    test("1000000000000", "1", 123, "Some(999999999877)");
    test("1000000000000", "123", 1, "Some(999999999877)");
    test("1000000000000", "123", 100, "Some(999999987700)");
    test("1000000000000", "100", 123, "Some(999999987700)");
    test("1000000000000", "65536", 0x1_0000, "Some(995705032704)");
    test("1000000000000", "1000000000000", 0, "Some(1000000000000)");
    test("1000000000000", "1000000000000", 1, "Some(0)");
    test("1000000000000", "1000000000000", 100, "None");
    test("0", "1000000000000", 100, "None");
    test("4294967296", "1", 1, "Some(4294967295)");
    test("3902609153", "88817093856604", 1, "None");
    test("1000000000000", "1000000000000", 1, "Some(0)");
    test(
        "1000000000000000000000",
        "1000000000000",
        1_000_000_000,
        "Some(0)",
    );
}

#[test]
fn sub_mul_u32_properties() {
    // a.sub_mul_assign(&b, c); a is valid.
    // a.sub_mul(&b, c) is valid.
    // (&a).sub_mul(&b, c) is valid.
    // a.sub_mul_assign(&b, c), a.sub_mul(&b, c), and (&a).sub_mul(&b, c) give the same result.
    // a.sub_mul(&b, c) is equivalent to a - b * c.
    // a.sub_mul(&b, c) is equivalent to a.sub_mul(&b, Natural::from(c))
    let natural_natural_and_u32 = |mut a: Natural, b: Natural, c: u32| {
        let old_a = a.clone();
        if a >= &b * c {
            a.sub_mul_assign(&b, c);
            assert!(a.is_valid());
        }
        let oa = if old_a >= &b * c { Some(a) } else { None };

        let a2 = old_a.clone();
        let result = (&a2).sub_mul(&b, c);
        assert!(result.clone().map_or(true, |n| n.is_valid()));
        assert_eq!(result, oa);

        let result = a2.sub_mul(&b, c);
        assert!(result.clone().map_or(true, |n| n.is_valid()));
        assert_eq!(result, oa);

        assert_eq!(&old_a - &(&b * c), oa);
        assert_eq!(old_a.sub_mul(&b, &Natural::from(c)), result);
    };

    // n.sub_mul(n, 1) == 0
    let single_natural = |n: &Natural| {
        assert_eq!(n.sub_mul(n, 1), Some(Natural::ZERO));
    };

    // n.sub_mul(0, c) == Some(n)
    // n.sub_mul(1, c) == n - c
    // (n * c).sub_mul(n, c) == 0
    let natural_and_u32 = |n: &Natural, c: u32| {
        assert_eq!(n.sub_mul(&Natural::ZERO, c), Some(n.clone()));
        assert_eq!(n.sub_mul(&Natural::ONE, c), n - c);
        assert_eq!((n * c).sub_mul(n, c), Some(Natural::ZERO));
    };

    // a.sub_mul(b, 0) == Some(a)
    // a.sub_mul(b, 1) == a - b
    let two_naturals = |a: &Natural, b: &Natural| {
        assert_eq!(a.sub_mul(b, 0), Some(a.clone()));
        assert_eq!(a.sub_mul(b, 1), a - b);
    };

    for (a, b, c) in
        triples_of_natural_natural_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_natural_and_u32(a, b, c);
    }

    for (a, b, c) in
        triples_of_natural_natural_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_natural_and_u32(a, b, c);
    }

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        single_natural(&n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        single_natural(&n);
    }

    for (n, c) in pairs_of_natural_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(&n, c);
    }

    for (n, c) in pairs_of_natural_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_u32(&n, c);
    }

    for (a, b) in pairs_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_naturals(&a, &b);
    }

    for (a, b) in pairs_of_naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_naturals(&a, &b);
    }
}
