use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::test_util::generators::unsigned_gen;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::common::natural_to_rug_integer;
use malachite_nz::test_util::generators::natural_gen;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

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
    natural_gen().test_properties(|n| {
        let sign = n.sign();
        assert_eq!(natural_to_rug_integer(&n).cmp0(), sign);
        assert_ne!(sign, Ordering::Less);
        assert_eq!(n.partial_cmp(&0), Some(sign));
        assert_eq!((-n).sign(), sign.reverse());
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(Natural::from(u).sign(), u.sign());
    });
}
