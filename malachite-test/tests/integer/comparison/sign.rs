use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::integer::integers;
use malachite_test::integer::comparison::sign::num_sign;
use num::BigInt;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_sign() {
    let test = |s, out| {
        assert_eq!(Integer::from_str(s).unwrap().sign(), out);
        assert_eq!(num_sign(&BigInt::from_str(s).unwrap()), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().cmp0(), out);
    };
    test("0", Ordering::Equal);
    test("123", Ordering::Greater);
    test("-123", Ordering::Less);
    test("1000000000000", Ordering::Greater);
    test("-1000000000000", Ordering::Less);
}

#[test]
fn sign_properties() {
    // n.sign() == n.partial_cmp(&0)
    // (-n).sign() == n.sign().reverse()
    let one_integer = |n: Integer| {
        let sign = n.sign();
        assert_eq!(n.partial_cmp(&0), Some(sign));
        assert_eq!((-n).sign(), sign.reverse());
    };

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
