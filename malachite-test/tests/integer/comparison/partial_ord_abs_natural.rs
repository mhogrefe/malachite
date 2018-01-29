use common::LARGE_LIMIT;
use malachite_base::traits::{OrdAbs, PartialOrdAbs};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rugint_integer, natural_to_rugint_integer, GenerationMode};
use malachite_test::inputs::integer::{pairs_of_natural_and_integer,
                                      triples_of_integer_natural_and_integer,
                                      triples_of_natural_integer_and_natural};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_integer_natural() {
    let test = |u, v, out| {
        assert_eq!(
            Integer::from_str(u)
                .unwrap()
                .partial_cmp_abs(&Natural::from_str(v).unwrap()),
            out
        );

        assert_eq!(
            Natural::from_str(v)
                .unwrap()
                .partial_cmp_abs(&Integer::from_str(u).unwrap())
                .map(|o| o.reverse()),
            out
        );
    };
    test("0", "0", Some(Ordering::Equal));
    test("0", "5", Some(Ordering::Less));
    test("123", "123", Some(Ordering::Equal));
    test("123", "124", Some(Ordering::Less));
    test("123", "122", Some(Ordering::Greater));
    test("1000000000000", "123", Some(Ordering::Greater));
    test("123", "1000000000000", Some(Ordering::Less));
    test("1000000000000", "1000000000000", Some(Ordering::Equal));
    test("-1000000000000", "1000000000000", Some(Ordering::Equal));
    test("-1000000000000", "0", Some(Ordering::Greater));
}

#[test]
fn partial_cmp_integer_natural_properties() {
    // x.partial_cmp_abs(&y) is equivalent for malachite and rugint.
    // x.into_integer().partial_cmp_abs(&y) is equivalent to x.partial_cmp_abs(&y).
    // x.lt_abs(&y) <=> y.gt_abs(&x), x.gt_abs(&y) <=> y.lt_abs(&x), and x == y <=> y == x.
    let natural_and_integer = |x: Natural, y: Integer| {
        let cmp_1 = x.partial_cmp_abs(&y);
        assert_eq!(
            Some(natural_to_rugint_integer(&x).cmp_abs(&integer_to_rugint_integer(&y),)),
            cmp_1
        );
        assert_eq!(x.to_integer().cmp_abs(&y), cmp_1.unwrap());

        let cmp_2 = y.partial_cmp_abs(&x);
        assert_eq!(
            Some(integer_to_rugint_integer(&y).cmp_abs(&natural_to_rugint_integer(&x))),
            cmp_2
        );
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!(y.cmp_abs(&x.into_integer()), cmp_2.unwrap());
    };

    // x.lt_abs(&y) and y.lt_abs(&z) => x < z
    // x.gt_abs(&y) and y.gt_abs(&z) => x > z
    let natural_integer_and_natural = |x: Natural, y: Integer, z: Natural| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x < z);
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x > z);
        }
    };

    // y.lt_abs(&x) and x.lt_abs(&z) => y.lt_abs(&z)
    // y.gt_abs(&x) and x.gt_abs(&z) => y.gt_abs(&z)
    let integer_natural_and_integer = |x: Integer, y: Natural, z: Integer| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    };

    for (x, y) in pairs_of_natural_and_integer(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_integer(x, y);
    }

    for (x, y) in pairs_of_natural_and_integer(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        natural_and_integer(x, y);
    }

    for (x, y, z) in
        triples_of_natural_integer_and_natural(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        natural_integer_and_natural(x, y, z);
    }

    for (x, y, z) in
        triples_of_natural_integer_and_natural(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        natural_integer_and_natural(x, y, z);
    }

    for (x, y, z) in
        triples_of_integer_natural_and_integer(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        integer_natural_and_integer(x, y, z);
    }

    for (x, y, z) in
        triples_of_integer_natural_and_integer(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        integer_natural_and_integer(x, y, z);
    }
}
