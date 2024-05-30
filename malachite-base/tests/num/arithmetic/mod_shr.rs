// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShr, ModShl, ModShr, ModShrAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::generators::{
    signed_gen_var_5, signed_unsigned_pair_gen_var_13, unsigned_pair_gen_var_16,
    unsigned_signed_unsigned_triple_gen_var_2,
};
use std::panic::catch_unwind;

#[test]
fn test_mod_shr() {
    fn test<
        T: ModShr<U, T, Output = T> + ModShrAssign<U, T> + PrimitiveUnsigned,
        U: PrimitiveInt,
    >(
        t: T,
        i: U,
        m: T,
        out: T,
    ) {
        assert_eq!(t.mod_shr(i, m), out);

        let mut t = t;
        t.mod_shr_assign(i, m);
        assert_eq!(t, out);
    }
    test::<u64, i8>(0, 0, 1, 0);
    test::<u64, i8>(0, 0, 5, 0);
    test::<u32, i16>(8, -2, 10, 2);
    test::<u16, i32>(10, -100, 17, 7);

    test::<u8, i64>(10, 2, 15, 2);
    test::<u8, i64>(10, 100, 19, 0);
    test::<u128, i8>(10, 100, 19, 0);
}

fn mod_shr_fail_helper<T: PrimitiveUnsigned + ModShr<U, T, Output = T>, U: PrimitiveSigned>() {
    assert_panic!(T::ZERO.mod_shr(U::TWO, T::ZERO));
    assert_panic!(T::from(123u8).mod_shr(U::TWO, T::from(123u8)));
}

#[test]
fn mod_shr_fail() {
    apply_fn_to_unsigneds_and_signeds!(mod_shr_fail_helper);
}

fn mod_shr_assign_fail_helper<T: PrimitiveUnsigned + ModShrAssign<U, T>, U: PrimitiveSigned>() {
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_shr_assign(U::TWO, T::ZERO);
    });
    assert_panic!({
        let mut x = T::from(123u8);
        x.mod_shr_assign(U::TWO, T::from(123u8));
    });
}

#[test]
fn mod_shr_assign_fail() {
    apply_fn_to_unsigneds_and_signeds!(mod_shr_assign_fail_helper);
}

fn mod_shr_properties_helper<
    T: ArithmeticCheckedShr<S, Output = T>
        + ModShl<S, Output = T>
        + ModShr<S, Output = T>
        + ModShrAssign<S>
        + PrimitiveUnsigned,
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    unsigned_signed_unsigned_triple_gen_var_2::<T, U, S>().test_properties(|(n, i, m)| {
        assert!(n.mod_is_reduced(&m));
        let shifted = n.mod_shr(i, m);
        assert!(shifted.mod_is_reduced(&m));

        let mut shifted_alt = n;
        shifted_alt.mod_shr_assign(i, m);
        assert_eq!(shifted_alt, shifted);

        if let Some(shifted_alt) = n.arithmetic_checked_shr(i) {
            assert_eq!(shifted_alt % m, shifted);
        }

        if i != S::MIN {
            assert_eq!(n.mod_shl(-i, m), shifted);
        }
    });

    unsigned_pair_gen_var_16::<T>().test_properties(|(n, m)| {
        assert_eq!(n.mod_shr(S::ZERO, m), n);
    });

    signed_unsigned_pair_gen_var_13::<U, S, T>().test_properties(|(i, m)| {
        assert_eq!(T::ZERO.mod_shr(i, m), T::ZERO);
    });

    signed_gen_var_5::<S>().test_properties(|i| {
        assert_eq!(T::ZERO.mod_shl(i, T::ONE), T::ZERO);
    });
}

#[test]
fn mod_shr_properties() {
    apply_fn_to_unsigneds_and_unsigned_signed_pairs!(mod_shr_properties_helper);
}
