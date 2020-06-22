use malachite_base::num::arithmetic::traits::{
    RoundToMultipleOfPowerOfTwo, RoundToMultipleOfPowerOfTwoAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;

#[test]
fn test_round_to_multiple_of_power_of_two() {
    fn test<T: PrimitiveInteger>(n: T, pow: u64, rm: RoundingMode, out: T) {
        assert_eq!(n.round_to_multiple_of_power_of_two(pow, rm), out);

        let mut n = n;
        n.round_to_multiple_of_power_of_two_assign(pow, rm);
        assert_eq!(n, out);
    };

    test::<u8>(0, 10, RoundingMode::Exact, 0);
    test::<u8>(17, 0, RoundingMode::Exact, 17);

    test::<u8>(10, 2, RoundingMode::Floor, 8);
    test::<u16>(10, 2, RoundingMode::Ceiling, 12);
    test::<u32>(10, 2, RoundingMode::Down, 8);
    test::<u64>(10, 2, RoundingMode::Up, 12);
    test::<u128>(10, 2, RoundingMode::Nearest, 8);
    test::<usize>(12, 2, RoundingMode::Exact, 12);

    test::<i8>(-10, 2, RoundingMode::Floor, -12);
    test::<i16>(-10, 2, RoundingMode::Ceiling, -8);
    test::<i32>(-10, 2, RoundingMode::Down, -8);
    test::<i64>(-10, 2, RoundingMode::Up, -12);
    test::<i128>(-10, 2, RoundingMode::Nearest, -8);
    test::<isize>(-12, 2, RoundingMode::Exact, -12);

    test::<u8>(0xff, 4, RoundingMode::Down, 0xf0);
    test::<u8>(0xff, 4, RoundingMode::Floor, 0xf0);
    test::<u8>(0xef, 4, RoundingMode::Up, 0xf0);
    test::<u8>(0xef, 4, RoundingMode::Ceiling, 0xf0);
    test::<u8>(0xe8, 4, RoundingMode::Nearest, 0xe0);
    test::<u8>(1, 8, RoundingMode::Nearest, 0);

    test::<i8>(0x7f, 4, RoundingMode::Down, 0x70);
    test::<i8>(0x7f, 4, RoundingMode::Floor, 0x70);
    test::<i8>(0x6f, 4, RoundingMode::Up, 0x70);
    test::<i8>(0x6f, 4, RoundingMode::Ceiling, 0x70);
    test::<i8>(0x68, 4, RoundingMode::Nearest, 0x60);
    test::<i8>(-0x7f, 4, RoundingMode::Down, -0x70);
    test::<i8>(-0x7f, 4, RoundingMode::Floor, -0x80);
    test::<i8>(-0x7f, 4, RoundingMode::Up, -0x80);
    test::<i8>(-0x7f, 4, RoundingMode::Ceiling, -0x70);
    test::<i8>(-0x78, 4, RoundingMode::Nearest, -0x80);
}

macro_rules! round_to_multiple_of_power_of_two_fail {
    (
        $t:ident,
        $round_to_multiple_of_power_of_two_fail_1:ident,
        $round_to_multiple_of_power_of_two_fail_2:ident,
        $round_to_multiple_of_power_of_two_fail_3:ident,
        $round_to_multiple_of_power_of_two_fail_4:ident,
        $round_to_multiple_of_power_of_two_fail_5:ident,
        $round_to_multiple_of_power_of_two_assign_fail_1:ident,
        $round_to_multiple_of_power_of_two_assign_fail_2:ident,
        $round_to_multiple_of_power_of_two_assign_fail_3:ident,
        $round_to_multiple_of_power_of_two_assign_fail_4:ident,
        $round_to_multiple_of_power_of_two_assign_fail_5:ident
    ) => {
        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_fail_1() {
            $t::exact_from(10).round_to_multiple_of_power_of_two(4, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_fail_2() {
            $t::MAX.round_to_multiple_of_power_of_two(4, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_fail_3() {
            $t::MAX.round_to_multiple_of_power_of_two(4, RoundingMode::Ceiling);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_fail_4() {
            $t::MAX.round_to_multiple_of_power_of_two(4, RoundingMode::Nearest);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_fail_5() {
            $t::ONE.round_to_multiple_of_power_of_two($t::WIDTH, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_assign_fail_1() {
            $t::exact_from(10).round_to_multiple_of_power_of_two_assign(4, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_assign_fail_2() {
            $t::MAX.round_to_multiple_of_power_of_two_assign(4, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_assign_fail_3() {
            $t::MAX.round_to_multiple_of_power_of_two_assign(4, RoundingMode::Ceiling);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_assign_fail_4() {
            $t::MAX.round_to_multiple_of_power_of_two_assign(4, RoundingMode::Nearest);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_assign_fail_5() {
            $t::ONE.round_to_multiple_of_power_of_two_assign($t::WIDTH, RoundingMode::Up);
        }
    };
}

macro_rules! round_to_multiple_of_power_of_two_signed_fail {
    (
        $t:ident,
        $round_to_multiple_of_power_of_two_fail_6:ident,
        $round_to_multiple_of_power_of_two_fail_7:ident,
        $round_to_multiple_of_power_of_two_assign_fail_6:ident,
        $round_to_multiple_of_power_of_two_assign_fail_7:ident
    ) => {
        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_fail_6() {
            (-$t::MAX).round_to_multiple_of_power_of_two($t::WIDTH, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_fail_7() {
            (-$t::MAX).round_to_multiple_of_power_of_two($t::WIDTH, RoundingMode::Floor);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_assign_fail_6() {
            (-$t::MAX).round_to_multiple_of_power_of_two_assign($t::WIDTH, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_of_power_of_two_assign_fail_7() {
            (-$t::MAX).round_to_multiple_of_power_of_two_assign($t::WIDTH, RoundingMode::Floor);
        }
    };
}

round_to_multiple_of_power_of_two_fail!(
    u8,
    round_to_multiple_of_power_of_two_u8_fail_1,
    round_to_multiple_of_power_of_two_u8_fail_2,
    round_to_multiple_of_power_of_two_u8_fail_3,
    round_to_multiple_of_power_of_two_u8_fail_4,
    round_to_multiple_of_power_of_two_u8_fail_5,
    round_to_multiple_of_power_of_two_assign_u8_fail_1,
    round_to_multiple_of_power_of_two_assign_u8_fail_2,
    round_to_multiple_of_power_of_two_assign_u8_fail_3,
    round_to_multiple_of_power_of_two_assign_u8_fail_4,
    round_to_multiple_of_power_of_two_assign_u8_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    u16,
    round_to_multiple_of_power_of_two_u16_fail_1,
    round_to_multiple_of_power_of_two_u16_fail_2,
    round_to_multiple_of_power_of_two_u16_fail_3,
    round_to_multiple_of_power_of_two_u16_fail_4,
    round_to_multiple_of_power_of_two_u16_fail_5,
    round_to_multiple_of_power_of_two_assign_u16_fail_1,
    round_to_multiple_of_power_of_two_assign_u16_fail_2,
    round_to_multiple_of_power_of_two_assign_u16_fail_3,
    round_to_multiple_of_power_of_two_assign_u16_fail_4,
    round_to_multiple_of_power_of_two_assign_u16_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    u32,
    round_to_multiple_of_power_of_two_u32_fail_1,
    round_to_multiple_of_power_of_two_u32_fail_2,
    round_to_multiple_of_power_of_two_u32_fail_3,
    round_to_multiple_of_power_of_two_u32_fail_4,
    round_to_multiple_of_power_of_two_u32_fail_5,
    round_to_multiple_of_power_of_two_assign_u32_fail_1,
    round_to_multiple_of_power_of_two_assign_u32_fail_2,
    round_to_multiple_of_power_of_two_assign_u32_fail_3,
    round_to_multiple_of_power_of_two_assign_u32_fail_4,
    round_to_multiple_of_power_of_two_assign_u32_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    u64,
    round_to_multiple_of_power_of_two_u64_fail_1,
    round_to_multiple_of_power_of_two_u64_fail_2,
    round_to_multiple_of_power_of_two_u64_fail_3,
    round_to_multiple_of_power_of_two_u64_fail_4,
    round_to_multiple_of_power_of_two_u64_fail_5,
    round_to_multiple_of_power_of_two_assign_u64_fail_1,
    round_to_multiple_of_power_of_two_assign_u64_fail_2,
    round_to_multiple_of_power_of_two_assign_u64_fail_3,
    round_to_multiple_of_power_of_two_assign_u64_fail_4,
    round_to_multiple_of_power_of_two_assign_u64_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    u128,
    round_to_multiple_of_power_of_two_u128_fail_1,
    round_to_multiple_of_power_of_two_u128_fail_2,
    round_to_multiple_of_power_of_two_u128_fail_3,
    round_to_multiple_of_power_of_two_u128_fail_4,
    round_to_multiple_of_power_of_two_u128_fail_5,
    round_to_multiple_of_power_of_two_assign_u128_fail_1,
    round_to_multiple_of_power_of_two_assign_u128_fail_2,
    round_to_multiple_of_power_of_two_assign_u128_fail_3,
    round_to_multiple_of_power_of_two_assign_u128_fail_4,
    round_to_multiple_of_power_of_two_assign_u128_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    usize,
    round_to_multiple_of_power_of_two_usize_fail_1,
    round_to_multiple_of_power_of_two_usize_fail_2,
    round_to_multiple_of_power_of_two_usize_fail_3,
    round_to_multiple_of_power_of_two_usize_fail_4,
    round_to_multiple_of_power_of_two_usize_fail_5,
    round_to_multiple_of_power_of_two_assign_usize_fail_1,
    round_to_multiple_of_power_of_two_assign_usize_fail_2,
    round_to_multiple_of_power_of_two_assign_usize_fail_3,
    round_to_multiple_of_power_of_two_assign_usize_fail_4,
    round_to_multiple_of_power_of_two_assign_usize_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    i8,
    round_to_multiple_of_power_of_two_i8_fail_1,
    round_to_multiple_of_power_of_two_i8_fail_2,
    round_to_multiple_of_power_of_two_i8_fail_3,
    round_to_multiple_of_power_of_two_i8_fail_4,
    round_to_multiple_of_power_of_two_i8_fail_5,
    round_to_multiple_of_power_of_two_assign_i8_fail_1,
    round_to_multiple_of_power_of_two_assign_i8_fail_2,
    round_to_multiple_of_power_of_two_assign_i8_fail_3,
    round_to_multiple_of_power_of_two_assign_i8_fail_4,
    round_to_multiple_of_power_of_two_assign_i8_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    i16,
    round_to_multiple_of_power_of_two_i16_fail_1,
    round_to_multiple_of_power_of_two_i16_fail_2,
    round_to_multiple_of_power_of_two_i16_fail_3,
    round_to_multiple_of_power_of_two_i16_fail_4,
    round_to_multiple_of_power_of_two_i16_fail_5,
    round_to_multiple_of_power_of_two_assign_i16_fail_1,
    round_to_multiple_of_power_of_two_assign_i16_fail_2,
    round_to_multiple_of_power_of_two_assign_i16_fail_3,
    round_to_multiple_of_power_of_two_assign_i16_fail_4,
    round_to_multiple_of_power_of_two_assign_i16_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    i32,
    round_to_multiple_of_power_of_two_i32_fail_1,
    round_to_multiple_of_power_of_two_i32_fail_2,
    round_to_multiple_of_power_of_two_i32_fail_3,
    round_to_multiple_of_power_of_two_i32_fail_4,
    round_to_multiple_of_power_of_two_i32_fail_5,
    round_to_multiple_of_power_of_two_assign_i32_fail_1,
    round_to_multiple_of_power_of_two_assign_i32_fail_2,
    round_to_multiple_of_power_of_two_assign_i32_fail_3,
    round_to_multiple_of_power_of_two_assign_i32_fail_4,
    round_to_multiple_of_power_of_two_assign_i32_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    i64,
    round_to_multiple_of_power_of_two_i64_fail_1,
    round_to_multiple_of_power_of_two_i64_fail_2,
    round_to_multiple_of_power_of_two_i64_fail_3,
    round_to_multiple_of_power_of_two_i64_fail_4,
    round_to_multiple_of_power_of_two_i64_fail_5,
    round_to_multiple_of_power_of_two_assign_i64_fail_1,
    round_to_multiple_of_power_of_two_assign_i64_fail_2,
    round_to_multiple_of_power_of_two_assign_i64_fail_3,
    round_to_multiple_of_power_of_two_assign_i64_fail_4,
    round_to_multiple_of_power_of_two_assign_i64_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    i128,
    round_to_multiple_of_power_of_two_i128_fail_1,
    round_to_multiple_of_power_of_two_i128_fail_2,
    round_to_multiple_of_power_of_two_i128_fail_3,
    round_to_multiple_of_power_of_two_i128_fail_4,
    round_to_multiple_of_power_of_two_i128_fail_5,
    round_to_multiple_of_power_of_two_assign_i128_fail_1,
    round_to_multiple_of_power_of_two_assign_i128_fail_2,
    round_to_multiple_of_power_of_two_assign_i128_fail_3,
    round_to_multiple_of_power_of_two_assign_i128_fail_4,
    round_to_multiple_of_power_of_two_assign_i128_fail_5
);
round_to_multiple_of_power_of_two_fail!(
    isize,
    round_to_multiple_of_power_of_two_isize_fail_1,
    round_to_multiple_of_power_of_two_isize_fail_2,
    round_to_multiple_of_power_of_two_isize_fail_3,
    round_to_multiple_of_power_of_two_isize_fail_4,
    round_to_multiple_of_power_of_two_isize_fail_5,
    round_to_multiple_of_power_of_two_assign_isize_fail_1,
    round_to_multiple_of_power_of_two_assign_isize_fail_2,
    round_to_multiple_of_power_of_two_assign_isize_fail_3,
    round_to_multiple_of_power_of_two_assign_isize_fail_4,
    round_to_multiple_of_power_of_two_assign_isize_fail_5
);

round_to_multiple_of_power_of_two_signed_fail!(
    i8,
    round_to_multiple_of_power_of_two_i8_fail_6,
    round_to_multiple_of_power_of_two_i8_fail_7,
    round_to_multiple_of_power_of_two_assign_i8_fail_6,
    round_to_multiple_of_power_of_two_assign_i8_fail_7
);
round_to_multiple_of_power_of_two_signed_fail!(
    i16,
    round_to_multiple_of_power_of_two_i16_fail_6,
    round_to_multiple_of_power_of_two_i16_fail_7,
    round_to_multiple_of_power_of_two_assign_i16_fail_6,
    round_to_multiple_of_power_of_two_assign_i16_fail_7
);
round_to_multiple_of_power_of_two_signed_fail!(
    i32,
    round_to_multiple_of_power_of_two_i32_fail_6,
    round_to_multiple_of_power_of_two_i32_fail_7,
    round_to_multiple_of_power_of_two_assign_i32_fail_6,
    round_to_multiple_of_power_of_two_assign_i32_fail_7
);
round_to_multiple_of_power_of_two_signed_fail!(
    i64,
    round_to_multiple_of_power_of_two_i64_fail_6,
    round_to_multiple_of_power_of_two_i64_fail_7,
    round_to_multiple_of_power_of_two_assign_i64_fail_6,
    round_to_multiple_of_power_of_two_assign_i64_fail_7
);
round_to_multiple_of_power_of_two_signed_fail!(
    i128,
    round_to_multiple_of_power_of_two_i128_fail_6,
    round_to_multiple_of_power_of_two_i128_fail_7,
    round_to_multiple_of_power_of_two_assign_i128_fail_6,
    round_to_multiple_of_power_of_two_assign_i128_fail_7
);
round_to_multiple_of_power_of_two_signed_fail!(
    isize,
    round_to_multiple_of_power_of_two_isize_fail_6,
    round_to_multiple_of_power_of_two_isize_fail_7,
    round_to_multiple_of_power_of_two_assign_isize_fail_6,
    round_to_multiple_of_power_of_two_assign_isize_fail_7
);
