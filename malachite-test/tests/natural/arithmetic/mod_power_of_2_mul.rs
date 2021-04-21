use malachite_base::num::arithmetic::traits::{
    ModMul, ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul,
    ModPowerOf2MulAssign, ModPowerOf2Neg, PowerOf2,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::arithmetic::mod_power_of_2_mul::{
    limbs_mod_power_of_2_mul, limbs_mod_power_of_2_mul_ref_ref, limbs_mod_power_of_2_mul_val_ref,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    triples_of_limb_vec_limb_vec_and_u64_var_16, triples_of_unsigned_unsigned_and_small_u64_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_u64_var_1, quadruples_of_three_naturals_and_u64_var_1,
    triples_of_natural_natural_and_u64_var_1,
};

#[test]
fn limbs_mod_power_of_2_mul_properties() {
    test_properties(
        triples_of_limb_vec_limb_vec_and_u64_var_16,
        |&(ref xs, ref ys, pow)| {
            let product =
                Natural::from_limbs_asc(xs).mod_power_of_2_mul(Natural::from_limbs_asc(ys), pow);
            assert_eq!(
                (Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys)).mod_power_of_2(pow),
                product
            );
            let mut mut_xs = xs.clone();
            let mut mut_ys = ys.clone();
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_mul(
                    &mut mut_xs,
                    &mut mut_ys,
                    pow
                )),
                product,
            );
            let mut mut_xs = xs.clone();
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_mul_val_ref(
                    &mut mut_xs,
                    ys,
                    pow
                )),
                product,
            );
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_mul_ref_ref(xs, ys, pow)),
                product,
            );
        },
    );
}

#[test]
fn mod_power_of_2_mul_properties() {
    test_properties(
        triples_of_natural_natural_and_u64_var_1,
        |&(ref x, ref y, pow)| {
            assert!(x.mod_power_of_2_is_reduced(pow));
            assert!(y.mod_power_of_2_is_reduced(pow));
            let product_val_val = x.clone().mod_power_of_2_mul(y.clone(), pow);
            let product_val_ref = x.clone().mod_power_of_2_mul(y, pow);
            let product_ref_val = x.mod_power_of_2_mul(y.clone(), pow);
            let product = x.mod_power_of_2_mul(y, pow);
            assert!(product_val_val.is_valid());
            assert!(product_val_ref.is_valid());
            assert!(product_ref_val.is_valid());
            assert!(product.is_valid());
            assert!(product.mod_power_of_2_is_reduced(pow));
            assert_eq!(product_val_val, product);
            assert_eq!(product_val_ref, product);
            assert_eq!(product_ref_val, product);

            assert_eq!((x * y).mod_power_of_2(pow), product);
            assert_eq!(x.mod_mul(y, Natural::power_of_2(pow)), product);

            let mut mut_x = x.clone();
            mut_x.mod_power_of_2_mul_assign(y.clone(), pow);
            assert!(mut_x.is_valid());
            assert_eq!(mut_x, product);
            let mut mut_x = x.clone();
            mut_x.mod_power_of_2_mul_assign(y, pow);
            assert_eq!(mut_x, product);
            assert!(mut_x.is_valid());

            assert_eq!(y.mod_power_of_2_mul(x, pow), product);
            assert_eq!(
                x.mod_power_of_2_mul(y.mod_power_of_2_neg(pow), pow),
                (&product).mod_power_of_2_neg(pow)
            );
            assert_eq!(
                x.mod_power_of_2_neg(pow).mod_power_of_2_mul(y, pow),
                product.mod_power_of_2_neg(pow)
            );
        },
    );

    test_properties(pairs_of_natural_and_u64_var_1, |&(ref x, pow)| {
        assert_eq!(x.mod_power_of_2_mul(Natural::ZERO, pow), 0);
        assert_eq!(Natural::ZERO.mod_power_of_2_mul(x, pow), 0);
        assert_eq!(x.mod_power_of_2_mul(Natural::ONE, pow), *x);
        assert_eq!(Natural::ONE.mod_power_of_2_mul(x, pow), *x);
        //TODO assert_eq!(x * x, x.square());
    });

    test_properties(
        quadruples_of_three_naturals_and_u64_var_1,
        |&(ref x, ref y, ref z, pow)| {
            assert_eq!(
                x.mod_power_of_2_mul(y, pow).mod_power_of_2_mul(z, pow),
                x.mod_power_of_2_mul(y.mod_power_of_2_mul(z, pow), pow)
            );
            assert_eq!(
                x.mod_power_of_2_mul(y.mod_power_of_2_add(z, pow), pow),
                x.mod_power_of_2_mul(y, pow)
                    .mod_power_of_2_add(x.mod_power_of_2_mul(z, pow), pow)
            );
            assert_eq!(
                x.mod_power_of_2_add(y, pow).mod_power_of_2_mul(z, pow),
                x.mod_power_of_2_mul(z, pow)
                    .mod_power_of_2_add(y.mod_power_of_2_mul(z, pow), pow)
            );
        },
    );

    test_properties_no_special(
        triples_of_unsigned_unsigned_and_small_u64_var_1::<Limb>,
        |&(x, y, pow)| {
            assert_eq!(
                x.mod_power_of_2_mul(y, pow),
                Natural::from(x).mod_power_of_2_mul(Natural::from(y), pow)
            );
        },
    );
}
