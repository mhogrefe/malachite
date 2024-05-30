// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Mod, ModShl};
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModShr, ModShrAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::test_util::generators::{
    signed_gen_var_5, unsigned_signed_unsigned_triple_gen_var_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_signed_triple_gen_var_1, natural_pair_gen_var_8, natural_signed_pair_gen_var_3,
};
use std::ops::Shr;
use std::panic::catch_unwind;
use std::str::FromStr;

macro_rules! test_mod_shr_signed {
    ($t:ident) => {
        let test = |s, v: $t, t, out| {
            let u = Natural::from_str(s).unwrap();
            let m = Natural::from_str(t).unwrap();

            let mut n = u.clone();
            assert!(n.mod_is_reduced(&m));
            n.mod_shr_assign(v, m.clone());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);
            assert!(n.mod_is_reduced(&m));

            let mut n = u.clone();
            n.mod_shr_assign(v, &m);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = u.clone().mod_shr(v, m.clone());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = u.clone().mod_shr(v, &m);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&u).mod_shr(v, m.clone());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&u).mod_shr(v, &m);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            assert_eq!(((u >> v) % m).to_string(), out);
        };
        test("0", 0, "1", "0");
        test("0", 0, "5", "0");
        test("8", -2, "10", "2");
        test("10", -100, "17", "7");
        test("10", 100, "19", "0");
        test("123456", -100, "12345678987654321", "7436663564915145");
    };
}

#[test]
fn test_mod_shr() {
    apply_to_signeds!(test_mod_shr_signed);
}

fn mod_shr_fail_helper<T: PrimitiveSigned>()
where
    for<'a> Natural: ModShrAssign<T, Natural>
        + ModShrAssign<T, &'a Natural>
        + ModShr<T, Natural, Output = Natural>
        + ModShr<T, &'a Natural, Output = Natural>,
    for<'a, 'b> &'a Natural:
        ModShr<T, Natural, Output = Natural> + ModShr<T, &'b Natural, Output = Natural>,
{
    assert_panic!(Natural::ZERO.mod_shr(T::exact_from(3u8), Natural::ZERO));
    assert_panic!(Natural::exact_from(30).mod_shr(T::exact_from(3u8), Natural::ONE));

    assert_panic!(Natural::ZERO.mod_shr(T::exact_from(3u8), &Natural::ZERO));
    assert_panic!(Natural::exact_from(30).mod_shr(T::exact_from(3u8), &Natural::ONE));

    assert_panic!((&Natural::ZERO).mod_shr(T::exact_from(3u8), Natural::ZERO));
    assert_panic!((&Natural::exact_from(30)).mod_shr(T::exact_from(3u8), Natural::ONE));

    assert_panic!((&Natural::ZERO).mod_shr(T::exact_from(3u8), &Natural::ZERO));
    assert_panic!((&Natural::exact_from(30)).mod_shr(T::exact_from(3u8), &Natural::ONE));

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_shr_assign(T::exact_from(3u8), Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::exact_from(30);
        x.mod_shr_assign(T::exact_from(3u8), Natural::ONE);
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_shr_assign(T::exact_from(3u8), &Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::exact_from(30);
        x.mod_shr_assign(T::exact_from(3u8), &Natural::ONE);
    });
}

#[test]
fn mod_shr_fail() {
    apply_fn_to_signeds!(mod_shr_fail_helper);
}

#[allow(clippy::trait_duplication_in_bounds)]
fn properties_helper<U: PrimitiveUnsigned + WrappingFrom<T>, T: PrimitiveSigned + WrappingFrom<U>>()
where
    for<'a> Natural: ModShrAssign<T>
        + ModShrAssign<T, &'a Natural>
        + ModShr<T, Output = Natural>
        + ModShr<T, &'a Natural, Output = Natural>
        + ModShl<T, Output = Natural>,
    for<'a, 'b> &'a Natural: ModShr<T, Natural, Output = Natural>
        + ModShr<T, &'b Natural, Output = Natural>
        + Shr<T, Output = Natural>,
    Limb: ModShr<T, Output = Limb>,
{
    natural_natural_signed_triple_gen_var_1::<T>().test_properties(|(n, m, i)| {
        assert!(n.mod_is_reduced(&m));
        let mut mut_n = n.clone();
        mut_n.mod_shr_assign(i, &m);
        assert!(mut_n.is_valid());
        let shifted = mut_n;
        assert!(shifted.mod_is_reduced(&m));

        let mut mut_n = n.clone();
        mut_n.mod_shr_assign(i, m.clone());
        let shifted_alt = mut_n;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        let shifted_alt = (&n).mod_shr(i, &m);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = (&n).mod_shr(i, m.clone());
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().mod_shr(i, &m);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().mod_shr(i, m.clone());
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!((&n >> i).mod_op(&m), shifted);

        if i != T::MIN {
            assert_eq!(n.mod_shl(-i, m), shifted);
        }
    });

    natural_pair_gen_var_8().test_properties(|(n, m)| {
        assert_eq!((&n).mod_shr(T::ZERO, m), n);
    });

    natural_signed_pair_gen_var_3::<T>().test_properties(|(m, i)| {
        assert_eq!(Natural::ZERO.mod_shr(i, m), 0);
    });

    signed_gen_var_5::<T>().test_properties(|i| {
        assert_eq!(Natural::ZERO.mod_shr(i, Natural::ONE), 0);
    });

    unsigned_signed_unsigned_triple_gen_var_2::<Limb, U, T>().test_properties(|(n, i, m)| {
        assert_eq!(
            Natural::from(n).mod_shr(i, Natural::from(m)),
            n.mod_shr(i, m)
        );
    });
}

#[test]
fn mod_shr_properties() {
    apply_fn_to_unsigned_signed_pairs!(properties_helper);
}
