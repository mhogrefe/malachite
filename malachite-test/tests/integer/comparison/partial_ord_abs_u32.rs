use common::LARGE_LIMIT;
use malachite_base::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::integer::{pairs_of_integer_and_unsigned,
                                      triples_of_integer_unsigned_and_integer,
                                      triples_of_unsigned_integer_and_unsigned};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_u32_abs() {
    let test = |u, v: u32, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp_abs(&v), out);

        assert_eq!(
            PartialOrdAbs::partial_cmp_abs(&v, &Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Equal));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Greater));
    test("3000000000", 3_000_000_000, Some(Ordering::Equal));
    test("3000000000", 3_000_000_001, Some(Ordering::Less));
    test("3000000000", 2_999_999_999, Some(Ordering::Greater));
}

#[test]
fn partial_cmp_u32_properties() {
    // n.partial_cmp_abs(&Integer::from(u)) is equivalent to n.partial_cmp_abs(&u).
    // n.partial_cmp_abs(&u) == n.abs().partial_cmp(&u.abs())
    //
    // Integer::from(u).partial_cmp_abs(&n) is equivalent to u.partial_cmp_abs(&n).
    // u.partial_cmp_abs(&n) == u.partial_cmp(&n.abs())
    //
    // n.lt_abs(u) <=> u.gt_abs(n) and n.gt_abs(u) <=> u.lt_abs(n).
    let integer_and_u32 = |n: Integer, u: u32| {
        let cmp_1 = n.partial_cmp_abs(&u);
        assert_eq!(n.partial_cmp_abs(&Integer::from(u)), cmp_1);
        assert_eq!(n.abs_ref().partial_cmp(&u), cmp_1);

        let cmp_2 = PartialOrdAbs::partial_cmp_abs(&u, &n);
        assert_eq!(Integer::from(u).partial_cmp_abs(&n), cmp_2);
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!(u.partial_cmp(&n.abs_ref()), cmp_2);
    };

    // n.lt_abs(u) and u.lt_abs(m) => n.lt_abs(m)
    // n.gt_abs(u) and u.gt_abs(m) => n.gt_abs(m)
    let integer_u32_and_integer = |n: Integer, u: u32, m: Integer| {
        if n.lt_abs(&u) && PartialOrdAbs::lt_abs(&u, &m) {
            assert!(n.lt_abs(&m));
        } else if n.gt_abs(&u) && PartialOrdAbs::gt_abs(&u, &m) {
            assert!(n.gt_abs(&m));
        }
    };

    // u.lt_abs(n) and n.lt_abs(v) => u < v
    // u.gt_abs(n) and n.gt_abs(v) => u > v
    let u32_integer_and_u32 = |u: u32, n: Integer, v: u32| {
        if PartialOrdAbs::lt_abs(&u, &n) && n.lt_abs(&v) {
            assert!(u < v);
        } else if PartialOrdAbs::gt_abs(&u, &n) && n.gt_abs(&v) {
            assert!(u > v);
        }
    };

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u, m) in
        triples_of_integer_unsigned_and_integer(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_u32_and_integer(n, u, m);
    }

    for (n, u, m) in
        triples_of_integer_unsigned_and_integer(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_u32_and_integer(n, u, m);
    }

    for (u, n, v) in
        triples_of_unsigned_integer_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        u32_integer_and_u32(u, n, v);
    }

    for (u, n, v) in
        triples_of_unsigned_integer_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        u32_integer_and_u32(u, n, v);
    }
}
