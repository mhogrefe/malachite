use common::LARGE_LIMIT;
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::PartialOrdAbs as gmp_ord_abs;
use malachite_native::integer as native;
use malachite_native::traits::PartialOrdAbs as native_ord_abs;
use malachite_test::common::gmp_integer_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_triples, random_pairs,
                                     random_triples};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_i32_abs() {
    let test = |u, v: i32, out| {
        assert_eq!(
            native::Integer::from_str(u).unwrap().partial_cmp_abs(&v),
            out
        );
        assert_eq!(gmp::Integer::from_str(u).unwrap().partial_cmp_abs(&v), out);

        assert_eq!(
            native_ord_abs::partial_cmp_abs(&v, &native::Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
        assert_eq!(
            gmp_ord_abs::partial_cmp_abs(&v, &gmp::Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Equal));
    test("-123", -123, Some(Ordering::Equal));
    test("-123", -122, Some(Ordering::Greater));
    test("-123", -124, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", -123, Some(Ordering::Greater));
}

#[test]
fn partial_cmp_i32_properties() {
    // n.partial_cmp_abs(&i) is equivalent for malachite-gmp and malachite-native.
    // n.partial_cmp_abs(&Integer::from(i)) is equivalent to n.partial_cmp_abs(&i).
    // n.partial_cmp_abs(&i) == n.abs().partial_cmp(&i.abs())
    //
    // i.partial_cmp_abs(&n) is equivalent for malachite-gmp and malachite-native.
    // Integer::from(i).partial_cmp_abs(&n) is equivalent to i.partial_cmp_abs(&n).
    // i.partial_cmp_abs(&n) == i.abs().partial_cmp(&n.abs())
    //
    // n.lt_abs(u) <=> u.gt_abs(n) and n.gt_abs(u) <=> u.lt_abs(n).
    let integer_and_i32 = |gmp_n: gmp::Integer, i: i32| {
        let n = gmp_integer_to_native(&gmp_n);
        let cmp_1 = n.partial_cmp_abs(&i);
        assert_eq!(gmp_n.partial_cmp_abs(&i), cmp_1);
        assert_eq!(n.partial_cmp_abs(&native::Integer::from(i)), cmp_1);
        assert_eq!(n.abs_ref().partial_cmp(&(i.abs() as u32)), cmp_1);

        let cmp_2 = native_ord_abs::partial_cmp_abs(&i, &n);
        assert_eq!(gmp_ord_abs::partial_cmp_abs(&i, &gmp_n), cmp_2);
        assert_eq!(native::Integer::from(i).partial_cmp_abs(&n), cmp_2);
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!((i.abs() as u32).partial_cmp(&n.abs_ref()), cmp_2);
    };

    // n.lt_abs(i) and i.lt_abs(m) => n.lt_abs(m)
    // n.gt_abs(i) and i.gt_abs(m) => n.gt_abs(m)
    let integer_i32_and_integer = |gmp_n: gmp::Integer, i: i32, gmp_m: gmp::Integer| {
        let n = gmp_integer_to_native(&gmp_n);
        let m = gmp_integer_to_native(&gmp_m);
        if n.lt_abs(&i) && native_ord_abs::lt_abs(&i, &m) {
            assert!(n.lt_abs(&m));
        } else if n.gt_abs(&i) && native_ord_abs::gt_abs(&i, &m) {
            assert!(n.gt_abs(&m));
        }
    };

    // i.lt_abs(n) and n.lt_abs(j) => i < j
    // i.gt_abs(n) and n.gt_abs(j) => i > j
    let i32_integer_and_i32 = |i: i32, gmp_n: gmp::Integer, j: i32| {
        let n = gmp_integer_to_native(&gmp_n);
        if native_ord_abs::lt_abs(&i, &n) && n.lt_abs(&j) {
            assert!((i.abs() as u32) < (j.abs() as u32));
        } else if native_ord_abs::gt_abs(&i, &n) && n.gt_abs(&j) {
            assert!((i.abs() as u32) > (j.abs() as u32));
        }
    };

    for (n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(LARGE_LIMIT)
    {
        integer_and_i32(n, i);
    }

    for (n, i, m) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_i::<i32>(),
        exhaustive_integers(),
    ).take(LARGE_LIMIT)
    {
        integer_i32_and_integer(n, i, m);
    }

    for (n, i, m) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(LARGE_LIMIT)
    {
        integer_i32_and_integer(n, i, m);
    }

    for (i, n, j) in exhaustive_triples(
        exhaustive_i::<i32>(),
        exhaustive_integers(),
        exhaustive_i::<i32>(),
    ).take(LARGE_LIMIT)
    {
        i32_integer_and_i32(i, n, j);
    }

    for (i, n, j) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_x::<i32>(seed)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(LARGE_LIMIT)
    {
        i32_integer_and_i32(i, n, j);
    }
}
