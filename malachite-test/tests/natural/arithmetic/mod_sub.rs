use malachite_base::num::arithmetic::traits::{
    Mod, ModAdd, ModIsReduced, ModNeg, ModSub, ModSubAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::triples_of_unsigneds_var_1;
use malachite_test::inputs::natural::{pairs_of_naturals_var_2, triples_of_naturals_var_4};

#[test]
fn mod_sub_properties() {
    test_properties(triples_of_naturals_var_4, |&(ref x, ref y, ref m)| {
        assert!(x.mod_is_reduced(m));
        assert!(y.mod_is_reduced(m));
        let diff_val_val_val = x.clone().mod_sub(y.clone(), m.clone());
        let diff_val_ref_val = x.clone().mod_sub(y, m.clone());
        let diff_ref_val_val = x.mod_sub(y.clone(), m.clone());
        let diff_ref_ref_val = x.mod_sub(y, m.clone());
        let diff_val_val_ref = x.clone().mod_sub(y.clone(), m);
        let diff_val_ref_ref = x.clone().mod_sub(y, m);
        let diff_ref_val_ref = x.mod_sub(y.clone(), m);
        let diff = x.mod_sub(y, m);
        assert!(diff_val_val_val.is_valid());
        assert!(diff_val_ref_val.is_valid());
        assert!(diff_ref_val_val.is_valid());
        assert!(diff_val_val_ref.is_valid());
        assert!(diff_val_val_ref.is_valid());
        assert!(diff_val_ref_ref.is_valid());
        assert!(diff_ref_val_ref.is_valid());
        assert!(diff.is_valid());
        assert!(diff.mod_is_reduced(m));
        assert_eq!(diff_val_val_val, diff);
        assert_eq!(diff_val_ref_val, diff);
        assert_eq!(diff_ref_val_val, diff);
        assert_eq!(diff_ref_ref_val, diff);
        assert_eq!(diff_val_val_ref, diff);
        assert_eq!(diff_val_ref_ref, diff);
        assert_eq!(diff_ref_val_ref, diff);

        assert_eq!(
            (Integer::from(x) - Integer::from(y)).mod_op(Integer::from(m)),
            diff
        );

        let mut mut_x = x.clone();
        mut_x.mod_sub_assign(y.clone(), m.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);
        let mut mut_x = x.clone();
        mut_x.mod_sub_assign(y, m.clone());
        assert_eq!(mut_x, diff);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x.mod_sub_assign(y.clone(), m);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);
        let mut mut_x = x.clone();
        mut_x.mod_sub_assign(y, m);
        assert_eq!(mut_x, diff);
        assert!(mut_x.is_valid());

        assert_eq!(y.mod_sub(x, m), (&diff).mod_neg(m));
        assert_eq!(x.mod_add(y.mod_neg(m), m), diff);
        assert_eq!((&diff).mod_add(y, m), *x);
        assert_eq!(diff.mod_sub(x, m), y.mod_neg(m));
    });

    test_properties(pairs_of_naturals_var_2, |&(ref x, ref m)| {
        assert_eq!(x.mod_sub(Natural::ZERO, m), *x);
        assert_eq!(Natural::ZERO.mod_sub(x, m), x.mod_neg(m));
        assert_eq!(x.mod_sub(x, m), 0);
    });

    test_properties(triples_of_unsigneds_var_1::<Limb>, |&(x, y, m)| {
        assert_eq!(
            x.mod_sub(y, m),
            Natural::from(x).mod_sub(Natural::from(y), Natural::from(m))
        );
    });
}
