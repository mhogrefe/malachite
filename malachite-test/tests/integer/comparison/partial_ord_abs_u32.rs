use common::LARGE_LIMIT;
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::PartialOrdAbs as gmp_ord_abs;
use malachite_native::integer as native;
use malachite_native::traits::PartialOrdAbs as native_ord_abs;
use malachite_test::common::gmp_integer_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_triples, random_pairs,
                                     random_triples};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_u32_abs() {
    let test = |u, v: u32, out| {
        assert_eq!(native::Integer::from_str(u).unwrap().partial_cmp_abs(&v),
                   out);
        assert_eq!(gmp::Integer::from_str(u).unwrap().partial_cmp_abs(&v), out);

        assert_eq!(native_ord_abs::partial_cmp_abs(&v, &native::Integer::from_str(u).unwrap()),
                   out.map(|o| o.reverse()));
        assert_eq!(gmp_ord_abs::partial_cmp_abs(&v, &gmp::Integer::from_str(u).unwrap()),
                   out.map(|o| o.reverse()));
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Equal));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Greater));
    test("3000000000", 3000000000, Some(Ordering::Equal));
    test("3000000000", 3000000001, Some(Ordering::Less));
    test("3000000000", 2999999999, Some(Ordering::Greater));
}

#[test]
fn partial_cmp_u32_properties() {
    // n.partial_cmp_abs(&u) is equivalent for malachite-gmp and malachite-native.
    // TODO n.partial_cmp_abs(&Integer::from(u)) is equivalent to n.partial_cmp_abs(&u).
    //
    // u.partial_cmp_abs(&n) is equivalent for malachite-gmp and malachite-native.
    // TODO Integer::from(u).partial_cmp_abs(&n) is equivalent to u.partial_cmp_abs(&n).
    // n.lt_abs(u) <=> u.gt_abs(n) and n.gt_abs(u) <=> u.lt_abs(n).
    //
    // n.partial_cmp_abs(&u) == n.abs().partial_cmp(&u.abs())
    // u.partial_cmp_abs(&n) == u.partial_cmp(&n.abs())
    let integer_and_u32 = |gmp_n: gmp::Integer, u: u32| {
        let n = gmp_integer_to_native(&gmp_n);
        let cmp_1 = n.partial_cmp_abs(&u);
        assert_eq!(gmp_n.partial_cmp_abs(&u), cmp_1);
        assert_eq!(n.abs_ref().partial_cmp(&u), cmp_1);

        let cmp_2 = native_ord_abs::partial_cmp_abs(&u, &n);
        assert_eq!(gmp_ord_abs::partial_cmp_abs(&u, &gmp_n), cmp_2);
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!(u.partial_cmp(&n.abs_ref()), cmp_2);
    };

    // n.lt_abs(u) and u.lt_abs(m) => n.lt_abs(m)
    // n.gt_abs(u) and u.gt_abs(m) => n.gt_abs(m)
    let integer_u32_and_integer = |gmp_n: gmp::Integer, u: u32, gmp_m: gmp::Integer| {
        let n = gmp_integer_to_native(&gmp_n);
        let m = gmp_integer_to_native(&gmp_m);
        //TODO
        /*
        if n.lt_abs(&u) && native_ord_abs::lt_abs(&u, &m) {
            assert!(n.lt_abs(&m));
        } else if n.gt_abs(&u) && u.gt_abs(&m) {
            assert!(n.gt_abs(&m));
        }*/

    };

    // u.lt_abs(n) and n.lt_abs(v) => u < v
    // u.gt_abs(n) and n.gt_abs(v) => u > v
    let u32_integer_and_u32 = |u: u32, gmp_n: gmp::Integer, v: u32| {
        let n = gmp_integer_to_native(&gmp_n);
        if native_ord_abs::lt_abs(&u, &n) && n.lt_abs(&v) {
            assert!(u < v);
        } else if native_ord_abs::gt_abs(&u, &n) && n.gt_abs(&v) {
            assert!(u > v);
        }
    };

    for (n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_integers(seed, 32)),
                               &(|seed| random_x::<u32>(seed)))
                .take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u, m) in exhaustive_triples(exhaustive_integers(),
                                        exhaustive_u::<u32>(),
                                        exhaustive_integers())
                .take(LARGE_LIMIT) {
        integer_u32_and_integer(n, u, m);
    }

    for (n, u, m) in random_triples(&EXAMPLE_SEED,
                                    &(|seed| random_integers(seed, 32)),
                                    &(|seed| random_x::<u32>(seed)),
                                    &(|seed| random_integers(seed, 32)))
                .take(LARGE_LIMIT) {
        integer_u32_and_integer(n, u, m);
    }

    for (u, n, v) in exhaustive_triples(exhaustive_u::<u32>(),
                                        exhaustive_integers(),
                                        exhaustive_u::<u32>())
                .take(LARGE_LIMIT) {
        u32_integer_and_u32(u, n, v);
    }

    for (u, n, v) in random_triples(&EXAMPLE_SEED,
                                    &(|seed| random_x::<u32>(seed)),
                                    &(|seed| random_integers(seed, 32)),
                                    &(|seed| random_x::<u32>(seed)))
                .take(LARGE_LIMIT) {
        u32_integer_and_u32(u, n, v);
    }
}
