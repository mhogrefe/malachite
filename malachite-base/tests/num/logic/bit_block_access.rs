use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::bit_block_access::{_assign_bits_naive, _get_bits_naive};
use malachite_base::num::logic::traits::BitBlockAccess;

#[test]
pub fn test_get_bits_unsigned() {
    fn test<T: PrimitiveUnsigned>(x: T, start: u64, end: u64, out: T)
    where
        T: BitBlockAccess<Bits = T>,
    {
        assert_eq!(x.get_bits(start, end), out);
        assert_eq!(_get_bits_naive::<T, T>(&x, start, end), out)
    };
    test(0xabcdu16, 4, 8, 0xc);
    test(0xabcdu16, 12, 100, 0xa);
    test(0xabcdu16, 5, 9, 14);
    test(0xabcdu16, 5, 5, 0);
    test(0xabcdu16, 100, 200, 0);

    test(0xabcdu64, 4, 8, 0xc);
    test(0xabcdu64, 12, 100, 0xa);
    test(0xabcdu64, 5, 9, 14);
    test(0xabcdu64, 5, 5, 0);
    test(0xabcdu64, 100, 200, 0);
}

macro_rules! get_bits_fail_helper_unsigned {
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            $t::from(100u8).get_bits(10, 5);
        }
    };
}

get_bits_fail_helper_unsigned!(u8, get_bits_u8_fail);
get_bits_fail_helper_unsigned!(u16, get_bits_u16_fail);
get_bits_fail_helper_unsigned!(u32, get_bits_u32_fail);
get_bits_fail_helper_unsigned!(u64, get_bits_u64_fail);
get_bits_fail_helper_unsigned!(u128, get_bits_u128_fail);
get_bits_fail_helper_unsigned!(usize, get_bits_usize_fail);

#[test]
pub fn test_get_bits_signed() {
    fn test<T: PrimitiveSigned, U: PrimitiveUnsigned>(x: T, start: u64, end: u64, out: U)
    where
        T: BitBlockAccess<Bits = U>,
    {
        assert_eq!(x.get_bits(start, end), out);
        assert_eq!(_get_bits_naive::<T, U>(&x, start, end), out)
    };
    test(-0x5433i16, 4, 8, 0xc);
    test(-0x5433i16, 5, 9, 14);
    test(-0x5433i16, 5, 5, 0);
    test(-0x5433i16, 100, 104, 0xf);

    test(-0x5433i64, 4, 8, 0xc);
    test(-0x5433i64, 5, 9, 14);
    test(-0x5433i64, 5, 5, 0);
    test(-0x5433i64, 100, 104, 0xf);

    test(-1i8, 0, 8, 0xff);
}

macro_rules! get_bits_fail_helper_signed {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            $t::from(10i8).get_bits(10, 5);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            $t::from(-10i8).get_bits(100, 300);
        }
    };
}

get_bits_fail_helper_signed!(i8, get_bits_i8_fail_1, get_bits_i8_fail_2);
get_bits_fail_helper_signed!(i16, get_bits_i16_fail_1, get_bits_i16_fail_2);
get_bits_fail_helper_signed!(i32, get_bits_i32_fail_1, get_bits_i32_fail_2);
get_bits_fail_helper_signed!(i64, get_bits_i64_fail_1, get_bits_i64_fail_2);
get_bits_fail_helper_signed!(i128, get_bits_i128_fail_1, get_bits_i128_fail_2);
get_bits_fail_helper_signed!(isize, get_bits_isize_fail_1, get_bits_isize_fail_2);

#[test]
pub fn test_assign_bits_unsigned() {
    fn test<T: PrimitiveUnsigned>(x_in: T, start: u64, end: u64, bits: T, x_out: T)
    where
        T: BitBlockAccess<Bits = T>,
    {
        let mut x = x_in;
        x.assign_bits(start, end, &bits);
        assert_eq!(x, x_out);

        let mut x = x_in;
        _assign_bits_naive(&mut x, start, end, &bits);
        assert_eq!(x, x_out);
    };
    // assign partially
    test(0xab5du16, 4, 8, 0xc, 0xabcd);
    test(0x5bcdu16, 12, 100, 0xa, 0xabcd);
    test(0xabcdu16, 5, 9, 10, 43_853);
    test(0xabcdu16, 5, 5, 123, 0xabcd);
    // assign zeros above width
    test(0xabcdu16, 100, 200, 0, 0xabcd);
    test(0xabcdu16, 8, 24, 0, 0xcd);
    // assign everything
    test(0xabcdu16, 0, 100, 0x1234, 0x1234);

    test(0xab5du64, 4, 8, 0xc, 0xabcd);
    test(0x5bcdu64, 12, 100, 0xa, 0xabcd);
    test(0xabcdu64, 5, 9, 10, 43_853);
    test(0xabcdu64, 5, 5, 123, 0xabcd);
    test(0xabcdu64, 100, 200, 0, 0xabcd);
    test(0xabcdu64, 0, 100, 0x1234, 0x1234);
}

macro_rules! assign_bits_fail_helper_unsigned {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            $t::from(100u8).assign_bits(10, 5, &3);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            $t::from(100u8).assign_bits(3, 3 + $t::WIDTH, &$t::MAX);
        }
    };
}

assign_bits_fail_helper_unsigned!(u8, assign_bits_u8_fail_1, assign_bits_u8_fail_2);
assign_bits_fail_helper_unsigned!(u16, assign_bits_u16_fail_1, assign_bits_u16_fail_2);
assign_bits_fail_helper_unsigned!(u32, assign_bits_u32_fail_1, assign_bits_u32_fail_2);
assign_bits_fail_helper_unsigned!(u64, assign_bits_u64_fail_1, assign_bits_u64_fail_2);
assign_bits_fail_helper_unsigned!(u128, assign_bits_u128_fail_1, assign_bits_u128_fail_2);
assign_bits_fail_helper_unsigned!(usize, assign_bits_usize_fail_1, assign_bits_usize_fail_2);

#[test]
pub fn test_assign_bits_signed() {
    fn test<T: PrimitiveSigned, U: PrimitiveUnsigned>(
        x_in: T,
        start: u64,
        end: u64,
        bits: U,
        x_out: T,
    ) where
        T: BitBlockAccess<Bits = U>,
    {
        let mut x = x_in;
        x.assign_bits(start, end, &bits);
        assert_eq!(x, x_out);

        let mut x = x_in;
        _assign_bits_naive(&mut x, start, end, &bits);
        assert_eq!(x, x_out);
    };
    // *self >= 0
    test(0x2b5di16, 4, 8, 0xc, 0x2bcd);
    // *self < 0
    // assign within width
    test(-0x5413i16, 4, 8, 0xc, -0x5433);
    test(-0x54a3i16, 5, 9, 14, -21_539);
    test(-0x5433i16, 5, 5, 0, -0x5433);
    // assign ones above width
    test(-0x5433i16, 100, 104, 0xf, -0x5433);
    // assign everything
    test(-57i8, 0, 8, 0xff, -1);

    test(0x2b5di64, 4, 8, 0xc, 0x2bcd);
    test(-0x5413i64, 4, 8, 0xc, -0x5433);
    test(-0x54a3i64, 5, 9, 14, -21_539);
    test(-0x5433i64, 5, 5, 0, -0x5433);
    test(-0x5433i64, 100, 104, 0xf, -0x5433);
    test(-57i64, 0, 64, u64::MAX, -1);
}

macro_rules! assign_bits_fail_helper_signed {
    ($t:ident, $fail_1:ident, $fail_2:ident, $fail_3:ident, $fail_4:ident, $fail_5:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            $t::from(100i8).assign_bits(7, 5, &3);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            $t::from(100i8).assign_bits(
                0,
                $t::WIDTH,
                &<$t as PrimitiveSigned>::UnsignedOfEqualWidth::MAX,
            );
        }

        #[test]
        #[should_panic]
        fn $fail_3() {
            $t::from(-100i8).assign_bits(0, $t::WIDTH + 1, &0);
        }

        #[test]
        #[should_panic]
        fn $fail_4() {
            $t::from(-100i8).assign_bits($t::WIDTH + 1, $t::WIDTH + 2, &0);
        }

        #[test]
        #[should_panic]
        fn $fail_5() {
            let half_width = $t::WIDTH >> 1;
            $t::from(-100i8).assign_bits(half_width, 3 * half_width - 4, &0);
        }
    };
}

assign_bits_fail_helper_signed!(
    i8,
    assign_bits_i8_fail_1,
    assign_bits_i8_fail_2,
    assign_bits_i8_fail_3,
    assign_bits_i8_fail_4,
    assign_bits_i8_fail_5
);
assign_bits_fail_helper_signed!(
    i16,
    assign_bits_i16_fail_1,
    assign_bits_i16_fail_2,
    assign_bits_i16_fail_3,
    assign_bits_i16_fail_4,
    assign_bits_i16_fail_5
);
assign_bits_fail_helper_signed!(
    i32,
    assign_bits_i32_fail_1,
    assign_bits_i32_fail_2,
    assign_bits_i32_fail_3,
    assign_bits_i32_fail_4,
    assign_bits_i32_fail_5
);
assign_bits_fail_helper_signed!(
    i64,
    assign_bits_i64_fail_1,
    assign_bits_i64_fail_2,
    assign_bits_i64_fail_3,
    assign_bits_i64_fail_4,
    assign_bits_i64_fail_5
);
assign_bits_fail_helper_signed!(
    i128,
    assign_bits_i128_fail_1,
    assign_bits_i128_fail_2,
    assign_bits_i128_fail_3,
    assign_bits_i128_fail_4,
    assign_bits_i128_fail_5
);
assign_bits_fail_helper_signed!(
    isize,
    assign_bits_isize_fail_1,
    assign_bits_isize_fail_2,
    assign_bits_isize_fail_3,
    assign_bits_isize_fail_4,
    assign_bits_isize_fail_5
);
