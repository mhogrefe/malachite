use std::cmp::Ordering;
use std::str::FromStr;

use malachite_base::num::arithmetic::traits::Sign;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use malachite_test::common::{natural_to_rug_integer, test_properties};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::naturals;

#[test]
fn test_sign() {
    let test = |s, out| {
        assert_eq!(Natural::from_str(s).unwrap().sign(), out);
        assert_eq!(rug::Integer::from_str(s).unwrap().cmp0(), out);
    };
    test("0", Ordering::Equal);
    test("123", Ordering::Greater);
    test("1000000000000", Ordering::Greater);
}

#[test]
fn sign_properties() {
    test_properties(naturals, |n| {
        let sign = n.sign();
        assert_eq!(natural_to_rug_integer(n).cmp0(), sign);
        assert_ne!(sign, Ordering::Less);
        assert_eq!(n.partial_cmp(&0), Some(sign));
        assert_eq!((-n).sign(), sign.reverse());
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(Natural::from(u).sign(), u.sign());
    });
}
