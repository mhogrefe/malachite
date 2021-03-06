use malachite_base::num::arithmetic::traits::{DivMod, Square};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::arithmetic::mul::mul_low::{
    _limbs_mul_low_same_length_basecase, _limbs_mul_low_same_length_basecase_alt,
    _limbs_mul_low_same_length_divide_and_conquer,
    _limbs_mul_low_same_length_divide_and_conquer_shared_scratch, limbs_mul_low_same_length,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::base::{
    pairs_of_unsigneds, triples_of_unsigned_vec_var_25, triples_of_unsigned_vec_var_46,
    triples_of_unsigned_vec_var_48,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};

fn verify_mul_low_1(out_before: &[Limb], xs: &[Limb], ys: &[Limb], out_after: &[Limb]) {
    let n = Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys);
    let mut ns = n.into_limbs_asc();
    let len = xs.len();
    ns.resize(len, 0);
    assert_eq!(ns, &out_after[..len]);
    assert_eq!(&out_after[len..], &out_before[len..]);
}

fn verify_mul_low_2(xs: &[Limb], ys: &[Limb], out: &[Limb]) {
    let n = Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys);
    let mut ns = n.into_limbs_asc();
    let len = xs.len();
    ns.resize(len, 0);
    assert_eq!(ns, out);
}

#[test]
fn limbs_mul_low_same_length_basecase_properties() {
    test_properties(
        triples_of_unsigned_vec_var_46,
        |&(ref out_before, ref xs, ref ys)| {
            let mut out = out_before.to_vec();
            _limbs_mul_low_same_length_basecase(&mut out, xs, ys);

            let out_after = out;
            let mut out = out_before.to_vec();
            _limbs_mul_low_same_length_basecase_alt(&mut out, xs, ys);
            assert_eq!(out, out_after);

            verify_mul_low_1(out_before, xs, ys, &out_after);
        },
    );
}

#[test]
fn limbs_mul_low_same_length_divide_and_conquer_properties() {
    test_properties_custom_scale(
        512,
        triples_of_unsigned_vec_var_48,
        |&(ref out_before, ref xs, ref ys)| {
            let mut out = out_before.to_vec();
            _limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, xs, ys);

            let len = xs.len();
            let out_after = out[..len].to_vec();
            let mut out = out_before.to_vec();
            let mut scratch = vec![0; xs.len() << 1];
            _limbs_mul_low_same_length_divide_and_conquer(&mut out, xs, ys, &mut scratch);
            let out_after: &[Limb] = &out_after;
            assert_eq!(&out[..len], out_after);

            verify_mul_low_2(xs, ys, out_after);
        },
    );
}

#[test]
fn limbs_mul_low_same_length_properties() {
    test_properties_custom_scale(
        512,
        triples_of_unsigned_vec_var_25,
        |&(ref out_before, ref xs, ref ys)| {
            let mut out = out_before.to_vec();
            limbs_mul_low_same_length(&mut out, xs, ys);
            verify_mul_low_1(out_before, xs, ys, &out);
        },
    );
}

#[test]
fn mul_properties() {
    test_properties_custom_scale(2_048, pairs_of_naturals, |&(ref x, ref y)| {
        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * y;
        let product_ref_val = x * y.clone();
        let product = x * y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);

        let mut mut_x = x.clone();
        mut_x *= y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) * natural_to_biguint(y))),
            product
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) * natural_to_rug_integer(y))),
            product
        );
        assert_eq!(y * x, product);
        if *x != 0 {
            let (q, r) = (&product).div_mod(x);
            assert_eq!(q, *y);
            assert_eq!(r, 0);
        }
        if *y != 0 {
            let (q, r) = (&product).div_mod(y);
            assert_eq!(q, *x);
            assert_eq!(r, 0);
        }

        if *x != 0 && *y != 0 {
            assert!(product >= *x);
            assert!(product >= *y);
        }
    });

    test_properties_custom_scale(2_048, pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(
            Natural::from(DoubleLimb::from(x) * DoubleLimb::from(y)),
            Natural::from(x) * Natural::from(y)
        );
    });

    #[allow(unknown_lints, erasing_op)]
    test_properties_custom_scale(2_048, naturals, |x| {
        assert_eq!(x * Natural::ZERO, 0);
        assert_eq!(Natural::ZERO * x, 0);
        assert_eq!(x * Natural::ONE, *x);
        assert_eq!(Natural::ONE * x, *x);
        assert_eq!(x * x, x.square());
    });

    test_properties_custom_scale(2_048, triples_of_naturals, |&(ref x, ref y, ref z)| {
        assert_eq!((x * y) * z, x * (y * z));
        assert_eq!(x * (y + z), x * y + x * z);
        assert_eq!((x + y) * z, x * z + y * z);
    });
}
