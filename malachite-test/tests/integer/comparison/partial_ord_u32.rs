use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::integer::comparison::partial_cmp_u32::num_partial_cmp_u32;
use num;
use rugint;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(native::Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(gmp::Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(num_partial_cmp_u32(&num::BigInt::from_str(u).unwrap(), v),
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
    test("1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("3000000000", 3000000000, Some(Ordering::Equal));
    test("3000000000", 3000000001, Some(Ordering::Less));
    test("3000000000", 2999999999, Some(Ordering::Greater));
}
