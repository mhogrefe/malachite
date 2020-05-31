use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::natural::arithmetic::checked_sub::checked_sub;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};

#[test]
fn checked_sub_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let diff = if *x >= *y {
            let mut mut_x = x.clone();
            mut_x -= y;
            assert!(mut_x.is_valid());
            let diff = mut_x;

            let mut rug_x = natural_to_rug_integer(x);
            rug_x -= natural_to_rug_integer(y);
            assert_eq!(rug_integer_to_natural(&rug_x), diff);
            Some(diff)
        } else {
            None
        };

        let diff_alt = x.clone().checked_sub(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, |x| x.is_valid()));

        let diff_alt = x.clone().checked_sub(y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, |x| x.is_valid()));

        let diff_alt = x.checked_sub(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, |x| x.is_valid()));

        let diff_alt = x.checked_sub(y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.as_ref().map_or(true, |x| x.is_valid()));

        let reverse_diff = y.checked_sub(x);
        assert_eq!(reverse_diff.is_some(), *x == *y || diff.is_none());

        assert_eq!(
            checked_sub(natural_to_biguint(x), natural_to_biguint(y))
                .map(|x| biguint_to_natural(&x)),
            diff
        );
        assert_eq!(
            checked_sub(natural_to_rug_integer(x), natural_to_rug_integer(y))
                .map(|x| rug_integer_to_natural(&x)),
            diff
        );

        if let Some(diff) = diff {
            assert!(diff <= *x);
            assert_eq!(diff + y, *x);
        }
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(
            x.checked_sub(y).map(Natural::from),
            Natural::from(x).checked_sub(Natural::from(y))
        );
    });

    #[allow(unknown_lints, identity_op, eq_op)]
    test_properties(naturals, |x| {
        assert_eq!(x.checked_sub(Natural::ZERO).as_ref(), Some(x));
        assert_eq!(x.checked_sub(x), Some(Natural::ZERO));
        if *x != 0 {
            assert!((Natural::ZERO.checked_sub(x)).is_none());
        }
    });
}
