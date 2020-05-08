use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::num::arithmetic::traits::Sign;
use num::bigint::Sign as NumSign;
use num::BigInt;
use rug;

use malachite_nz::integer::Integer;

fn num_sign(x: &BigInt) -> Ordering {
    match x.sign() {
        NumSign::NoSign => Ordering::Equal,
        NumSign::Plus => Ordering::Greater,
        NumSign::Minus => Ordering::Less,
    }
}

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
