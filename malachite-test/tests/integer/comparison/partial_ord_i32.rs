use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_num_bigint,
                             native_integer_to_rugint};
use malachite_test::integer::comparison::partial_ord_i32::num_partial_cmp_i32;
use num;
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_triples, random_pairs,
                                     random_triples};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(native::Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(gmp::Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(num_partial_cmp_i32(&num::BigInt::from_str(u).unwrap(), v),
                   out);
        assert_eq!(rugint::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(v.partial_cmp(&native::Integer::from_str(u).unwrap()),
                   out.map(|o| o.reverse()));
        assert_eq!(v.partial_cmp(&gmp::Integer::from_str(u).unwrap()),
                   out.map(|o| o.reverse()));
        assert_eq!(v.partial_cmp(&rugint::Integer::from_str(u).unwrap()),
                   out.map(|o| o.reverse()));
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Less));
    test("-123", -123, Some(Ordering::Equal));
    test("-123", -122, Some(Ordering::Less));
    test("-123", -124, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("-1000000000000", -123, Some(Ordering::Less));
}

#[test]
fn partial_cmp_i32_properties() {
    // n.partial_cmp(&i) is equivalent for malachite-gmp, malachite-native, num, and rugint.
    // n.partial_cmp(&Integer::from(i)) is equivalent to n.partial_cmp(&u).
    //
    // i.partial_cmp(&n) is equivalent for malachite-gmp, malachite-native, and rugint.
    // Integer::from(i).partial_cmp(&n) is equivalent to i.partial_cmp(&n).
    // n < i <=> i > n, n > i <=> i < n, and n == i <=> i == n.
    let integer_and_i32 = |gmp_n: gmp::Integer, i: i32| {
        let n = gmp_integer_to_native(&gmp_n);
        let cmp_1 = n.partial_cmp(&i);
        assert_eq!(gmp_n.partial_cmp(&i), cmp_1);
        assert_eq!(num_partial_cmp_i32(&native_integer_to_num_bigint(&n), i),
                   cmp_1);
        assert_eq!(native_integer_to_rugint(&n).partial_cmp(&i), cmp_1);
        assert_eq!(n.partial_cmp(&native::Integer::from(i)), cmp_1);

        let cmp_2 = i.partial_cmp(&n);
        assert_eq!(i.partial_cmp(&gmp_n), cmp_2);
        assert_eq!(i.partial_cmp(&native_integer_to_rugint(&n)), cmp_2);
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!(native::Integer::from(i).partial_cmp(&n), cmp_2);
    };

    // n < i and i < m => n < m
    // n > i and i > m => n > m
    let integer_i32_and_integer = |gmp_n: gmp::Integer, i: i32, gmp_m: gmp::Integer| {
        let n = gmp_integer_to_native(&gmp_n);
        let m = gmp_integer_to_native(&gmp_m);
        if n < i && i < m {
            assert!(n < m);
        } else if n > i && i > m {
            assert!(n > m);
        }
    };

    // i < n and n < v => i < v
    // i > n and n > v => i > v
    let i32_integer_and_i32 = |i: i32, gmp_n: gmp::Integer, j: i32| {
        let n = gmp_integer_to_native(&gmp_n);
        if i < n && n < j {
            assert!(i < j);
        } else if i > n && n > j {
            assert!(i > j);
        }
    };

    for (n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_integers(seed, 32)),
                               &(|seed| random_x::<i32>(seed)))
                .take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i, m) in exhaustive_triples(exhaustive_integers(),
                                        exhaustive_i::<i32>(),
                                        exhaustive_integers())
                .take(LARGE_LIMIT) {
        integer_i32_and_integer(n, i, m);
    }

    for (n, i, m) in random_triples(&EXAMPLE_SEED,
                                    &(|seed| random_integers(seed, 32)),
                                    &(|seed| random_x::<i32>(seed)),
                                    &(|seed| random_integers(seed, 32)))
                .take(LARGE_LIMIT) {
        integer_i32_and_integer(n, i, m);
    }

    for (i, n, j) in exhaustive_triples(exhaustive_i::<i32>(),
                                        exhaustive_integers(),
                                        exhaustive_i::<i32>())
                .take(LARGE_LIMIT) {
        i32_integer_and_i32(i, n, j);
    }

    for (i, n, j) in random_triples(&EXAMPLE_SEED,
                                    &(|seed| random_x::<i32>(seed)),
                                    &(|seed| random_integers(seed, 32)),
                                    &(|seed| random_x::<i32>(seed)))
                .take(LARGE_LIMIT) {
        i32_integer_and_i32(i, n, j);
    }
}
