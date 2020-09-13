use malachite_base_test_util::num::arithmetic::mod_mul::limbs_invert_limb_naive;

use malachite_base::num::arithmetic::mod_mul::{
    _limbs_invert_limb_u32, _limbs_invert_limb_u64, _limbs_mod_preinverted, _naive_mod_mul,
    test_invert_u32_table, test_invert_u64_table,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, HasHalf, JoinHalves, SplitInHalf};
use malachite_base::num::logic::traits::LeadingZeros;

#[test]
fn test_test_invert_u32_table() {
    test_invert_u32_table();
}

#[test]
fn test_test_invert_u64_table() {
    test_invert_u64_table();
}

#[test]
fn test_limbs_invert_limb_u32() {
    let test = |x, out| {
        assert_eq!(_limbs_invert_limb_u32(x), out);
        assert_eq!(limbs_invert_limb_naive::<u32, u64>(x), out);
    };
    test(0x8000_0000, u32::MAX);
    test(0x8000_0001, 0xffff_fffc);
    test(u32::MAX - 1, 2);
    test(u32::MAX, 1);
    test(0x89ab_cdef, 0xdc08_767e);
}

#[test]
#[should_panic]
fn limbs_invert_limb_u32_fail() {
    _limbs_invert_limb_u32(123);
}

#[test]
fn test_limbs_invert_limb_u64() {
    let test = |x, out| {
        assert_eq!(_limbs_invert_limb_u64(x), out);
        assert_eq!(limbs_invert_limb_naive::<u64, u128>(x), out);
    };
    test(0x8000_0000_0000_0000, u64::MAX);
    test(0x8000_0000_0000_0001, 0xffff_ffff_ffff_fffc);
    test(0xffff_ffff_ffff_fffe, 2);
    test(u64::MAX, 1);
    test(0x89ab_cdef_edcb_a987, 0xdc08_767b_33d7_ec8f);
}

#[test]
#[should_panic]
fn limbs_invert_limb_u64_fail() {
    _limbs_invert_limb_u64(123);
}

#[test]
fn test_limbs_mod_preinverted() {
    fn test<T: PrimitiveUnsigned, DT: JoinHalves + PrimitiveUnsigned + SplitInHalf>(
        x_1: T,
        x_0: T,
        d: T,
        out: T,
    ) where
        DT: From<T> + HasHalf<Half = T>,
        T: CheckedFrom<DT>,
    {
        let d_inv = limbs_invert_limb_naive::<T, DT>(d << LeadingZeros::leading_zeros(d));
        assert_eq!(_limbs_mod_preinverted::<T, DT>(x_1, x_0, d, d_inv), out);
        assert_eq!(T::exact_from(DT::join_halves(x_1, x_0) % DT::from(d)), out);
    };
    test::<u8, u16>(0, 0, 1, 0);
    test::<u32, u64>(0, 1, 1, 0);
    test::<u16, u32>(1, 0, 2, 0);
    test::<u16, u32>(1, 7, 2, 1);
    test::<u8, u16>(0x78, 0x9a, 0xbc, 0x2a);
    test::<u64, u128>(0x12, 0x34, 0x33, 0x13);
}

fn mod_mul_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, m, out| {
        assert_eq!(x.mod_mul(y, m), out);

        let mut mut_x = x;
        mut_x.mod_mul_assign(y, m);
        assert_eq!(mut_x, out);

        let data = T::precompute_mod_mul_data(&m);
        assert_eq!(x.mod_mul_precomputed(y, m, &data), out);

        let mut mut_x = x;
        mut_x.mod_mul_precomputed_assign(y, m, &data);
        assert_eq!(mut_x, out);

        assert_eq!(_naive_mod_mul(x, y, m), out);
    };
    test(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    test(T::TWO, T::exact_from(3), T::exact_from(7), T::exact_from(6));
    test(
        T::exact_from(7),
        T::exact_from(3),
        T::exact_from(10),
        T::ONE,
    );
    test(
        T::exact_from(100),
        T::exact_from(100),
        T::exact_from(123),
        T::exact_from(37),
    );
    test(T::MAX - T::ONE, T::MAX - T::ONE, T::MAX, T::ONE);
}

#[test]
fn test_mod_mul() {
    apply_fn_to_unsigneds!(mod_mul_helper);
}
