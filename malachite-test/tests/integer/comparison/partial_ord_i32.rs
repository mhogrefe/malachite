use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use malachite_test::inputs::integer::{pairs_of_integer_and_signed,
                                      triples_of_integer_signed_and_integer,
                                      triples_of_signed_integer_and_signed};
use malachite_test::integer::comparison::partial_ord_i32::num_partial_cmp_i32;
use num::BigInt;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(num_partial_cmp_i32(&BigInt::from_str(u).unwrap(), v), out);
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
        assert_eq!(
            v.partial_cmp(&rug::Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
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
    // n.partial_cmp(&i) is equivalent for malachite, num, and rug.
    // n.partial_cmp(&Integer::from(i)) is equivalent to n.partial_cmp(&u).
    //
    // i.partial_cmp(&n) is equivalent for malachite and rug.
    // Integer::from(i).partial_cmp(&n) is equivalent to i.partial_cmp(&n).
    // n < i <=> i > n, n > i <=> i < n, and n == i <=> i == n.
    let integer_and_i32 = |n: Integer, i: i32| {
        let cmp_1 = n.partial_cmp(&i);
        assert_eq!(num_partial_cmp_i32(&integer_to_bigint(&n), i), cmp_1);
        assert_eq!(integer_to_rug_integer(&n).partial_cmp(&i), cmp_1);
        assert_eq!(n.partial_cmp(&Integer::from(i)), cmp_1);

        let cmp_2 = i.partial_cmp(&n);
        assert_eq!(i.partial_cmp(&integer_to_rug_integer(&n)), cmp_2);
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!(Integer::from(i).partial_cmp(&n), cmp_2);
    };

    // n < i and i < m => n < m
    // n > i and i > m => n > m
    let integer_i32_and_integer = |n: Integer, i: i32, m: Integer| {
        if n < i && i < m {
            assert!(n < m);
        } else if n > i && i > m {
            assert!(n > m);
        }
    };

    // i < n and n < v => i < v
    // i > n and n > v => i > v
    let i32_integer_and_i32 = |i: i32, n: Integer, j: i32| {
        if i < n && n < j {
            assert!(i < j);
        } else if i > n && n > j {
            assert!(i > j);
        }
    };

    for (n, i) in pairs_of_integer_and_signed(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i) in pairs_of_integer_and_signed(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_i32(n, i);
    }

    for (n, i, m) in
        triples_of_integer_signed_and_integer(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_i32_and_integer(n, i, m);
    }

    for (n, i, m) in
        triples_of_integer_signed_and_integer(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_i32_and_integer(n, i, m);
    }

    for (i, n, j) in
        triples_of_signed_integer_and_signed(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        i32_integer_and_i32(i, n, j);
    }

    for (i, n, j) in
        triples_of_signed_integer_and_signed(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        i32_integer_and_i32(i, n, j);
    }
}
