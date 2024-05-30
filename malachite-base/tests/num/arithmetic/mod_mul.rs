// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::mod_mul::{
    fast_mod_mul, limbs_invert_limb_u32, limbs_invert_limb_u64, limbs_mod_preinverted,
    naive_mod_mul, test_invert_u32_table, test_invert_u64_table,
};
use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{HasHalf, JoinHalves, SplitInHalf};
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::test_util::generators::{
    unsigned_gen_var_12, unsigned_pair_gen_var_16, unsigned_quadruple_gen_var_4,
    unsigned_quadruple_gen_var_5, unsigned_triple_gen_var_12,
};
use malachite_base::test_util::num::arithmetic::mod_mul::limbs_invert_limb_naive;
use std::panic::catch_unwind;

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
        assert_eq!(limbs_invert_limb_u32(x), out);
        assert_eq!(limbs_invert_limb_naive::<u32, u64>(x), out);
    };
    test(0x80000000, u32::MAX);
    test(0x80000001, 0xfffffffc);
    test(u32::MAX - 1, 2);
    test(u32::MAX, 1);
    test(0x89abcdef, 0xdc08767e);
}

#[test]
#[should_panic]
fn limbs_invert_limb_u32_fail() {
    limbs_invert_limb_u32(123);
}

#[test]
fn test_limbs_invert_limb_u64() {
    let test = |x, out| {
        assert_eq!(limbs_invert_limb_u64(x), out);
        assert_eq!(limbs_invert_limb_naive::<u64, u128>(x), out);
    };
    test(0x8000000000000000, u64::MAX);
    test(0x8000000000000001, 0xfffffffffffffffc);
    test(0xfffffffffffffffe, 2);
    test(u64::MAX, 1);
    test(0x89abcdefedcba987, 0xdc08767b33d7ec8f);
}

#[test]
#[should_panic]
fn limbs_invert_limb_u64_fail() {
    limbs_invert_limb_u64(123);
}

#[test]
fn test_limbs_mod_preinverted() {
    fn test<
        T: TryFrom<DT> + PrimitiveUnsigned,
        DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
    >(
        x_1: T,
        x_0: T,
        d: T,
        out: T,
    ) {
        let d_inv = limbs_invert_limb_naive::<T, DT>(d << LeadingZeros::leading_zeros(d));
        assert_eq!(limbs_mod_preinverted::<T, DT>(x_1, x_0, d, d_inv), out);
        assert_eq!(T::exact_from(DT::join_halves(x_1, x_0) % DT::from(d)), out);
    }
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

        assert_eq!(naive_mod_mul(x, y, m), out);
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

fn mod_mul_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_mul(T::ZERO, T::ZERO));
    assert_panic!(T::from(123u8).mod_mul(T::from(200u8), T::from(200u8)));
    assert_panic!(T::from(200u8).mod_mul(T::from(123u8), T::from(200u8)));
}

#[test]
fn mod_mul_fail() {
    apply_fn_to_unsigneds!(mod_mul_fail_helper);
}

fn mod_mul_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_mul_assign(T::ZERO, T::ZERO);
    });
    assert_panic!({
        let mut x = T::from(123u8);
        x.mod_mul_assign(T::from(200u8), T::from(200u8));
    });
    assert_panic!({
        let mut x = T::from(200u8);
        x.mod_mul_assign(T::from(123u8), T::from(200u8));
    });
}

#[test]
fn mod_mul_assign_fail() {
    apply_fn_to_unsigneds!(mod_mul_assign_fail_helper);
}

#[test]
fn invert_limb_u32_properties() {
    unsigned_gen_var_12().test_properties(|x| {
        let inverse = limbs_invert_limb_u32(x);
        assert_eq!(limbs_invert_limb_naive::<u32, u64>(x), inverse);
        assert_ne!(inverse, 0);
    });
}

#[test]
fn invert_limb_u64_properties() {
    unsigned_gen_var_12().test_properties(|x| {
        let inverse = limbs_invert_limb_u64(x);
        assert_eq!(limbs_invert_limb_naive::<u64, u128>(x), inverse);
        assert_ne!(inverse, 0);
    });
}

fn mod_mul_preinverted_properties_helper<
    T: TryFrom<DT> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>() {
    unsigned_quadruple_gen_var_5::<T, DT>().test_properties(|(x_1, x_0, d, d_inv)| {
        let r = limbs_mod_preinverted::<T, DT>(x_1, x_0, d, d_inv);
        let n = DT::join_halves(x_1, x_0);
        assert_eq!(T::exact_from(n % DT::from(d)), r);
        assert!(r < d);
        let q = DT::join_halves(x_1, x_0) / DT::from(d);
        assert_eq!(q * DT::from(d) + DT::from(r), n);
    });
}

#[test]
fn mod_mul_preinverted_properties() {
    mod_mul_preinverted_properties_helper::<u8, u16>();
    mod_mul_preinverted_properties_helper::<u16, u32>();
    mod_mul_preinverted_properties_helper::<u32, u64>();
    mod_mul_preinverted_properties_helper::<u64, u128>();
}

fn mod_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_12::<T>().test_properties(|(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let product = x.mod_mul(y, m);
        assert!(product.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_mul_assign(y, m);
        assert_eq!(x_alt, product);

        let data = T::precompute_mod_mul_data(&m);

        assert_eq!(x.mod_mul_precomputed(y, m, &data), product);

        let mut x_alt = x;
        x_alt.mod_mul_precomputed_assign(y, m, &data);
        assert_eq!(x_alt, product);

        assert_eq!(naive_mod_mul(x, y, m), product);
        assert_eq!(y.mod_mul(x, m), product);
        assert_eq!(x.mod_mul(y.mod_neg(m), m), product.mod_neg(m));
        assert_eq!(x.mod_neg(m).mod_mul(y, m), product.mod_neg(m));
    });

    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        assert_eq!(x.mod_mul(T::ZERO, m), T::ZERO);
        assert_eq!(T::ZERO.mod_mul(x, m), T::ZERO);
        if m > T::ONE {
            assert_eq!(x.mod_mul(T::ONE, m), x);
            assert_eq!(T::ONE.mod_mul(x, m), x);
        }
    });

    unsigned_quadruple_gen_var_4::<T>().test_properties(|(x, y, z, m)| {
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
    T: TryFrom<DT> + ModMulPrecomputed<Data = T> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>() {
    unsigned_triple_gen_var_12::<T>().test_properties(|(x, y, m)| {
        let product = x.mod_mul(y, m);
        assert_eq!(
            fast_mod_mul::<T, DT>(x, y, m, T::precompute_mod_mul_data(&m)),
            product
        );
    });
}

#[test]
fn mod_mul_properties() {
    apply_fn_to_unsigneds!(mod_mul_properties_helper);

    mod_mul_properties_fast_helper::<u32, u64>();
    mod_mul_properties_fast_helper::<u64, u128>();
}
