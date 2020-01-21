use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::integers::_get_bits_naive;
use malachite_base::num::logic::traits::BitBlockAccess;

#[test]
pub fn test_get_bits_unsigned() {
    fn test<T: PrimitiveUnsigned>(x: T, start: u64, end: u64, out: T)
    where
        <T as BitBlockAccess>::Output: PrimitiveUnsigned,
        T: ExactFrom<<T as BitBlockAccess>::Output>,
    {
        // The return type of get_bits is just T, but the type system doesn't know that.
        assert_eq!(T::exact_from(x.get_bits(start, end)), out);
        assert_eq!(_get_bits_naive::<T, T>(x, start, end), out)
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

#[test]
pub fn test_get_bits_signed() {
    fn test<T: PrimitiveSigned>(x: T, start: u64, end: u64, out: T::UnsignedOfEqualWidth)
    where
        <T as BitBlockAccess>::Output: PrimitiveUnsigned,
        T::UnsignedOfEqualWidth: ExactFrom<<T as BitBlockAccess>::Output>,
    {
        // The return type of get_bits is just T::UnsignedOfEqualWidth, but the type system doesn't
        // know that.
        assert_eq!(
            T::UnsignedOfEqualWidth::exact_from(x.get_bits(start, end)),
            out
        );
        assert_eq!(
            _get_bits_naive::<T, T::UnsignedOfEqualWidth>(x, start, end),
            out
        )
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
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            $t::from(-10i8).get_bits(100, 200);
        }
    };
}

get_bits_fail_helper_signed!(i8, get_bits_i8_fail_helper);
get_bits_fail_helper_signed!(i16, get_bits_i16_fail_helper);
get_bits_fail_helper_signed!(i32, get_bits_i32_fail_helper);
get_bits_fail_helper_signed!(i64, get_bits_i64_fail_helper);
get_bits_fail_helper_signed!(isize, get_bits_isize_fail_helper);
