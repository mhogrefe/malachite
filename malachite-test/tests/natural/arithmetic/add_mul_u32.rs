use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_native::traits::AddMul as native_add_mul;
use malachite_native::traits::AddMulAssign as native_add_mul_assign;
use malachite_gmp::natural as gmp;
use malachite_gmp::traits::AddMul as gmp_add_mul;
use malachite_gmp::traits::AddMulAssign as gmp_add_mul_assign;
use malachite_test::common::gmp_natural_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single,
                                     exhaustive_triples, random_pairs, random_pairs_from_single,
                                     random_triples};
use std::str::FromStr;

#[test]
fn test_add_u32() {
    #[allow(cyclomatic_complexity)]
    let test = |u, v, c: u32, out| {
        let mut a = native::Natural::from_str(u).unwrap();
        a.add_mul_assign(&native::Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Natural::from_str(u).unwrap();
        a.add_mul_assign(&gmp::Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Natural::from_str(u).unwrap().add_mul(
            &native::Natural::from_str(v).unwrap(),
            c,
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Natural::from_str(u).unwrap().add_mul(
            &gmp::Natural::from_str(v)
                .unwrap(),
            c,
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&native::Natural::from_str(u).unwrap()).add_mul(
            &native::Natural::from_str(v).unwrap(),
            c,
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a =
            (&gmp::Natural::from_str(u).unwrap()).add_mul(&gmp::Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 123, "0");
    test("123", "0", 5, "123");
    test("123", "5", 1, "128");
    test("123", "5", 100, "623");
    test("10", "3", 4, "22");
    test("1000000000000", "0", 123, "1000000000000");
    test("1000000000000", "1", 123, "1000000000123");
    test("1000000000000", "123", 1, "1000000000123");
    test("1000000000000", "123", 100, "1000000012300");
    test("1000000000000", "100", 123, "1000000012300");
    test("1000000000000", "65536", 65536, "1004294967296");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", 1, "2000000000000");
    test("1000000000000", "1000000000000", 100, "101000000000000");
    test("0", "1000000000000", 100, "100000000000000");
}

#[test]
fn add_mul_u32_properties() {
    // a.add_mul_assign(&b, c) is equivalent for malachite-gmp and malachite-native.
    // a.add_mul(&b, c) is equivalent for malachite-gmp and malachite-native.
    // (&a).add_mul(&b, c) is equivalent for malachite-gmp and malachite-native.
    // a.add_mul_assign(&b, c); a is valid.
    // a.add_mul(&b, c) is valid.
    // (&a).add_mul(&b, c) is valid.
    // a.add_mul_assign(&b, c), a.add_mul(&b, c), and (&a).add_mul(&b, c) give the same result.
    // a.add_mul(&b, c) is equivalent to a + b * c.
    let natural_natural_and_u32 = |mut gmp_a: gmp::Natural, gmp_b: gmp::Natural, c: u32| {
        let mut a = gmp_natural_to_native(&gmp_a);
        let b = gmp_natural_to_native(&gmp_b);
        let old_a = a.clone();
        gmp_a.add_mul_assign(&gmp_b, c);
        assert!(gmp_a.is_valid());

        a.add_mul_assign(&b, c);
        assert!(a.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_a), a);

        let a2 = old_a.clone();
        let result = (&a2).add_mul(&b, c);
        assert!(result.is_valid());
        assert_eq!(result, a);
        let result = a2.add_mul(&b, c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        assert_eq!(&old_a + b * c, result);
    };

    // n.add_mul(0, c) == n
    // n.add_mul(1, c) == n + c
    // 0.add_mul(n, c) == n * c
    let natural_and_u32 = |gmp_n: gmp::Natural, c: u32| {
        let n = &gmp_natural_to_native(&gmp_n);
        assert_eq!(n.add_mul(&native::Natural::from(0u32), c), *n);
        assert_eq!(n.add_mul(&native::Natural::from(1u32), c), n + c);
        assert_eq!(native::Natural::from(0u32).add_mul(n, c), n * c);
    };

    // a.add_mul(b, 0) == a
    // a.add_mul(b, 1) == a + b
    let two_naturals = |gmp_a: gmp::Natural, gmp_b: gmp::Natural| {
        let a = &gmp_natural_to_native(&gmp_a);
        let b = &gmp_natural_to_native(&gmp_b);
        assert_eq!(a.add_mul(b, 0), *a);
        assert_eq!(a.add_mul(b, 1), a + b);
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
