use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{ModIsReduced, ModSub, ModSubAssign};

use malachite_nz::natural::Natural;

#[test]
fn test_mod_sub() {
    let test = |u, v, m, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_is_reduced(&Natural::from_str(m).unwrap()));
        assert!(Natural::from_str(v)
            .unwrap()
            .mod_is_reduced(&Natural::from_str(m).unwrap()));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_sub_assign(Natural::from_str(v).unwrap(), Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&Natural::from_str(m).unwrap()));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_sub_assign(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.mod_sub_assign(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.mod_sub_assign(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_sub(Natural::from_str(v).unwrap(), Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_sub(Natural::from_str(v).unwrap(), Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_sub(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_sub(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_sub(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_sub(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_sub(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_sub(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "1", "0");
    test("0", "0", "32", "0");
    test("0", "27", "32", "5");
    test("10", "2", "16", "8");
    test("2", "10", "16", "8");
    test("0", "5", "128", "123");
    test("123", "0", "128", "123");
    test("123", "56", "512", "67");
    test("56", "123", "512", "445");
    test("7", "9", "10", "8");
}
