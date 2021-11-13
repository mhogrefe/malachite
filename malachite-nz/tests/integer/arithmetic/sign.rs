use malachite_base::num::arithmetic::traits::Sign;
use malachite_base_test_util::generators::signed_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_nz_test_util::generators::integer_gen;
use malachite_nz_test_util::integer::arithmetic::sign::num_sign;
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
    integer_gen().test_properties(|n| {
        let sign = n.sign();
        assert_eq!(integer_to_rug_integer(&n).cmp0(), sign);
        assert_eq!(num_sign(&integer_to_bigint(&n)), sign);
        assert_eq!(n.partial_cmp(&0), Some(sign));
        assert_eq!((-n).sign(), sign.reverse());
    });

    signed_gen::<SignedLimb>().test_properties(|i| {
        assert_eq!(Integer::from(i).sign(), i.sign());
    });
}
