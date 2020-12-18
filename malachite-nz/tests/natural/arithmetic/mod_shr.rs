use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{ModIsReduced, ModShr, ModShrAssign};

use malachite_nz::natural::Natural;

macro_rules! test_mod_shr_signed {
    ($t:ident) => {
        let test = |u, v: $t, m, out| {
            let mut n = Natural::from_str(u).unwrap();
            assert!(n.mod_is_reduced(&Natural::from_str(m).unwrap()));
            n.mod_shr_assign(v, Natural::from_str(m).unwrap());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);
            assert!(n.mod_is_reduced(&Natural::from_str(m).unwrap()));

            let mut n = Natural::from_str(u).unwrap();
            n.mod_shr_assign(v, &Natural::from_str(m).unwrap());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = Natural::from_str(u)
                .unwrap()
                .mod_shr(v, Natural::from_str(m).unwrap());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = Natural::from_str(u)
                .unwrap()
                .mod_shr(v, &Natural::from_str(m).unwrap());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&Natural::from_str(u).unwrap()).mod_shr(v, Natural::from_str(m).unwrap());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&Natural::from_str(u).unwrap()).mod_shr(v, &Natural::from_str(m).unwrap());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            assert_eq!(
                ((Natural::from_str(u).unwrap() >> v) % Natural::from_str(m).unwrap()).to_string(),
                out
            );
        };
        test("0", 0, "1", "0");
        test("0", 0, "5", "0");
        test("8", -2, "10", "2");
        test("10", -100, "17", "7");
        test("10", 100, "19", "0");
        test("123456", -100, "12345678987654321", "7436663564915145");
    };
}

#[test]
fn test_mod_shr() {
    apply_to_signeds!(test_mod_shr_signed);
}
