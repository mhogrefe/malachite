use common::LARGE_LIMIT;
use malachite_base::traits::{SubMul, SubMulAssign};
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_gmp};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single,
                                     exhaustive_triples, random_pairs, random_pairs_from_single,
                                     random_triples};
use std::str::FromStr;

#[test]
fn test_sub_mul_assign_u32() {
    let test = |u, v, c, out: &str| {
        let mut n = native::Natural::from_str(u).unwrap();
        n.sub_mul_assign(&native::Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Natural::from_str(u).unwrap();
        n.sub_mul_assign(&gmp::Natural::from_str(v).unwrap(), c);
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
    test("1000000000000", "65536", 65536, "995705032704");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", 1, "0");
    test("4294967296", "1", 1, "4294967295");
}


#[test]
#[should_panic(expected = "Natural sub_mul_assign cannot have a negative result")]
fn sub_mul_assign_fail_native_1() {
    let mut x = native::Natural::from_str("123").unwrap();
    x.sub_mul_assign(&native::Natural::from_str("5").unwrap(), 100);
}

#[test]
#[should_panic(expected = "Natural sub_mul_assign cannot have a negative result")]
fn sub_mul_assign_fail_native_2() {
    let mut x = native::Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(&native::Natural::from_str("1000000000000").unwrap(), 100);
}

#[test]
#[should_panic(expected = "Natural sub_mul_assign cannot have a negative result")]
fn sub_mul_assign_fail_gmp_1() {
    let mut x = gmp::Natural::from_str("123").unwrap();
    x.sub_mul_assign(&gmp::Natural::from_str("5").unwrap(), 100);
}

#[test]
#[should_panic(expected = "Natural sub_mul_assign cannot have a negative result")]
fn sub_mul_assign_fail_gmp_2() {
    let mut x = gmp::Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(&gmp::Natural::from_str("1000000000000").unwrap(), 100);
}

#[test]
fn test_sub_mul_u32() {
    let test = |u, v, c: u32, out| {
        let on = native::Natural::from_str(u).unwrap().sub_mul(
            &native::Natural::from_str(v).unwrap(),
            c,
        );
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = gmp::Natural::from_str(u).unwrap().sub_mul(
            &gmp::Natural::from_str(v)
                .unwrap(),
            c,
        );
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&native::Natural::from_str(u).unwrap()).sub_mul(
            &native::Natural::from_str(v).unwrap(),
            c,
        );
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on =
            (&gmp::Natural::from_str(u).unwrap()).sub_mul(&gmp::Natural::from_str(v).unwrap(), c);
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
    test("1000000000000", "65536", 65536, "Some(995705032704)");
    test("1000000000000", "1000000000000", 0, "Some(1000000000000)");
    test("1000000000000", "1000000000000", 1, "Some(0)");
    test("1000000000000", "1000000000000", 100, "None");
    test("0", "1000000000000", 100, "None");
    test("4294967296", "1", 1, "Some(4294967295)");
    test("3902609153", "88817093856604", 1, "None");
}

#[test]
fn sub_mul_u32_properties() {
    // a.sub_mul_assign(&b, c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul(&b, c) is equivalent for malachite-gmp and malachite-native.
    // (&a).sub_mul(&b, c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul_assign(&b, c); a is valid.
    // a.sub_mul(&b, c) is valid.
    // (&a).sub_mul(&b, c) is valid.
    // a.sub_mul_assign(&b, c), a.sub_mul(&b, c), and (&a).sub_mul(&b, c) give the same result.
    // a.sub_mul(&b, c) is equivalent to a - b * c.
    let natural_natural_and_u32 = |mut gmp_a: gmp::Natural, gmp_b: gmp::Natural, c: u32| {
        let mut a = gmp_natural_to_native(&gmp_a);
        let b = gmp_natural_to_native(&gmp_b);
        let old_a = a.clone();

        if a >= &b * c {
            gmp_a.sub_mul_assign(&gmp_b, c);
            assert!(gmp_a.is_valid());

            a.sub_mul_assign(&b, c);
            assert!(a.is_valid());
            assert_eq!(gmp_natural_to_native(&gmp_a), a);
        }
        let oa = if old_a >= &b * c { Some(a) } else { None };

        let gmp_a_2 = native_natural_to_gmp(&old_a);
        let result = (&gmp_a_2).sub_mul(&gmp_b, c);
        assert!(result.clone().map_or(true, |n| n.is_valid()));
        assert_eq!(result.map(|x| gmp_natural_to_native(&x)), oa);

        let result = gmp_a_2.sub_mul(&gmp_b, c);
        assert!(result.clone().map_or(true, |n| n.is_valid()));
        assert_eq!(result.map(|x| gmp_natural_to_native(&x)), oa);

        let a2 = old_a.clone();
        let result = (&a2).sub_mul(&b, c);
        assert!(result.clone().map_or(true, |n| n.is_valid()));
        assert_eq!(result, oa);

        let result = a2.sub_mul(&b, c);
        assert!(result.clone().map_or(true, |n| n.is_valid()));
        assert_eq!(result, oa);

        assert_eq!(old_a - &(b * c), oa);
    };

    // n.sub_mul(0, c) == Some(n)
    // n.sub_mul(1, c) == n - c
    let natural_and_u32 = |gmp_n: gmp::Natural, c: u32| {
        let n = &gmp_natural_to_native(&gmp_n);
        assert_eq!(n.sub_mul(&native::Natural::from(0u32), c), Some(n.clone()));
        assert_eq!(n.sub_mul(&native::Natural::from(1u32), c), n - c);
    };

    // a.sub_mul(b, 0) == Some(a)
    // a.sub_mul(b, 1) == a - b
    let two_naturals = |gmp_a: gmp::Natural, gmp_b: gmp::Natural| {
        let a = &gmp_natural_to_native(&gmp_a);
        let b = &gmp_natural_to_native(&gmp_b);
        assert_eq!(a.sub_mul(b, 0), Some(a.clone()));
        assert_eq!(a.sub_mul(b, 1), a - b);
    };

    for (a, b, c) in exhaustive_triples(
        exhaustive_naturals(),
        exhaustive_naturals(),
        exhaustive_u::<u32>(),
    ).take(LARGE_LIMIT)
    {
        natural_natural_and_u32(a, b, c);
    }

    for (a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        natural_natural_and_u32(a, b, c);
    }

    for (n, c) in exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        natural_and_u32(n, c);
    }

    for (n, c) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(LARGE_LIMIT)
    {
        natural_and_u32(n, c);
    }

    for (a, b) in exhaustive_pairs_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        two_naturals(a, b);
    }

    for (a, b) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_naturals(a, b);
    }
}
