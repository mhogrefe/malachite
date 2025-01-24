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
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_6, signed_pair_gen_var_4, unsigned_gen, unsigned_gen_var_1,
    unsigned_pair_gen_var_12,
};
use std::panic::catch_unwind;

#[test]
fn test_div_mod_and_div_rem_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, q: T, r: T) {
        assert_eq!(n.div_mod(d), (q, r));

        let mut mut_n = n;
        assert_eq!(mut_n.div_assign_mod(d), r);
        assert_eq!(mut_n, q);

        assert_eq!(n.div_rem(d), (q, r));

        let mut mut_n = n;
        assert_eq!(mut_n.div_assign_rem(d), r);
        assert_eq!(mut_n, q);
    }
    test::<u8>(0, 1, 0, 0);
    test::<u16>(0, 123, 0, 0);
    test::<u32>(1, 1, 1, 0);
    test::<u64>(123, 1, 123, 0);
    test::<usize>(123, 123, 1, 0);
    test::<u128>(123, 456, 0, 123);
    test::<u16>(456, 123, 3, 87);
    test::<u32>(u32::MAX, 1, u32::MAX, 0);
    test::<usize>(0xffffffff, 0xffffffff, 1, 0);
    test::<u64>(1000000000000, 1, 1000000000000, 0);
    test::<u64>(1000000000000, 3, 333333333333, 1);
    test::<u64>(1000000000000, 123, 8130081300, 100);
    test::<u64>(1000000000000, 0xffffffff, 232, 3567587560);
    test::<u128>(1000000000000000000000000, 1, 1000000000000000000000000, 0);
    test::<u128>(1000000000000000000000000, 3, 333333333333333333333333, 1);
    test::<u128>(1000000000000000000000000, 123, 8130081300813008130081, 37);
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        232830643708079,
        3167723695,
    );
    test::<u128>(
        1000000000000000000000000,
        1234567890987,
        810000006723,
        530068894399,
    );
    test::<u128>(
        253640751230376270397812803167,
        2669936877441,
        94998781946290113,
        1520301762334,
    );
    test::<u64>(3768477692975601, 11447376614057827956, 0, 3768477692975601);
    test::<u64>(3356605361737854, 3081095617839357, 1, 275509743898497);
    test::<u128>(
        1098730198198174614195,
        953382298040157850476,
        1,
        145347900158016763719,
    );
    test::<u128>(
        69738658860594537152875081748,
        69738658860594537152875081748,
        1,
        0,
    );
    test::<u128>(1000000000000000000000000, 1000000000000000000000000, 1, 0);
    test::<u128>(0, 1000000000000000000000000, 0, 0);
    test::<u128>(123, 1000000000000000000000000, 0, 123);
}

fn div_mod_and_div_rem_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_12::<T, T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        let r = mut_x.div_assign_mod(y);
        let q = mut_x;

        assert_eq!(x.div_mod(y), (q, r));

        let mut mut_x = x;
        let r_alt = mut_x.div_assign_rem(y);
        let q_alt = mut_x;
        assert_eq!((q_alt, r_alt), (q, r));

        assert_eq!(x.div_rem(y), (q, r));

        assert_eq!((x / y, x % y), (q, r));
        assert!(r < y);
        assert_eq!(q * y + r, x);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.div_mod(T::ONE), (x, T::ZERO));
        assert_panic!(x.div_mod(T::ZERO));
        assert_panic!({
            let mut y = x;
            y.div_assign_mod(T::ZERO)
        });
    });

    unsigned_gen_var_1::<T>().test_properties(|x| {
        assert_eq!(x.div_mod(x), (T::ONE, T::ZERO));
        assert_eq!(T::ZERO.div_mod(x), (T::ZERO, T::ZERO));
        if x > T::ONE {
            assert_eq!(T::ONE.div_mod(x), (T::ZERO, T::ONE));
        }
    });
}

#[test]
fn test_div_mod_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, q: T, r: T) {
        assert_eq!(n.div_mod(d), (q, r));

        let mut mut_n = n;
        assert_eq!(mut_n.div_assign_mod(d), r);
        assert_eq!(mut_n, q);
    }
    test::<i8>(0, 1, 0, 0);
    test::<i16>(0, 123, 0, 0);
    test::<i32>(1, 1, 1, 0);
    test::<i64>(123, 1, 123, 0);
    test::<i128>(123, 123, 1, 0);
    test::<isize>(123, 456, 0, 123);
    test::<i16>(456, 123, 3, 87);
    test::<i64>(0xffffffff, 1, 0xffffffff, 0);
    test::<i64>(0xffffffff, 0xffffffff, 1, 0);
    test::<i64>(1000000000000, 1, 1000000000000, 0);
    test::<i64>(1000000000000, 3, 333333333333, 1);
    test::<i64>(1000000000000, 123, 8130081300, 100);
    test::<i64>(1000000000000, 0xffffffff, 232, 3567587560);
    test::<i128>(1000000000000000000000000, 1, 1000000000000000000000000, 0);
    test::<i128>(1000000000000000000000000, 3, 333333333333333333333333, 1);
    test::<i128>(1000000000000000000000000, 123, 8130081300813008130081, 37);
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        232830643708079,
        3167723695,
    );
    test::<i128>(
        1000000000000000000000000,
        1234567890987,
        810000006723,
        530068894399,
    );
    test::<i128>(
        253640751230376270397812803167,
        2669936877441,
        94998781946290113,
        1520301762334,
    );
    test::<i128>(3768477692975601, 11447376614057827956, 0, 3768477692975601);
    test::<i64>(3356605361737854, 3081095617839357, 1, 275509743898497);
    test::<i128>(
        1098730198198174614195,
        953382298040157850476,
        1,
        145347900158016763719,
    );
    test::<i128>(
        69738658860594537152875081748,
        69738658860594537152875081748,
        1,
        0,
    );
    test::<i128>(1000000000000000000000000, 1000000000000000000000000, 1, 0);
    test::<i128>(0, 1000000000000000000000000, 0, 0);
    test::<i128>(123, 1000000000000000000000000, 0, 123);

    test::<i8>(0, -1, 0, 0);
    test::<i16>(0, -123, 0, 0);
    test::<i32>(1, -1, -1, 0);
    test::<i64>(123, -1, -123, 0);
    test::<i128>(123, -123, -1, 0);
    test::<isize>(123, -456, -1, -333);
    test::<i16>(456, -123, -4, -36);
    test::<i64>(0xffffffff, -1, -0xffffffff, 0);
    test::<i64>(0xffffffff, -0xffffffff, -1, 0);
    test::<i64>(1000000000000, -1, -1000000000000, 0);
    test::<i64>(1000000000000, -3, -333333333334, -2);
    test::<i64>(1000000000000, -123, -8130081301, -23);
    test::<i64>(1000000000000, -0xffffffff, -233, -727379735);
    test::<i128>(1000000000000000000000000, -1, -1000000000000000000000000, 0);
    test::<i128>(1000000000000000000000000, -3, -333333333333333333333334, -2);
    test::<i128>(
        1000000000000000000000000,
        -123,
        -8130081300813008130082,
        -86,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        -232830643708080,
        -1127243600,
    );
    test::<i128>(
        1000000000000000000000000,
        -1234567890987,
        -810000006724,
        -704498996588,
    );
    test::<i128>(
        253640751230376270397812803167,
        -2669936877441,
        -94998781946290114,
        -1149635115107,
    );
    test::<i128>(
        3768477692975601,
        -11447376614057827956,
        -1,
        -11443608136364852355,
    );
    test::<i64>(3356605361737854, -3081095617839357, -2, -2805585873940860);
    test::<i128>(
        1098730198198174614195,
        -953382298040157850476,
        -2,
        -808034397882141086757,
    );
    test::<i128>(
        69738658860594537152875081748,
        -69738658860594537152875081748,
        -1,
        0,
    );
    test::<i128>(1000000000000000000000000, -1000000000000000000000000, -1, 0);
    test::<i128>(0, -1000000000000000000000000, 0, 0);
    test::<i128>(
        123,
        -1000000000000000000000000,
        -1,
        -999999999999999999999877,
    );

    test::<i8>(-1, 1, -1, 0);
    test::<i16>(-123, 1, -123, 0);
    test::<i32>(-123, 123, -1, 0);
    test::<i64>(-123, 456, -1, 333);
    test::<isize>(-456, 123, -4, 36);
    test::<i64>(-0xffffffff, -1, 0xffffffff, 0);
    test::<i64>(-0xffffffff, 0xffffffff, -1, 0);
    test::<i64>(-1000000000000, 1, -1000000000000, 0);
    test::<i64>(-1000000000000, 3, -333333333334, 2);
    test::<i64>(-1000000000000, 123, -8130081301, 23);
    test::<i64>(-1000000000000, 0xffffffff, -233, 727379735);
    test::<i128>(-1000000000000000000000000, 1, -1000000000000000000000000, 0);
    test::<i128>(-1000000000000000000000000, 3, -333333333333333333333334, 2);
    test::<i128>(-1000000000000000000000000, 123, -8130081300813008130082, 86);
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        -232830643708080,
        1127243600,
    );
    test::<i128>(
        -1000000000000000000000000,
        1234567890987,
        -810000006724,
        704498996588,
    );
    test::<i128>(
        -253640751230376270397812803167,
        2669936877441,
        -94998781946290114,
        1149635115107,
    );
    test::<i128>(
        -3768477692975601,
        11447376614057827956,
        -1,
        11443608136364852355,
    );
    test::<i64>(-3356605361737854, 3081095617839357, -2, 2805585873940860);
    test::<i128>(
        -1098730198198174614195,
        953382298040157850476,
        -2,
        808034397882141086757,
    );
    test::<i128>(
        -69738658860594537152875081748,
        69738658860594537152875081748,
        -1,
        0,
    );
    test::<i128>(-1000000000000000000000000, 1000000000000000000000000, -1, 0);
    test::<i128>(
        -123,
        1000000000000000000000000,
        -1,
        999999999999999999999877,
    );

    test::<i8>(-1, -1, 1, 0);
    test::<i16>(-123, -1, 123, 0);
    test::<i32>(-123, -123, 1, 0);
    test::<i64>(-123, -456, 0, -123);
    test::<isize>(-456, -123, 3, -87);
    test::<i128>(-0xffffffff, -1, 0xffffffff, 0);
    test::<i64>(-0xffffffff, -0xffffffff, 1, 0);
    test::<i64>(-1000000000000, -1, 1000000000000, 0);
    test::<i64>(-1000000000000, -3, 333333333333, -1);
    test::<i64>(-1000000000000, -123, 8130081300, -100);
    test::<i64>(-1000000000000, -0xffffffff, 232, -3567587560);
    test::<i128>(-1000000000000000000000000, -1, 1000000000000000000000000, 0);
    test::<i128>(-1000000000000000000000000, -3, 333333333333333333333333, -1);
    test::<i128>(
        -1000000000000000000000000,
        -123,
        8130081300813008130081,
        -37,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        232830643708079,
        -3167723695,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1234567890987,
        810000006723,
        -530068894399,
    );
    test::<i128>(
        -253640751230376270397812803167,
        -2669936877441,
        94998781946290113,
        -1520301762334,
    );
    test::<i128>(
        -3768477692975601,
        -11447376614057827956,
        0,
        -3768477692975601,
    );
    test::<i64>(-3356605361737854, -3081095617839357, 1, -275509743898497);
    test::<i128>(
        -1098730198198174614195,
        -953382298040157850476,
        1,
        -145347900158016763719,
    );
    test::<i128>(
        -69738658860594537152875081748,
        -69738658860594537152875081748,
        1,
        0,
    );
    test::<i128>(-1000000000000000000000000, -1000000000000000000000000, 1, 0);
    test::<i128>(-123, -1000000000000000000000000, 0, -123);
}

fn div_mod_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::ONE.div_mod(T::ZERO));
    assert_panic!({
        let mut n = T::ONE;
        n.div_assign_mod(T::ZERO);
    });
}

fn div_mod_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::MIN.div_mod(T::NEGATIVE_ONE));
    assert_panic!({
        let mut n = T::MIN;
        n.div_assign_mod(T::NEGATIVE_ONE);
    });
}

#[test]
pub fn div_mod_fail() {
    apply_fn_to_primitive_ints!(div_mod_fail_helper);
    apply_fn_to_signeds!(div_mod_signed_fail_helper);
}

fn div_mod_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen_var_4::<T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        let r = mut_x.div_assign_mod(y);
        let q = mut_x;

        assert_eq!(x.div_mod(y), (q, r));

        let (q_alt, r_alt) = (x.div_round(y, Floor).0, x.mod_op(y));
        assert_eq!(q_alt, q);
        assert_eq!(r_alt, r);

        assert!(r.lt_abs(&y));
        assert!(r == T::ZERO || (r > T::ZERO) == (y > T::ZERO));
        if let Some(product) = q.checked_mul(y) {
            assert_eq!(product + r, x);
        } else if q > T::ZERO {
            assert_eq!((q - T::ONE) * y + r + y, x);
        } else {
            assert_eq!((q + T::ONE) * y + r - y, x);
        }
        if x != T::MIN {
            let (neg_q, neg_r) = (-x).div_mod(y);
            assert_eq!(x.ceiling_div_mod(y), (-neg_q, -neg_r));
        }
        if y != T::MIN && (x != T::MIN || y != T::ONE) {
            let (neg_q, r) = x.div_mod(-y);
            assert_eq!(x.ceiling_div_mod(y), (-neg_q, r));
        }
    });

    signed_gen::<T>().test_properties(|x| {
        let (q, r) = x.div_mod(T::ONE);
        assert_eq!(q, x);
        assert_eq!(r, T::ZERO);

        if x != T::MIN {
            let (q, r) = x.div_mod(T::NEGATIVE_ONE);
            assert_eq!(q, -x);
            assert_eq!(r, T::ZERO);
        }
        assert_panic!(x.div_mod(T::ZERO));
        assert_panic!({
            let mut y = x;
            y.div_assign_mod(T::ZERO)
        });
    });

    signed_gen_var_6::<T>().test_properties(|x| {
        assert_eq!(x.div_mod(T::ONE), (x, T::ZERO));
        assert_eq!(x.div_mod(x), (T::ONE, T::ZERO));
        assert_eq!(T::ZERO.div_mod(x), (T::ZERO, T::ZERO));
        if x != T::MIN {
            assert_eq!(x.div_mod(T::NEGATIVE_ONE), (-x, T::ZERO));
            assert_eq!(x.div_mod(-x), (T::NEGATIVE_ONE, T::ZERO));
        }
        if x > T::ONE {
            assert_eq!(T::ONE.div_mod(x), (T::ZERO, T::ONE));
            assert_eq!(T::NEGATIVE_ONE.div_mod(x), (T::NEGATIVE_ONE, x - T::ONE));
        }
    });
}

#[test]
fn div_mod_properties() {
    apply_fn_to_unsigneds!(div_mod_and_div_rem_properties_helper_unsigned);
    apply_fn_to_signeds!(div_mod_properties_helper_signed);
}

#[test]
fn test_div_rem_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, q: T, r: T) {
        assert_eq!(n.div_rem(d), (q, r));

        let mut mut_n = n;
        assert_eq!(mut_n.div_assign_rem(d), r);
        assert_eq!(mut_n, q);
    }
    test::<i8>(0, 1, 0, 0);
    test::<i16>(0, 123, 0, 0);
    test::<i32>(1, 1, 1, 0);
    test::<i64>(123, 1, 123, 0);
    test::<i128>(123, 123, 1, 0);
    test::<isize>(123, 456, 0, 123);
    test::<i16>(456, 123, 3, 87);
    test::<i64>(0xffffffff, 1, 0xffffffff, 0);
    test::<i64>(0xffffffff, 0xffffffff, 1, 0);
    test::<i64>(1000000000000, 1, 1000000000000, 0);
    test::<i64>(1000000000000, 3, 333333333333, 1);
    test::<i64>(1000000000000, 123, 8130081300, 100);
    test::<i64>(1000000000000, 0xffffffff, 232, 3567587560);
    test::<i128>(1000000000000000000000000, 1, 1000000000000000000000000, 0);
    test::<i128>(1000000000000000000000000, 3, 333333333333333333333333, 1);
    test::<i128>(1000000000000000000000000, 123, 8130081300813008130081, 37);
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        232830643708079,
        3167723695,
    );
    test::<i128>(
        1000000000000000000000000,
        1234567890987,
        810000006723,
        530068894399,
    );
    test::<i128>(
        253640751230376270397812803167,
        2669936877441,
        94998781946290113,
        1520301762334,
    );
    test::<i128>(3768477692975601, 11447376614057827956, 0, 3768477692975601);
    test::<i64>(3356605361737854, 3081095617839357, 1, 275509743898497);
    test::<i128>(
        1098730198198174614195,
        953382298040157850476,
        1,
        145347900158016763719,
    );
    test::<i128>(
        69738658860594537152875081748,
        69738658860594537152875081748,
        1,
        0,
    );
    test::<i128>(1000000000000000000000000, 1000000000000000000000000, 1, 0);
    test::<i128>(0, 1000000000000000000000000, 0, 0);
    test::<i128>(123, 1000000000000000000000000, 0, 123);

    test::<i8>(0, -1, 0, 0);
    test::<i16>(0, -123, 0, 0);
    test::<i32>(1, -1, -1, 0);
    test::<i64>(123, -1, -123, 0);
    test::<i128>(123, -123, -1, 0);
    test::<isize>(123, -456, 0, 123);
    test::<i16>(456, -123, -3, 87);
    test::<i64>(0xffffffff, -1, -0xffffffff, 0);
    test::<i64>(0xffffffff, -0xffffffff, -1, 0);
    test::<i64>(1000000000000, -1, -1000000000000, 0);
    test::<i64>(1000000000000, -3, -333333333333, 1);
    test::<i64>(1000000000000, -123, -8130081300, 100);
    test::<i64>(1000000000000, -0xffffffff, -232, 3567587560);
    test::<i128>(1000000000000000000000000, -1, -1000000000000000000000000, 0);
    test::<i128>(1000000000000000000000000, -3, -333333333333333333333333, 1);
    test::<i128>(1000000000000000000000000, -123, -8130081300813008130081, 37);
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        -232830643708079,
        3167723695,
    );
    test::<i128>(
        1000000000000000000000000,
        -1234567890987,
        -810000006723,
        530068894399,
    );
    test::<i128>(
        253640751230376270397812803167,
        -2669936877441,
        -94998781946290113,
        1520301762334,
    );
    test::<i128>(3768477692975601, -11447376614057827956, 0, 3768477692975601);
    test::<i64>(3356605361737854, -3081095617839357, -1, 275509743898497);
    test::<i128>(
        1098730198198174614195,
        -953382298040157850476,
        -1,
        145347900158016763719,
    );
    test::<i128>(
        69738658860594537152875081748,
        -69738658860594537152875081748,
        -1,
        0,
    );
    test::<i128>(1000000000000000000000000, -1000000000000000000000000, -1, 0);
    test::<i128>(0, -1000000000000000000000000, 0, 0);
    test::<i128>(123, -1000000000000000000000000, 0, 123);

    test::<i8>(-1, 1, -1, 0);
    test::<i16>(-123, 1, -123, 0);
    test::<i32>(-123, 123, -1, 0);
    test::<i64>(-123, 456, 0, -123);
    test::<isize>(-456, 123, -3, -87);
    test::<i64>(-0xffffffff, 1, -0xffffffff, 0);
    test::<i64>(-0xffffffff, 0xffffffff, -1, 0);
    test::<i64>(-1000000000000, 1, -1000000000000, 0);
    test::<i64>(-1000000000000, 3, -333333333333, -1);
    test::<i64>(-1000000000000, 123, -8130081300, -100);
    test::<i64>(-1000000000000, 0xffffffff, -232, -3567587560);
    test::<i128>(-1000000000000000000000000, 1, -1000000000000000000000000, 0);
    test::<i128>(-1000000000000000000000000, 3, -333333333333333333333333, -1);
    test::<i128>(
        -1000000000000000000000000,
        123,
        -8130081300813008130081,
        -37,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        -232830643708079,
        -3167723695,
    );
    test::<i128>(
        -1000000000000000000000000,
        1234567890987,
        -810000006723,
        -530068894399,
    );
    test::<i128>(
        -253640751230376270397812803167,
        2669936877441,
        -94998781946290113,
        -1520301762334,
    );
    test::<i128>(
        -3768477692975601,
        11447376614057827956,
        0,
        -3768477692975601,
    );
    test::<i64>(-3356605361737854, 3081095617839357, -1, -275509743898497);
    test::<i128>(
        -1098730198198174614195,
        953382298040157850476,
        -1,
        -145347900158016763719,
    );
    test::<i128>(
        -69738658860594537152875081748,
        69738658860594537152875081748,
        -1,
        0,
    );
    test::<i128>(-1000000000000000000000000, 1000000000000000000000000, -1, 0);
    test::<i128>(-123, 1000000000000000000000000, 0, -123);

    test::<i8>(-1, -1, 1, 0);
    test::<i16>(-123, -1, 123, 0);
    test::<i32>(-123, -123, 1, 0);
    test::<i64>(-123, -456, 0, -123);
    test::<isize>(-456, -123, 3, -87);
    test::<i64>(-0xffffffff, -1, 0xffffffff, 0);
    test::<i64>(-0xffffffff, -0xffffffff, 1, 0);
    test::<i64>(-1000000000000, -1, 1000000000000, 0);
    test::<i64>(-1000000000000, -3, 333333333333, -1);
    test::<i64>(-1000000000000, -123, 8130081300, -100);
    test::<i64>(-1000000000000, -0xffffffff, 232, -3567587560);
    test::<i128>(-1000000000000000000000000, -1, 1000000000000000000000000, 0);
    test::<i128>(-1000000000000000000000000, -3, 333333333333333333333333, -1);
    test::<i128>(
        -1000000000000000000000000,
        -123,
        8130081300813008130081,
        -37,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        232830643708079,
        -3167723695,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1234567890987,
        810000006723,
        -530068894399,
    );
    test::<i128>(
        -253640751230376270397812803167,
        -2669936877441,
        94998781946290113,
        -1520301762334,
    );
    test::<i128>(
        -3768477692975601,
        -11447376614057827956,
        0,
        -3768477692975601,
    );
    test::<i64>(-3356605361737854, -3081095617839357, 1, -275509743898497);
    test::<i128>(
        -1098730198198174614195,
        -953382298040157850476,
        1,
        -145347900158016763719,
    );
    test::<i128>(
        -69738658860594537152875081748,
        -69738658860594537152875081748,
        1,
        0,
    );
    test::<i128>(-1000000000000000000000000, -1000000000000000000000000, 1, 0);
    test::<i128>(-123, -1000000000000000000000000, 0, -123);
}

fn div_rem_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::ONE.div_rem(T::ZERO));
    assert_panic!({
        let mut n = T::ONE;
        n.div_assign_rem(T::ZERO);
    });
}

fn div_rem_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::MIN.div_rem(T::NEGATIVE_ONE));
    assert_panic!({
        let mut n = T::MIN;
        n.div_assign_rem(T::NEGATIVE_ONE);
    });
}

#[test]
pub fn div_rem_fail() {
    apply_fn_to_primitive_ints!(div_rem_fail_helper);
    apply_fn_to_signeds!(div_rem_signed_fail_helper);
}

fn div_rem_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen_var_4::<T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        let r = mut_x.div_assign_rem(y);
        let q = mut_x;

        assert_eq!(x.div_rem(y), (q, r));

        assert_eq!((x / y, x % y), (q, r));

        assert!(r.lt_abs(&y));
        assert!(r == T::ZERO || (r > T::ZERO) == (x > T::ZERO));
        assert_eq!(q * y + r, x);

        if x != T::MIN {
            assert_eq!((-x).div_rem(y), (-q, -r));
        }
        if y != T::MIN && (x != T::MIN || y != T::ONE) {
            assert_eq!(x.div_rem(-y), (-q, r));
        }
    });

    signed_gen::<T>().test_properties(|x| {
        let (q, r) = x.div_rem(T::ONE);
        assert_eq!(q, x);
        assert_eq!(r, T::ZERO);

        if x != T::MIN {
            let (q, r) = x.div_rem(T::NEGATIVE_ONE);
            assert_eq!(q, -x);
            assert_eq!(r, T::ZERO);
        }
        assert_panic!(x.div_rem(T::ZERO));
        assert_panic!({
            let mut y = x;
            y.div_assign_rem(T::ZERO)
        });
    });

    signed_gen_var_6::<T>().test_properties(|x| {
        assert_eq!(x.div_rem(T::ONE), (x, T::ZERO));
        assert_eq!(x.div_rem(x), (T::ONE, T::ZERO));
        assert_eq!(T::ZERO.div_rem(x), (T::ZERO, T::ZERO));
        if x != T::MIN {
            assert_eq!(x.div_rem(T::NEGATIVE_ONE), (-x, T::ZERO));
            assert_eq!(x.div_rem(-x), (T::NEGATIVE_ONE, T::ZERO));
        }
        if x > T::ONE {
            assert_eq!(T::ONE.div_rem(x), (T::ZERO, T::ONE));
            assert_eq!(T::NEGATIVE_ONE.div_rem(x), (T::ZERO, T::NEGATIVE_ONE));
        }
    });
}

#[test]
fn div_rem_properties() {
    apply_fn_to_signeds!(div_rem_properties_helper_signed);
}

#[test]
fn test_ceiling_div_neg_mod() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, q: T, r: T) {
        assert_eq!(n.ceiling_div_neg_mod(d), (q, r));

        let mut mut_n = n;
        assert_eq!(mut_n.ceiling_div_assign_neg_mod(d), r);
        assert_eq!(mut_n, q);
    }
    test::<u8>(0, 1, 0, 0);
    test::<u16>(0, 123, 0, 0);
    test::<u32>(1, 1, 1, 0);
    test::<u64>(123, 1, 123, 0);
    test::<u128>(123, 123, 1, 0);
    test::<usize>(123, 456, 1, 333);
    test::<u16>(456, 123, 4, 36);
    test::<u64>(0xffffffff, 1, 0xffffffff, 0);
    test::<u64>(0xffffffff, 0xffffffff, 1, 0);
    test::<u64>(1000000000000, 1, 1000000000000, 0);
    test::<u64>(1000000000000, 3, 333333333334, 2);
    test::<u64>(1000000000000, 123, 8130081301, 23);
    test::<u64>(1000000000000, 0xffffffff, 233, 727379735);
    test::<u128>(1000000000000000000000000, 1, 1000000000000000000000000, 0);
    test::<u128>(1000000000000000000000000, 3, 333333333333333333333334, 2);
    test::<u128>(1000000000000000000000000, 123, 8130081300813008130082, 86);
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        232830643708080,
        1127243600,
    );
    test::<u128>(
        1000000000000000000000000,
        1234567890987,
        810000006724,
        704498996588,
    );
    test::<u128>(
        253640751230376270397812803167,
        2669936877441,
        94998781946290114,
        1149635115107,
    );
    test::<u64>(
        3768477692975601,
        11447376614057827956,
        1,
        11443608136364852355,
    );
    test::<u64>(3356605361737854, 3081095617839357, 2, 2805585873940860);
    test::<u128>(
        1098730198198174614195,
        953382298040157850476,
        2,
        808034397882141086757,
    );
    test::<u128>(
        69738658860594537152875081748,
        69738658860594537152875081748,
        1,
        0,
    );
    test::<u128>(1000000000000000000000000, 1000000000000000000000000, 1, 0);
    test::<u128>(0, 1000000000000000000000000, 0, 0);
    test::<u128>(123, 1000000000000000000000000, 1, 999999999999999999999877);
}

fn ceiling_div_neg_mod_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.ceiling_div_neg_mod(T::ZERO));
    assert_panic!({
        let mut n = T::ONE;
        n.ceiling_div_assign_neg_mod(T::ZERO);
    });
}

#[test]
pub fn ceiling_div_neg_mod_fail() {
    apply_fn_to_unsigneds!(ceiling_div_neg_mod_fail_helper);
}

fn ceiling_div_neg_mod_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_12::<T, T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        let r = mut_x.ceiling_div_assign_neg_mod(y);
        let q = mut_x;

        assert_eq!(x.ceiling_div_neg_mod(y), (q, r));

        let (q_alt, r_alt) = (x.div_round(y, Ceiling).0, x.neg_mod(y));
        assert_eq!(q_alt, q);
        assert_eq!(r_alt, r);

        assert!(r < y);
        if let Some(product) = q.checked_mul(y) {
            assert_eq!(product - r, x);
        } else {
            assert_eq!((q - T::ONE) * y - r + y, x);
        }
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.ceiling_div_neg_mod(T::ONE), (x, T::ZERO));
        assert_panic!(x.ceiling_div_neg_mod(T::ZERO));
        assert_panic!({
            let mut y = x;
            y.ceiling_div_assign_neg_mod(T::ZERO)
        });
    });

    unsigned_gen_var_1::<T>().test_properties(|x| {
        assert_eq!(x.ceiling_div_neg_mod(x), (T::ONE, T::ZERO));
        assert_eq!(T::ZERO.ceiling_div_neg_mod(x), (T::ZERO, T::ZERO));
        if x > T::ONE {
            assert_eq!(T::ONE.ceiling_div_neg_mod(x), (T::ONE, x - T::ONE));
        }
    });
}

#[test]
fn ceiling_div_neg_mod_properties() {
    apply_fn_to_unsigneds!(ceiling_div_neg_mod_properties_helper);
}

#[test]
fn test_ceiling_div_mod() {
    fn test<T: PrimitiveSigned>(n: T, d: T, q: T, r: T) {
        assert_eq!(n.ceiling_div_mod(d), (q, r));

        let mut mut_n = n;
        assert_eq!(mut_n.ceiling_div_assign_mod(d), r);
        assert_eq!(mut_n, q);
    }
    test::<i8>(0, 1, 0, 0);
    test::<i16>(0, 123, 0, 0);
    test::<i32>(1, 1, 1, 0);
    test::<i64>(123, 1, 123, 0);
    test::<i128>(123, 123, 1, 0);
    test::<isize>(123, 456, 1, -333);
    test::<i16>(456, 123, 4, -36);
    test::<i64>(0xffffffff, 1, 0xffffffff, 0);
    test::<i64>(0xffffffff, 0xffffffff, 1, 0);
    test::<i64>(1000000000000, 1, 1000000000000, 0);
    test::<i64>(1000000000000, 3, 333333333334, -2);
    test::<i64>(1000000000000, 123, 8130081301, -23);
    test::<i64>(1000000000000, 0xffffffff, 233, -727379735);
    test::<i128>(1000000000000000000000000, 1, 1000000000000000000000000, 0);
    test::<i128>(1000000000000000000000000, 3, 333333333333333333333334, -2);
    test::<i128>(1000000000000000000000000, 123, 8130081300813008130082, -86);
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        232830643708080,
        -1127243600,
    );
    test::<i128>(
        1000000000000000000000000,
        1234567890987,
        810000006724,
        -704498996588,
    );
    test::<i128>(
        253640751230376270397812803167,
        2669936877441,
        94998781946290114,
        -1149635115107,
    );
    test::<i128>(
        3768477692975601,
        11447376614057827956,
        1,
        -11443608136364852355,
    );
    test::<i64>(3356605361737854, 3081095617839357, 2, -2805585873940860);
    test::<i128>(
        1098730198198174614195,
        953382298040157850476,
        2,
        -808034397882141086757,
    );
    test::<i128>(
        69738658860594537152875081748,
        69738658860594537152875081748,
        1,
        0,
    );
    test::<i128>(1000000000000000000000000, 1000000000000000000000000, 1, 0);
    test::<i128>(0, 1000000000000000000000000, 0, 0);
    test::<i128>(123, 1000000000000000000000000, 1, -999999999999999999999877);

    test::<i8>(0, -1, 0, 0);
    test::<i16>(0, -123, 0, 0);
    test::<i32>(1, -1, -1, 0);
    test::<i64>(123, -1, -123, 0);
    test::<i128>(123, -123, -1, 0);
    test::<isize>(123, -456, 0, 123);
    test::<i16>(456, -123, -3, 87);
    test::<i64>(0xffffffff, -1, -0xffffffff, 0);
    test::<i64>(0xffffffff, -0xffffffff, -1, 0);
    test::<i64>(1000000000000, -1, -1000000000000, 0);
    test::<i64>(1000000000000, -3, -333333333333, 1);
    test::<i64>(1000000000000, -123, -8130081300, 100);
    test::<i64>(1000000000000, -0xffffffff, -232, 3567587560);
    test::<i128>(1000000000000000000000000, -1, -1000000000000000000000000, 0);
    test::<i128>(1000000000000000000000000, -3, -333333333333333333333333, 1);
    test::<i128>(1000000000000000000000000, -123, -8130081300813008130081, 37);
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        -232830643708079,
        3167723695,
    );
    test::<i128>(
        1000000000000000000000000,
        -1234567890987,
        -810000006723,
        530068894399,
    );
    test::<i128>(
        253640751230376270397812803167,
        -2669936877441,
        -94998781946290113,
        1520301762334,
    );
    test::<i128>(3768477692975601, -11447376614057827956, 0, 3768477692975601);
    test::<i64>(3356605361737854, -3081095617839357, -1, 275509743898497);
    test::<i128>(
        1098730198198174614195,
        -953382298040157850476,
        -1,
        145347900158016763719,
    );
    test::<i128>(
        69738658860594537152875081748,
        -69738658860594537152875081748,
        -1,
        0,
    );
    test::<i128>(1000000000000000000000000, -1000000000000000000000000, -1, 0);
    test::<i128>(0, -1000000000000000000000000, 0, 0);
    test::<i128>(123, -1000000000000000000000000, 0, 123);

    test::<i8>(-1, 1, -1, 0);
    test::<i16>(-123, 1, -123, 0);
    test::<i32>(-123, 123, -1, 0);
    test::<i64>(-123, 456, 0, -123);
    test::<i128>(-456, 123, -3, -87);
    test::<isize>(-0xffffffff, 1, -0xffffffff, 0);
    test::<i64>(-0xffffffff, 0xffffffff, -1, 0);
    test::<i64>(-1000000000000, 1, -1000000000000, 0);
    test::<i64>(-1000000000000, 3, -333333333333, -1);
    test::<i64>(-1000000000000, 123, -8130081300, -100);
    test::<i64>(-1000000000000, 0xffffffff, -232, -3567587560);
    test::<i128>(-1000000000000000000000000, 1, -1000000000000000000000000, 0);
    test::<i128>(-1000000000000000000000000, 3, -333333333333333333333333, -1);
    test::<i128>(
        -1000000000000000000000000,
        123,
        -8130081300813008130081,
        -37,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        -232830643708079,
        -3167723695,
    );
    test::<i128>(
        -1000000000000000000000000,
        1234567890987,
        -810000006723,
        -530068894399,
    );
    test::<i128>(
        -253640751230376270397812803167,
        2669936877441,
        -94998781946290113,
        -1520301762334,
    );
    test::<i128>(
        -3768477692975601,
        11447376614057827956,
        0,
        -3768477692975601,
    );
    test::<i64>(-3356605361737854, 3081095617839357, -1, -275509743898497);
    test::<i128>(
        -1098730198198174614195,
        953382298040157850476,
        -1,
        -145347900158016763719,
    );
    test::<i128>(
        -69738658860594537152875081748,
        69738658860594537152875081748,
        -1,
        0,
    );
    test::<i128>(-1000000000000000000000000, 1000000000000000000000000, -1, 0);
    test::<i128>(0, 1000000000000000000000000, 0, 0);
    test::<i128>(-123, 1000000000000000000000000, 0, -123);

    test::<i8>(-1, -1, 1, 0);
    test::<i16>(-123, -1, 123, 0);
    test::<i32>(-123, -123, 1, 0);
    test::<i64>(-123, -456, 1, 333);
    test::<i128>(-456, -123, 4, 36);
    test::<isize>(-0xffffffff, -1, 0xffffffff, 0);
    test::<i64>(-0xffffffff, -0xffffffff, 1, 0);
    test::<i64>(-1000000000000, -1, 1000000000000, 0);
    test::<i64>(-1000000000000, -3, 333333333334, 2);
    test::<i64>(-1000000000000, -123, 8130081301, 23);
    test::<i64>(-1000000000000, -0xffffffff, 233, 727379735);
    test::<i128>(-1000000000000000000000000, -1, 1000000000000000000000000, 0);
    test::<i128>(-1000000000000000000000000, -3, 333333333333333333333334, 2);
    test::<i128>(-1000000000000000000000000, -123, 8130081300813008130082, 86);
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        232830643708080,
        1127243600,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1234567890987,
        810000006724,
        704498996588,
    );
    test::<i128>(
        -253640751230376270397812803167,
        -2669936877441,
        94998781946290114,
        1149635115107,
    );
    test::<i128>(
        -3768477692975601,
        -11447376614057827956,
        1,
        11443608136364852355,
    );
    test::<i64>(-3356605361737854, -3081095617839357, 2, 2805585873940860);
    test::<i128>(
        -1098730198198174614195,
        -953382298040157850476,
        2,
        808034397882141086757,
    );
    test::<i128>(
        -69738658860594537152875081748,
        -69738658860594537152875081748,
        1,
        0,
    );
    test::<i128>(-1000000000000000000000000, -1000000000000000000000000, 1, 0);
    test::<i128>(0, -1000000000000000000000000, 0, 0);
    test::<i128>(
        -123,
        -1000000000000000000000000,
        1,
        999999999999999999999877,
    );
}

fn ceiling_div_mod_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::ONE.ceiling_div_mod(T::ZERO));
    assert_panic!({
        let mut n = T::ONE;
        n.ceiling_div_assign_mod(T::ZERO);
    });
    assert_panic!(T::MIN.ceiling_div_mod(T::NEGATIVE_ONE));
    assert_panic!({
        let mut n = T::MIN;
        n.ceiling_div_assign_mod(T::NEGATIVE_ONE);
    });
}

#[test]
pub fn ceiling_div_mod_fail() {
    apply_fn_to_signeds!(ceiling_div_mod_fail_helper);
}

fn ceiling_div_mod_properties_helper<T: PrimitiveSigned>() {
    signed_pair_gen_var_4::<T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        let r = mut_x.ceiling_div_assign_mod(y);
        let q = mut_x;

        assert_eq!(x.ceiling_div_mod(y), (q, r));

        let (q_alt, r_alt) = (x.div_round(y, Ceiling).0, x.ceiling_mod(y));
        assert_eq!(q_alt, q);
        assert_eq!(r_alt, r);

        assert!(r.lt_abs(&y));
        assert!(r == T::ZERO || (r > T::ZERO) != (y > T::ZERO));
        if let Some(product) = q.checked_mul(y) {
            assert_eq!(product + r, x);
        } else if q > T::ZERO {
            assert_eq!((q - T::ONE) * y + r + y, x);
        } else {
            assert_eq!((q + T::ONE) * y + r - y, x);
        }

        if x != T::MIN {
            let (neg_q, neg_r) = (-x).ceiling_div_mod(y);
            assert_eq!(x.div_mod(y), (-neg_q, -neg_r));
        }
        if y != T::MIN && (x != T::MIN || y != T::ONE) {
            let (neg_q, r) = x.ceiling_div_mod(-y);
            assert_eq!(x.div_mod(y), (-neg_q, r));
        }
    });

    signed_gen::<T>().test_properties(|x| {
        let (q, r) = x.ceiling_div_mod(T::ONE);
        assert_eq!(q, x);
        assert_eq!(r, T::ZERO);

        if x != T::MIN {
            let (q, r) = x.ceiling_div_mod(T::NEGATIVE_ONE);
            assert_eq!(q, -x);
            assert_eq!(r, T::ZERO);
        }
        assert_panic!(x.ceiling_div_mod(T::ZERO));
        assert_panic!({
            let mut y = x;
            y.ceiling_div_assign_mod(T::ZERO)
        });
    });

    signed_gen_var_6::<T>().test_properties(|x| {
        assert_eq!(x.ceiling_div_mod(T::ONE), (x, T::ZERO));
        if x != T::MIN {
            assert_eq!(x.ceiling_div_mod(T::NEGATIVE_ONE), (-x, T::ZERO));
        }
        assert_eq!(x.ceiling_div_mod(x), (T::ONE, T::ZERO));
        if x != T::MIN {
            assert_eq!(x.ceiling_div_mod(-x), (T::NEGATIVE_ONE, T::ZERO));
        }
        assert_eq!(T::ZERO.ceiling_div_mod(x), (T::ZERO, T::ZERO));
    });
}

#[test]
fn ceiling_div_mod_properties() {
    apply_fn_to_signeds!(ceiling_div_mod_properties_helper);
}
