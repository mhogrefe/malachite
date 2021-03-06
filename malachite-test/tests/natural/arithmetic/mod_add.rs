use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign, ModIsReduced, ModNeg, ModSub};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::triples_of_unsigneds_var_1;
use malachite_test::inputs::natural::{
    pairs_of_naturals_var_2, quadruples_of_naturals_var_1, triples_of_naturals_var_4,
};

#[test]
fn mod_add_properties() {
    test_properties(triples_of_naturals_var_4, |&(ref x, ref y, ref m)| {
        assert!(x.mod_is_reduced(m));
        assert!(y.mod_is_reduced(m));
        let sum_val_val_val = x.clone().mod_add(y.clone(), m.clone());
        let sum_val_ref_val = x.clone().mod_add(y, m.clone());
        let sum_ref_val_val = x.mod_add(y.clone(), m.clone());
        let sum_ref_ref_val = x.mod_add(y, m.clone());
        let sum_val_val_ref = x.clone().mod_add(y.clone(), m);
        let sum_val_ref_ref = x.clone().mod_add(y, m);
        let sum_ref_val_ref = x.mod_add(y.clone(), m);
        let sum = x.mod_add(y, m);
        assert!(sum_val_val_val.is_valid());
        assert!(sum_val_ref_val.is_valid());
        assert!(sum_ref_val_val.is_valid());
        assert!(sum_val_val_ref.is_valid());
        assert!(sum_val_val_ref.is_valid());
        assert!(sum_val_ref_ref.is_valid());
        assert!(sum_ref_val_ref.is_valid());
        assert!(sum.is_valid());
        assert!(sum.mod_is_reduced(m));
        assert_eq!(sum_val_val_val, sum);
        assert_eq!(sum_val_ref_val, sum);
        assert_eq!(sum_ref_val_val, sum);
        assert_eq!(sum_ref_ref_val, sum);
        assert_eq!(sum_val_val_ref, sum);
        assert_eq!(sum_val_ref_ref, sum);
        assert_eq!(sum_ref_val_ref, sum);

        assert_eq!((x + y) % m, sum);

        let mut mut_x = x.clone();
        mut_x.mod_add_assign(y.clone(), m.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x.mod_add_assign(y, m.clone());
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x.mod_add_assign(y.clone(), m);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x.mod_add_assign(y, m);
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        assert_eq!(y.mod_add(x, m), sum);
        assert_eq!(x.mod_sub(y.mod_neg(m), m), sum);
        assert_eq!((&sum).mod_sub(x, m), *y);
        assert_eq!(sum.mod_sub(y, m), *x);
    });

    test_properties(pairs_of_naturals_var_2, |&(ref x, ref m)| {
        assert_eq!(x.mod_add(Natural::ZERO, m), *x);
        assert_eq!(Natural::ZERO.mod_add(x, m), *x);
        //TODO assert_eq!(x + x, x << 1);
    });

    test_properties(
        quadruples_of_naturals_var_1,
        |&(ref x, ref y, ref z, ref m)| {
            assert_eq!(x.mod_add(y, m).mod_add(z, m), x.mod_add(y.mod_add(z, m), m));
        },
    );

    test_properties(triples_of_unsigneds_var_1::<Limb>, |&(x, y, m)| {
        assert_eq!(
            x.mod_add(y, m),
            Natural::from(x).mod_add(Natural::from(y), Natural::from(m))
        );
    });
}
