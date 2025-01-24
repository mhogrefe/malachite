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
use malachite_base::test_util::generators::{
    signed_gen, signed_unsigned_pair_gen_var_1, signed_unsigned_pair_gen_var_8,
    signed_unsigned_pair_gen_var_9, unsigned_gen, unsigned_pair_gen_var_14,
    unsigned_pair_gen_var_15, unsigned_pair_gen_var_2,
};

fn divisible_by_power_of_2_primitive_helper<T: PrimitiveInt>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.divisible_by_power_of_2(pow), out);
    };
    test(T::ZERO, 0, true);
    test(T::ZERO, 10, true);
    test(T::ZERO, 100, true);
    test(T::exact_from(123), 0, true);
    test(T::exact_from(123), 1, false);
    if T::WIDTH >= u64::WIDTH {
        test(T::exact_from(1000000000000u64), 0, true);
        test(T::exact_from(1000000000000u64), 12, true);
        test(T::exact_from(1000000000000u64), 13, false);
    }
    test(T::MAX, 0, true);
    test(T::MAX, 1, false);
    test(T::power_of_2(T::WIDTH >> 1), 0, true);
    test(T::power_of_2(T::WIDTH >> 1), T::WIDTH >> 1, true);
    test(T::power_of_2(T::WIDTH >> 1), (T::WIDTH >> 1) + 1, false);
}

fn divisible_by_power_of_2_signed_helper<T: PrimitiveSigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.divisible_by_power_of_2(pow), out);
    };
    test(T::exact_from(-123), 0, true);
    test(T::exact_from(-123), 1, false);
    if T::WIDTH >= u64::WIDTH {
        test(T::exact_from(-1000000000000i64), 0, true);
        test(T::exact_from(-1000000000000i64), 12, true);
        test(T::exact_from(-1000000000000i64), 13, false);
    }
    test(T::MIN + T::ONE, 0, true);
    test(T::MIN + T::ONE, 1, false);
    test(T::MIN, 0, true);
    test(T::MIN, T::WIDTH - 1, true);
    test(T::MIN, T::WIDTH, false);
}

#[test]
fn test_divisible_by_power_of_2() {
    apply_fn_to_primitive_ints!(divisible_by_power_of_2_primitive_helper);
    apply_fn_to_signeds!(divisible_by_power_of_2_signed_helper);
}

fn divisible_by_power_of_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(x, pow)| {
        let divisible = x.divisible_by_power_of_2(pow);
        if x != T::ZERO {
            assert_eq!(x.trailing_zeros() >= pow, divisible);
        }
    });

    unsigned_pair_gen_var_15::<T>().test_properties(|(x, pow)| {
        assert!(x.divisible_by_power_of_2(pow));
        if x != T::ZERO {
            assert!(x.trailing_zeros() >= pow);
        }
    });

    unsigned_pair_gen_var_14::<T, u64>().test_properties(|(x, pow)| {
        assert!(!x.divisible_by_power_of_2(pow));
        if x != T::ZERO {
            assert!(x.trailing_zeros() < pow);
        }
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert!(x.divisible_by_power_of_2(0));
    });

    unsigned_gen().test_properties(|pow| {
        assert!(T::ZERO.divisible_by_power_of_2(pow));
    });
}

fn divisible_by_power_of_2_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, pow)| {
        let divisible = x.divisible_by_power_of_2(pow);
        if x != T::ZERO {
            assert_eq!(x.trailing_zeros() >= pow, divisible);
        }
        if x != T::MIN {
            assert_eq!((-x).divisible_by_power_of_2(pow), divisible);
        }
    });

    signed_unsigned_pair_gen_var_9::<T, u64>().test_properties(|(x, pow)| {
        assert!(x.divisible_by_power_of_2(pow));
        if x != T::ZERO {
            assert!(x.trailing_zeros() >= pow);
        }
        if x != T::MIN {
            assert!((-x).divisible_by_power_of_2(pow));
        }
    });

    signed_unsigned_pair_gen_var_8::<T, u64>().test_properties(|(x, pow)| {
        assert!(!x.divisible_by_power_of_2(pow));
        if x != T::ZERO {
            assert!(x.trailing_zeros() < pow);
        }
        if x != T::MIN {
            assert!(!(-x).divisible_by_power_of_2(pow));
        }
    });

    signed_gen::<T>().test_properties(|x| {
        assert!(x.divisible_by_power_of_2(0));
    });

    unsigned_gen().test_properties(|pow| {
        assert!(T::ZERO.divisible_by_power_of_2(pow));
    });
}

#[test]
fn divisible_by_power_of_2_properties() {
    apply_fn_to_unsigneds!(divisible_by_power_of_2_properties_helper_unsigned);
    apply_fn_to_signeds!(divisible_by_power_of_2_properties_helper_signed);
}
