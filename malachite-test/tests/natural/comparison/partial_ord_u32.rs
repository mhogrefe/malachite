use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::natural::comparison::partial_cmp_u32::num_partial_cmp_u32;
use num;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(native::Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(gmp::Natural::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(num_partial_cmp_u32(&num::BigUint::from_str(u).unwrap(), v),
                   out);

        assert_eq!(v.partial_cmp(&native::Natural::from_str(u).unwrap()),
                   out.map(|o| o.reverse()));
        assert_eq!(v.partial_cmp(&gmp::Natural::from_str(u).unwrap()),
                   out.map(|o| o.reverse()));
    };
    test("0", 0, Some(Ordering::Equal));
    test("0", 5, Some(Ordering::Less));
    test("123", 123, Some(Ordering::Equal));
    test("123", 124, Some(Ordering::Less));
    test("123", 122, Some(Ordering::Greater));
    test("1000000000000", 123, Some(Ordering::Greater));
}
