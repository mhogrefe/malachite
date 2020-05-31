use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{integers, pairs_of_integers};
use malachite_test::inputs::natural::pairs_of_naturals_var_1;

#[test]
fn sub_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let diff_val_val = x.clone() - y.clone();
        let diff_val_ref = x.clone() - y;
        let diff_ref_val = x - y.clone();
        let diff = x - y;
        assert!(diff_val_val.is_valid());
        assert!(diff_val_ref.is_valid());
        assert!(diff_ref_val.is_valid());
        assert!(diff.is_valid());
        assert_eq!(diff_val_val, diff);
        assert_eq!(diff_val_ref, diff);
        assert_eq!(diff_ref_val, diff);

        let mut mut_x = x.clone();
        mut_x -= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);
        let mut mut_x = x.clone();
        mut_x -= y;
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);

        let mut mut_x = integer_to_rug_integer(x);
        mut_x -= integer_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), diff);

        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(x) - integer_to_bigint(y))),
            diff
        );
        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) - integer_to_rug_integer(y))),
            diff
        );
        assert_eq!(y - x, -&diff);
        assert_eq!(&diff + y, *x);
        assert_eq!(x - diff, *y);
    });

    #[allow(unknown_lints, eq_op)]
    test_properties(integers, |x| {
        assert_eq!(x - Integer::ZERO, *x);
        assert_eq!(Integer::ZERO - x, -x);
        assert_eq!(x - -x, x << 1);
        assert_eq!(x - x, 0)
    });

    test_properties(pairs_of_naturals_var_1, |&(ref x, ref y)| {
        assert_eq!(x + y, Integer::from(x) + Integer::from(y));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(
            Integer::from(SignedDoubleLimb::from(x) - SignedDoubleLimb::from(y)),
            Integer::from(x) - Integer::from(y)
        );
    });
}
