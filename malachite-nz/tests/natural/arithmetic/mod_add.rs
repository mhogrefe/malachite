use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign, ModIsReduced};
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_mod_add() {
    let test = |u, v, m, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_is_reduced(&Natural::from_str(m).unwrap()));
        assert!(Natural::from_str(v)
            .unwrap()
            .mod_is_reduced(&Natural::from_str(m).unwrap()));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_add_assign(Natural::from_str(v).unwrap(), Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&Natural::from_str(m).unwrap()));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_add_assign(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.mod_add_assign(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.mod_add_assign(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_add(Natural::from_str(v).unwrap(), Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_add(Natural::from_str(v).unwrap(), Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_add(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_add(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_add(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_add(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_add(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_add(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            ((Natural::from_str(u).unwrap() + Natural::from_str(v).unwrap())
                % Natural::from_str(m).unwrap())
            .to_string(),
            out
        );
    };
    test("0", "0", "1", "0");
    test("0", "0", "32", "0");
    test("0", "2", "32", "2");
    test("10", "14", "16", "8");
    test("0", "123", "128", "123");
    test("123", "0", "128", "123");
    test("123", "456", "512", "67");
    test("0", "3", "5", "3");
    test("7", "5", "10", "2");
}
