use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::Sign;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::natural_to_rug_integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::naturals;

#[test]
fn sign_properties() {
    test_properties(naturals, |n| {
        let sign = n.sign();
        assert_eq!(natural_to_rug_integer(n).cmp0(), sign);
        assert_ne!(sign, Ordering::Less);
        assert_eq!(n.partial_cmp(&0), Some(sign));
        assert_eq!((-n).sign(), sign.reverse());
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(Natural::from(u).sign(), u.sign());
    });
}
