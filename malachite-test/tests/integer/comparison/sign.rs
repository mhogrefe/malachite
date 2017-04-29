use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::integer::comparison::sign::num_sign;
use num;
use rugint;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_sign() {
    let test = |s, out| {
        assert_eq!(native::Integer::from_str(s).unwrap().sign(), out);

        assert_eq!(gmp::Integer::from_str(s).unwrap().sign(), out);

        assert_eq!(num_sign(&num::BigInt::from_str(s).unwrap()), out);

        assert_eq!(rugint::Integer::from_str(s).unwrap().sign(), out);
    };
    test("0", Ordering::Equal);
    test("123", Ordering::Greater);
    test("-123", Ordering::Less);
    test("1000000000000", Ordering::Greater);
    test("-1000000000000", Ordering::Less);
}
