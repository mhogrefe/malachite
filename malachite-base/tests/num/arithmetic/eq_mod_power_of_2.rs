// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::generators::{
    signed_pair_gen, signed_signed_signed_unsigned_quadruple_gen_var_1,
    signed_signed_unsigned_triple_gen_var_1, signed_signed_unsigned_triple_gen_var_2,
    signed_signed_unsigned_triple_gen_var_3, signed_unsigned_pair_gen_var_1,
    unsigned_pair_gen_var_2, unsigned_pair_gen_var_27, unsigned_quadruple_gen_var_2,
    unsigned_triple_gen_var_10, unsigned_triple_gen_var_4, unsigned_triple_gen_var_9,
};

fn eq_mod_power_of_2_primitive_helper<T: PrimitiveInt>() {
    let test = |n: T, other, pow, out| {
        assert_eq!(n.eq_mod_power_of_2(other, pow), out);
    };
    test(T::ZERO, T::power_of_2(T::WIDTH >> 1), T::WIDTH >> 1, true);
    test(
        T::ZERO,
        T::power_of_2(T::WIDTH >> 1),
        (T::WIDTH >> 1) + 1,
        false,
    );
    test(T::exact_from(13), T::exact_from(21), 0, true);
    test(T::exact_from(13), T::exact_from(21), 1, true);
    test(T::exact_from(13), T::exact_from(21), 2, true);
    test(T::exact_from(13), T::exact_from(21), 3, true);
    test(T::exact_from(13), T::exact_from(21), 4, false);
    test(T::exact_from(13), T::exact_from(21), 100, false);
    test(T::MAX, T::MAX, T::WIDTH, true);
    test(T::MAX, T::MAX, 100, true);
    if T::WIDTH >= u64::WIDTH {
        test(T::exact_from(1000000000001u64), T::ONE, 12, true);
        test(T::exact_from(1000000000001u64), T::ONE, 13, false);
        test(
            T::exact_from(281474976710672u64),
            T::exact_from(844424930131984u64),
            49,
            true,
        );
        test(
            T::exact_from(281474976710672u64),
            T::exact_from(844424930131984u64),
            50,
            false,
        );
    }
}

fn eq_mod_power_of_2_signed_helper<T: PrimitiveSigned>() {
    let test = |n: T, other, pow, out| {
        assert_eq!(n.eq_mod_power_of_2(other, pow), out);
    };
    test(T::ZERO, -T::power_of_2(T::WIDTH >> 1), T::WIDTH >> 1, true);
    test(
        T::ZERO,
        -T::power_of_2(T::WIDTH >> 1),
        (T::WIDTH >> 1) + 1,
        false,
    );
    test(T::exact_from(-13), T::exact_from(27), 0, true);
    test(T::exact_from(-13), T::exact_from(27), 1, true);
    test(T::exact_from(-13), T::exact_from(27), 2, true);
    test(T::exact_from(-13), T::exact_from(27), 3, true);
    test(T::exact_from(-13), T::exact_from(27), 4, false);
    test(T::exact_from(-13), T::exact_from(27), 100, false);
    test(T::exact_from(13), T::exact_from(-27), 0, true);
    test(T::exact_from(13), T::exact_from(-27), 1, true);
    test(T::exact_from(13), T::exact_from(-27), 2, true);
    test(T::exact_from(13), T::exact_from(-27), 3, true);
    test(T::exact_from(13), T::exact_from(-27), 4, false);
    test(T::exact_from(13), T::exact_from(-27), 100, false);
    test(
        T::NEGATIVE_ONE,
        T::power_of_2(T::WIDTH >> 1) - T::ONE,
        T::WIDTH >> 1,
        true,
    );
    test(
        T::power_of_2(T::WIDTH >> 1) - T::ONE,
        T::NEGATIVE_ONE,
        T::WIDTH >> 1,
        true,
    );
    if T::WIDTH >= u64::WIDTH {
        test(
            T::exact_from(-1000000000001i64),
            T::exact_from(4095),
            13,
            true,
        );
        test(
            T::exact_from(-1000000000001i64),
            T::exact_from(4095),
            14,
            false,
        );
        test(
            T::exact_from(1000000000001i64),
            T::exact_from(-4095),
            13,
            true,
        );
        test(
            T::exact_from(1000000000001i64),
            T::exact_from(-4095),
            14,
            false,
        );
    }

    test(T::exact_from(-13), T::exact_from(-21), 0, true);
    test(T::exact_from(-13), T::exact_from(-21), 1, true);
    test(T::exact_from(-13), T::exact_from(-21), 2, true);
    test(T::exact_from(-13), T::exact_from(-21), 3, true);
    test(T::exact_from(-13), T::exact_from(-21), 4, false);
    test(T::exact_from(-13), T::exact_from(-21), 100, false);
    test(
        T::power_of_2(T::WIDTH >> 1) - T::ONE,
        T::power_of_2(T::WIDTH >> 1) - T::ONE,
        T::WIDTH >> 1,
        true,
    );
    if T::WIDTH >= u64::WIDTH {
        test(T::exact_from(-1000000000001i64), T::NEGATIVE_ONE, 12, true);
        test(T::exact_from(-1000000000001i64), T::NEGATIVE_ONE, 13, false);
        test(
            T::exact_from(-281474976710672i64),
            T::exact_from(-844424930131984i64),
            49,
            true,
        );
        test(
            T::exact_from(-281474976710672i64),
            T::exact_from(-844424930131984i64),
            50,
            false,
        );
    }

    if T::WIDTH >= u128::WIDTH {
        test(
            T::exact_from(1311693408901639117i128),
            T::exact_from(-17135050664807912499i128),
            64,
            true,
        );
        test(
            T::exact_from(1311693408901639117i128),
            T::exact_from(-17135050663395328000i128),
            64,
            false,
        );
        test(
            T::exact_from(1311693408901639117i128),
            T::exact_from(-17135050664807912499i128),
            65,
            false,
        );
        test(
            T::exact_from(1311693408901639117i128),
            T::exact_from(-17135050664807912499i128),
            128,
            false,
        );
        test(
            T::exact_from(5633680281231555440641310720i128),
            T::exact_from(-5634717283396403096794955776i128),
            80,
            true,
        );

        test(
            T::exact_from(-1311693408901639117i128),
            T::exact_from(17135050664807912499i128),
            64,
            true,
        );
        test(
            T::exact_from(-1311693408901639117i128),
            T::exact_from(17135050663395328000i128),
            64,
            false,
        );
        test(
            T::exact_from(-1311693408901639117i128),
            T::exact_from(17135050664807912499i128),
            65,
            false,
        );
        test(
            T::exact_from(-1311693408901639117i128),
            T::exact_from(17135050664807912499i128),
            128,
            false,
        );
        test(
            T::exact_from(-5633680281231555440641310720i128),
            T::exact_from(5634717283396403096794955776i128),
            80,
            true,
        );
    }
}

#[test]
fn test_eq_mod_power_of_2() {
    apply_fn_to_primitive_ints!(eq_mod_power_of_2_primitive_helper);
    apply_fn_to_signeds!(eq_mod_power_of_2_signed_helper);
}

fn eq_mod_power_of_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_4::<T, u64>().test_properties(|(x, y, pow)| {
        let eq_mod_power_of_2 = x.eq_mod_power_of_2(y, pow);
        assert_eq!(y.eq_mod_power_of_2(x, pow), eq_mod_power_of_2);
        assert_eq!(
            x.mod_power_of_2(pow) == y.mod_power_of_2(pow),
            eq_mod_power_of_2
        );
    });

    unsigned_triple_gen_var_9::<T>().test_properties(|(x, y, pow)| {
        assert!(x.eq_mod_power_of_2(y, pow));
        assert!(y.eq_mod_power_of_2(x, pow));
        assert_eq!(x.mod_power_of_2(pow), y.mod_power_of_2(pow));
    });

    unsigned_triple_gen_var_10::<T>().test_properties(|(x, y, pow)| {
        assert!(!x.eq_mod_power_of_2(y, pow));
        assert!(!y.eq_mod_power_of_2(x, pow));
        assert_ne!(x.mod_power_of_2(pow), y.mod_power_of_2(pow));
    });

    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(n, pow)| {
        assert!(n.eq_mod_power_of_2(n, pow));
        assert_eq!(
            n.eq_mod_power_of_2(T::ZERO, pow),
            n.divisible_by_power_of_2(pow)
        );
        assert_eq!(
            T::ZERO.eq_mod_power_of_2(n, pow),
            n.divisible_by_power_of_2(pow)
        );
    });

    unsigned_quadruple_gen_var_2::<T, u64>().test_properties(|(x, y, z, pow)| {
        if x.eq_mod_power_of_2(y, pow) && y.eq_mod_power_of_2(z, pow) {
            assert!(x.eq_mod_power_of_2(z, pow));
        }
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert!(x.eq_mod_power_of_2(y, 0));
    });
}

fn eq_mod_power_of_2_properties_helper_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    signed_signed_unsigned_triple_gen_var_2::<S, u64>().test_properties(|(x, y, pow)| {
        let eq_mod_power_of_2 = x.eq_mod_power_of_2(y, pow);
        assert_eq!(y.eq_mod_power_of_2(x, pow), eq_mod_power_of_2);
        assert_eq!(
            U::wrapping_from(x).mod_power_of_2(pow) == U::wrapping_from(y).mod_power_of_2(pow),
            eq_mod_power_of_2,
        );
    });

    signed_signed_unsigned_triple_gen_var_1::<U, S>().test_properties(|(x, y, pow)| {
        assert!(x.eq_mod_power_of_2(y, pow));
        assert!(y.eq_mod_power_of_2(x, pow));
        assert_eq!(
            U::wrapping_from(x).mod_power_of_2(pow),
            U::wrapping_from(y).mod_power_of_2(pow),
        );
    });

    signed_signed_unsigned_triple_gen_var_3::<S>().test_properties(|(x, y, pow)| {
        assert!(!x.eq_mod_power_of_2(y, pow));
        assert!(!y.eq_mod_power_of_2(x, pow));
        assert_ne!(
            U::wrapping_from(x).mod_power_of_2(pow),
            U::wrapping_from(y).mod_power_of_2(pow),
        );
    });

    signed_unsigned_pair_gen_var_1::<S, u64>().test_properties(|(n, pow)| {
        assert!(n.eq_mod_power_of_2(n, pow));
        assert_eq!(
            n.eq_mod_power_of_2(S::ZERO, pow),
            n.divisible_by_power_of_2(pow)
        );
        assert_eq!(
            S::ZERO.eq_mod_power_of_2(n, pow),
            n.divisible_by_power_of_2(pow)
        );
    });

    signed_signed_signed_unsigned_quadruple_gen_var_1::<S, u64>().test_properties(
        |(x, y, z, pow)| {
            if x.eq_mod_power_of_2(y, pow) && y.eq_mod_power_of_2(z, pow) {
                assert!(x.eq_mod_power_of_2(z, pow));
            }
        },
    );

    signed_pair_gen::<S>().test_properties(|(x, y)| {
        assert!(x.eq_mod_power_of_2(y, 0));
    });
}

#[test]
fn eq_mod_power_of_2_properties() {
    apply_fn_to_unsigneds!(eq_mod_power_of_2_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(eq_mod_power_of_2_properties_helper_signed);
}
