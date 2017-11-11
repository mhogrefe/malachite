use common::LARGE_LIMIT;
use malachite_base::traits::{SubMul, SubMulAssign};
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_gmp};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, exhaustive_triples_from_single,
                                     random_pairs_from_single, random_triples_from_single};
use std::str::FromStr;

#[test]
fn test_sub_mul() {
    #[allow(cyclomatic_complexity)]
    let test = |u, v, w, out| {
        let mut a = native::Integer::from_str(u).unwrap();
        a.sub_mul_assign(
            native::Integer::from_str(v).unwrap(),
            native::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Integer::from_str(u).unwrap();
        a.sub_mul_assign(
            gmp::Integer::from_str(v).unwrap(),
            gmp::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = native::Integer::from_str(u).unwrap();
        a.sub_mul_assign(
            native::Integer::from_str(v).unwrap(),
            &native::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Integer::from_str(u).unwrap();
        a.sub_mul_assign(
            gmp::Integer::from_str(v).unwrap(),
            &gmp::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = native::Integer::from_str(u).unwrap();
        a.sub_mul_assign(
            &native::Integer::from_str(v).unwrap(),
            native::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Integer::from_str(u).unwrap();
        a.sub_mul_assign(
            &gmp::Integer::from_str(v).unwrap(),
            gmp::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = native::Integer::from_str(u).unwrap();
        a.sub_mul_assign(
            &native::Integer::from_str(v).unwrap(),
            &native::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = gmp::Integer::from_str(u).unwrap();
        a.sub_mul_assign(
            &gmp::Integer::from_str(v).unwrap(),
            &gmp::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Integer::from_str(u).unwrap().sub_mul(
            native::Integer::from_str(v).unwrap(),
            native::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Integer::from_str(u).unwrap().sub_mul(
            gmp::Integer::from_str(v).unwrap(),
            gmp::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Integer::from_str(u).unwrap().sub_mul(
            native::Integer::from_str(v).unwrap(),
            &native::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Integer::from_str(u).unwrap().sub_mul(
            gmp::Integer::from_str(v).unwrap(),
            &gmp::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Integer::from_str(u).unwrap().sub_mul(
            &native::Integer::from_str(v).unwrap(),
            native::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Integer::from_str(u).unwrap().sub_mul(
            &gmp::Integer::from_str(v).unwrap(),
            gmp::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = native::Integer::from_str(u).unwrap().sub_mul(
            &native::Integer::from_str(v).unwrap(),
            &native::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = gmp::Integer::from_str(u).unwrap().sub_mul(
            &gmp::Integer::from_str(v).unwrap(),
            &gmp::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&native::Integer::from_str(u).unwrap()).sub_mul(
            &native::Integer::from_str(v).unwrap(),
            &native::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&gmp::Integer::from_str(u).unwrap()).sub_mul(
            &gmp::Integer::from_str(v).unwrap(),
            &gmp::Integer::from_str(w).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "-5", "1", "128");
    test("123", "-5", "100", "623");
    test("10", "-3", "4", "22");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "-1", "123", "1000000000123");
    test("1000000000000", "-123", "1", "1000000000123");
    test("1000000000000", "-123", "100", "1000000012300");
    test("1000000000000", "-100", "123", "1000000012300");
    test("1000000000000", "-65536", "65536", "1004294967296");
    test("1000000000000", "-1000000000000", "0", "1000000000000");
    test("1000000000000", "-1000000000000", "1", "2000000000000");
    test("1000000000000", "-1000000000000", "100", "101000000000000");
    test("0", "-1000000000000", "100", "100000000000000");
    test(
        "1000000000000",
        "-65536",
        "1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "-1000000000000",
        "1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "-1000000000000",
        "1000000000000",
        "1000000000000000000000000",
    );

    test("123", "5", "-1", "128");
    test("123", "5", "-100", "623");
    test("10", "3", "-4", "22");
    test("1000000000000", "1", "-123", "1000000000123");
    test("1000000000000", "123", "-1", "1000000000123");
    test("1000000000000", "123", "-100", "1000000012300");
    test("1000000000000", "100", "-123", "1000000012300");
    test("1000000000000", "65536", "-65536", "1004294967296");
    test("1000000000000", "1000000000000", "-1", "2000000000000");
    test("1000000000000", "1000000000000", "-100", "101000000000000");
    test("0", "1000000000000", "-100", "100000000000000");
    test(
        "1000000000000",
        "65536",
        "-1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "1000000000000",
        "-1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "1000000000000",
        "-1000000000000",
        "1000000000000000000000000",
    );

    test("0", "0", "-123", "0");
    test("123", "0", "-5", "123");
    test("123", "5", "1", "118");
    test("123", "-5", "-1", "118");
    test("123", "5", "100", "-377");
    test("123", "-5", "-100", "-377");
    test("10", "3", "4", "-2");
    test("10", "-3", "-4", "-2");
    test("15", "3", "4", "3");
    test("15", "-3", "-4", "3");
    test("1000000000000", "0", "-123", "1000000000000");
    test("1000000000000", "1", "123", "999999999877");
    test("1000000000000", "-1", "-123", "999999999877");
    test("1000000000000", "123", "1", "999999999877");
    test("1000000000000", "-123", "-1", "999999999877");
    test("1000000000000", "123", "100", "999999987700");
    test("1000000000000", "-123", "-100", "999999987700");
    test("1000000000000", "100", "123", "999999987700");
    test("1000000000000", "-100", "-123", "999999987700");
    test("1000000000000", "65536", "65536", "995705032704");
    test("1000000000000", "-65536", "-65536", "995705032704");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "0");
    test("1000000000000", "-1000000000000", "-1", "0");
    test("1000000000000", "1000000000000", "100", "-99000000000000");
    test("1000000000000", "-1000000000000", "-100", "-99000000000000");
    test("0", "1000000000000", "100", "-100000000000000");
    test("4294967296", "1", "1", "4294967295");
    test("4294967296", "-1", "-1", "4294967295");
    test("3902609153", "88817093856604", "1", "-88813191247451");
    test("3902609153", "-88817093856604", "-1", "-88813191247451");

    test("-123", "0", "5", "-123");
    test("-123", "5", "1", "-128");
    test("-123", "5", "100", "-623");
    test("-10", "3", "4", "-22");
    test("-1000000000000", "0", "123", "-1000000000000");
    test("-1000000000000", "1", "123", "-1000000000123");
    test("-1000000000000", "123", "1", "-1000000000123");
    test("-1000000000000", "123", "100", "-1000000012300");
    test("-1000000000000", "100", "123", "-1000000012300");
    test("-1000000000000", "65536", "65536", "-1004294967296");
    test("-1000000000000", "1000000000000", "0", "-1000000000000");
    test("-1000000000000", "1000000000000", "1", "-2000000000000");
    test("-1000000000000", "1000000000000", "100", "-101000000000000");
    test(
        "-1000000000000",
        "65536",
        "1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "1000000000000",
        "1000000000000",
        "-1000000000001000000000000",
    );
    test(
        "0",
        "1000000000000",
        "1000000000000",
        "-1000000000000000000000000",
    );

    test("-123", "-5", "-1", "-128");
    test("-123", "-5", "-100", "-623");
    test("-10", "-3", "-4", "-22");
    test("-1000000000000", "-1", "-123", "-1000000000123");
    test("-1000000000000", "-123", "-1", "-1000000000123");
    test("-1000000000000", "-123", "-100", "-1000000012300");
    test("-1000000000000", "-100", "-123", "-1000000012300");
    test("-1000000000000", "-65536", "-65536", "-1004294967296");
    test("-1000000000000", "-1000000000000", "-1", "-2000000000000");
    test(
        "-1000000000000",
        "-1000000000000",
        "-100",
        "-101000000000000",
    );
    test(
        "-1000000000000",
        "-65536",
        "-1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "-1000000000000",
        "-1000000000000",
        "-1000000000001000000000000",
    );

    test("-123", "0", "-5", "-123");
    test("-123", "-5", "1", "-118");
    test("-123", "5", "-1", "-118");
    test("-123", "-5", "100", "377");
    test("-123", "5", "-100", "377");
    test("-10", "-3", "4", "2");
    test("-10", "3", "-4", "2");
    test("-15", "-3", "4", "-3");
    test("-15", "3", "-4", "-3");
    test("-1000000000000", "0", "-123", "-1000000000000");
    test("-1000000000000", "-1", "123", "-999999999877");
    test("-1000000000000", "1", "-123", "-999999999877");
    test("-1000000000000", "-123", "1", "-999999999877");
    test("-1000000000000", "123", "-1", "-999999999877");
    test("-1000000000000", "-123", "100", "-999999987700");
    test("-1000000000000", "123", "-100", "-999999987700");
    test("-1000000000000", "-100", "123", "-999999987700");
    test("-1000000000000", "100", "-123", "-999999987700");
    test("-1000000000000", "-65536", "65536", "-995705032704");
    test("-1000000000000", "65536", "-65536", "-995705032704");
    test("-1000000000000", "-1000000000000", "0", "-1000000000000");
    test("-1000000000000", "-1000000000000", "1", "0");
    test("-1000000000000", "1000000000000", "-1", "0");
    test("-1000000000000", "-1000000000000", "100", "99000000000000");
    test("-1000000000000", "1000000000000", "-100", "99000000000000");
    test("-4294967296", "-1", "1", "-4294967295");
    test("-4294967296", "1", "-1", "-4294967295");
    test("-3902609153", "-88817093856604", "1", "88813191247451");
    test("-3902609153", "88817093856604", "-1", "88813191247451");
    test(
        "1000000000000000000000000",
        "1000000000000",
        "1000000000000",
        "0",
    );
}

#[test]
fn sub_mul_properties() {
    // a.sub_mul_assign(b, c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul_assign(b, &c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul_assign(&b, c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul_assign(&b, &c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul(b, c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul(b, &c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul(&b, c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul(&b, &c) is equivalent for malachite-gmp and malachite-native.
    // (&a).sub_mul(&b, &c) is equivalent for malachite-gmp and malachite-native.
    // a.sub_mul_assign(b, c); a is valid.
    // a.sub_mul_assign(b, &c); a is valid.
    // a.sub_mul_assign(&b, c); a is valid.
    // a.sub_mul_assign(&b, &c); a is valid.
    // a.sub_mul(b, c) is valid.
    // a.sub_mul(b, &c) is valid.
    // a.sub_mul(&b, c) is valid.
    // a.sub_mul(&b, &c) is valid.
    // (&a).sub_mul(&b, &c) is valid.
    // a.sub_mul_assign(b, c), a.sub_mul_assign(b, &c), a.sub_mul_assign(&b, c),
    // a.sub_mul_assign(&b, &c), a.sub_mul(b, c), a.sub_mul(b, &c), a.sub_mul(&b, c),
    //      a.sub_mul(&b, &c), and (&a).sub_mul(&b, &c) give the same result.
    // a.sub_mul(b, c) is equivalent to a - b * c.
    // a.sub_mul(b, c) is equivalent to a.sub_mul(c, b).
    // a.sub_mul(b, c) = a.sub_mul(-b, -c)
    // -(a.sub_mul(b, c)) = (-a).sub_mul(-b, c) = (-a).sub_mul(b, -c)
    let three_integers = |mut gmp_a: gmp::Integer, gmp_b: gmp::Integer, gmp_c: gmp::Integer| {
        let mut a = gmp_integer_to_native(&gmp_a);
        let b = gmp_integer_to_native(&gmp_b);
        let c = gmp_integer_to_native(&gmp_c);
        let old_a = a.clone();
        gmp_a.sub_mul_assign(gmp_b.clone(), gmp_c.clone());
        assert!(gmp_a.is_valid());

        let mut gmp_a_2 = native_integer_to_gmp(&old_a);
        gmp_a_2.sub_mul_assign(gmp_b.clone(), &gmp_c);
        assert!(gmp_a_2.is_valid());
        assert_eq!(gmp_a_2, gmp_a);

        let mut gmp_a_2 = native_integer_to_gmp(&old_a);
        gmp_a_2.sub_mul_assign(&gmp_b, gmp_c.clone());
        assert!(gmp_a_2.is_valid());
        assert_eq!(gmp_a_2, gmp_a);

        let mut gmp_a_2 = native_integer_to_gmp(&old_a);
        gmp_a_2.sub_mul_assign(&gmp_b, &gmp_c);
        assert!(gmp_a_2.is_valid());
        assert_eq!(gmp_a_2, gmp_a);

        a.sub_mul_assign(b.clone(), c.clone());
        assert!(a.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_a), a);

        let mut a2 = old_a.clone();
        a2.sub_mul_assign(b.clone(), &c);
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let mut a2 = old_a.clone();
        a2.sub_mul_assign(&b, c.clone());
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let mut a2 = old_a.clone();
        a2.sub_mul_assign(&b, &c);
        assert!(a2.is_valid());
        assert_eq!(a2, a);

        let gmp_a_2 = native_integer_to_gmp(&old_a);
        let result = gmp_a_2.clone().sub_mul(gmp_b.clone(), gmp_c.clone());
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = gmp_a_2.clone().sub_mul(gmp_b.clone(), &gmp_c);
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = gmp_a_2.clone().sub_mul(&gmp_b, gmp_c.clone());
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = gmp_a_2.clone().sub_mul(&gmp_b, &gmp_c);
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let result = (&gmp_a_2).sub_mul(&gmp_b, &gmp_c);
        assert!(result.is_valid());
        assert_eq!(result, gmp_a);

        let a2 = old_a.clone();
        let result = a2.clone().sub_mul(b.clone(), c.clone());
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().sub_mul(b.clone(), &c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().sub_mul(&b, c.clone());
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = a2.clone().sub_mul(&b, &c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        let result = (&a2).sub_mul(&b, &c);
        assert!(result.is_valid());
        assert_eq!(result, a);

        assert_eq!(&old_a - &b * &c, result);
        assert_eq!((&old_a).sub_mul(&c, &b), result);
        assert_eq!((&old_a).sub_mul(&(-&b), &(-&c)), result);
        assert_eq!((-&old_a).sub_mul(&(-&b), &c), -&result);
        assert_eq!((-old_a).sub_mul(b, -c), -&result);
    };

    // a.sub_mul(a, 1) == 0
    // a.sub_mul(-a, -1) == 0
    let single_integer = |gmp_n: gmp::Integer| {
        let a = &gmp_integer_to_native(&gmp_n);
        assert_eq!(a.sub_mul(a, &native::Integer::from(1u32)), 0);
        assert_eq!(a.sub_mul(&(-a), &native::Integer::from(-1)), 0);
    };

    // a.sub_mul(0, b) == a
    // a.sub_mul(1, b) == a - b
    // 0.sub_mul(a, b) == a * b
    // a.sub_mul(b, 0) == a
    // a.sub_mul(b, 1) == a - b
    // (a * b).sub_mul(a, b) == 0
    // (a * b).sub_mul(-a, -b) == 0
    let two_integers = |gmp_a: gmp::Integer, gmp_b: gmp::Integer| {
        let a = &gmp_integer_to_native(&gmp_a);
        let b = &gmp_integer_to_native(&gmp_b);
        assert_eq!(a.sub_mul(&native::Integer::from(0u32), b), *a);
        assert_eq!(a.sub_mul(&native::Integer::from(1u32), b), a - b);
        assert_eq!(native::Integer::from(0u32).sub_mul(a, b), -a * b);
        assert_eq!(a.sub_mul(b, &native::Integer::from(0u32)), *a);
        assert_eq!(a.sub_mul(b, &native::Integer::from(1u32)), a - b);
        assert_eq!((a * b).sub_mul(a, b), 0);
        assert_eq!((a * b).sub_mul(-a, -b), 0);
    };

    for (a, b, c) in exhaustive_triples_from_single(exhaustive_integers()).take(LARGE_LIMIT) {
        three_integers(a, b, c);
    }

    for (a, b, c) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32))
        .take(LARGE_LIMIT)
    {
        three_integers(a, b, c);
    }

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        single_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        single_integer(n);
    }

    for (a, b) in exhaustive_pairs_from_single(exhaustive_integers()).take(LARGE_LIMIT) {
        two_integers(a, b);
    }

    for (a, b) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(LARGE_LIMIT) {
        two_integers(a, b);
    }
}
