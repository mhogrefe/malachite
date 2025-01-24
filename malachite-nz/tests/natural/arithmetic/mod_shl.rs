// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModIsReduced, ModNeg, ModShl, ModShlAssign, ModShr};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::test_util::generators::{
    signed_gen_var_5, unsigned_gen_var_5, unsigned_signed_unsigned_triple_gen_var_2,
    unsigned_triple_gen_var_18,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_signed_triple_gen_var_1, natural_natural_unsigned_triple_gen_var_6,
    natural_pair_gen_var_8, natural_signed_pair_gen_var_3, natural_unsigned_pair_gen_var_12,
};
use std::ops::Shl;
use std::panic::catch_unwind;
use std::str::FromStr;

macro_rules! test_mod_shl_unsigned {
    ($t:ident) => {
        let test = |s, v: $t, t, out| {
            let u = Natural::from_str(s).unwrap();
            let m = Natural::from_str(t).unwrap();

            let mut n = u.clone();
            assert!(n.mod_is_reduced(&m));
            n.mod_shl_assign(v, m.clone());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);
            assert!(n.mod_is_reduced(&m));

            let mut n = u.clone();
            n.mod_shl_assign(v, &m);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = u.clone().mod_shl(v, m.clone());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = u.clone().mod_shl(v, &m);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&u).mod_shl(v, m.clone());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&u).mod_shl(v, &m);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            assert_eq!(((u << v) % m).to_string(), out);
        };
        test("0", 0, "1", "0");
        test("0", 0, "5", "0");
        test("8", 2, "10", "2");
        test("10", 100, "17", "7");
        test("123456", 100, "12345678987654321", "7436663564915145");
    };
}

macro_rules! test_mod_shl_signed {
    ($t:ident) => {
        let test = |s, v: $t, t, out| {
            let u = Natural::from_str(s).unwrap();
            let m = Natural::from_str(t).unwrap();

            let mut n = u.clone();
            assert!(n.mod_is_reduced(&m));
            n.mod_shl_assign(v, m.clone());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);
            assert!(n.mod_is_reduced(&m));

            let mut n = u.clone();
            n.mod_shl_assign(v, &m);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = u.clone().mod_shl(v, m.clone());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = u.clone().mod_shl(v, &m);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&u).mod_shl(v, m.clone());
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            let n = (&u).mod_shl(v, &m);
            assert!(n.is_valid());
            assert_eq!(n.to_string(), out);

            assert_eq!(((u << v) % m).to_string(), out);
        };
        test("0", 0, "1", "0");
        test("0", 0, "5", "0");
        test("8", 2, "10", "2");
        test("10", 100, "17", "7");
        test("10", -100, "19", "0");
        test("123456", 100, "12345678987654321", "7436663564915145");
    };
}

#[test]
fn test_mod_shl() {
    apply_to_unsigneds!(test_mod_shl_unsigned);
    apply_to_signeds!(test_mod_shl_signed);
}

fn mod_shl_fail_helper<T: PrimitiveInt>()
where
    for<'a> Natural: ModShlAssign<T, Natural>
        + ModShlAssign<T, &'a Natural>
        + ModShl<T, Natural, Output = Natural>
        + ModShl<T, &'a Natural, Output = Natural>,
    for<'a, 'b> &'a Natural:
        ModShl<T, Natural, Output = Natural> + ModShl<T, &'b Natural, Output = Natural>,
{
    assert_panic!(Natural::ZERO.mod_shl(T::exact_from(3u8), Natural::ZERO));
    assert_panic!(Natural::exact_from(30).mod_shl(T::exact_from(3u8), Natural::ONE));

    assert_panic!(Natural::ZERO.mod_shl(T::exact_from(3u8), &Natural::ZERO));
    assert_panic!(Natural::exact_from(30).mod_shl(T::exact_from(3u8), &Natural::ONE));

    assert_panic!((&Natural::ZERO).mod_shl(T::exact_from(3u8), Natural::ZERO));
    assert_panic!((&Natural::exact_from(30)).mod_shl(T::exact_from(3u8), Natural::ONE));

    assert_panic!((&Natural::ZERO).mod_shl(T::exact_from(3u8), &Natural::ZERO));
    assert_panic!((&Natural::exact_from(30)).mod_shl(T::exact_from(3u8), &Natural::ONE));

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_shl_assign(T::exact_from(3u8), Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::exact_from(30);
        x.mod_shl_assign(T::exact_from(3u8), Natural::ONE);
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_shl_assign(T::exact_from(3u8), &Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::exact_from(30);
        x.mod_shl_assign(T::exact_from(3u8), &Natural::ONE);
    });
}

#[test]
fn mod_shl_fail() {
    apply_fn_to_primitive_ints!(mod_shl_fail_helper);
}

#[allow(clippy::trait_duplication_in_bounds)]
fn unsigned_properties<T: PrimitiveUnsigned>()
where
    for<'a> Natural: ModShlAssign<T>
        + ModShlAssign<T, &'a Natural>
        + ModShl<T, Output = Natural>
        + ModShl<T, &'a Natural, Output = Natural>,
    for<'a, 'b> &'a Natural: ModShl<T, Natural, Output = Natural>
        + ModShl<T, &'b Natural, Output = Natural>
        + Shl<T, Output = Natural>,
    Limb: ModShl<T, Output = Limb>,
{
    natural_natural_unsigned_triple_gen_var_6::<T>().test_properties(|(n, m, u)| {
        assert!(n.mod_is_reduced(&m));
        let mut mut_n = n.clone();
        mut_n.mod_shl_assign(u, &m);
        assert!(mut_n.is_valid());
        let shifted = mut_n;
        assert!(shifted.mod_is_reduced(&m));

        let mut mut_n = n.clone();
        mut_n.mod_shl_assign(u, m.clone());
        let shifted_alt = mut_n;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        let shifted_alt = (&n).mod_shl(u, &m);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = (&n).mod_shl(u, m.clone());
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().mod_shl(u, &m);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().mod_shl(u, m.clone());
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!((&n << u) % &m, shifted);
        assert_eq!(
            (&n).mod_neg(&m).mod_shl(u, &m),
            n.mod_shl(u, &m).mod_neg(&m)
        );
    });

    natural_pair_gen_var_8().test_properties(|(n, m)| {
        assert_eq!((&n).mod_shl(T::ZERO, m), n);
    });

    natural_unsigned_pair_gen_var_12::<T>().test_properties(|(m, u)| {
        assert_eq!(Natural::ZERO.mod_shl(u, m), 0);
    });

    unsigned_gen_var_5::<T>().test_properties(|u| {
        assert_eq!(Natural::ZERO.mod_shl(u, Natural::ONE), 0);
    });

    unsigned_triple_gen_var_18::<Limb, T>().test_properties(|(n, u, m)| {
        assert_eq!(
            Natural::from(n).mod_shl(u, Natural::from(m)),
            n.mod_shl(u, m)
        );
    });
}

#[allow(clippy::trait_duplication_in_bounds)]
fn signed_properties<U: PrimitiveUnsigned + WrappingFrom<T>, T: PrimitiveSigned + WrappingFrom<U>>()
where
    for<'a> Natural: ModShlAssign<T>
        + ModShlAssign<T, &'a Natural>
        + ModShl<T, Output = Natural>
        + ModShl<T, &'a Natural, Output = Natural>
        + ModShr<T, Output = Natural>,
    for<'a, 'b> &'a Natural: ModShl<T, Natural, Output = Natural>
        + ModShl<T, &'b Natural, Output = Natural>
        + Shl<T, Output = Natural>,
    Limb: ModShl<T, Output = Limb>,
{
    natural_natural_signed_triple_gen_var_1::<T>().test_properties(|(n, m, i)| {
        assert!(n.mod_is_reduced(&m));
        let mut mut_n = n.clone();
        mut_n.mod_shl_assign(i, &m);
        assert!(mut_n.is_valid());
        let shifted = mut_n;
        assert!(shifted.mod_is_reduced(&m));

        let mut mut_n = n.clone();
        mut_n.mod_shl_assign(i, m.clone());
        let shifted_alt = mut_n;
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        let shifted_alt = (&n).mod_shl(i, &m);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = (&n).mod_shl(i, m.clone());
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().mod_shl(i, &m);
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);
        let shifted_alt = n.clone().mod_shl(i, m.clone());
        assert!(shifted_alt.is_valid());
        assert_eq!(shifted_alt, shifted);

        assert_eq!((&n << i) % &m, shifted);

        if i != T::MIN {
            assert_eq!(n.mod_shr(-i, m), shifted);
        }
    });

    natural_pair_gen_var_8().test_properties(|(n, m)| {
        assert_eq!((&n).mod_shl(T::ZERO, m), n);
    });

    natural_signed_pair_gen_var_3::<T>().test_properties(|(m, i)| {
        assert_eq!(Natural::ZERO.mod_shl(i, m), 0);
    });

    signed_gen_var_5::<T>().test_properties(|i| {
        assert_eq!(Natural::ZERO.mod_shl(i, Natural::ONE), 0);
    });

    unsigned_signed_unsigned_triple_gen_var_2::<Limb, U, T>().test_properties(|(n, i, m)| {
        assert_eq!(
            Natural::from(n).mod_shl(i, Natural::from(m)),
            n.mod_shl(i, m)
        );
    });
}

#[test]
fn mod_shl_properties() {
    apply_fn_to_unsigneds!(unsigned_properties);
    apply_fn_to_unsigned_signed_pairs!(signed_properties);
}
