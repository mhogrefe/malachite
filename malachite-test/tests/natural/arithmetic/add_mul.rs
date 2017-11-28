use common::LARGE_LIMIT;
use malachite_base::traits::{One, Zero};
use malachite_base::traits::{AddMul, AddMulAssign};
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, native_natural_to_gmp};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, exhaustive_triples_from_single,
                                     random_pairs_from_single, random_triples_from_single};
use std::str::FromStr;

#[test]
fn test_add_mul() {
    #[allow(unknown_lints, cyclomatic_complexity)]
    let test = |u, v, w, out| {
        let mut a = native::Natural::from_str(u).unwrap();
        a.add_mul_assign(
            native::Natural::from_str(v).unwrap(),
            native::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Natural::from_str(u).unwrap();
        a.add_mul_assign(
            gmp::Natural::from_str(v).unwrap(),
            gmp::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = native::Natural::from_str(u).unwrap();
        a.add_mul_assign(
            native::Natural::from_str(v).unwrap(),
            &native::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Natural::from_str(u).unwrap();
        a.add_mul_assign(
            gmp::Natural::from_str(v).unwrap(),
            &gmp::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = native::Natural::from_str(u).unwrap();
        a.add_mul_assign(
            &native::Natural::from_str(v).unwrap(),
            native::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Natural::from_str(u).unwrap();
        a.add_mul_assign(
            &gmp::Natural::from_str(v).unwrap(),
            gmp::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = native::Natural::from_str(u).unwrap();
        a.add_mul_assign(
            &native::Natural::from_str(v).unwrap(),
            &native::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Natural::from_str(u).unwrap();
        a.add_mul_assign(
            &gmp::Natural::from_str(v).unwrap(),
            &gmp::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Natural::from_str(u).unwrap().add_mul(
            native::Natural::from_str(v).unwrap(),
            native::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Natural::from_str(u).unwrap().add_mul(
            gmp::Natural::from_str(v).unwrap(),
            gmp::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Natural::from_str(u).unwrap().add_mul(
            native::Natural::from_str(v).unwrap(),
            &native::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Natural::from_str(u).unwrap().add_mul(
            gmp::Natural::from_str(v).unwrap(),
            &gmp::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Natural::from_str(u).unwrap().add_mul(
            &native::Natural::from_str(v).unwrap(),
            native::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Natural::from_str(u).unwrap().add_mul(
            &gmp::Natural::from_str(v).unwrap(),
            gmp::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Natural::from_str(u).unwrap().add_mul(
            &native::Natural::from_str(v).unwrap(),
            &native::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Natural::from_str(u).unwrap().add_mul(
            &gmp::Natural::from_str(v).unwrap(),
            &gmp::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&native::Natural::from_str(u).unwrap()).add_mul(
            &native::Natural::from_str(v).unwrap(),
            &native::Natural::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&gmp::Natural::from_str(u).unwrap()).add_mul(
            &gmp::Natural::from_str(v).unwrap(),
            &gmp::Natural::from_str(w).unwrap(),
        );
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
}

#[test]
fn add_mul_u32_properties() {
    // a.add_mul_assign(b, c) is equivalent for malachite-gmp and malachite-native.
    // a.add_mul_assign(b, &c) is equivalent for malachite-gmp and malachite-native.
    // a.add_mul_assign(&b, c) is equivalent for malachite-gmp and malachite-native.
    // a.add_mul_assign(&b, &c) is equivalent for malachite-gmp and malachite-native.
    // a.add_mul(b, c) is equivalent for malachite-gmp and malachite-native.
    // a.add_mul(b, &c) is equivalent for malachite-gmp and malachite-native.
    // a.add_mul(&b, c) is equivalent for malachite-gmp and malachite-native.
    // a.add_mul(&b, &c) is equivalent for malachite-gmp and malachite-native.
    // (&a).add_mul(&b, &c) is equivalent for malachite-gmp and malachite-native.
    // a.add_mul_assign(b, c); a is valid.
    // a.add_mul_assign(b, &c); a is valid.
    // a.add_mul_assign(&b, c); a is valid.
    // a.add_mul_assign(&b, &c); a is valid.
    // a.add_mul(b, c) is valid.
    // a.add_mul(b, &c) is valid.
    // a.add_mul(&b, c) is valid.
    // a.add_mul(&b, &c) is valid.
    // (&a).add_mul(&b, &c) is valid.
    // a.add_mul_assign(b, c), a.add_mul_assign(b, &c), a.add_mul_assign(&b, c),
    // a.add_mul_assign(&b, &c), a.add_mul(b, c), a.add_mul(b, &c), a.add_mul(&b, c),
    //      a.add_mul(&b, &c), and (&a).add_mul(&b, &c) give the same result.
    // a.add_mul(b, c) is equivalent to a + b * c.
    // a.add_mul(b, c) is equivalent to a.add_mul(c, b).
    let three_naturals = |mut gmp_a: gmp::Natural, gmp_b: gmp::Natural, gmp_c: gmp::Natural| {
        let mut a = gmp_natural_to_native(&gmp_a);
        let b = gmp_natural_to_native(&gmp_b);
        let c = gmp_natural_to_native(&gmp_c);
        let old_a = a.clone();
        gmp_a.add_mul_assign(gmp_b.clone(), gmp_c.clone());
        assert!(gmp_a.is_valid());

        let mut gmp_a_2 = native_natural_to_gmp(&old_a);
        gmp_a_2.add_mul_assign(gmp_b.clone(), &gmp_c);
        assert!(gmp_a_2.is_valid());
        assert_eq!(gmp_a_2, gmp_a);

        let mut gmp_a_2 = native_natural_to_gmp(&old_a);
        gmp_a_2.add_mul_assign(&gmp_b, gmp_c.clone());
        assert!(gmp_a_2.is_valid());
        assert_eq!(gmp_a_2, gmp_a);

        let mut gmp_a_2 = native_natural_to_gmp(&old_a);
        gmp_a_2.add_mul_assign(&gmp_b, &gmp_c);
        assert!(gmp_a_2.is_valid());
        assert_eq!(gmp_a_2, gmp_a);

        a.add_mul_assign(b.clone(), c.clone());
        assert!(a.is_valid());
        assert_eq!(gmp_natural_to_native(&gmp_a), a);

        let mut a2 = old_a.clone();
        a2.add_mul_assign(b.clone(), &c);
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let mut a2 = old_a.clone();
        a2.add_mul_assign(&b, c.clone());
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let mut a2 = old_a.clone();
        a2.add_mul_assign(&b, &c);
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let gmp_a_2 = native_natural_to_gmp(&old_a);
        let result = gmp_a_2.clone().add_mul(gmp_b.clone(), gmp_c.clone());
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = gmp_a_2.clone().add_mul(gmp_b.clone(), &gmp_c);
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = gmp_a_2.clone().add_mul(&gmp_b, gmp_c.clone());
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = gmp_a_2.clone().add_mul(&gmp_b, &gmp_c);
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = (&gmp_a_2).add_mul(&gmp_b, &gmp_c);
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let a2 = old_a.clone();
        let result = a2.clone().add_mul(b.clone(), c.clone());
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().add_mul(b.clone(), &c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().add_mul(&b, c.clone());
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().add_mul(&b, &c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = (&a2).add_mul(&b, &c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        assert_eq!(&old_a + &b * &c, result);
        assert_eq!(old_a.add_mul(c, b), result);
    };

    // a.add_mul(0, b) == a
    // a.add_mul(1, b) == a + b
    // 0.add_mul(a, b) == a * b
    // a.add_mul(b, 0) == a
    // a.add_mul(b, 1) == a + b
    let two_naturals = |gmp_a: gmp::Natural, gmp_b: gmp::Natural| {
        let a = &gmp_natural_to_native(&gmp_a);
        let b = &gmp_natural_to_native(&gmp_b);
        assert_eq!(a.add_mul(&native::Natural::ZERO, b), *a);
        assert_eq!(a.add_mul(&native::Natural::ONE, b), a + b);
        assert_eq!(native::Natural::ZERO.add_mul(a, b), a * b);
        assert_eq!(a.add_mul(b, &native::Natural::ZERO), *a);
        assert_eq!(a.add_mul(b, &native::Natural::ONE), a + b);
    };

    for (a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        three_naturals(a, b, c);
    }

    for (a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32))
        .take(LARGE_LIMIT)
    {
        three_naturals(a, b, c);
    }

    for (a, b) in exhaustive_pairs_from_single(exhaustive_naturals()).take(LARGE_LIMIT) {
        two_naturals(a, b);
    }

    for (a, b) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_naturals(a, b);
    }
}
