use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::arithmetic::traits::{ModMul, ModNeg, ModPow, ModPowAssign, Parity};
use malachite_base::num::basic::traits::{Iverson, One, Two, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::triples_of_unsigned_unsigned_and_unsigned_var_1;
use malachite_test::inputs::natural::{
    pairs_of_natural_and_positive_natural, pairs_of_naturals_var_2, quadruples_of_naturals_var_2,
    quadruples_of_naturals_var_3, triples_of_naturals_var_5,
};

#[test]
fn mod_pow_properties() {
    test_properties(triples_of_naturals_var_5, |&(ref x, ref exp, ref m)| {
        assert!(x.mod_is_reduced(m));
        let power_val_val_val = x.clone().mod_pow(exp.clone(), m.clone());
        let power_val_ref_val = x.clone().mod_pow(exp, m.clone());
        let power_ref_val_val = x.mod_pow(exp.clone(), m.clone());
        let power_ref_ref_val = x.mod_pow(exp, m.clone());
        let power_val_val_ref = x.clone().mod_pow(exp.clone(), m);
        let power_val_ref_ref = x.clone().mod_pow(exp, m);
        let power_ref_val_ref = x.mod_pow(exp.clone(), m);
        let power = x.mod_pow(exp, m);
        assert!(power_val_val_val.is_valid());
        assert!(power_val_ref_val.is_valid());
        assert!(power_ref_val_val.is_valid());
        assert!(power_val_val_ref.is_valid());
        assert!(power_val_val_ref.is_valid());
        assert!(power_val_ref_ref.is_valid());
        assert!(power_ref_val_ref.is_valid());
        assert!(power.is_valid());
        assert!(power.mod_is_reduced(m));
        assert_eq!(power_val_val_val, power);
        assert_eq!(power_val_ref_val, power);
        assert_eq!(power_ref_val_val, power);
        assert_eq!(power_ref_ref_val, power);
        assert_eq!(power_val_val_ref, power);
        assert_eq!(power_val_ref_ref, power);
        assert_eq!(power_ref_val_ref, power);

        let mut mut_x = x.clone();
        mut_x.mod_pow_assign(exp.clone(), m.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, power);
        let mut mut_x = x.clone();
        mut_x.mod_pow_assign(exp, m.clone());
        assert_eq!(mut_x, power);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x.mod_pow_assign(exp.clone(), m);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, power);
        let mut mut_x = x.clone();
        mut_x.mod_pow_assign(exp, m);
        assert_eq!(mut_x, power);
        assert!(mut_x.is_valid());

        if exp.even() {
            assert_eq!(x.mod_neg(m).mod_pow(exp, m), power);
        } else {
            assert_eq!(x.mod_neg(m).mod_pow(exp, m), power.mod_neg(m));
        }
    });

    test_properties(
        pairs_of_natural_and_positive_natural,
        |&(ref exp, ref m)| {
            assert_eq!(
                Natural::ZERO.mod_pow(exp, m),
                Natural::iverson(*exp == 0 && *m != 1),
            );
            if *m != 1 {
                assert_eq!(Natural::ONE.mod_pow(exp, m), 1);
            }
        },
    );

    test_properties(pairs_of_naturals_var_2, |&(ref x, ref m)| {
        assert_eq!(x.mod_pow(Natural::ZERO, m), Natural::iverson(*m != 1));
        assert_eq!(x.mod_pow(Natural::ONE, m), *x);
        assert_eq!(x.mod_pow(Natural::TWO, m), x.mod_mul(x, m));
    });

    test_properties(
        quadruples_of_naturals_var_2,
        |&(ref x, ref y, ref exp, ref m)| {
            assert_eq!(
                x.mod_mul(y, m).mod_pow(exp, m),
                x.mod_pow(exp, m).mod_mul(y.mod_pow(exp, m), m)
            );
        },
    );

    test_properties(
        quadruples_of_naturals_var_3,
        |&(ref x, ref e, ref f, ref m)| {
            assert_eq!(
                x.mod_pow(e + f, m),
                x.mod_pow(e, m).mod_mul(x.mod_pow(f, m), m)
            );
            assert_eq!(x.mod_pow(e * f, m), x.mod_pow(e, m).mod_pow(f, m));
        },
    );

    test_properties(
        triples_of_unsigned_unsigned_and_unsigned_var_1::<Limb, u64>,
        |&(x, exp, m)| {
            assert_eq!(
                x.mod_pow(exp, m),
                Natural::from(x).mod_pow(Natural::from(exp), Natural::from(m))
            );
        },
    );
}
