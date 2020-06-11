use malachite_base::num::arithmetic::traits::{SaturatingSub, SaturatingSubAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};

#[test]
fn saturating_sub_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let mut mut_x = x.clone();
        mut_x.saturating_sub_assign(y);
        assert!(mut_x.is_valid());
        let diff = mut_x;

        let mut mut_x = x.clone();
        mut_x.saturating_sub_assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);

        let diff_alt = x.clone().saturating_sub(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = x.clone().saturating_sub(y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = x.saturating_sub(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = x.saturating_sub(y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let reverse_diff = y.saturating_sub(x);
        if diff > 0 {
            assert_eq!(reverse_diff, 0);
        }
        if reverse_diff > 0 {
            assert_eq!(diff, 0);
        }

        assert!(diff <= *x);
        assert!(diff + y >= *x);
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(
            x.saturating_sub(y),
            Natural::from(x).saturating_sub(Natural::from(y))
        );
    });

    #[allow(unknown_lints, identity_op, eq_op)]
    test_properties(naturals, |x| {
        assert_eq!(x.saturating_sub(Natural::ZERO), *x);
        assert_eq!(x.saturating_sub(x), 0);
        assert_eq!(Natural::ZERO.saturating_sub(x), 0);
    });
}
