use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign, ModIsReduced};
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_mod_add() {
    let test = |r, s, t, out| {
        let u = Natural::from_str(r).unwrap();
        let v = Natural::from_str(s).unwrap();
        let m = Natural::from_str(t).unwrap();

        assert!(u.mod_is_reduced(&m));
        assert!(v.mod_is_reduced(&m));

        let mut n = u.clone();
        n.mod_add_assign(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&m));

        let mut n = u.clone();
        n.mod_add_assign(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_add_assign(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_add_assign(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_add(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_add(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_add(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_add(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_add(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_add(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_add(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_add(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(((u + v) % m).to_string(), out);
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
