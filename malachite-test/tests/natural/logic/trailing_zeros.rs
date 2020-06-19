use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_nz::natural::logic::trailing_zeros::limbs_trailing_zeros;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::natural::logic::trailing_zeros::natural_trailing_zeros_alt;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{positive_unsigneds, vecs_of_unsigned_var_3};
use malachite_test::inputs::natural::naturals;

#[test]
fn limbs_trailing_zeros_properties() {
    test_properties(vecs_of_unsigned_var_3, |limbs| {
        assert_eq!(
            Some(limbs_trailing_zeros(limbs)),
            Natural::from_limbs_asc(limbs).trailing_zeros()
        );
    });
}

#[test]
fn trailing_zeros_properties() {
    test_properties(naturals, |x| {
        let trailing_zeros = x.trailing_zeros();
        assert_eq!(natural_trailing_zeros_alt(x), trailing_zeros);
        assert_eq!(trailing_zeros.is_none(), *x == 0);
        if *x != 0 {
            let trailing_zeros = trailing_zeros.unwrap();
            if trailing_zeros <= u64::from(Limb::MAX) {
                assert!((x >> trailing_zeros).odd());
                assert_eq!(x >> trailing_zeros << trailing_zeros, *x);
            }
        }
    });

    test_properties(positive_unsigneds::<Limb>, |&u| {
        assert_eq!(
            Natural::from(u).trailing_zeros(),
            Some(TrailingZeros::trailing_zeros(u))
        );
    });
}
