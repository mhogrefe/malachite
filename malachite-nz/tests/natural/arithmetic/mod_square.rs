use malachite_base::num::arithmetic::traits::{ModIsReduced, ModSquare, ModSquareAssign};
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_mod_square() {
    let test = |u, m, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_is_reduced(&Natural::from_str(m).unwrap()));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_square_assign(Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&Natural::from_str(m).unwrap()));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_square_assign(&Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_square(Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_square(Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_square(&Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_square(&Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "1", "0");
    test("1", "10", "1");
    test("2", "10", "4");
    test("100", "497", "60");
    test("200", "497", "240");
    test("300", "497", "43");
    test("1234567890", "123456789876", "100296296172");
}
