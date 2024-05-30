// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2Shl,
    ModPowerOf2ShlAssign, ModPowerOf2Shr,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_14, unsigned_pair_gen_var_28,
    unsigned_signed_unsigned_triple_gen_var_1, unsigned_triple_gen_var_17,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_signed_unsigned_triple_gen_var_1, natural_unsigned_pair_gen_var_11,
    natural_unsigned_unsigned_triple_gen_var_6,
};
use std::ops::Shl;
use std::panic::catch_unwind;
use std::str::FromStr;

macro_rules! test_mod_power_of_2_shl_unsigned {
    ($t:ident) => {
        let test = |s, v: $t, pow, out| {
            let u = Natural::from_str(s).unwrap();

            let mut n = u.clone();
            assert!(n.mod_power_of_2_is_reduced(pow));
            n.mod_power_of_2_shl_assign(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);
            assert!(n.mod_power_of_2_is_reduced(pow));

            let n = u.clone().mod_power_of_2_shl(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&u).mod_power_of_2_shl(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            assert_eq!((u << v).mod_power_of_2(pow).to_string(), out);
        };
        test("0", 10, 0, "0");
        test("0", 10, 8, "0");
        test("123", 5, 8, "96");
        test("123", 100, 80, "0");
    };
}

macro_rules! test_mod_power_of_2_shl_signed {
    ($t:ident) => {
        let test = |s, v: $t, pow, out| {
            let u = Natural::from_str(s).unwrap();

            let mut n = u.clone();
            assert!(n.mod_power_of_2_is_reduced(pow));
            n.mod_power_of_2_shl_assign(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);
            assert!(n.mod_power_of_2_is_reduced(pow));

            let n = u.clone().mod_power_of_2_shl(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&u).mod_power_of_2_shl(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            assert_eq!((u << v).mod_power_of_2(pow).to_string(), out);
        };
        test("0", 10, 0, "0");
        test("0", 10, 8, "0");
        test("123", 5, 8, "96");
        test("123", 100, 80, "0");
        test("123", -2, 8, "30");
        test("123", -10, 8, "0");
    };
}

#[test]
fn test_mod_power_of_2_shl() {
    apply_to_unsigneds!(test_mod_power_of_2_shl_unsigned);
    apply_to_signeds!(test_mod_power_of_2_shl_signed);
}

fn mod_power_of_2_shl_fail_helper<T: PrimitiveInt>()
where
    Natural: ModPowerOf2Shl<T, Output = Natural> + ModPowerOf2ShlAssign<T>,
    for<'a> &'a Natural: ModPowerOf2Shl<T, Output = Natural>,
{
    assert_panic!(Natural::ONE.mod_power_of_2_shl(T::exact_from(3u8), 0));
    assert_panic!((&Natural::ONE).mod_power_of_2_shl(T::exact_from(3u8), 0));
    assert_panic!({
        let mut x = Natural::ONE;
        x.mod_power_of_2_shl_assign(T::exact_from(3u8), 0);
    });
}

#[test]
fn mod_power_of_2_shl_fail() {
    apply_fn_to_primitive_ints!(mod_power_of_2_shl_fail_helper);
}

fn unsigned_properties<T: PrimitiveUnsigned>()
where
    Natural: ModPowerOf2Shl<T, Output = Natural> + ModPowerOf2ShlAssign<T>,
    for<'a> &'a Natural: ModPowerOf2Shl<T, Output = Natural> + Shl<T, Output = Natural>,
    Limb: ModPowerOf2Shl<T, Output = Limb>,
{
    natural_unsigned_unsigned_triple_gen_var_6::<T>().test_properties(|(n, u, pow)| {
        assert!(n.mod_power_of_2_is_reduced(pow));
        let mut mut_n = n.clone();
        mut_n.mod_power_of_2_shl_assign(u, pow);
        assert!(mut_n.is_valid());
        let shifted = mut_n;
        assert!(shifted.mod_power_of_2_is_reduced(pow));

        let shifted_alt = (&n).mod_power_of_2_shl(u, pow);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().mod_power_of_2_shl(u, pow);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!((&n << u).mod_power_of_2(pow), shifted);
        assert_eq!(
            (&n).mod_power_of_2_neg(pow).mod_power_of_2_shl(u, pow),
            n.mod_power_of_2_shl(u, pow).mod_power_of_2_neg(pow)
        );
    });

    natural_unsigned_pair_gen_var_11().test_properties(|(n, pow)| {
        assert_eq!((&n).mod_power_of_2_shl(T::ZERO, pow), n);
    });

    unsigned_pair_gen_var_28::<T, u64>().test_properties(|(u, pow)| {
        assert_eq!(Natural::ZERO.mod_power_of_2_shl(u, pow), 0);
        if pow != 0 {
            let shifted = Natural::ONE.mod_power_of_2_shl(u, pow);
            assert!(shifted == 0 || shifted.is_power_of_2());
        }
    });

    unsigned_triple_gen_var_17::<Limb, T>().test_properties(|(n, u, pow)| {
        assert_eq!(
            Natural::from(n).mod_power_of_2_shl(u, pow),
            n.mod_power_of_2_shl(u, pow)
        );
    });
}

fn signed_properties<T: PrimitiveSigned>()
where
    Natural: ModPowerOf2Shl<T, Output = Natural>
        + ModPowerOf2Shr<T, Output = Natural>
        + ModPowerOf2ShlAssign<T>,
    for<'a> &'a Natural: ModPowerOf2Shl<T, Output = Natural>
        + ModPowerOf2Shr<T, Output = Natural>
        + Shl<T, Output = Natural>,
    Limb: ModPowerOf2Shl<T, Output = Limb>,
{
    natural_signed_unsigned_triple_gen_var_1::<T>().test_properties(|(n, i, pow)| {
        assert!(n.mod_power_of_2_is_reduced(pow));
        let mut mut_n = n.clone();
        mut_n.mod_power_of_2_shl_assign(i, pow);
        assert!(mut_n.is_valid());
        let shifted = mut_n;
        assert!(shifted.mod_power_of_2_is_reduced(pow));

        let shifted_alt = (&n).mod_power_of_2_shl(i, pow);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().mod_power_of_2_shl(i, pow);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!((&n << i).mod_power_of_2(pow), shifted);

        if i != T::MIN {
            assert_eq!(n.mod_power_of_2_shr(-i, pow), shifted);
        }
    });

    natural_unsigned_pair_gen_var_11().test_properties(|(n, pow)| {
        assert_eq!((&n).mod_power_of_2_shl(T::ZERO, pow), n);
    });

    signed_unsigned_pair_gen_var_14::<T, u64>().test_properties(|(i, pow)| {
        assert_eq!(Natural::ZERO.mod_power_of_2_shl(i, pow), 0);
        if pow != 0 {
            let shifted = Natural::ONE.mod_power_of_2_shl(i, pow);
            assert!(shifted == 0 || shifted.is_power_of_2());
        }
    });

    unsigned_signed_unsigned_triple_gen_var_1::<Limb, T>().test_properties(|(n, i, pow)| {
        assert_eq!(
            Natural::from(n).mod_power_of_2_shl(i, pow),
            n.mod_power_of_2_shl(i, pow)
        );
    });
}

#[test]
fn mod_power_of_2_shl_properties() {
    apply_fn_to_unsigneds!(unsigned_properties);
    apply_fn_to_signeds!(signed_properties);
}
