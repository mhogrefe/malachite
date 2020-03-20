use std::str::FromStr;

use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds_var_4;
use malachite_test::inputs::natural::pairs_of_natural_and_positive_natural;

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

#[test]
fn mod_is_reduced_properties() {
    test_properties(pairs_of_natural_and_positive_natural, |(n, modulus)| {
        assert_eq!(n.mod_is_reduced(modulus), n % modulus == *n);
    });

    test_properties(pairs_of_unsigneds_var_4::<Limb>, |&(n, modulus)| {
        assert_eq!(
            n.mod_is_reduced(&modulus),
            Natural::from(n).mod_is_reduced(&Natural::from(modulus))
        );
    });
}
