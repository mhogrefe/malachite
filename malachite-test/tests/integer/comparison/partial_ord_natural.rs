use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rug_integer, natural_to_rug_integer, GenerationMode};
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
                .partial_cmp(&Natural::from_str(v).unwrap(),),
            out
        );

        assert_eq!(
            Natural::from_str(v)
                .unwrap()
                .partial_cmp(&Integer::from_str(u).unwrap())
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
    test("-1000000000000", "1000000000000", Some(Ordering::Less));
    test("-1000000000000", "0", Some(Ordering::Less));
}

#[test]
fn partial_cmp_integer_natural_properties() {
    // x.partial_cmp(&y) is equivalent for malachite and rug.
    // x.into_integer().partial_cmp(&y) is equivalent to x.partial_cmp(&y).
    // x < y <=> y > x, x > y <=> y < x, and x == y <=> y == x.
    let natural_and_integer = |x: Natural, y: Integer| {
        let cmp_1 = x.partial_cmp(&y);
        assert_eq!(
            natural_to_rug_integer(&x).partial_cmp(&integer_to_rug_integer(&y)),
            cmp_1
        );
        assert_eq!(x.to_integer().cmp(&y), cmp_1.unwrap());

        let cmp_2 = y.partial_cmp(&x);
        assert_eq!(
            integer_to_rug_integer(&y).partial_cmp(&natural_to_rug_integer(&x)),
            cmp_2
        );
        assert_eq!(cmp_2, cmp_1.map(|o| o.reverse()));
        assert_eq!(y.cmp(&x.into_integer()), cmp_2.unwrap());
    };

    // x < y and y < z => x < z
    // x > y and y > z => x > z
    let natural_integer_and_natural = |x: Natural, y: Integer, z: Natural| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    };

    // y < x and x < z => y < z
    // y > x and x > z => y > z
    let integer_natural_and_integer = |x: Integer, y: Natural, z: Integer| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
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
