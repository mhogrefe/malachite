use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_num_bigint,
                             native_integer_to_rugint, GenerationMode};
use malachite_test::integer::comparison::partial_ord_u32::{num_partial_cmp_u32, select_inputs_1};
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_triples, random_triples};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(native::Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(gmp::Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_u32(&num::BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rugint::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&native::Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
        assert_eq!(
            v.partial_cmp(&gmp::Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
        assert_eq!(
            v.partial_cmp(&rugint::Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("3000000000", 3000000000, Some(Ordering::Equal));
    test("3000000000", 3000000001, Some(Ordering::Less));
    test("3000000000", 2999999999, Some(Ordering::Greater));
}

#[test]
fn partial_cmp_u32_properties() {
    // n.partial_cmp(&u) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // n.partial_cmp(&Integer::from(u)) is equivalent to n.partial_cmp(&u).
    //
    // u.partial_cmp(&n) is equivalent for malachite-gmp, malachite-native, and rugint.
    // Integer::from(u).partial_cmp(&n) is equivalent to u.partial_cmp(&n).
    // n < u <=> u > n, n > u <=> u < n, and n == u <=> u == n.
    let integer_and_u32 = |gmp_n: gmp::Integer, u: u32| {
        let n = gmp_integer_to_native(&gmp_n);
        let cmp_1 = n.partial_cmp(&u);
        assert_eq!(gmp_n.partial_cmp(&u), cmp_1);
        assert_eq!(
            num_partial_cmp_u32(&native_integer_to_num_bigint(&n), u),
            cmp_1
        );
        assert_eq!(native_integer_to_rugint(&n).partial_cmp(&u), cmp_1);
        assert_eq!(n.partial_cmp(&native::Integer::from(u)), cmp_1);

        let cmp_2 = u.partial_cmp(&n);
        assert_eq!(u.partial_cmp(&gmp_n), cmp_2);
        assert_eq!(u.partial_cmp(&native_integer_to_rugint(&n)), cmp_2);
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!(native::Integer::from(u).partial_cmp(&n), cmp_2);
    };

    // n < u and u < m => n < m
    // n > u and u > m => n > m
    let integer_u32_and_integer = |gmp_n: gmp::Integer, u: u32, gmp_m: gmp::Integer| {
        let n = gmp_integer_to_native(&gmp_n);
        let m = gmp_integer_to_native(&gmp_m);
        if n < u && u < m {
            assert!(n < m);
        } else if n > u && u > m {
            assert!(n > m);
        }
    };

    // u < n and n < v => u < v
    // u > n and n > v => u > v
    let u32_integer_and_u32 = |u: u32, gmp_n: gmp::Integer, v: u32| {
        let n = gmp_integer_to_native(&gmp_n);
        if u < n && n < v {
            assert!(u < v);
        } else if u > n && n > v {
            assert!(u > v);
        }
    };

    for (n, u) in select_inputs_1(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in select_inputs_1(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u, m) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_u::<u32>(),
        exhaustive_integers(),
    ).take(LARGE_LIMIT)
    {
        integer_u32_and_integer(n, u, m);
    }

    for (n, u, m) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        integer_u32_and_integer(n, u, m);
    }

    for (u, n, v) in exhaustive_triples(
        exhaustive_u::<u32>(),
        exhaustive_integers(),
        exhaustive_u::<u32>(),
    ).take(LARGE_LIMIT)
    {
        u32_integer_and_u32(u, n, v);
    }

    for (u, n, v) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_x::<u32>(seed)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(LARGE_LIMIT)
    {
        u32_integer_and_u32(u, n, v);
    }
}
