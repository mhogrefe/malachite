use std::cmp::Ordering;
use std::str::FromStr;

use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

#[test]
fn test_partial_ord_integer_natural() {
    let test = |u, v, out| {
        assert_eq!(
            Integer::from_str(u)
                .unwrap()
                .partial_cmp(&Natural::from_str(v).unwrap()),
            out
        );

        assert_eq!(
            Natural::from_str(v)
                .unwrap()
                .partial_cmp(&Integer::from_str(u).unwrap())
                .map(Ordering::reverse),
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
