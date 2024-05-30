// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2Shr,
    ModPowerOf2ShrAssign,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_14, unsigned_signed_unsigned_triple_gen_var_1,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_signed_unsigned_triple_gen_var_1, natural_unsigned_pair_gen_var_11,
};
use std::ops::Shr;
use std::panic::catch_unwind;
use std::str::FromStr;

macro_rules! test_mod_power_of_2_shr {
    ($t:ident) => {
        let test = |s, v: $t, pow, out| {
            let u = Natural::from_str(s).unwrap();

            let mut n = u.clone();
            assert!(n.mod_power_of_2_is_reduced(pow));
            n.mod_power_of_2_shr_assign(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);
            assert!(n.mod_power_of_2_is_reduced(pow));

            let n = u.clone().mod_power_of_2_shr(v, pow);
            assert!(n.is_valid());

            let n = (&u).mod_power_of_2_shr(v, pow);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            assert_eq!((u >> v).mod_power_of_2(pow).to_string(), out);
        };
        test("0", -10, 0, "0");
        test("0", -10, 8, "0");
        test("123", -5, 8, "96");
        test("123", -100, 80, "0");
        test("123", 2, 8, "30");
        test("123", 10, 8, "0");
    };
}

#[test]
fn test_mod_power_of_2_shr() {
    apply_to_signeds!(test_mod_power_of_2_shr);
}

fn mod_power_of_2_shr_fail_helper<T: PrimitiveSigned>()
where
    Natural: ModPowerOf2Shr<T, Output = Natural> + ModPowerOf2ShrAssign<T>,
    for<'a> &'a Natural: ModPowerOf2Shr<T, Output = Natural>,
{
    assert_panic!(Natural::ONE.mod_power_of_2_shr(T::exact_from(3u8), 0));
    assert_panic!((&Natural::ONE).mod_power_of_2_shr(T::exact_from(3u8), 0));
    assert_panic!({
        let mut x = Natural::ONE;
        x.mod_power_of_2_shr_assign(T::exact_from(3u8), 0);
    });
}

#[test]
fn mod_power_of_2_shr_fail() {
    apply_fn_to_signeds!(mod_power_of_2_shr_fail_helper);
}

fn properties_helper<T: PrimitiveSigned>()
where
    Natural: ModPowerOf2Shr<T, Output = Natural>
        + ModPowerOf2Shl<T, Output = Natural>
        + ModPowerOf2ShrAssign<T>,
    for<'a> &'a Natural: ModPowerOf2Shr<T, Output = Natural>
        + ModPowerOf2Shl<T, Output = Natural>
        + Shr<T, Output = Natural>,
    Limb: ModPowerOf2Shr<T, Output = Limb>,
{
    natural_signed_unsigned_triple_gen_var_1::<T>().test_properties(|(n, i, pow)| {
        assert!(n.mod_power_of_2_is_reduced(pow));
        let mut mut_n = n.clone();
        mut_n.mod_power_of_2_shr_assign(i, pow);
        assert!(mut_n.is_valid());
        let shifted = mut_n;
        assert!(shifted.mod_power_of_2_is_reduced(pow));

        let shifted_alt = (&n).mod_power_of_2_shr(i, pow);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().mod_power_of_2_shr(i, pow);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!((&n >> i).mod_power_of_2(pow), shifted);

        if i != T::MIN {
            assert_eq!(n.mod_power_of_2_shl(-i, pow), shifted);
        }
    });

    natural_unsigned_pair_gen_var_11().test_properties(|(n, pow)| {
        assert_eq!((&n).mod_power_of_2_shr(T::ZERO, pow), n);
    });

    signed_unsigned_pair_gen_var_14::<T, u64>().test_properties(|(i, pow)| {
        assert_eq!(Natural::ZERO.mod_power_of_2_shr(i, pow), 0);
        if pow != 0 {
            let shifted = Natural::ONE.mod_power_of_2_shr(i, pow);
            assert!(shifted == 0 || shifted.is_power_of_2());
        }
    });

    unsigned_signed_unsigned_triple_gen_var_1::<Limb, T>().test_properties(|(n, i, pow)| {
        assert_eq!(
            Natural::from(n).mod_power_of_2_shr(i, pow),
            n.mod_power_of_2_shr(i, pow)
        );
    });
}

#[test]
fn mod_power_of_2_shr_properties() {
    apply_fn_to_signeds!(properties_helper);
}
