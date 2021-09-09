use malachite_base::num::arithmetic::traits::{ModIsReduced, ModNeg, ModNegAssign};
use malachite_nz::natural::Natural;
use std::str::FromStr;

//TODO clean from_str

#[test]
fn test_mod_neg() {
    let test = |u, v, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_is_reduced(&Natural::from_str(v).unwrap()));
        let n = Natural::from_str(u)
            .unwrap()
            .mod_neg(Natural::from_str(v).unwrap());
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&Natural::from_str(v).unwrap()));
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u)
            .unwrap()
            .mod_neg(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_neg(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_neg(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.mod_neg_assign(Natural::from_str(v).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.mod_neg_assign(&Natural::from_str(v).unwrap());
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
