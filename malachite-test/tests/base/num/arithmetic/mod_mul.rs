use malachite_base::num::arithmetic::mod_mul::{
    _fast_mod_mul, _limbs_invert_limb_naive, _limbs_invert_limb_u32, _limbs_invert_limb_u64,
    _limbs_mod_preinverted, _naive_mod_mul,
};
use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, HasHalf, JoinHalves, SplitInHalf};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigneds_var_5, quadruples_of_unsigneds_var_1, quadruples_of_unsigneds_var_2,
    triples_of_unsigneds_var_1, unsigneds_var_1,
};

#[test]
fn invert_limb_u32_properties() {
    test_properties(unsigneds_var_1::<u32>, |&x| {
        let inverse = _limbs_invert_limb_u32(x);
        assert_eq!(_limbs_invert_limb_naive::<u32, u64>(x), inverse);
        assert_ne!(inverse, 0);
    });
}

#[test]
fn invert_limb_u64_properties() {
    test_properties(unsigneds_var_1::<u64>, |&x| {
        let inverse = _limbs_invert_limb_u64(x);
        assert_eq!(_limbs_invert_limb_naive::<u64, u128>(x), inverse);
        assert_ne!(inverse, 0);
    });
}

fn limbs_mod_preinverted_properties_helper<
    T: PrimitiveUnsigned + Rand,
    DT: JoinHalves + PrimitiveUnsigned + SplitInHalf,
>()
where
    DT: From<T> + HasHalf<Half = T>,
    T: CheckedFrom<DT>,
{
    test_properties(
        quadruples_of_unsigneds_var_2::<T, DT>,
        |&(x_1, x_0, d, d_inv)| {
            let r = _limbs_mod_preinverted::<T, DT>(x_1, x_0, d, d_inv);
            let n = DT::join_halves(x_1, x_0);
            assert_eq!(T::exact_from(n % DT::from(d)), r);
            assert!(r < d);
            let q = DT::join_halves(x_1, x_0) / DT::from(d);
            assert_eq!(q * DT::from(d) + DT::from(r), n);
        },
    );
}

#[test]
fn limbs_mod_preinverted_properties() {
    limbs_mod_preinverted_properties_helper::<u8, u16>();
    limbs_mod_preinverted_properties_helper::<u16, u32>();
    limbs_mod_preinverted_properties_helper::<u32, u64>();
    limbs_mod_preinverted_properties_helper::<u64, u128>();
}

fn mod_mul_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(triples_of_unsigneds_var_1::<T>, |&(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let product = x.mod_mul(y, m);
        assert!(product.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_mul_assign(y, m);
        assert_eq!(x_alt, product);

        let data = T::precompute_mod_mul_data(m);

        assert_eq!(x.mod_mul_precomputed(y, m, &data), product);

        let mut x_alt = x;
        x_alt.mod_mul_precomputed_assign(y, m, &data);
        assert_eq!(x_alt, product);

        assert_eq!(_naive_mod_mul(x, y, m), product);
        assert_eq!(y.mod_mul(x, m), product);
        assert_eq!(x.mod_mul(y.mod_neg(m), m), product.mod_neg(m));
        assert_eq!(x.mod_neg(m).mod_mul(y, m), product.mod_neg(m));
    });

    test_properties(pairs_of_unsigneds_var_5::<T>, |&(x, m)| {
        assert_eq!(x.mod_mul(T::ZERO, m), T::ZERO);
        assert_eq!(T::ZERO.mod_mul(x, m), T::ZERO);
        assert_eq!(x.mod_mul(T::ONE, m), x);
        assert_eq!(T::ONE.mod_mul(x, m), x);
    });

    test_properties(quadruples_of_unsigneds_var_1::<T>, |&(x, y, z, m)| {
        assert_eq!(x.mod_mul(y, m).mod_mul(z, m), x.mod_mul(y.mod_mul(z, m), m));
        assert_eq!(
            x.mod_mul(y.mod_add(z, m), m),
            x.mod_mul(y, m).mod_add(x.mod_mul(z, m), m)
        );
        assert_eq!(
            x.mod_add(y, m).mod_mul(z, m),
            x.mod_mul(z, m).mod_add(y.mod_mul(z, m), m)
        );
    });
}

fn mod_mul_properties_fast_helper<
    T: PrimitiveUnsigned + Rand + SampleRange,
    DT: JoinHalves + PrimitiveUnsigned + SplitInHalf,
>()
where
    T: ModMulPrecomputed<Data = T>,
    DT: From<T> + HasHalf<Half = T>,
{
    test_properties(triples_of_unsigneds_var_1::<T>, |&(x, y, m)| {
        let product = x.mod_mul(y, m);
        assert_eq!(
            _fast_mod_mul::<T, DT>(x, y, m, T::precompute_mod_mul_data(m)),
            product
        );
    });
}

#[test]
fn mod_mul_properties() {
    mod_mul_properties_helper::<u8>();
    mod_mul_properties_helper::<u16>();
    mod_mul_properties_helper::<u32>();
    mod_mul_properties_helper::<u64>();
    mod_mul_properties_helper::<usize>();

    mod_mul_properties_fast_helper::<u32, u64>();
    mod_mul_properties_fast_helper::<u64, u128>();
}
