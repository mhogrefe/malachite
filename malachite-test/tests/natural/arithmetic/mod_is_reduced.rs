use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds_var_4;
use malachite_test::inputs::natural::pairs_of_natural_and_positive_natural;

#[test]
fn mod_is_reduced_properties() {
    test_properties(pairs_of_natural_and_positive_natural, |(n, m)| {
        assert_eq!(n.mod_is_reduced(m), n % m == *n);
    });

    test_properties(pairs_of_unsigneds_var_4::<Limb>, |&(n, m)| {
        assert_eq!(
            n.mod_is_reduced(&m),
            Natural::from(n).mod_is_reduced(&Natural::from(m))
        );
    });
}
