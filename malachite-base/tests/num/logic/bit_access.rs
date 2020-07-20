use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeOne;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;

fn get_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out| {
        assert_eq!(T::exact_from(n).get_bit(index), out);
    };

    test(0, 0, false);
    test(0, 100, false);
    test(123, 2, false);
    test(123, 3, true);
    test(123, 100, false);
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 12, true);
        test(1_000_000_000_000, 100, false);
    }
}

fn get_bit_helper_signed<T: PrimitiveSigned>() {
    get_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out| {
        assert_eq!(T::exact_from(n).get_bit(index), out);
    };

    test(-123, 0, true);
    test(-123, 1, false);
    test(-123, 100, true);
    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 12, true);
        test(-1_000_000_000_000, 100, true);
        test(-i64::from(u32::MAX), 0, true);
        test(-i64::from(u32::MAX), 1, false);
        test(-i64::from(u32::MAX), 31, false);
        test(-i64::from(u32::MAX), 32, true);
        test(-i64::from(u32::MAX), 33, true);
        test(-i64::from(u32::MAX) - 1, 0, false);
        test(-i64::from(u32::MAX) - 1, 31, false);
        test(-i64::from(u32::MAX) - 1, 32, true);
        test(-i64::from(u32::MAX) - 1, 33, true);
    }
}

#[test]
fn test_get_bit() {
    apply_fn_to_unsigneds!(get_bit_helper_unsigned);
    apply_fn_to_signeds!(get_bit_helper_signed);
}

fn set_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.set_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, 101);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, 1024);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 10, 1_000_000_001_024);
    }
}

fn set_bit_helper_signed<T: PrimitiveSigned>() {
    set_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.set_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -1);
    test(-1, 100, -1);
    test(-33, 5, -1);
    test(-32, 0, -31);

    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 10, -999_999_998_976);
        test(-1_000_000_000_000, 100, -1_000_000_000_000);
    }
}

#[test]
fn test_set_bit() {
    apply_fn_to_unsigneds!(set_bit_helper_unsigned);
    apply_fn_to_signeds!(set_bit_helper_signed);
}

macro_rules! set_bit_fail_helper {
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            let mut n = $t::exact_from(5);
            n.set_bit(200);
        }
    };
}

set_bit_fail_helper!(u8, set_bit_u8_fail);
set_bit_fail_helper!(u16, set_bit_u16_fail);
set_bit_fail_helper!(u32, set_bit_u32_fail);
set_bit_fail_helper!(u64, set_bit_u64_fail);
set_bit_fail_helper!(u128, set_bit_u128_fail);
set_bit_fail_helper!(usize, set_bit_usize_fail);
set_bit_fail_helper!(i8, set_bit_i8_fail);
set_bit_fail_helper!(i16, set_bit_i16_fail);
set_bit_fail_helper!(i32, set_bit_i32_fail);
set_bit_fail_helper!(i64, set_bit_i64_fail);
set_bit_fail_helper!(i128, set_bit_i128_fail);
set_bit_fail_helper!(isize, set_bit_isize_fail);

fn clear_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.clear_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(0, 10, 0);
    test(0, 100, 0);
    test(101, 0, 100);
    if T::WIDTH >= u16::WIDTH {
        test(1024, 10, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_001_024, 10, 1_000_000_000_000);
        test(1_000_000_001_024, 100, 1_000_000_001_024);
    }
}

fn clear_bit_helper_signed<T: PrimitiveSigned>() {
    clear_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.clear_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -33);
    test(-31, 0, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-999_999_998_976, 10, -1_000_000_000_000);
    }
}

#[test]
fn test_clear_bit() {
    apply_fn_to_unsigneds!(clear_bit_helper_unsigned);
    apply_fn_to_signeds!(clear_bit_helper_signed);
}

macro_rules! clear_bit_fail_helper {
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            let mut n = $t::NEGATIVE_ONE;
            n.clear_bit(200);
        }
    };
}

clear_bit_fail_helper!(i8, clear_bit_i8_fail);
clear_bit_fail_helper!(i16, clear_bit_i16_fail);
clear_bit_fail_helper!(i32, clear_bit_i32_fail);
clear_bit_fail_helper!(i64, clear_bit_i64_fail);
clear_bit_fail_helper!(i128, clear_bit_i128_fail);
clear_bit_fail_helper!(isize, clear_bit_isize_fail);

fn assign_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, bit, out: u64| {
        let mut n = T::exact_from(n);
        n.assign_bit(index, bit);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, true, 101);
    test(0, 10, false, 0);
    test(0, 100, false, 0);
    test(101, 0, false, 100);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, true, 1024);
        test(1024, 10, false, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 10, true, 1_000_000_001_024);
        test(1_000_000_001_024, 10, false, 1_000_000_000_000);
        test(1_000_000_001_024, 100, false, 1_000_000_001_024);
    }
}

fn assign_bit_helper_signed<T: PrimitiveSigned>() {
    assign_bit_helper_unsigned::<T>();

    let test = |n: i64, index, bit, out: i64| {
        let mut n = T::exact_from(n);
        n.assign_bit(index, bit);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, true, -1);
    test(-1, 100, true, -1);
    test(-33, 5, true, -1);
    test(-32, 0, true, -31);
    test(-1, 5, false, -33);
    test(-31, 0, false, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 10, true, -999_999_998_976);
        test(-1_000_000_000_000, 100, true, -1_000_000_000_000);
        test(-999_999_998_976, 10, false, -1_000_000_000_000);
    }
}

#[test]
fn test_assign_bit() {
    apply_fn_to_unsigneds!(assign_bit_helper_unsigned);
    apply_fn_to_signeds!(assign_bit_helper_signed);
}

macro_rules! assign_bit_fail_helper_unsigned {
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            let mut n = $t::exact_from(5);
            n.assign_bit(200, true);
        }
    };
}

macro_rules! assign_bit_fail_helper_signed {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        assign_bit_fail_helper_unsigned!($t, $fail_1);

        #[test]
        #[should_panic]
        fn $fail_2() {
            let mut n = $t::NEGATIVE_ONE;
            n.assign_bit(200, false);
        }
    };
}

assign_bit_fail_helper_unsigned!(u8, assign_bit_u8_fail);
assign_bit_fail_helper_unsigned!(u16, assign_bit_u16_fail);
assign_bit_fail_helper_unsigned!(u32, assign_bit_u32_fail);
assign_bit_fail_helper_unsigned!(u64, assign_bit_u64_fail);
assign_bit_fail_helper_unsigned!(u128, assign_bit_u128_fail);
assign_bit_fail_helper_unsigned!(usize, assign_bit_usize_fail);
assign_bit_fail_helper_signed!(i8, assign_bit_i8_fail_1, assign_bit_i8_fail_2);
assign_bit_fail_helper_signed!(i16, assign_bit_i16_fail_1, assign_bit_i16_fail_2);
assign_bit_fail_helper_signed!(
    i32,
    assign_bit_signed_u32_fail_1,
    assign_bit_signed_u32_fail_2
);
assign_bit_fail_helper_signed!(i64, assign_bit_i64_fail_1, assign_bit_i64_fail_2);
assign_bit_fail_helper_signed!(i128, assign_bit_i128_fail_1, assign_bit_i128_fail_2);
assign_bit_fail_helper_signed!(isize, assign_bit_isize_fail_1, assign_bit_isize_fail_2);

fn flip_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.flip_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, 101);
    test(101, 0, 100);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, 1024);
        test(1024, 10, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 10, 1_000_000_001_024);
        test(1_000_000_001_024, 10, 1_000_000_000_000);
    }
}

fn flip_bit_helper_signed<T: PrimitiveSigned>() {
    flip_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.flip_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -33);
    test(-33, 5, -1);
    test(-32, 0, -31);
    test(-31, 0, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 10, -999_999_998_976);
        test(-999_999_998_976, 10, -1_000_000_000_000);
    }
}

#[test]
fn test_flip_bit() {
    apply_fn_to_unsigneds!(flip_bit_helper_unsigned);
    apply_fn_to_signeds!(flip_bit_helper_signed);
}

macro_rules! flip_bit_fail_helper_unsigned {
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            let mut n = $t::from(5u8);
            n.flip_bit(200);
        }
    };
}

macro_rules! flip_bit_fail_helper_signed {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            let mut n = $t::from(5i8);
            n.flip_bit(200);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            let mut n = $t::NEGATIVE_ONE;
            n.flip_bit(200);
        }
    };
}

flip_bit_fail_helper_unsigned!(u8, flip_bit_u8_fail);
flip_bit_fail_helper_unsigned!(u16, flip_bit_u16_fail);
flip_bit_fail_helper_unsigned!(u32, flip_bit_u32_fail);
flip_bit_fail_helper_unsigned!(u64, flip_bit_u64_fail);
flip_bit_fail_helper_unsigned!(u128, flip_bit_u128_fail);
flip_bit_fail_helper_unsigned!(usize, flip_bit_usize_fail);
flip_bit_fail_helper_signed!(i8, flip_bit_i8_fail_1, flip_bit_i8_fail_2);
flip_bit_fail_helper_signed!(i16, flip_bit_i16_fail_1, flip_bit_i16_fail_2);
flip_bit_fail_helper_signed!(i32, flip_bit_signed_u32_fail_1, flip_bit_signed_u32_fail_2);
flip_bit_fail_helper_signed!(i64, flip_bit_i64_fail_1, flip_bit_i64_fail_2);
flip_bit_fail_helper_signed!(i128, flip_bit_i128_fail_1, flip_bit_i128_fail_2);
flip_bit_fail_helper_signed!(isize, flip_bit_isize_fail_1, flip_bit_isize_fail_2);
