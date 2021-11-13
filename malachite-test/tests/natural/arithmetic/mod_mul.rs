use malachite_base::num::arithmetic::traits::{
    ModAdd, ModIsReduced, ModMul, ModMulAssign, ModMulPrecomputed, ModMulPrecomputedAssign, ModNeg,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::JoinHalves;
use malachite_nz::natural::arithmetic::mod_mul::{
    _limbs_mod_mul_two_limbs, limbs_precompute_mod_mul_two_limbs,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_nz_test_util::natural::arithmetic::mod_mul::{
    limbs_mod_mul_two_limbs_naive, limbs_precompute_mod_mul_two_limbs_alt,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    nonuples_of_limbs_var_1, pairs_of_unsigneds_var_6, triples_of_unsigneds_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_naturals_var_2, quadruples_of_naturals_var_1, triples_of_naturals_var_4,
};

#[test]
fn limbs_precompute_mod_mul_two_limbs_properties() {
    test_properties(pairs_of_unsigneds_var_6, |&(m_1, m_0)| {
        let (inv_2, inv_1, inv_0) = limbs_precompute_mod_mul_two_limbs(m_1, m_0);
        assert_eq!(
            limbs_precompute_mod_mul_two_limbs_alt(m_1, m_0),
            (inv_2, inv_1, inv_0)
        );
        assert!(inv_2 != 0 || inv_1 != 0 || inv_0 != 0);
    });
}

#[test]
fn limbs_mod_mul_two_limbs_properties() {
    test_properties(
        nonuples_of_limbs_var_1,
        |&(x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0)| {
            let (r_1, r_0) =
                _limbs_mod_mul_two_limbs(x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0);
            assert_eq!(
                limbs_mod_mul_two_limbs_naive(x_1, x_0, y_1, y_0, m_1, m_0),
                (r_1, r_0)
            );

            let x = Natural::from(DoubleLimb::join_halves(x_1, x_0));
            let y = Natural::from(DoubleLimb::join_halves(y_1, y_0));
            let m = Natural::from(DoubleLimb::join_halves(m_1, m_0));
            let q = &x * &y / &m;
            let r = Natural::from(DoubleLimb::join_halves(r_1, r_0));
            assert_eq!(q * m + r, x * y);
        },
    );
}

#[test]
fn mod_mul_properties() {
    test_properties(triples_of_naturals_var_4, |&(ref x, ref y, ref m)| {
        assert!(x.mod_is_reduced(m));
        assert!(y.mod_is_reduced(m));
        let product_val_val_val = x.clone().mod_mul(y.clone(), m.clone());
        let product_val_ref_val = x.clone().mod_mul(y, m.clone());
        let product_ref_val_val = x.mod_mul(y.clone(), m.clone());
        let product_ref_ref_val = x.mod_mul(y, m.clone());
        let product_val_val_ref = x.clone().mod_mul(y.clone(), m);
        let product_val_ref_ref = x.clone().mod_mul(y, m);
        let product_ref_val_ref = x.mod_mul(y.clone(), m);
        let product = x.mod_mul(y, m);
        assert!(product_val_val_val.is_valid());
        assert!(product_val_ref_val.is_valid());
        assert!(product_ref_val_val.is_valid());
        assert!(product_val_val_ref.is_valid());
        assert!(product_val_val_ref.is_valid());
        assert!(product_val_ref_ref.is_valid());
        assert!(product_ref_val_ref.is_valid());
        assert!(product.is_valid());
        assert!(product.mod_is_reduced(m));
        assert_eq!(product_val_val_val, product);
        assert_eq!(product_val_ref_val, product);
        assert_eq!(product_ref_val_val, product);
        assert_eq!(product_ref_ref_val, product);
        assert_eq!(product_val_val_ref, product);
        assert_eq!(product_val_ref_ref, product);
        assert_eq!(product_ref_val_ref, product);

        let mut mut_x = x.clone();
        mut_x.mod_mul_assign(y.clone(), m.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x.mod_mul_assign(y, m.clone());
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x.mod_mul_assign(y.clone(), m);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x.mod_mul_assign(y, m);
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&m);

        let product_pre_val_val_val = x.clone().mod_mul_precomputed(y.clone(), m.clone(), &data);
        let product_pre_val_ref_val = x.clone().mod_mul_precomputed(y, m.clone(), &data);
        let product_pre_ref_val_val = x.mod_mul_precomputed(y.clone(), m.clone(), &data);
        let product_pre_ref_ref_val = x.mod_mul_precomputed(y, m.clone(), &data);
        let product_pre_val_val_ref = x.clone().mod_mul_precomputed(y.clone(), m, &data);
        let product_pre_val_ref_ref = x.clone().mod_mul_precomputed(y, m, &data);
        let product_pre_ref_val_ref = x.mod_mul_precomputed(y.clone(), m, &data);
        let product_pre_ref_ref_ref = x.mod_mul_precomputed(y, m, &data);
        assert!(product_pre_val_val_val.is_valid());
        assert!(product_pre_val_ref_val.is_valid());
        assert!(product_pre_ref_val_val.is_valid());
        assert!(product_pre_val_val_ref.is_valid());
        assert!(product_pre_val_val_ref.is_valid());
        assert!(product_pre_val_ref_ref.is_valid());
        assert!(product_pre_ref_val_ref.is_valid());
        assert!(product_pre_ref_ref_ref.is_valid());
        assert_eq!(product_pre_val_val_val, product);
        assert_eq!(product_pre_val_ref_val, product);
        assert_eq!(product_pre_ref_val_val, product);
        assert_eq!(product_pre_ref_ref_val, product);
        assert_eq!(product_pre_val_val_ref, product);
        assert_eq!(product_pre_val_ref_ref, product);
        assert_eq!(product_pre_ref_val_ref, product);

        let mut mut_x = x.clone();
        mut_x.mod_mul_precomputed_assign(y.clone(), m.clone(), &data);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x.mod_mul_precomputed_assign(y, m.clone(), &data);
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x.mod_mul_precomputed_assign(y.clone(), m, &data);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x.mod_mul_precomputed_assign(y, m, &data);
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        assert_eq!((x * y) % m, product);

        assert_eq!(y.mod_mul(x, m), product);
        assert_eq!(x.mod_mul(y.mod_neg(m), m), (&product).mod_neg(m));
        assert_eq!(x.mod_neg(m).mod_mul(y, m), product.mod_neg(m));
    });

    test_properties(pairs_of_naturals_var_2, |&(ref x, ref m)| {
        assert_eq!(x.mod_mul(Natural::ZERO, m), 0);
        assert_eq!(Natural::ZERO.mod_mul(x, m), 0);
        assert_eq!(x.mod_mul(Natural::ONE, m), *x);
        assert_eq!(Natural::ONE.mod_mul(x, m), *x);
        //TODO assert_eq!(x * x, x.square());
    });

    test_properties(
        quadruples_of_naturals_var_1,
        |&(ref x, ref y, ref z, ref m)| {
            assert_eq!(x.mod_mul(y, m).mod_mul(z, m), x.mod_mul(y.mod_mul(z, m), m));
            assert_eq!(
                x.mod_mul(y.mod_add(z, m), m),
                x.mod_mul(y, m).mod_add(x.mod_mul(z, m), m)
            );
            assert_eq!(
                x.mod_add(y, m).mod_mul(z, m),
                x.mod_mul(z, m).mod_add(y.mod_mul(z, m), m)
            );
        },
    );

    test_properties(triples_of_unsigneds_var_1::<Limb>, |&(x, y, m)| {
        assert_eq!(
            x.mod_mul(y, m),
            Natural::from(x).mod_mul(Natural::from(y), Natural::from(m))
        );
    });
}
