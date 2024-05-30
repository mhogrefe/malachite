// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2IsReduced};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::test_util::generators::{
    signed_gen, signed_unsigned_pair_gen_var_1, signed_unsigned_pair_gen_var_10,
    signed_unsigned_pair_gen_var_11, signed_unsigned_pair_gen_var_4, unsigned_gen,
    unsigned_pair_gen_var_2, unsigned_pair_gen_var_20, unsigned_triple_gen_var_13,
    unsigned_triple_gen_var_4,
};
use std::cmp::min;
use std::fmt::Debug;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_and_rem_power_of_2_unsigned() {
    fn test<T: PrimitiveUnsigned>(x: T, pow: u64, out: T) {
        assert_eq!(x.mod_power_of_2(pow), out);

        let mut mut_x = x;
        mut_x.mod_power_of_2_assign(pow);
        assert_eq!(mut_x, out);

        assert_eq!(x.rem_power_of_2(pow), out);

        let mut mut_x = x;
        mut_x.rem_power_of_2_assign(pow);
        assert_eq!(mut_x, out);
    }
    test::<u8>(0, 0, 0);
    test::<u16>(260, 8, 4);
    test::<u32>(1611, 4, 11);
    test::<u8>(123, 100, 123);
    test::<u64>(1000000000000, 0, 0);
    test::<u64>(1000000000000, 12, 0);
    test::<u64>(1000000000001, 12, 1);
    test::<u64>(999999999999, 12, 4095);
    test::<u64>(1000000000000, 15, 4096);
    test::<u64>(1000000000000, 100, 1000000000000);
    test::<u128>(1000000000000000000000000, 40, 1020608380928);
    test::<u128>(1000000000000000000000000, 64, 2003764205206896640);
    test::<u32>(u32::MAX, 31, 0x7fffffff);
    test::<u32>(u32::MAX, 32, u32::MAX);
    test::<usize>(0xffffffff, 33, 0xffffffff);
    test::<u64>(0x100000000, 31, 0);
    test::<u64>(0x100000000, 32, 0);
    test::<u64>(0x100000000, 33, 0x100000000);
    test::<u64>(0x100000001, 31, 1);
    test::<u64>(0x100000001, 32, 1);
    test::<u64>(0x100000001, 33, 0x100000001);
}

#[test]
fn test_mod_power_of_2_signed() {
    fn test<U: Copy + Debug + Eq, S: ModPowerOf2<Output = U> + PrimitiveSigned>(
        x: S,
        pow: u64,
        out: U,
    ) {
        assert_eq!(x.mod_power_of_2(pow), out);
    }
    test::<_, i8>(0, 0, 0);
    test::<_, i16>(2, 1, 0);
    test::<_, i32>(260, 8, 4);
    test::<_, i16>(1611, 4, 11);
    test::<_, i8>(123, 100, 123);
    test::<_, i64>(1000000000000, 0, 0);
    test::<_, i64>(1000000000000, 12, 0);
    test::<_, i64>(1000000000001, 12, 1);
    test::<_, i64>(999999999999, 12, 4095);
    test::<_, i64>(1000000000000, 15, 4096);
    test::<_, i64>(1000000000000, 100, 1000000000000);
    test::<_, i128>(1000000000000000000000000, 40, 1020608380928);
    test::<_, i128>(1000000000000000000000000, 64, 2003764205206896640);
    test::<_, i32>(0x7fffffff, 30, 0x3fffffff);
    test::<_, i32>(0x7fffffff, 31, 0x7fffffff);
    test::<_, isize>(0x7fffffff, 32, 0x7fffffff);
    test::<_, i64>(0x80000000, 30, 0);
    test::<_, i64>(0x80000000, 31, 0);
    test::<_, i64>(0x80000000, 32, 0x80000000);
    test::<_, i64>(0x80000001, 30, 1);
    test::<_, i64>(0x80000001, 31, 1);
    test::<_, i64>(0x80000001, 32, 0x80000001);
    test::<_, i64>(0xffffffff, 31, 0x7fffffff);
    test::<_, i64>(0xffffffff, 32, 0xffffffff);
    test::<_, i64>(0xffffffff, 33, 0xffffffff);
    test::<_, i64>(0x100000000, 31, 0);
    test::<_, i64>(0x100000000, 32, 0);
    test::<_, i64>(0x100000000, 33, 0x100000000);
    test::<_, i64>(0x100000001, 31, 1);
    test::<_, i64>(0x100000001, 32, 1);
    test::<_, i64>(0x100000001, 33, 0x100000001);

    test::<_, i8>(-2, 1, 0);
    test::<_, i16>(-260, 8, 252);
    test::<_, i32>(-1611, 4, 5);
    test::<_, i128>(-123, 100, 1267650600228229401496703205253);
    test::<_, i64>(-1000000000000, 0, 0);
    test::<_, i64>(-1000000000000, 12, 0);
    test::<_, i64>(-1000000000001, 12, 4095);
    test::<_, i64>(-999999999999, 12, 1);
    test::<_, i64>(-1000000000000, 15, 0x7000);
    test::<_, i128>(-1000000000000, 100, 1267650600228229400496703205376);
    test::<_, i128>(-1000000000000000000000000, 40, 78903246848);
    test::<_, i128>(-1000000000000000000000000, 64, 16442979868502654976);
    test::<_, i32>(-0x7fffffff, 30, 1);
    test::<_, i32>(-0x7fffffff, 31, 1);
    test::<_, i32>(-0x7fffffff, 32, 0x80000001);
    test::<_, isize>(-0x80000000, 30, 0);
    test::<_, isize>(-0x80000000, 31, 0);
    test::<_, isize>(-0x80000000, 32, 0x80000000);
    test::<_, i64>(-0x80000001, 30, 0x3fffffff);
    test::<_, i64>(-0x80000001, 31, 0x7fffffff);
    test::<_, i64>(-0x80000001, 32, 0x7fffffff);
    test::<_, i64>(-0xffffffff, 31, 1);
    test::<_, i64>(-0xffffffff, 32, 1);
    test::<_, i64>(-0xffffffff, 33, 0x100000001);
    test::<_, i64>(-0x100000000, 31, 0);
    test::<_, i64>(-0x100000000, 32, 0);
    test::<_, i64>(-0x100000000, 33, 0x100000000);
    test::<_, i64>(-0x100000001, 31, 0x7fffffff);
    test::<_, i64>(-0x100000001, 32, 0xffffffff);
    test::<_, i64>(-0x100000001, 33, 0xffffffff);
}

fn mod_power_of_2_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::NEGATIVE_ONE.mod_power_of_2(200));
}

#[test]
fn mod_power_of_2_signed_fail() {
    apply_fn_to_signeds!(mod_power_of_2_signed_fail_helper);
}

#[test]
fn test_mod_power_of_2_assign_signed() {
    fn test<T: PrimitiveSigned>(x: T, pow: u64, out: T) {
        let mut mut_x = x;
        mut_x.mod_power_of_2_assign(pow);
        assert_eq!(mut_x, out);
    }
    test::<i8>(0, 0, 0);
    test::<i16>(2, 1, 0);
    test::<i32>(260, 8, 4);
    test::<i16>(1611, 4, 11);
    test::<i8>(123, 100, 123);
    test::<i64>(1000000000000, 0, 0);
    test::<i64>(1000000000000, 12, 0);
    test::<i64>(1000000000001, 12, 1);
    test::<i64>(999999999999, 12, 4095);
    test::<i64>(1000000000000, 15, 4096);
    test::<i64>(1000000000000, 100, 1000000000000);
    test::<i128>(1000000000000000000000000, 40, 1020608380928);
    test::<i128>(1000000000000000000000000, 64, 2003764205206896640);
    test::<i32>(0x7fffffff, 30, 0x3fffffff);
    test::<i32>(0x7fffffff, 31, 0x7fffffff);
    test::<isize>(0x7fffffff, 32, 0x7fffffff);
    test::<i64>(0x80000000, 30, 0);
    test::<i64>(0x80000000, 31, 0);
    test::<i64>(0x80000000, 32, 0x80000000);
    test::<i64>(0x80000001, 30, 1);
    test::<i64>(0x80000001, 31, 1);
    test::<i64>(0x80000001, 32, 0x80000001);
    test::<i64>(0xffffffff, 31, 0x7fffffff);
    test::<i64>(0xffffffff, 32, 0xffffffff);
    test::<i64>(0xffffffff, 33, 0xffffffff);
    test::<i64>(0x100000000, 31, 0);
    test::<i64>(0x100000000, 32, 0);
    test::<i64>(0x100000000, 33, 0x100000000);
    test::<i64>(0x100000001, 31, 1);
    test::<i64>(0x100000001, 32, 1);
    test::<i64>(0x100000001, 33, 0x100000001);

    test::<i8>(-2, 1, 0);
    test::<i16>(-260, 8, 252);
    test::<i32>(-1611, 4, 5);
    test::<i128>(-123, 100, 1267650600228229401496703205253);
    test::<i64>(-1000000000000, 0, 0);
    test::<i64>(-1000000000000, 12, 0);
    test::<i64>(-1000000000001, 12, 4095);
    test::<i64>(-999999999999, 12, 1);
    test::<i64>(-1000000000000, 15, 0x7000);
    test::<i128>(-1000000000000, 100, 1267650600228229400496703205376);
    test::<i128>(-1000000000000000000000000, 40, 78903246848);
    test::<i128>(-1000000000000000000000000, 64, 16442979868502654976);
    test::<i32>(-0x7fffffff, 30, 1);
    test::<i32>(-0x7fffffff, 31, 1);
    test::<i64>(-0x7fffffff, 32, 0x80000001);
    test::<isize>(-0x80000000, 30, 0);
    test::<isize>(-0x80000000, 31, 0);
    test::<i64>(-0x80000000, 32, 0x80000000);
    test::<i64>(-0x80000001, 30, 0x3fffffff);
    test::<i64>(-0x80000001, 31, 0x7fffffff);
    test::<i64>(-0x80000001, 32, 0x7fffffff);
    test::<i64>(-0xffffffff, 31, 1);
    test::<i64>(-0xffffffff, 32, 1);
    test::<i64>(-0xffffffff, 33, 0x100000001);
    test::<i64>(-0x100000000, 31, 0);
    test::<i64>(-0x100000000, 32, 0);
    test::<i64>(-0x100000000, 33, 0x100000000);
    test::<i64>(-0x100000001, 31, 0x7fffffff);
    test::<i64>(-0x100000001, 32, 0xffffffff);
    test::<i64>(-0x100000001, 33, 0xffffffff);
}

fn mod_power_of_2_assign_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!({
        let mut x = T::NEGATIVE_ONE;
        x.mod_power_of_2_assign(200);
    });
    assert_panic!({
        let mut x = T::MIN;
        x.mod_power_of_2_assign(T::WIDTH);
    });
}

#[test]
fn mod_power_of_2_assign_signed_fail() {
    apply_fn_to_signeds!(mod_power_of_2_assign_signed_fail_helper);
}

#[test]
fn test_rem_power_of_2_signed() {
    fn test<T: PrimitiveSigned>(x: T, pow: u64, out: T) {
        assert_eq!(x.rem_power_of_2(pow), out);

        let mut mut_x = x;
        mut_x.rem_power_of_2_assign(pow);
        assert_eq!(mut_x, out);
    }
    test::<i8>(0, 0, 0);
    test::<i16>(2, 1, 0);
    test::<i32>(260, 8, 4);
    test::<i64>(1611, 4, 11);
    test::<i8>(123, 100, 123);
    test::<i64>(1000000000000, 0, 0);
    test::<i64>(1000000000000, 12, 0);
    test::<i64>(1000000000001, 12, 1);
    test::<i64>(999999999999, 12, 4095);
    test::<i64>(1000000000000, 15, 4096);
    test::<i64>(1000000000000, 100, 1000000000000);
    test::<i128>(1000000000000000000000000, 40, 1020608380928);
    test::<i128>(1000000000000000000000000, 64, 2003764205206896640);
    test::<i32>(0x7fffffff, 30, 0x3fffffff);
    test::<i32>(0x7fffffff, 31, 0x7fffffff);
    test::<isize>(0x7fffffff, 32, 0x7fffffff);
    test::<i64>(0x80000000, 30, 0);
    test::<i64>(0x80000000, 31, 0);
    test::<i64>(0x80000000, 32, 0x80000000);
    test::<i64>(0x80000001, 30, 1);
    test::<i64>(0x80000001, 31, 1);
    test::<i64>(0x80000001, 32, 0x80000001);
    test::<i64>(0xffffffff, 31, 0x7fffffff);
    test::<i64>(0xffffffff, 32, 0xffffffff);
    test::<i64>(0xffffffff, 33, 0xffffffff);
    test::<i64>(0x100000000, 31, 0);
    test::<i64>(0x100000000, 32, 0);
    test::<i64>(0x100000000, 33, 0x100000000);
    test::<i64>(0x100000001, 31, 1);
    test::<i64>(0x100000001, 32, 1);
    test::<i64>(0x100000001, 33, 0x100000001);

    test::<i8>(-2, 1, 0);
    test::<i16>(-260, 8, -4);
    test::<i32>(-1611, 4, -11);
    test::<i64>(-123, 100, -123);
    test::<i64>(-1000000000000, 0, 0);
    test::<i64>(-1000000000000, 12, 0);
    test::<i64>(-1000000000001, 12, -1);
    test::<i64>(-999999999999, 12, -4095);
    test::<i64>(-1000000000000, 15, -4096);
    test::<i64>(-1000000000000, 100, -1000000000000);
    test::<i128>(-1000000000000000000000000, 40, -1020608380928);
    test::<i128>(-1000000000000000000000000, 64, -2003764205206896640);
    test::<i32>(-0x7fffffff, 30, -0x3fffffff);
    test::<i32>(-0x7fffffff, 31, -0x7fffffff);
    test::<isize>(-0x7fffffff, 32, -0x7fffffff);
    test::<i64>(-0x80000000, 30, 0);
    test::<i64>(-0x80000000, 31, 0);
    test::<i64>(-0x80000000, 32, -0x80000000);
    test::<i64>(-0x80000001, 30, -1);
    test::<i64>(-0x80000001, 31, -1);
    test::<i64>(-0x80000001, 32, -0x80000001);
    test::<i64>(-0xffffffff, 31, -0x7fffffff);
    test::<i64>(-0xffffffff, 32, -0xffffffff);
    test::<i64>(-0xffffffff, 33, -0xffffffff);
    test::<i64>(-0x100000000, 31, 0);
    test::<i64>(-0x100000000, 32, 0);
    test::<i64>(-0x100000000, 33, -0x100000000);
    test::<i64>(-0x100000001, 31, -1);
    test::<i64>(-0x100000001, 32, -1);
    test::<i64>(-0x100000001, 33, -0x100000001);
}

#[test]
fn test_neg_mod_power_of_2_unsigned() {
    fn test<T: PrimitiveUnsigned>(x: T, pow: u64, out: T) {
        assert_eq!(x.neg_mod_power_of_2(pow), out);

        let mut mut_x = x;
        mut_x.neg_mod_power_of_2_assign(pow);
        assert_eq!(mut_x, out);
    }
    test::<u8>(0, 0, 0);
    test::<u16>(260, 8, 252);
    test::<u32>(1611, 4, 5);
    test::<u32>(1, 32, u32::MAX);
    test::<u128>(123, 100, 1267650600228229401496703205253);
    test::<u64>(1000000000000, 0, 0);
    test::<u64>(1000000000000, 12, 0);
    test::<u64>(1000000000001, 12, 4095);
    test::<u64>(999999999999, 12, 1);
    test::<u64>(1000000000000, 15, 0x7000);
    test::<u128>(1000000000000, 100, 1267650600228229400496703205376);
    test::<u128>(1000000000000000000000000, 40, 78903246848);
    test::<u128>(1000000000000000000000000, 64, 16442979868502654976);
    test::<u32>(u32::MAX, 31, 1);
    test::<usize>(0xffffffff, 32, 1);
    test::<u64>(0xffffffff, 33, 0x100000001);
    test::<u64>(0x100000000, 31, 0);
    test::<u64>(0x100000000, 32, 0);
    test::<u64>(0x100000000, 33, 0x100000000);
    test::<u64>(0x100000001, 31, 0x7fffffff);
    test::<u64>(0x100000001, 32, 0xffffffff);
    test::<u64>(0x100000001, 33, 0xffffffff);
}

fn neg_mod_power_of_2_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.neg_mod_power_of_2(200));
    assert_panic!(T::MAX.neg_mod_power_of_2(T::WIDTH + 1));
    assert_panic!({
        let mut x = T::ONE;
        x.neg_mod_power_of_2_assign(200);
    });
    assert_panic!({
        let mut x = T::MAX;
        x.neg_mod_power_of_2_assign(T::WIDTH + 1);
    });
}

#[test]
fn neg_mod_power_of_2_fail() {
    apply_fn_to_unsigneds!(neg_mod_power_of_2_fail_helper);
}

#[test]
fn test_ceiling_mod_power_of_2_signed() {
    fn test<T: PrimitiveSigned>(x: T, pow: u64, out: T) {
        assert_eq!(x.ceiling_mod_power_of_2(pow), out);

        let mut mut_x = x;
        mut_x.ceiling_mod_power_of_2_assign(pow);
        assert_eq!(mut_x, out);
    }
    test::<i8>(0, 0, 0);
    test::<i16>(2, 1, 0);
    test::<i32>(260, 8, -252);
    test::<i64>(1611, 4, -5);
    test::<i128>(123, 100, -1267650600228229401496703205253);
    test::<i64>(1000000000000, 0, 0);
    test::<i64>(1000000000000, 12, 0);
    test::<i64>(1000000000001, 12, -4095);
    test::<i64>(999999999999, 12, -1);
    test::<i64>(1000000000000, 15, -0x7000);
    test::<i128>(1000000000000, 100, -1267650600228229400496703205376);
    test::<i128>(1000000000000000000000000, 40, -78903246848);
    test::<i128>(1000000000000000000000000, 64, -16442979868502654976);
    test::<i32>(0x7fffffff, 30, -1);
    test::<isize>(0x7fffffff, 31, -1);
    test::<i64>(0x7fffffff, 32, -0x80000001);
    test::<i64>(0x80000000, 30, 0);
    test::<i64>(0x80000000, 31, 0);
    test::<i64>(0x80000000, 32, -0x80000000);
    test::<i64>(0x80000001, 30, -0x3fffffff);
    test::<i64>(0x80000001, 31, -0x7fffffff);
    test::<i64>(0x80000001, 32, -0x7fffffff);
    test::<i64>(0xffffffff, 31, -1);
    test::<i64>(0xffffffff, 32, -1);
    test::<i64>(0xffffffff, 33, -0x100000001);
    test::<i64>(0x100000000, 31, 0);
    test::<i64>(0x100000000, 32, 0);
    test::<i64>(0x100000000, 33, -0x100000000);
    test::<i64>(0x100000001, 31, -0x7fffffff);
    test::<i64>(0x100000001, 32, -0xffffffff);
    test::<i64>(0x100000001, 33, -0xffffffff);

    test::<i8>(-2, 1, 0);
    test::<i16>(-260, 8, -4);
    test::<i32>(-1611, 4, -11);
    test::<i64>(-123, 100, -123);
    test::<i64>(-1000000000000, 0, 0);
    test::<i64>(-1000000000000, 12, 0);
    test::<i64>(-1000000000001, 12, -1);
    test::<i64>(-999999999999, 12, -4095);
    test::<i64>(-1000000000000, 15, -4096);
    test::<i64>(-1000000000000, 100, -1000000000000);
    test::<i128>(-1000000000000000000000000, 40, -1020608380928);
    test::<i128>(-1000000000000000000000000, 64, -2003764205206896640);
    test::<i32>(-0x7fffffff, 30, -0x3fffffff);
    test::<i32>(-0x7fffffff, 31, -0x7fffffff);
    test::<i32>(-0x7fffffff, 32, -0x7fffffff);
    test::<i32>(-0x80000000, 31, 0);
    test::<isize>(-0x80000000, 30, 0);
    test::<isize>(-0x80000000, 31, 0);
    test::<isize>(-0x80000000, 32, -0x80000000);
    test::<i64>(-0x80000001, 30, -1);
    test::<i64>(-0x80000001, 31, -1);
    test::<i64>(-0x80000001, 32, -0x80000001);
    test::<i64>(-0xffffffff, 31, -0x7fffffff);
    test::<i64>(-0xffffffff, 32, -0xffffffff);
    test::<i64>(-0xffffffff, 33, -0xffffffff);
    test::<i64>(-0x100000000, 31, 0);
    test::<i64>(-0x100000000, 32, 0);
    test::<i64>(-0x100000000, 33, -0x100000000);
    test::<i64>(-0x100000001, 31, -1);
    test::<i64>(-0x100000001, 32, -1);
    test::<i64>(-0x100000001, 33, -0x100000001);
}

fn ceiling_mod_power_of_2_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::ONE.ceiling_mod_power_of_2(T::WIDTH));
    assert_panic!(T::MIN.ceiling_mod_power_of_2(T::WIDTH));
    assert_panic!({
        let mut x = T::ONE;
        x.ceiling_mod_power_of_2_assign(T::WIDTH);
    });
    assert_panic!({
        let mut x = T::MIN;
        x.ceiling_mod_power_of_2_assign(T::WIDTH);
    });
}

#[test]
fn ceiling_mod_power_of_2_fail() {
    apply_fn_to_signeds!(ceiling_mod_power_of_2_fail_helper);
}

fn mod_power_of_2_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(n, pow)| {
        let mut mut_n = n;
        mut_n.mod_power_of_2_assign(pow);
        let result = mut_n;
        assert!(result.mod_power_of_2_is_reduced(pow));
        assert_eq!(n.mod_power_of_2(pow), result);

        let mut mut_n = n;
        mut_n.rem_power_of_2_assign(pow);
        assert_eq!(mut_n, result);
        assert_eq!(n.rem_power_of_2(pow), result);

        assert!(result <= n);
        assert_eq!(result == T::ZERO, n.divisible_by_power_of_2(pow));
        assert_eq!(result.mod_power_of_2(pow), result);
    });

    unsigned_triple_gen_var_4::<T, u64>().test_properties(|(x, y, pow)| {
        assert_eq!(
            x.wrapping_add(y).mod_power_of_2(pow),
            x.mod_power_of_2(pow)
                .wrapping_add(y.mod_power_of_2(pow))
                .mod_power_of_2(pow)
        );
        assert_eq!(
            x.wrapping_mul(y).mod_power_of_2(pow),
            x.mod_power_of_2(pow)
                .wrapping_mul(y.mod_power_of_2(pow))
                .mod_power_of_2(pow)
        );
    });

    unsigned_triple_gen_var_13::<T, u64>().test_properties(|(n, u, v)| {
        assert_eq!(
            n.mod_power_of_2(u).mod_power_of_2(v),
            n.mod_power_of_2(min(u, v))
        );
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(n.mod_power_of_2(0), T::ZERO);
    });

    unsigned_gen().test_properties(|pow| {
        assert_eq!(T::ZERO.mod_power_of_2(pow), T::ZERO);
    });
}

fn mod_power_of_2_properties_helper_signed<T: PrimitiveSigned>()
where
    <T as ModPowerOf2>::Output: ExactFrom<T> + PrimitiveUnsigned,
{
    signed_unsigned_pair_gen_var_10::<T>().test_properties(|(n, pow)| {
        let result = n.mod_power_of_2(pow);
        assert!(result.mod_power_of_2_is_reduced(pow));
        assert_eq!(
            result == <T as ModPowerOf2>::Output::ZERO,
            n.divisible_by_power_of_2(pow)
        );
        assert_eq!(result.mod_power_of_2(pow), result);
    });

    signed_unsigned_pair_gen_var_4::<T>().test_properties(|(n, pow)| {
        let mut mut_n = n;
        mut_n.mod_power_of_2_assign(pow);
        let result = mut_n;
        assert_eq!(
            n.mod_power_of_2(pow),
            <T as ModPowerOf2>::Output::exact_from(result)
        );

        assert!(result >= T::ZERO);
        assert_eq!(result == T::ZERO, n.divisible_by_power_of_2(pow));
        assert_eq!(
            result.mod_power_of_2(pow),
            <T as ModPowerOf2>::Output::exact_from(result)
        );
    });

    signed_gen::<T>().test_properties(|n| {
        assert_eq!(n.mod_power_of_2(0), <T as ModPowerOf2>::Output::ZERO);
    });

    unsigned_gen().test_properties(|pow| {
        assert_eq!(
            T::ZERO.mod_power_of_2(pow),
            <T as ModPowerOf2>::Output::ZERO
        );
    });
}

#[test]
fn mod_power_of_2_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_properties_helper_unsigned);
    apply_fn_to_signeds!(mod_power_of_2_properties_helper_signed);
}

fn rem_power_of_2_properties_helper<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(n, pow)| {
        let mut mut_n = n;
        mut_n.rem_power_of_2_assign(pow);
        let result = mut_n;
        assert_eq!(n.rem_power_of_2(pow), result);

        if n != T::MIN {
            assert_eq!((-n).rem_power_of_2(pow), -result);
        }
        assert!(result.le_abs(&n));
        assert_eq!(result == T::ZERO, n.divisible_by_power_of_2(pow));
        assert_eq!(result.rem_power_of_2(pow), result);
        assert!(result == T::ZERO || (result > T::ZERO) == (n > T::ZERO));
    });

    signed_gen::<T>().test_properties(|n| {
        assert_eq!(n.rem_power_of_2(0), T::ZERO);
    });

    unsigned_gen().test_properties(|pow| {
        assert_eq!(T::ZERO.rem_power_of_2(pow), T::ZERO);
    });
}

#[test]
fn rem_power_of_2_properties() {
    apply_fn_to_signeds!(rem_power_of_2_properties_helper);
}

fn neg_mod_power_of_2_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_20::<T>().test_properties(|(n, pow)| {
        let mut mut_n = n;
        mut_n.neg_mod_power_of_2_assign(pow);
        let result = mut_n;
        assert!(result.mod_power_of_2_is_reduced(pow));
        assert_eq!(n.neg_mod_power_of_2(pow), result);

        assert_eq!(result == T::ZERO, n.divisible_by_power_of_2(pow));
        assert!(result
            .wrapping_add(n.mod_power_of_2(pow))
            .divisible_by_power_of_2(pow));
        assert_eq!(result.neg_mod_power_of_2(pow), n.mod_power_of_2(pow));
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(n.neg_mod_power_of_2(0), T::ZERO);
    });

    unsigned_gen().test_properties(|pow| {
        assert_eq!(T::ZERO.neg_mod_power_of_2(pow), T::ZERO);
    });
}

#[test]
fn neg_mod_power_of_2_properties() {
    apply_fn_to_unsigneds!(neg_mod_power_of_2_properties_helper);
}

fn ceiling_mod_power_of_2_properties_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    signed_unsigned_pair_gen_var_11::<U, S>().test_properties(|(n, pow)| {
        let mut mut_n = n;
        mut_n.ceiling_mod_power_of_2_assign(pow);
        let result = mut_n;
        assert_eq!(n.ceiling_mod_power_of_2(pow), result);

        assert!(result <= S::ZERO);
        assert_eq!(result == S::ZERO, n.divisible_by_power_of_2(pow));
    });

    signed_gen::<S>().test_properties(|n| {
        assert_eq!(n.ceiling_mod_power_of_2(0), S::ZERO);
    });

    unsigned_gen().test_properties(|pow| {
        assert_eq!(S::ZERO.ceiling_mod_power_of_2(pow), S::ZERO);
    });
}

#[test]
fn ceiling_mod_power_of_2_properties() {
    apply_fn_to_unsigned_signed_pairs!(ceiling_mod_power_of_2_properties_helper);
}
