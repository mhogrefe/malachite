// Copyright © 2024 Mikhail Hogrefe
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
    signed_gen, signed_gen_var_6, signed_pair_gen_var_4, signed_pair_gen_var_6, unsigned_gen,
    unsigned_gen_var_1, unsigned_pair_gen_var_12,
};
use std::panic::catch_unwind;

#[test]
fn test_mod_op_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, r: T) {
        assert_eq!(n.mod_op(d), r);

        let mut mut_n = n;
        mut_n.mod_assign(d);
        assert_eq!(mut_n, r);
    }
    test::<u8>(0, 1, 0);
    test::<u16>(0, 123, 0);
    test::<u32>(1, 1, 0);
    test::<u64>(123, 1, 0);
    test::<usize>(123, 123, 0);
    test::<u128>(123, 456, 123);
    test::<u16>(456, 123, 87);
    test::<u32>(u32::MAX, 1, 0);
    test::<usize>(0xffffffff, 0xffffffff, 0);
    test::<u64>(1000000000000, 1, 0);
    test::<u64>(1000000000000, 3, 1);
    test::<u64>(1000000000000, 123, 100);
    test::<u64>(1000000000000, 0xffffffff, 3567587560);
    test::<u128>(1000000000000000000000000, 1, 0);
    test::<u128>(1000000000000000000000000, 3, 1);
    test::<u128>(1000000000000000000000000, 123, 37);
    test::<u128>(1000000000000000000000000, 0xffffffff, 3167723695);
    test::<u128>(1000000000000000000000000, 1234567890987, 530068894399);
    test::<u128>(253640751230376270397812803167, 2669936877441, 1520301762334);
    test::<u64>(3768477692975601, 11447376614057827956, 3768477692975601);
    test::<u64>(3356605361737854, 3081095617839357, 275509743898497);
    test::<u128>(
        1098730198198174614195,
        953382298040157850476,
        145347900158016763719,
    );
    test::<u128>(
        69738658860594537152875081748,
        69738658860594537152875081748,
        0,
    );
    test::<u128>(1000000000000000000000000, 1000000000000000000000000, 0);
    test::<u128>(0, 1000000000000000000000000, 0);
    test::<u128>(123, 1000000000000000000000000, 123);
}

#[test]
fn test_div_mod_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, r: T) {
        assert_eq!(n.mod_op(d), r);

        let mut mut_n = n;
        mut_n.mod_assign(d);
        assert_eq!(mut_n, r);
    }
    test::<i8>(0, 1, 0);
    test::<i16>(0, 123, 0);
    test::<i32>(1, 1, 0);
    test::<i64>(123, 1, 0);
    test::<i128>(123, 123, 0);
    test::<isize>(123, 456, 123);
    test::<i16>(456, 123, 87);
    test::<i64>(0xffffffff, 1, 0);
    test::<i64>(0xffffffff, 0xffffffff, 0);
    test::<i64>(1000000000000, 1, 0);
    test::<i64>(1000000000000, 3, 1);
    test::<i64>(1000000000000, 123, 100);
    test::<i64>(1000000000000, 0xffffffff, 3567587560);
    test::<i128>(1000000000000000000000000, 1, 0);
    test::<i128>(1000000000000000000000000, 3, 1);
    test::<i128>(1000000000000000000000000, 123, 37);
    test::<i128>(1000000000000000000000000, 0xffffffff, 3167723695);
    test::<i128>(1000000000000000000000000, 1234567890987, 530068894399);
    test::<i128>(253640751230376270397812803167, 2669936877441, 1520301762334);
    test::<i128>(3768477692975601, 11447376614057827956, 3768477692975601);
    test::<i64>(3356605361737854, 3081095617839357, 275509743898497);
    test::<i128>(
        1098730198198174614195,
        953382298040157850476,
        145347900158016763719,
    );
    test::<i128>(
        69738658860594537152875081748,
        69738658860594537152875081748,
        0,
    );
    test::<i128>(1000000000000000000000000, 1000000000000000000000000, 0);
    test::<i128>(0, 1000000000000000000000000, 0);
    test::<i128>(123, 1000000000000000000000000, 123);

    test::<i8>(0, -1, 0);
    test::<i16>(0, -123, 0);
    test::<i32>(1, -1, 0);
    test::<i64>(123, -1, 0);
    test::<i128>(123, -123, 0);
    test::<isize>(123, -456, -333);
    test::<i16>(456, -123, -36);
    test::<i64>(0xffffffff, -1, 0);
    test::<i64>(0xffffffff, -0xffffffff, 0);
    test::<i64>(1000000000000, -1, 0);
    test::<i64>(1000000000000, -3, -2);
    test::<i64>(1000000000000, -123, -23);
    test::<i64>(1000000000000, -0xffffffff, -727379735);
    test::<i128>(1000000000000000000000000, -1, 0);
    test::<i128>(1000000000000000000000000, -3, -2);
    test::<i128>(1000000000000000000000000, -123, -86);
    test::<i128>(1000000000000000000000000, -0xffffffff, -1127243600);
    test::<i128>(1000000000000000000000000, -1234567890987, -704498996588);
    test::<i128>(
        253640751230376270397812803167,
        -2669936877441,
        -1149635115107,
    );
    test::<i128>(
        3768477692975601,
        -11447376614057827956,
        -11443608136364852355,
    );
    test::<i64>(3356605361737854, -3081095617839357, -2805585873940860);
    test::<i128>(
        1098730198198174614195,
        -953382298040157850476,
        -808034397882141086757,
    );
    test::<i128>(
        69738658860594537152875081748,
        -69738658860594537152875081748,
        0,
    );
    test::<i128>(1000000000000000000000000, -1000000000000000000000000, 0);
    test::<i128>(0, -1000000000000000000000000, 0);
    test::<i128>(123, -1000000000000000000000000, -999999999999999999999877);

    test::<i8>(-1, 1, 0);
    test::<i16>(-123, 1, 0);
    test::<i32>(-123, 123, 0);
    test::<i64>(-123, 456, 333);
    test::<isize>(-456, 123, 36);
    test::<i64>(-0xffffffff, -1, 0);
    test::<i64>(-0xffffffff, 0xffffffff, 0);
    test::<i64>(-1000000000000, 1, 0);
    test::<i64>(-1000000000000, 3, 2);
    test::<i64>(-1000000000000, 123, 23);
    test::<i64>(-1000000000000, 0xffffffff, 727379735);
    test::<i128>(-1000000000000000000000000, 1, 0);
    test::<i128>(-1000000000000000000000000, 3, 2);
    test::<i128>(-1000000000000000000000000, 123, 86);
    test::<i128>(-1000000000000000000000000, 0xffffffff, 1127243600);
    test::<i128>(-1000000000000000000000000, 1234567890987, 704498996588);
    test::<i128>(
        -253640751230376270397812803167,
        2669936877441,
        1149635115107,
    );
    test::<i128>(
        -3768477692975601,
        11447376614057827956,
        11443608136364852355,
    );
    test::<i64>(-3356605361737854, 3081095617839357, 2805585873940860);
    test::<i128>(
        -1098730198198174614195,
        953382298040157850476,
        808034397882141086757,
    );
    test::<i128>(
        -69738658860594537152875081748,
        69738658860594537152875081748,
        0,
    );
    test::<i128>(-1000000000000000000000000, 1000000000000000000000000, 0);
    test::<i128>(-123, 1000000000000000000000000, 999999999999999999999877);

    test::<i8>(-1, -1, 0);
    test::<i16>(-123, -1, 0);
    test::<i32>(-123, -123, 0);
    test::<i64>(-123, -456, -123);
    test::<isize>(-456, -123, -87);
    test::<i128>(-0xffffffff, -1, 0);
    test::<i64>(-0xffffffff, -0xffffffff, 0);
    test::<i64>(-1000000000000, -1, 0);
    test::<i64>(-1000000000000, -3, -1);
    test::<i64>(-1000000000000, -123, -100);
    test::<i64>(-1000000000000, -0xffffffff, -3567587560);
    test::<i128>(-1000000000000000000000000, -1, 0);
    test::<i128>(-1000000000000000000000000, -3, -1);
    test::<i128>(-1000000000000000000000000, -123, -37);
    test::<i128>(-1000000000000000000000000, -0xffffffff, -3167723695);
    test::<i128>(-1000000000000000000000000, -1234567890987, -530068894399);
    test::<i128>(
        -253640751230376270397812803167,
        -2669936877441,
        -1520301762334,
    );
    test::<i128>(-3768477692975601, -11447376614057827956, -3768477692975601);
    test::<i64>(-3356605361737854, -3081095617839357, -275509743898497);
    test::<i128>(
        -1098730198198174614195,
        -953382298040157850476,
        -145347900158016763719,
    );
    test::<i128>(
        -69738658860594537152875081748,
        -69738658860594537152875081748,
        0,
    );
    test::<i128>(-1000000000000000000000000, -1000000000000000000000000, 0);
    test::<i128>(-123, -1000000000000000000000000, -123);

    test::<i8>(-128, -1, 0);
}

fn mod_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::ONE.mod_op(T::ZERO));
    assert_panic!({
        let mut n = T::ONE;
        n.mod_assign(T::ZERO);
    });
}

#[test]
pub fn mod_fail() {
    apply_fn_to_primitive_ints!(mod_fail_helper);
}

#[test]
fn test_neg_mod() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, r: T) {
        assert_eq!(n.neg_mod(d), r);

        let mut mut_n = n;
        mut_n.neg_mod_assign(d);
        assert_eq!(mut_n, r);
    }
    test::<u8>(0, 1, 0);
    test::<u16>(0, 123, 0);
    test::<u32>(1, 1, 0);
    test::<u64>(123, 1, 0);
    test::<u128>(123, 123, 0);
    test::<usize>(123, 456, 333);
    test::<u16>(456, 123, 36);
    test::<u64>(0xffffffff, 1, 0);
    test::<u64>(0xffffffff, 0xffffffff, 0);
    test::<u64>(1000000000000, 1, 0);
    test::<u64>(1000000000000, 3, 2);
    test::<u64>(1000000000000, 123, 23);
    test::<u64>(1000000000000, 0xffffffff, 727379735);
    test::<u128>(1000000000000000000000000, 1, 0);
    test::<u128>(1000000000000000000000000, 3, 2);
    test::<u128>(1000000000000000000000000, 123, 86);
    test::<u128>(1000000000000000000000000, 0xffffffff, 1127243600);
    test::<u128>(1000000000000000000000000, 1234567890987, 704498996588);
    test::<u128>(253640751230376270397812803167, 2669936877441, 1149635115107);
    test::<u64>(3768477692975601, 11447376614057827956, 11443608136364852355);
    test::<u64>(3356605361737854, 3081095617839357, 2805585873940860);
    test::<u128>(
        1098730198198174614195,
        953382298040157850476,
        808034397882141086757,
    );
    test::<u128>(
        69738658860594537152875081748,
        69738658860594537152875081748,
        0,
    );
    test::<u128>(1000000000000000000000000, 1000000000000000000000000, 0);
    test::<u128>(0, 1000000000000000000000000, 0);
    test::<u128>(123, 1000000000000000000000000, 999999999999999999999877);
}

fn neg_mod_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.neg_mod(T::ZERO));
    assert_panic!({
        let mut n = T::ONE;
        n.neg_mod_assign(T::ZERO);
    });
}

#[test]
pub fn neg_mod_fail() {
    apply_fn_to_unsigneds!(neg_mod_fail_helper);
}

#[test]
fn test_ceiling_mod() {
    fn test<T: PrimitiveSigned>(n: T, d: T, r: T) {
        assert_eq!(n.ceiling_mod(d), r);

        let mut mut_n = n;
        mut_n.ceiling_mod_assign(d);
        assert_eq!(mut_n, r);
    }
    test::<i8>(0, 1, 0);
    test::<i16>(0, 123, 0);
    test::<i32>(1, 1, 0);
    test::<i64>(123, 1, 0);
    test::<i128>(123, 123, 0);
    test::<isize>(123, 456, -333);
    test::<i16>(456, 123, -36);
    test::<i64>(0xffffffff, 1, 0);
    test::<i64>(0xffffffff, 0xffffffff, 0);
    test::<i64>(1000000000000, 1, 0);
    test::<i64>(1000000000000, 3, -2);
    test::<i64>(1000000000000, 123, -23);
    test::<i64>(1000000000000, 0xffffffff, -727379735);
    test::<i128>(1000000000000000000000000, 1, 0);
    test::<i128>(1000000000000000000000000, 3, -2);
    test::<i128>(1000000000000000000000000, 123, -86);
    test::<i128>(1000000000000000000000000, 0xffffffff, -1127243600);
    test::<i128>(1000000000000000000000000, 1234567890987, -704498996588);
    test::<i128>(
        253640751230376270397812803167,
        2669936877441,
        -1149635115107,
    );
    test::<i128>(
        3768477692975601,
        11447376614057827956,
        -11443608136364852355,
    );
    test::<i64>(3356605361737854, 3081095617839357, -2805585873940860);
    test::<i128>(
        1098730198198174614195,
        953382298040157850476,
        -808034397882141086757,
    );
    test::<i128>(
        69738658860594537152875081748,
        69738658860594537152875081748,
        0,
    );
    test::<i128>(1000000000000000000000000, 1000000000000000000000000, 0);
    test::<i128>(0, 1000000000000000000000000, 0);
    test::<i128>(123, 1000000000000000000000000, -999999999999999999999877);

    test::<i8>(0, -1, 0);
    test::<i16>(0, -123, 0);
    test::<i32>(1, -1, 0);
    test::<i64>(123, -1, 0);
    test::<i128>(123, -123, 0);
    test::<isize>(123, -456, 123);
    test::<i16>(456, -123, 87);
    test::<i64>(0xffffffff, -1, 0);
    test::<i64>(0xffffffff, -0xffffffff, 0);
    test::<i64>(1000000000000, -1, 0);
    test::<i64>(1000000000000, -3, 1);
    test::<i64>(1000000000000, -123, 100);
    test::<i64>(1000000000000, -0xffffffff, 3567587560);
    test::<i128>(1000000000000000000000000, -1, 0);
    test::<i128>(1000000000000000000000000, -3, 1);
    test::<i128>(1000000000000000000000000, -123, 37);
    test::<i128>(1000000000000000000000000, -0xffffffff, 3167723695);
    test::<i128>(1000000000000000000000000, -1234567890987, 530068894399);
    test::<i128>(
        253640751230376270397812803167,
        -2669936877441,
        1520301762334,
    );
    test::<i128>(3768477692975601, -11447376614057827956, 3768477692975601);
    test::<i64>(3356605361737854, -3081095617839357, 275509743898497);
    test::<i128>(
        1098730198198174614195,
        -953382298040157850476,
        145347900158016763719,
    );
    test::<i128>(
        69738658860594537152875081748,
        -69738658860594537152875081748,
        0,
    );
    test::<i128>(1000000000000000000000000, -1000000000000000000000000, 0);
    test::<i128>(0, -1000000000000000000000000, 0);
    test::<i128>(123, -1000000000000000000000000, 123);

    test::<i8>(-1, 1, 0);
    test::<i16>(-123, 1, 0);
    test::<i32>(-123, 123, 0);
    test::<i64>(-123, 456, -123);
    test::<i128>(-456, 123, -87);
    test::<isize>(-0xffffffff, 1, 0);
    test::<i64>(-0xffffffff, 0xffffffff, 0);
    test::<i64>(-1000000000000, 1, 0);
    test::<i64>(-1000000000000, 3, -1);
    test::<i64>(-1000000000000, 123, -100);
    test::<i64>(-1000000000000, 0xffffffff, -3567587560);
    test::<i128>(-1000000000000000000000000, 1, 0);
    test::<i128>(-1000000000000000000000000, 3, -1);
    test::<i128>(-1000000000000000000000000, 123, -37);
    test::<i128>(-1000000000000000000000000, 0xffffffff, -3167723695);
    test::<i128>(-1000000000000000000000000, 1234567890987, -530068894399);
    test::<i128>(
        -253640751230376270397812803167,
        2669936877441,
        -1520301762334,
    );
    test::<i128>(-3768477692975601, 11447376614057827956, -3768477692975601);
    test::<i64>(-3356605361737854, 3081095617839357, -275509743898497);
    test::<i128>(
        -1098730198198174614195,
        953382298040157850476,
        -145347900158016763719,
    );
    test::<i128>(
        -69738658860594537152875081748,
        69738658860594537152875081748,
        0,
    );
    test::<i128>(-1000000000000000000000000, 1000000000000000000000000, 0);
    test::<i128>(0, 1000000000000000000000000, 0);
    test::<i128>(-123, 1000000000000000000000000, -123);

    test::<i8>(-1, -1, 0);
    test::<i16>(-123, -1, 0);
    test::<i32>(-123, -123, 0);
    test::<i64>(-123, -456, 333);
    test::<i128>(-456, -123, 36);
    test::<isize>(-0xffffffff, -1, 0);
    test::<i64>(-0xffffffff, -0xffffffff, 0);
    test::<i64>(-1000000000000, -1, 0);
    test::<i64>(-1000000000000, -3, 2);
    test::<i64>(-1000000000000, -123, 23);
    test::<i64>(-1000000000000, -0xffffffff, 727379735);
    test::<i128>(-1000000000000000000000000, -1, 0);
    test::<i128>(-1000000000000000000000000, -3, 2);
    test::<i128>(-1000000000000000000000000, -123, 86);
    test::<i128>(-1000000000000000000000000, -0xffffffff, 1127243600);
    test::<i128>(-1000000000000000000000000, -1234567890987, 704498996588);
    test::<i128>(
        -253640751230376270397812803167,
        -2669936877441,
        1149635115107,
    );
    test::<i128>(
        -3768477692975601,
        -11447376614057827956,
        11443608136364852355,
    );
    test::<i64>(-3356605361737854, -3081095617839357, 2805585873940860);
    test::<i128>(
        -1098730198198174614195,
        -953382298040157850476,
        808034397882141086757,
    );
    test::<i128>(
        -69738658860594537152875081748,
        -69738658860594537152875081748,
        0,
    );
    test::<i128>(-1000000000000000000000000, -1000000000000000000000000, 0);
    test::<i128>(0, -1000000000000000000000000, 0);
    test::<i128>(-123, -1000000000000000000000000, 999999999999999999999877);

    test::<i8>(-128, -1, 0);
}

fn ceiling_mod_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::ONE.ceiling_mod(T::ZERO));
    assert_panic!({
        let mut n = T::ONE;
        n.ceiling_mod_assign(T::ZERO);
    });
}

#[test]
pub fn ceiling_mod_fail() {
    apply_fn_to_signeds!(ceiling_mod_fail_helper);
}

fn mod_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_12::<T, T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        mut_x.mod_assign(y);
        let r = mut_x;

        assert_eq!(x.mod_op(y), r);

        let mut mut_x = x;
        mut_x %= y;
        assert_eq!(mut_x, r);
        assert_eq!(x % y, r);
        assert_eq!(x.div_mod(y).1, r);
        assert_eq!(x.div_rem(y).1, r);
        assert!(r < y);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.mod_op(T::ONE), T::ZERO);
    });

    unsigned_gen_var_1::<T>().test_properties(|x| {
        assert_eq!(x.mod_op(x), T::ZERO);
        assert_eq!(T::ZERO.mod_op(x), T::ZERO);
        if x > T::ONE {
            assert_eq!(T::ONE.mod_op(x), T::ONE);
        }
    });
}

fn mod_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen_var_6::<T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        mut_x.mod_assign(y);
        let r = mut_x;

        assert_eq!(x.mod_op(y), r);
        assert!(r.lt_abs(&y));
        assert!(r == T::ZERO || (r > T::ZERO) == (y > T::ZERO));
        if x != T::MIN {
            assert_eq!(x.ceiling_mod(y), -(-x).mod_op(y));
        }
        if y != T::MIN {
            assert_eq!(x.ceiling_mod(y), x.mod_op(-y));
        }
    });

    signed_pair_gen_var_4::<T>().test_properties(|(x, y)| {
        assert_eq!(x.mod_op(y), x.div_mod(y).1);
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.mod_op(T::ONE), T::ZERO);
        assert_eq!(x.mod_op(T::NEGATIVE_ONE), T::ZERO);
    });

    signed_gen_var_6::<T>().test_properties(|x| {
        assert_eq!(x.mod_op(T::ONE), T::ZERO);
        assert_eq!(x.mod_op(x), T::ZERO);
        assert_eq!(T::ZERO.mod_op(x), T::ZERO);
        assert_eq!(x.mod_op(T::NEGATIVE_ONE), T::ZERO);
        if x != T::MIN {
            assert_eq!(x.mod_op(-x), T::ZERO);
        }
        if x > T::ONE {
            assert_eq!(T::ONE.mod_op(x), T::ONE);
            assert_eq!(T::NEGATIVE_ONE.mod_op(x), x - T::ONE);
        }
    });
}

#[test]
fn mod_properties() {
    apply_fn_to_unsigneds!(mod_properties_helper_unsigned);
    apply_fn_to_signeds!(mod_properties_helper_signed);
}

fn neg_mod_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_12::<T, T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        mut_x.neg_mod_assign(y);
        let r = mut_x;
        assert_eq!(x.neg_mod(y), r);
        assert_eq!(x.ceiling_div_neg_mod(y).1, r);
        assert!(r < y);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.neg_mod(T::ONE), T::ZERO);
    });

    unsigned_gen_var_1::<T>().test_properties(|x| {
        assert_eq!(x.neg_mod(x), T::ZERO);
        assert_eq!(T::ZERO.neg_mod(x), T::ZERO);
        if x > T::ONE {
            assert_eq!(T::ONE.neg_mod(x), x - T::ONE);
        }
    });
}

#[test]
fn neg_mod_properties() {
    apply_fn_to_unsigneds!(neg_mod_properties_helper);
}

fn ceiling_mod_properties_helper<T: PrimitiveSigned>() {
    signed_pair_gen_var_6::<T>().test_properties(|(x, y)| {
        let mut mut_x = x;
        mut_x.ceiling_mod_assign(y);
        let r = mut_x;
        assert_eq!(x.ceiling_mod(y), r);
        assert!(r.lt_abs(&y));
        assert!(r == T::ZERO || (r > T::ZERO) != (y > T::ZERO));
        if x != T::MIN {
            assert_eq!(x.mod_op(y), -(-x).ceiling_mod(y));
        }
        if y != T::MIN {
            assert_eq!(x.mod_op(y), x.ceiling_mod(-y));
        }
    });

    signed_pair_gen_var_4::<T>().test_properties(|(x, y)| {
        assert_eq!(x.ceiling_mod(y), x.ceiling_div_mod(y).1);
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.ceiling_mod(T::ONE), T::ZERO);
        assert_eq!(x.ceiling_mod(T::NEGATIVE_ONE), T::ZERO);
    });

    signed_gen_var_6::<T>().test_properties(|x| {
        assert_eq!(x.ceiling_mod(T::ONE), T::ZERO);
        assert_eq!(x.ceiling_mod(T::NEGATIVE_ONE), T::ZERO);
        assert_eq!(x.ceiling_mod(x), T::ZERO);
        if x != T::MIN {
            assert_eq!(x.ceiling_mod(-x), T::ZERO);
        }
        assert_eq!(T::ZERO.ceiling_mod(x), T::ZERO);
    });
}

#[test]
fn ceiling_mod_properties() {
    apply_fn_to_signeds!(ceiling_mod_properties_helper);
}
