use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_mod_is_reduced() {
    let test = |u, v, out| {
        assert_eq!(
            Natural::from_str(u)
                .unwrap()
                .mod_is_reduced(&Natural::from_str(v).unwrap()),
            out
        );
    };

    test("0", "5", true);
    test("100", "100", false);
    test("100", "101", true);
    test("1000000000000", "1000000000000", false);
    test("1000000000000", "1000000000001", true);
}

#[test]
#[should_panic]
fn mod_is_reduced_fail() {
    Natural::from(123u32).mod_is_reduced(&Natural::ZERO);
}
