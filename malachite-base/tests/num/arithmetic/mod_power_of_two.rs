use std::fmt::Debug;

use malachite_base::num::arithmetic::traits::{
    CeilingModPowerOfTwo, CeilingModPowerOfTwoAssign, ModPowerOfTwo, ModPowerOfTwoAssign,
    NegModPowerOfTwo, NegModPowerOfTwoAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{NegativeOne, One};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_mod_power_of_two_and_rem_power_of_two_unsigned() {
    fn test<T: PrimitiveUnsigned>(x: T, pow: u64, out: T) {
        assert_eq!(x.mod_power_of_two(pow), out);

        let mut mut_x = x;
        mut_x.mod_power_of_two_assign(pow);
        assert_eq!(mut_x, out);

        assert_eq!(x.rem_power_of_two(pow), out);

        let mut mut_x = x;
        mut_x.rem_power_of_two_assign(pow);
        assert_eq!(mut_x, out);
    };
    test::<u8>(0, 0, 0);
    test::<u16>(260, 8, 4);
    test::<u32>(1_611, 4, 11);
    test::<u8>(123, 100, 123);
    test::<u64>(1_000_000_000_000, 0, 0);
    test::<u64>(1_000_000_000_000, 12, 0);
    test::<u64>(1_000_000_000_001, 12, 1);
    test::<u64>(999_999_999_999, 12, 4_095);
    test::<u64>(1_000_000_000_000, 15, 4_096);
    test::<u64>(1_000_000_000_000, 100, 1_000_000_000_000);
    test::<u128>(1_000_000_000_000_000_000_000_000, 40, 1_020_608_380_928);
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        64,
        2_003_764_205_206_896_640,
    );
    test::<u32>(u32::MAX, 31, 0x7fff_ffff);
    test::<u32>(u32::MAX, 32, u32::MAX);
    test::<usize>(0xffff_ffff, 33, 0xffff_ffff);
    test::<u64>(0x1_0000_0000, 31, 0);
    test::<u64>(0x1_0000_0000, 32, 0);
    test::<u64>(0x1_0000_0000, 33, 0x1_0000_0000);
    test::<u64>(0x1_0000_0001, 31, 1);
    test::<u64>(0x1_0000_0001, 32, 1);
    test::<u64>(0x1_0000_0001, 33, 0x1_0000_0001);
}

#[test]
fn test_mod_power_of_two_signed() {
    fn test<T: PrimitiveSigned>(x: T, pow: u64, out: <T as ModPowerOfTwo>::Output)
    where
        <T as ModPowerOfTwo>::Output: Copy + Debug + Eq,
    {
        assert_eq!(x.mod_power_of_two(pow), out);
    };
    test::<i8>(0, 0, 0);
    test::<i16>(2, 1, 0);
    test::<i32>(260, 8, 4);
    test::<i16>(1_611, 4, 11);
    test::<i8>(123, 100, 123);
    test::<i64>(1_000_000_000_000, 0, 0);
    test::<i64>(1_000_000_000_000, 12, 0);
    test::<i64>(1_000_000_000_001, 12, 1);
    test::<i64>(999_999_999_999, 12, 4_095);
    test::<i64>(1_000_000_000_000, 15, 4_096);
    test::<i64>(1_000_000_000_000, 100, 1_000_000_000_000);
    test::<i128>(1_000_000_000_000_000_000_000_000, 40, 1_020_608_380_928);
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        64,
        2_003_764_205_206_896_640,
    );
    test::<i32>(0x7fff_ffff, 30, 0x3fff_ffff);
    test::<i32>(0x7fff_ffff, 31, 0x7fff_ffff);
    test::<isize>(0x7fff_ffff, 32, 0x7fff_ffff);
    test::<i64>(0x8000_0000, 30, 0);
    test::<i64>(0x8000_0000, 31, 0);
    test::<i64>(0x8000_0000, 32, 0x8000_0000);
    test::<i64>(0x8000_0001, 30, 1);
    test::<i64>(0x8000_0001, 31, 1);
    test::<i64>(0x8000_0001, 32, 0x8000_0001);
    test::<i64>(0xffff_ffff, 31, 0x7fff_ffff);
    test::<i64>(0xffff_ffff, 32, 0xffff_ffff);
    test::<i64>(0xffff_ffff, 33, 0xffff_ffff);
    test::<i64>(0x1_0000_0000, 31, 0);
    test::<i64>(0x1_0000_0000, 32, 0);
    test::<i64>(0x1_0000_0000, 33, 0x1_0000_0000);
    test::<i64>(0x1_0000_0001, 31, 1);
    test::<i64>(0x1_0000_0001, 32, 1);
    test::<i64>(0x1_0000_0001, 33, 0x1_0000_0001);

    test::<i8>(-2, 1, 0);
    test::<i16>(-260, 8, 252);
    test::<i32>(-1_611, 4, 5);
    test::<i128>(-123, 100, 1_267_650_600_228_229_401_496_703_205_253);
    test::<i64>(-1_000_000_000_000, 0, 0);
    test::<i64>(-1_000_000_000_000, 12, 0);
    test::<i64>(-1_000_000_000_001, 12, 4_095);
    test::<i64>(-999_999_999_999, 12, 1);
    test::<i64>(-1_000_000_000_000, 15, 0x7000);
    test::<i128>(
        -1_000_000_000_000,
        100,
        1_267_650_600_228_229_400_496_703_205_376,
    );
    test::<i128>(-1_000_000_000_000_000_000_000_000, 40, 78_903_246_848);
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        64,
        16_442_979_868_502_654_976,
    );
    test::<i32>(-0x7fff_ffff, 30, 1);
    test::<i32>(-0x7fff_ffff, 31, 1);
    test::<i32>(-0x7fff_ffff, 32, 0x8000_0001);
    test::<isize>(-0x8000_0000, 30, 0);
    test::<isize>(-0x8000_0000, 31, 0);
    test::<isize>(-0x8000_0000, 32, 0x8000_0000);
    test::<i64>(-0x8000_0001, 30, 0x3fff_ffff);
    test::<i64>(-0x8000_0001, 31, 0x7fff_ffff);
    test::<i64>(-0x8000_0001, 32, 0x7fff_ffff);
    test::<i64>(-0xffff_ffff, 31, 1);
    test::<i64>(-0xffff_ffff, 32, 1);
    test::<i64>(-0xffff_ffff, 33, 0x1_0000_0001);
    test::<i64>(-0x1_0000_0000, 31, 0);
    test::<i64>(-0x1_0000_0000, 32, 0);
    test::<i64>(-0x1_0000_0000, 33, 0x1_0000_0000);
    test::<i64>(-0x1_0000_0001, 31, 0x7fff_ffff);
    test::<i64>(-0x1_0000_0001, 32, 0xffff_ffff);
    test::<i64>(-0x1_0000_0001, 33, 0xffff_ffff);
}

macro_rules! mod_power_of_two_signed_fail {
    ($t:ident, $mod_power_of_two_signed_fail:ident) => {
        #[test]
        #[should_panic]
        fn $mod_power_of_two_signed_fail() {
            $t::NEGATIVE_ONE.mod_power_of_two(200);
        }
    };
}
mod_power_of_two_signed_fail!(i8, mod_power_of_two_signed_i8_fail);
mod_power_of_two_signed_fail!(i16, mod_power_of_two_signed_i16_fail);
mod_power_of_two_signed_fail!(i32, mod_power_of_two_signed_i32_fail);
mod_power_of_two_signed_fail!(i64, mod_power_of_two_signed_i64_fail);
mod_power_of_two_signed_fail!(i128, mod_power_of_two_signed_i128_fail);
mod_power_of_two_signed_fail!(isize, mod_power_of_two_signed_isize_fail);

#[test]
fn test_mod_power_of_two_assign_signed() {
    fn test<T: PrimitiveSigned>(x: T, pow: u64, out: T) {
        let mut mut_x = x;
        mut_x.mod_power_of_two_assign(pow);
        assert_eq!(mut_x, out);
    };
    test::<i8>(0, 0, 0);
    test::<i16>(2, 1, 0);
    test::<i32>(260, 8, 4);
    test::<i16>(1_611, 4, 11);
    test::<i8>(123, 100, 123);
    test::<i64>(1_000_000_000_000, 0, 0);
    test::<i64>(1_000_000_000_000, 12, 0);
    test::<i64>(1_000_000_000_001, 12, 1);
    test::<i64>(999_999_999_999, 12, 4_095);
    test::<i64>(1_000_000_000_000, 15, 4_096);
    test::<i64>(1_000_000_000_000, 100, 1_000_000_000_000);
    test::<i128>(1_000_000_000_000_000_000_000_000, 40, 1_020_608_380_928);
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        64,
        2_003_764_205_206_896_640,
    );
    test::<i32>(0x7fff_ffff, 30, 0x3fff_ffff);
    test::<i32>(0x7fff_ffff, 31, 0x7fff_ffff);
    test::<isize>(0x7fff_ffff, 32, 0x7fff_ffff);
    test::<i64>(0x8000_0000, 30, 0);
    test::<i64>(0x8000_0000, 31, 0);
    test::<i64>(0x8000_0000, 32, 0x8000_0000);
    test::<i64>(0x8000_0001, 30, 1);
    test::<i64>(0x8000_0001, 31, 1);
    test::<i64>(0x8000_0001, 32, 0x8000_0001);
    test::<i64>(0xffff_ffff, 31, 0x7fff_ffff);
    test::<i64>(0xffff_ffff, 32, 0xffff_ffff);
    test::<i64>(0xffff_ffff, 33, 0xffff_ffff);
    test::<i64>(0x1_0000_0000, 31, 0);
    test::<i64>(0x1_0000_0000, 32, 0);
    test::<i64>(0x1_0000_0000, 33, 0x1_0000_0000);
    test::<i64>(0x1_0000_0001, 31, 1);
    test::<i64>(0x1_0000_0001, 32, 1);
    test::<i64>(0x1_0000_0001, 33, 0x1_0000_0001);

    test::<i8>(-2, 1, 0);
    test::<i16>(-260, 8, 252);
    test::<i32>(-1_611, 4, 5);
    test::<i128>(-123, 100, 1_267_650_600_228_229_401_496_703_205_253);
    test::<i64>(-1_000_000_000_000, 0, 0);
    test::<i64>(-1_000_000_000_000, 12, 0);
    test::<i64>(-1_000_000_000_001, 12, 4_095);
    test::<i64>(-999_999_999_999, 12, 1);
    test::<i64>(-1_000_000_000_000, 15, 0x7000);
    test::<i128>(
        -1_000_000_000_000,
        100,
        1_267_650_600_228_229_400_496_703_205_376,
    );
    test::<i128>(-1_000_000_000_000_000_000_000_000, 40, 78_903_246_848);
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        64,
        16_442_979_868_502_654_976,
    );
    test::<i32>(-0x7fff_ffff, 30, 1);
    test::<i32>(-0x7fff_ffff, 31, 1);
    test::<i64>(-0x7fff_ffff, 32, 0x8000_0001);
    test::<isize>(-0x8000_0000, 30, 0);
    test::<isize>(-0x8000_0000, 31, 0);
    test::<i64>(-0x8000_0000, 32, 0x8000_0000);
    test::<i64>(-0x8000_0001, 30, 0x3fff_ffff);
    test::<i64>(-0x8000_0001, 31, 0x7fff_ffff);
    test::<i64>(-0x8000_0001, 32, 0x7fff_ffff);
    test::<i64>(-0xffff_ffff, 31, 1);
    test::<i64>(-0xffff_ffff, 32, 1);
    test::<i64>(-0xffff_ffff, 33, 0x1_0000_0001);
    test::<i64>(-0x1_0000_0000, 31, 0);
    test::<i64>(-0x1_0000_0000, 32, 0);
    test::<i64>(-0x1_0000_0000, 33, 0x1_0000_0000);
    test::<i64>(-0x1_0000_0001, 31, 0x7fff_ffff);
    test::<i64>(-0x1_0000_0001, 32, 0xffff_ffff);
    test::<i64>(-0x1_0000_0001, 33, 0xffff_ffff);
}

macro_rules! mod_power_of_two_assign_signed_fail {
    ($t:ident, $mod_power_of_two_signed_fail_1:ident, $mod_power_of_two_signed_fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $mod_power_of_two_signed_fail_1() {
            let mut x = $t::NEGATIVE_ONE;
            x.mod_power_of_two_assign(200);
        }

        #[test]
        #[should_panic]
        fn $mod_power_of_two_signed_fail_2() {
            let mut x = $t::MIN;
            x.mod_power_of_two_assign($t::WIDTH);
        }
    };
}
mod_power_of_two_assign_signed_fail!(
    i8,
    mod_power_of_two_signed_i8_fail_1,
    mod_power_of_two_signed_i8_fail_2
);
mod_power_of_two_assign_signed_fail!(
    i16,
    mod_power_of_two_signed_i16_fail_1,
    mod_power_of_two_signed_i16_fail_2
);
mod_power_of_two_assign_signed_fail!(
    i32,
    mod_power_of_two_signed_i32_fail_1,
    mod_power_of_two_signed_i32_fail_2
);
mod_power_of_two_assign_signed_fail!(
    i64,
    mod_power_of_two_signed_i64_fail_1,
    mod_power_of_two_signed_i64_fail_2
);
mod_power_of_two_assign_signed_fail!(
    i128,
    mod_power_of_two_signed_i128_fail_1,
    mod_power_of_two_signed_i128_fail_2
);
mod_power_of_two_assign_signed_fail!(
    isize,
    mod_power_of_two_signed_isize_fail_1,
    mod_power_of_two_signed_isize_fail_2
);

#[test]
fn test_rem_power_of_two_signed() {
    fn test<T: PrimitiveSigned>(x: T, pow: u64, out: T) {
        assert_eq!(x.rem_power_of_two(pow), out);

        let mut mut_x = x;
        mut_x.rem_power_of_two_assign(pow);
        assert_eq!(mut_x, out);
    };
    test::<i8>(0, 0, 0);
    test::<i16>(2, 1, 0);
    test::<i32>(260, 8, 4);
    test::<i64>(1_611, 4, 11);
    test::<i8>(123, 100, 123);
    test::<i64>(1_000_000_000_000, 0, 0);
    test::<i64>(1_000_000_000_000, 12, 0);
    test::<i64>(1_000_000_000_001, 12, 1);
    test::<i64>(999_999_999_999, 12, 4_095);
    test::<i64>(1_000_000_000_000, 15, 4_096);
    test::<i64>(1_000_000_000_000, 100, 1_000_000_000_000);
    test::<i128>(1_000_000_000_000_000_000_000_000, 40, 1_020_608_380_928);
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        64,
        2_003_764_205_206_896_640,
    );
    test::<i32>(0x7fff_ffff, 30, 0x3fff_ffff);
    test::<i32>(0x7fff_ffff, 31, 0x7fff_ffff);
    test::<isize>(0x7fff_ffff, 32, 0x7fff_ffff);
    test::<i64>(0x8000_0000, 30, 0);
    test::<i64>(0x8000_0000, 31, 0);
    test::<i64>(0x8000_0000, 32, 0x8000_0000);
    test::<i64>(0x8000_0001, 30, 1);
    test::<i64>(0x8000_0001, 31, 1);
    test::<i64>(0x8000_0001, 32, 0x8000_0001);
    test::<i64>(0xffff_ffff, 31, 0x7fff_ffff);
    test::<i64>(0xffff_ffff, 32, 0xffff_ffff);
    test::<i64>(0xffff_ffff, 33, 0xffff_ffff);
    test::<i64>(0x1_0000_0000, 31, 0);
    test::<i64>(0x1_0000_0000, 32, 0);
    test::<i64>(0x1_0000_0000, 33, 0x1_0000_0000);
    test::<i64>(0x1_0000_0001, 31, 1);
    test::<i64>(0x1_0000_0001, 32, 1);
    test::<i64>(0x1_0000_0001, 33, 0x1_0000_0001);

    test::<i8>(-2, 1, 0);
    test::<i16>(-260, 8, -4);
    test::<i32>(-1_611, 4, -11);
    test::<i64>(-123, 100, -123);
    test::<i64>(-1_000_000_000_000, 0, 0);
    test::<i64>(-1_000_000_000_000, 12, 0);
    test::<i64>(-1_000_000_000_001, 12, -1);
    test::<i64>(-999_999_999_999, 12, -4_095);
    test::<i64>(-1_000_000_000_000, 15, -4_096);
    test::<i64>(-1_000_000_000_000, 100, -1_000_000_000_000);
    test::<i128>(-1_000_000_000_000_000_000_000_000, 40, -1_020_608_380_928);
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        64,
        -2_003_764_205_206_896_640,
    );
    test::<i32>(-0x7fff_ffff, 30, -0x3fff_ffff);
    test::<i32>(-0x7fff_ffff, 31, -0x7fff_ffff);
    test::<isize>(-0x7fff_ffff, 32, -0x7fff_ffff);
    test::<i64>(-0x8000_0000, 30, 0);
    test::<i64>(-0x8000_0000, 31, 0);
    test::<i64>(-0x8000_0000, 32, -0x8000_0000);
    test::<i64>(-0x8000_0001, 30, -1);
    test::<i64>(-0x8000_0001, 31, -1);
    test::<i64>(-0x8000_0001, 32, -0x8000_0001);
    test::<i64>(-0xffff_ffff, 31, -0x7fff_ffff);
    test::<i64>(-0xffff_ffff, 32, -0xffff_ffff);
    test::<i64>(-0xffff_ffff, 33, -0xffff_ffff);
    test::<i64>(-0x1_0000_0000, 31, 0);
    test::<i64>(-0x1_0000_0000, 32, 0);
    test::<i64>(-0x1_0000_0000, 33, -0x1_0000_0000);
    test::<i64>(-0x1_0000_0001, 31, -1);
    test::<i64>(-0x1_0000_0001, 32, -1);
    test::<i64>(-0x1_0000_0001, 33, -0x1_0000_0001);
}

#[test]
fn test_neg_mod_power_of_two_unsigned() {
    fn test<T: PrimitiveUnsigned>(x: T, pow: u64, out: T) {
        assert_eq!(x.neg_mod_power_of_two(pow), out);

        let mut mut_x = x;
        mut_x.neg_mod_power_of_two_assign(pow);
        assert_eq!(mut_x, out);
    };
    test::<u8>(0, 0, 0);
    test::<u16>(260, 8, 252);
    test::<u32>(1_611, 4, 5);
    test::<u32>(1, 32, u32::MAX);
    test::<u128>(123, 100, 1_267_650_600_228_229_401_496_703_205_253);
    test::<u64>(1_000_000_000_000, 0, 0);
    test::<u64>(1_000_000_000_000, 12, 0);
    test::<u64>(1_000_000_000_001, 12, 4_095);
    test::<u64>(999_999_999_999, 12, 1);
    test::<u64>(1_000_000_000_000, 15, 0x7000);
    test::<u128>(
        1_000_000_000_000,
        100,
        1_267_650_600_228_229_400_496_703_205_376,
    );
    test::<u128>(1_000_000_000_000_000_000_000_000, 40, 78_903_246_848);
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        64,
        16_442_979_868_502_654_976,
    );
    test::<u32>(u32::MAX, 31, 1);
    test::<usize>(0xffff_ffff, 32, 1);
    test::<u64>(0xffff_ffff, 33, 0x1_0000_0001);
    test::<u64>(0x1_0000_0000, 31, 0);
    test::<u64>(0x1_0000_0000, 32, 0);
    test::<u64>(0x1_0000_0000, 33, 0x1_0000_0000);
    test::<u64>(0x1_0000_0001, 31, 0x7fff_ffff);
    test::<u64>(0x1_0000_0001, 32, 0xffff_ffff);
    test::<u64>(0x1_0000_0001, 33, 0xffff_ffff);
}

macro_rules! neg_mod_power_of_two_unsigned_fail {
    (
        $t:ident,
        $neg_mod_power_of_two_unsigned_fail_1:ident,
        $neg_mod_power_of_two_unsigned_fail_2:ident,
        $neg_mod_power_of_two_assign_unsigned_fail_1:ident,
        $neg_mod_power_of_two_assign_unsigned_fail_2:ident
    ) => {
        #[test]
        #[should_panic]
        fn $neg_mod_power_of_two_unsigned_fail_1() {
            $t::ONE.neg_mod_power_of_two(200);
        }

        #[test]
        #[should_panic]
        fn $neg_mod_power_of_two_unsigned_fail_2() {
            $t::MAX.neg_mod_power_of_two($t::WIDTH + 1);
        }

        #[test]
        #[should_panic]
        fn $neg_mod_power_of_two_assign_unsigned_fail_1() {
            let mut x = $t::ONE;
            x.neg_mod_power_of_two_assign(200);
        }

        #[test]
        #[should_panic]
        fn $neg_mod_power_of_two_assign_unsigned_fail_2() {
            let mut x = $t::MAX;
            x.neg_mod_power_of_two_assign($t::WIDTH + 1);
        }
    };
}
neg_mod_power_of_two_unsigned_fail!(
    u8,
    neg_mod_power_of_two_unsigned_u8_fail_1,
    neg_mod_power_of_two_unsigned_u8_fail_2,
    neg_mod_power_of_two_assign_unsigned_u8_fail_1,
    neg_mod_power_of_two_assign_unsigned_u8_fail_2
);
neg_mod_power_of_two_unsigned_fail!(
    u16,
    neg_mod_power_of_two_unsigned_u16_fail_1,
    neg_mod_power_of_two_unsigned_u16_fail_2,
    neg_mod_power_of_two_assign_unsigned_u16_fail_1,
    neg_mod_power_of_two_assign_unsigned_u16_fail_2
);
neg_mod_power_of_two_unsigned_fail!(
    u32,
    neg_mod_power_of_two_unsigned_u32_fail_1,
    neg_mod_power_of_two_unsigned_u32_fail_2,
    neg_mod_power_of_two_assign_unsigned_u32_fail_1,
    neg_mod_power_of_two_assign_unsigned_u32_fail_2
);
neg_mod_power_of_two_unsigned_fail!(
    u64,
    neg_mod_power_of_two_unsigned_u64_fail_1,
    neg_mod_power_of_two_unsigned_u64_fail_2,
    neg_mod_power_of_two_assign_unsigned_u64_fail_1,
    neg_mod_power_of_two_assign_unsigned_u64_fail_2
);
neg_mod_power_of_two_unsigned_fail!(
    u128,
    neg_mod_power_of_two_unsigned_u128_fail_1,
    neg_mod_power_of_two_unsigned_u128_fail_2,
    neg_mod_power_of_two_assign_unsigned_u128_fail_1,
    neg_mod_power_of_two_assign_unsigned_u128_fail_2
);
neg_mod_power_of_two_unsigned_fail!(
    usize,
    neg_mod_power_of_two_unsigned_usize_fail_1,
    neg_mod_power_of_two_unsigned_usize_fail_2,
    neg_mod_power_of_two_assign_unsigned_usize_fail_1,
    neg_mod_power_of_two_assign_unsigned_usize_fail_2
);

#[test]
fn test_ceiling_mod_power_of_two_signed() {
    fn test<T: PrimitiveSigned>(x: T, pow: u64, out: T) {
        assert_eq!(x.ceiling_mod_power_of_two(pow), out);

        let mut mut_x = x;
        mut_x.ceiling_mod_power_of_two_assign(pow);
        assert_eq!(mut_x, out);
    };
    test::<i8>(0, 0, 0);
    test::<i16>(2, 1, 0);
    test::<i32>(260, 8, -252);
    test::<i64>(1_611, 4, -5);
    test::<i128>(123, 100, -1_267_650_600_228_229_401_496_703_205_253);
    test::<i64>(1_000_000_000_000, 0, 0);
    test::<i64>(1_000_000_000_000, 12, 0);
    test::<i64>(1_000_000_000_001, 12, -4_095);
    test::<i64>(999_999_999_999, 12, -1);
    test::<i64>(1_000_000_000_000, 15, -0x7000);
    test::<i128>(
        1_000_000_000_000,
        100,
        -1_267_650_600_228_229_400_496_703_205_376,
    );
    test::<i128>(1_000_000_000_000_000_000_000_000, 40, -78_903_246_848);
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        64,
        -16_442_979_868_502_654_976,
    );
    test::<i32>(0x7fff_ffff, 30, -1);
    test::<isize>(0x7fff_ffff, 31, -1);
    test::<i64>(0x7fff_ffff, 32, -0x8000_0001);
    test::<i64>(0x8000_0000, 30, 0);
    test::<i64>(0x8000_0000, 31, 0);
    test::<i64>(0x8000_0000, 32, -0x8000_0000);
    test::<i64>(0x8000_0001, 30, -0x3fff_ffff);
    test::<i64>(0x8000_0001, 31, -0x7fff_ffff);
    test::<i64>(0x8000_0001, 32, -0x7fff_ffff);
    test::<i64>(0xffff_ffff, 31, -1);
    test::<i64>(0xffff_ffff, 32, -1);
    test::<i64>(0xffff_ffff, 33, -0x1_0000_0001);
    test::<i64>(0x1_0000_0000, 31, 0);
    test::<i64>(0x1_0000_0000, 32, 0);
    test::<i64>(0x1_0000_0000, 33, -0x1_0000_0000);
    test::<i64>(0x1_0000_0001, 31, -0x7fff_ffff);
    test::<i64>(0x1_0000_0001, 32, -0xffff_ffff);
    test::<i64>(0x1_0000_0001, 33, -0xffff_ffff);

    test::<i8>(-2, 1, 0);
    test::<i16>(-260, 8, -4);
    test::<i32>(-1_611, 4, -11);
    test::<i64>(-123, 100, -123);
    test::<i64>(-1_000_000_000_000, 0, 0);
    test::<i64>(-1_000_000_000_000, 12, 0);
    test::<i64>(-1_000_000_000_001, 12, -1);
    test::<i64>(-999_999_999_999, 12, -4_095);
    test::<i64>(-1_000_000_000_000, 15, -4_096);
    test::<i64>(-1_000_000_000_000, 100, -1_000_000_000_000);
    test::<i128>(-1_000_000_000_000_000_000_000_000, 40, -1_020_608_380_928);
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        64,
        -2_003_764_205_206_896_640,
    );
    test::<i32>(-0x7fff_ffff, 30, -0x3fff_ffff);
    test::<i32>(-0x7fff_ffff, 31, -0x7fff_ffff);
    test::<i32>(-0x7fff_ffff, 32, -0x7fff_ffff);
    test::<i32>(-0x8000_0000, 31, 0);
    test::<isize>(-0x8000_0000, 30, 0);
    test::<isize>(-0x8000_0000, 31, 0);
    test::<isize>(-0x8000_0000, 32, -0x8000_0000);
    test::<i64>(-0x8000_0001, 30, -1);
    test::<i64>(-0x8000_0001, 31, -1);
    test::<i64>(-0x8000_0001, 32, -0x8000_0001);
    test::<i64>(-0xffff_ffff, 31, -0x7fff_ffff);
    test::<i64>(-0xffff_ffff, 32, -0xffff_ffff);
    test::<i64>(-0xffff_ffff, 33, -0xffff_ffff);
    test::<i64>(-0x1_0000_0000, 31, 0);
    test::<i64>(-0x1_0000_0000, 32, 0);
    test::<i64>(-0x1_0000_0000, 33, -0x1_0000_0000);
    test::<i64>(-0x1_0000_0001, 31, -1);
    test::<i64>(-0x1_0000_0001, 32, -1);
    test::<i64>(-0x1_0000_0001, 33, -0x1_0000_0001);
}

macro_rules! ceiling_mod_power_of_two_signed_fail {
    (
        $t:ident,
        $ceiling_mod_power_of_two_signed_fail_1:ident,
        $ceiling_mod_power_of_two_signed_fail_2:ident,
        $ceiling_mod_power_of_two_assign_signed_fail_1:ident,
        $ceiling_mod_power_of_two_assign_signed_fail_2:ident
    ) => {
        #[test]
        #[should_panic]
        fn $ceiling_mod_power_of_two_signed_fail_1() {
            $t::ONE.ceiling_mod_power_of_two($t::WIDTH);
        }

        #[test]
        #[should_panic]
        fn $ceiling_mod_power_of_two_signed_fail_2() {
            $t::MIN.ceiling_mod_power_of_two($t::WIDTH);
        }

        #[test]
        #[should_panic]
        fn $ceiling_mod_power_of_two_assign_signed_fail_1() {
            let mut x = $t::ONE;
            x.ceiling_mod_power_of_two_assign($t::WIDTH);
        }

        #[test]
        #[should_panic]
        fn $ceiling_mod_power_of_two_assign_signed_fail_2() {
            let mut x = $t::MIN;
            x.ceiling_mod_power_of_two_assign($t::WIDTH);
        }
    };
}
ceiling_mod_power_of_two_signed_fail!(
    i8,
    ceiling_mod_power_of_two_signed_i8_fail_1,
    ceiling_mod_power_of_two_signed_i8_fail_2,
    ceiling_mod_power_of_two_assign_signed_i8_fail_1,
    ceiling_mod_power_of_two_assign_signed_i8_fail_2
);
ceiling_mod_power_of_two_signed_fail!(
    i16,
    ceiling_mod_power_of_two_signed_i16_fail_1,
    ceiling_mod_power_of_two_signed_i16_fail_2,
    ceiling_mod_power_of_two_assign_signed_i16_fail_1,
    ceiling_mod_power_of_two_assign_signed_i16_fail_2
);
ceiling_mod_power_of_two_signed_fail!(
    i32,
    ceiling_mod_power_of_two_signed_i32_fail_1,
    ceiling_mod_power_of_two_signed_i32_fail_2,
    ceiling_mod_power_of_two_assign_signed_i32_fail_1,
    ceiling_mod_power_of_two_assign_signed_i32_fail_2
);
ceiling_mod_power_of_two_signed_fail!(
    i64,
    ceiling_mod_power_of_two_signed_i64_fail_1,
    ceiling_mod_power_of_two_signed_i64_fail_2,
    ceiling_mod_power_of_two_assign_signed_i64_fail_1,
    ceiling_mod_power_of_two_assign_signed_i64_fail_2
);
ceiling_mod_power_of_two_signed_fail!(
    i128,
    ceiling_mod_power_of_two_signed_i128_fail_1,
    ceiling_mod_power_of_two_signed_i128_fail_2,
    ceiling_mod_power_of_two_assign_signed_i128_fail_1,
    ceiling_mod_power_of_two_assign_signed_i128_fail_2
);
ceiling_mod_power_of_two_signed_fail!(
    isize,
    ceiling_mod_power_of_two_signed_isize_fail_1,
    ceiling_mod_power_of_two_signed_isize_fail_2,
    ceiling_mod_power_of_two_assign_signed_isize_fail_1,
    ceiling_mod_power_of_two_assign_signed_isize_fail_2
);
