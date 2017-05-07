use malachite_native as native;
use malachite_gmp as gmp;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_natural() {
    let test = |u, v, out| {
        assert_eq!(native::integer::Integer::from_str(u)
                       .unwrap()
                       .partial_cmp(&native::natural::Natural::from_str(v).unwrap()),
                   out);
        assert_eq!(gmp::integer::Integer::from_str(u)
                       .unwrap()
                       .partial_cmp(&gmp::natural::Natural::from_str(v).unwrap()),
                   out);
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
