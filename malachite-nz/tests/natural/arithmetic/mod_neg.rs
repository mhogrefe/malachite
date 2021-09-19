use malachite_base::num::arithmetic::traits::{ModIsReduced, ModNeg, ModNegAssign};
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_mod_neg() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert!(u.mod_is_reduced(&v));
        let n = u.clone().mod_neg(v.clone());
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&v));
        assert_eq!(n.to_string(), out);

        let n = u.clone().mod_neg(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_neg(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_neg(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_neg_assign(v.clone());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u;
        n.mod_neg_assign(&v);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", "5", "0");
    test("7", "10", "3");
    test("100", "101", "1");
    test("4294967294", "4294967295", "1");
    test("1", "4294967295", "4294967294");
    test("7", "1000000000000", "999999999993");
    test("999999999993", "1000000000000", "7");
}
