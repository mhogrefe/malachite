// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShr, ModPowerOf2Shl, ModPowerOf2Shr, ModPowerOf2ShrAssign,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_12, unsigned_pair_gen_var_17,
    unsigned_signed_unsigned_triple_gen_var_1,
};
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_shr() {
    fn test<
        T: ModPowerOf2Shr<U, Output = T> + ModPowerOf2ShrAssign<U> + PrimitiveUnsigned,
        U: PrimitiveSigned,
    >(
        t: T,
        u: U,
        pow: u64,
        out: T,
    ) {
        assert_eq!(t.mod_power_of_2_shr(u, pow), out);

        let mut t = t;
        t.mod_power_of_2_shr_assign(u, pow);
        assert_eq!(t, out);
    }
    test::<u64, i8>(0, 0, 0, 0);
    test::<u64, i8>(0, 0, 5, 0);
    test::<u32, i16>(12, -2, 5, 16);
    test::<u16, i32>(10, -100, 4, 0);
    test::<u8, i64>(10, 2, 4, 2);
    test::<u8, i64>(10, 100, 4, 0);
    test::<u128, i8>(10, 100, 4, 0);
}

fn mod_power_of_2_shr_fail_helper<
    T: PrimitiveUnsigned + ModPowerOf2Shr<U, Output = T>,
    U: PrimitiveSigned,
>() {
    assert_panic!(T::ONE.mod_power_of_2_shr(U::TWO, 0));
    assert_panic!(T::from(200u8).mod_power_of_2_shr(U::TWO, 7));
}

#[test]
fn mod_power_of_2_shr_fail() {
    apply_fn_to_unsigneds_and_signeds!(mod_power_of_2_shr_fail_helper);
}

fn mod_power_of_2_shr_assign_fail_helper<
    T: PrimitiveUnsigned + ModPowerOf2ShrAssign<U>,
    U: PrimitiveSigned,
>() {
    assert_panic!({
        let mut x = T::ONE;
        x.mod_power_of_2_shr_assign(U::TWO, 0);
    });
    assert_panic!({
        let mut x = T::from(200u8);
        x.mod_power_of_2_shr_assign(U::TWO, 7);
    });
}

#[test]
fn mod_power_of_2_shr_assign_fail() {
    apply_fn_to_unsigneds_and_signeds!(mod_power_of_2_shr_assign_fail_helper);
}

fn mod_power_of_2_shl_properties_helper<
    T: ArithmeticCheckedShr<U, Output = T>
        + ModPowerOf2Shl<U, Output = T>
        + ModPowerOf2Shr<U, Output = T>
        + ModPowerOf2ShrAssign<U>
        + PrimitiveUnsigned,
    U: PrimitiveSigned,
>() {
    unsigned_signed_unsigned_triple_gen_var_1::<T, U>().test_properties(|(n, i, pow)| {
        assert!(n.mod_power_of_2_is_reduced(pow));
        let shifted = n.mod_power_of_2_shr(i, pow);
        assert!(shifted.mod_power_of_2_is_reduced(pow));

        let mut shifted_alt = n;
        shifted_alt.mod_power_of_2_shr_assign(i, pow);
        assert_eq!(shifted_alt, shifted);

        if let Some(shifted_alt) = n.arithmetic_checked_shr(i) {
            assert_eq!(shifted_alt.mod_power_of_2(pow), shifted);
        }

        if i != U::MIN {
            assert_eq!(n.mod_power_of_2_shl(-i, pow), shifted);
        }
    });

    unsigned_pair_gen_var_17::<T>().test_properties(|(n, pow)| {
        assert_eq!(n.mod_power_of_2_shr(U::ZERO, pow), n);
    });

    signed_unsigned_pair_gen_var_12::<U, T>().test_properties(|(i, pow)| {
        assert_eq!(T::ZERO.mod_power_of_2_shr(i, pow), T::ZERO);
    });
}

#[test]
fn mod_power_of_2_shl_properties() {
    apply_fn_to_unsigneds_and_signeds!(mod_power_of_2_shl_properties_helper);
}
