use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
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
        x.mod_power_of_2_assign(200)
    });
    assert_panic!({
        let mut x = T::MIN;
        x.mod_power_of_2_assign(T::WIDTH)
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
        x.neg_mod_power_of_2_assign(200)
    });
    assert_panic!({
        let mut x = T::MAX;
        x.neg_mod_power_of_2_assign(T::WIDTH + 1)
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
        x.ceiling_mod_power_of_2_assign(T::WIDTH)
    });
    assert_panic!({
        let mut x = T::MIN;
        x.ceiling_mod_power_of_2_assign(T::WIDTH)
    });
}

#[test]
fn ceiling_mod_power_of_2_fail() {
    apply_fn_to_signeds!(ceiling_mod_power_of_2_fail_helper);
}
