use std::cmp::Ordering;
use std::str::FromStr;

use num::BigInt;
use rug;

use malachite_nz::integer::Integer;

fn num_partial_cmp_primitive<T>(x: &BigInt, u: T) -> Option<Ordering>
where
    BigInt: From<T>,
{
    x.partial_cmp(&BigInt::from(u))
}

#[test]
fn test_partial_cmp_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_primitive(&BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("-123", 123, Some(Ordering::Less));
    test("123", 124, Some(Ordering::Less));
    test("-123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 122, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
}

#[test]
fn test_partial_cmp_u64() {
    let test = |u, v: u64, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_primitive(&BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("-123", 123, Some(Ordering::Less));
    test("123", 124, Some(Ordering::Less));
    test("-123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("-123", 122, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("1000000000000", 1000000000000, Some(Ordering::Equal));
    test("-1000000000000", 1000000000000, Some(Ordering::Less));
    test("1000000000000", 1000000000001, Some(Ordering::Less));
    test("-1000000000000", 1000000000001, Some(Ordering::Less));
}

#[test]
fn test_partial_cmp_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_primitive(&BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("0", -5, Some(Ordering::Greater));
    test("123", 123, Some(Ordering::Equal));
    test("123", -123, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Less));
    test("-123", -123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", -124, Some(Ordering::Greater));
    test("-123", 124, Some(Ordering::Less));
    test("-123", -124, Some(Ordering::Greater));
    test("123", 122, Some(Ordering::Greater));
    test("123", -122, Some(Ordering::Greater));
    test("-123", 122, Some(Ordering::Less));
    test("-123", -122, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("-1000000000000", -123, Some(Ordering::Less));
}

#[test]
fn test_partial_cmp_i64() {
    let test = |u, v: i64, out| {
        assert_eq!(Integer::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(
            num_partial_cmp_primitive(&BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(
            v.partial_cmp(&Integer::from_str(u).unwrap()),
            out.map(|o| o.reverse())
        );
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("0", -5, Some(Ordering::Greater));
    test("123", 123, Some(Ordering::Equal));
    test("123", -123, Some(Ordering::Greater));
    test("-123", 123, Some(Ordering::Less));
    test("-123", -123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", -124, Some(Ordering::Greater));
    test("-123", 124, Some(Ordering::Less));
    test("-123", -124, Some(Ordering::Greater));
    test("123", 122, Some(Ordering::Greater));
    test("123", -122, Some(Ordering::Greater));
    test("-123", 122, Some(Ordering::Less));
    test("-123", -122, Some(Ordering::Less));
    test("1000000000000", 123, Some(Ordering::Greater));
    test("1000000000000", -123, Some(Ordering::Greater));
    test("-1000000000000", 123, Some(Ordering::Less));
    test("-1000000000000", -123, Some(Ordering::Less));
    test("1000000000000", 1000000000000, Some(Ordering::Equal));
    test("1000000000000", -1000000000000, Some(Ordering::Greater));
    test("-1000000000000", 1000000000000, Some(Ordering::Less));
    test("-1000000000000", -1000000000000, Some(Ordering::Equal));
    test("1000000000000", 1000000000001, Some(Ordering::Less));
    test("1000000000000", -1000000000001, Some(Ordering::Greater));
    test("-1000000000000", 1000000000001, Some(Ordering::Less));
    test("-1000000000000", -1000000000001, Some(Ordering::Greater));
}
