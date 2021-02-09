use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::string::to_string::{
    digit_to_display_byte_lower, digit_to_display_byte_upper,
};
use malachite_base::num::conversion::string::BaseFmtWrapper;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::strings::{
    string_is_subset, ToBinaryString, ToLowerHexString, ToOctalString, ToUpperHexString,
};
use malachite_base_test_util::generators::{
    signed_gen, signed_gen_var_2, signed_unsigned_pair_gen_var_5, unsigned_gen, unsigned_gen_var_7,
    unsigned_gen_var_8, unsigned_pair_gen_var_8,
};
use malachite_base_test_util::num::conversion::string::to_string::{
    _to_string_base_signed_naive, _to_string_base_unsigned_naive,
};
use std::fmt::{Debug, Display};
use std::panic::catch_unwind;

#[test]
fn test_digit_to_display_byte_lower() {
    let test = |x, y| {
        assert_eq!(digit_to_display_byte_lower(x), y);
    };
    test(0, b'0');
    test(1, b'1');
    test(2, b'2');
    test(3, b'3');
    test(4, b'4');
    test(5, b'5');
    test(6, b'6');
    test(7, b'7');
    test(8, b'8');
    test(9, b'9');
    test(10, b'a');
    test(11, b'b');
    test(12, b'c');
    test(33, b'x');
    test(34, b'y');
    test(35, b'z');
}

#[test]
#[should_panic]
fn digit_to_display_byte_lower_fail_1() {
    digit_to_display_byte_lower(36);
}

#[test]
#[should_panic]
fn digit_to_display_byte_lower_fail_2() {
    digit_to_display_byte_lower(100);
}

#[test]
fn digit_to_display_byte_lower_properties() {
    unsigned_gen_var_7().test_properties(|b| {
        let display_byte = digit_to_display_byte_lower(b);
        assert!((b'0'..=b'9').contains(&display_byte) || (b'a'..=b'z').contains(&display_byte));
        let display_byte_upper = digit_to_display_byte_upper(b);
        assert_eq!(display_byte == display_byte_upper, (0..=9).contains(&b));
    });
}

#[test]
fn test_digit_to_display_byte_upper() {
    let test = |x, y| {
        assert_eq!(digit_to_display_byte_upper(x), y);
    };
    test(0, b'0');
    test(1, b'1');
    test(2, b'2');
    test(3, b'3');
    test(4, b'4');
    test(5, b'5');
    test(6, b'6');
    test(7, b'7');
    test(8, b'8');
    test(9, b'9');
    test(10, b'A');
    test(11, b'B');
    test(12, b'C');
    test(33, b'X');
    test(34, b'Y');
    test(35, b'Z');
}

#[test]
#[should_panic]
fn digit_to_display_byte_upper_fail_1() {
    digit_to_display_byte_upper(36);
}

#[test]
#[should_panic]
fn digit_to_display_byte_upper_fail_2() {
    digit_to_display_byte_upper(100);
}

#[test]
fn digit_to_display_byte_upper_properties() {
    unsigned_gen_var_7().test_properties(|b| {
        let display_byte = digit_to_display_byte_upper(b);
        assert!((b'0'..=b'9').contains(&display_byte) || (b'A'..=b'Z').contains(&display_byte));
        let display_byte_lower = digit_to_display_byte_lower(b);
        assert_eq!(display_byte == display_byte_lower, (0..=9).contains(&b));
    });
}

#[test]
pub fn test_to_string_base() {
    fn test_u<T: PrimitiveUnsigned>(x: T, base: u64, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
        u8: WrappingFrom<T>,
    {
        assert_eq!(x.to_string_base(base), out);
        assert_eq!(_to_string_base_unsigned_naive(x, base), out);
        assert_eq!(format!("{}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(x, base)), out);
    };
    test_u::<u8>(0, 2, "0");
    test_u::<u8>(0, 3, "0");
    test_u::<u8>(0, 10, "0");
    test_u::<u8>(0, 16, "0");
    test_u::<u8>(0, 17, "0");
    test_u::<u16>(2, 3, "2");
    test_u::<u16>(2, 10, "2");
    test_u::<u16>(2, 16, "2");
    test_u::<u16>(2, 17, "2");
    test_u::<u32>(123, 8, "173");
    test_u::<u32>(1000000, 10, "1000000");
    test_u::<u32>(1000000, 20, "65000");
    test_u::<u32>(1000000, 36, "lfls");
    test_u::<u64>(1000, 2, "1111101000");
    test_u::<u64>(1000, 3, "1101001");
    test_u::<u64>(1000, 4, "33220");
    test_u::<u64>(1000, 10, "1000");
    test_u::<u64>(1000, 20, "2a0");
    test_u::<u64>(1000, 36, "rs");

    fn test_i<T: PrimitiveSigned>(x: T, base: u64, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
        u8: WrappingFrom<<T as UnsignedAbs>::Output>,
        <T as UnsignedAbs>::Output: PrimitiveUnsigned,
    {
        assert_eq!(x.to_string_base(base), out);
        assert_eq!(_to_string_base_signed_naive(x, base), out);
        assert_eq!(format!("{}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(x, base)), out);
    };
    test_i::<i8>(0, 2, "0");
    test_i::<i8>(0, 3, "0");
    test_i::<i8>(0, 10, "0");
    test_i::<i8>(0, 16, "0");
    test_i::<i8>(0, 17, "0");
    test_i::<i16>(2, 3, "2");
    test_i::<i16>(2, 10, "2");
    test_i::<i16>(2, 16, "2");
    test_i::<i16>(2, 17, "2");
    test_i::<i32>(123, 8, "173");
    test_i::<i32>(1000000, 10, "1000000");
    test_i::<i32>(1000000, 20, "65000");
    test_i::<i32>(1000000, 36, "lfls");
    test_i::<i64>(1000, 2, "1111101000");
    test_i::<i64>(1000, 3, "1101001");
    test_i::<i64>(1000, 4, "33220");
    test_i::<i64>(1000, 10, "1000");
    test_i::<i64>(1000, 20, "2a0");
    test_i::<i64>(1000, 36, "rs");

    test_i::<i16>(-2, 3, "-2");
    test_i::<i16>(-2, 10, "-2");
    test_i::<i16>(-2, 16, "-2");
    test_i::<i16>(-2, 17, "-2");
    test_i::<i32>(-123, 8, "-173");
    test_i::<i32>(-1000000, 10, "-1000000");
    test_i::<i32>(-1000000, 20, "-65000");
    test_i::<i32>(-1000000, 36, "-lfls");
    test_i::<i64>(-1000, 2, "-1111101000");
    test_i::<i64>(-1000, 3, "-1101001");
    test_i::<i64>(-1000, 4, "-33220");
    test_i::<i64>(-1000, 10, "-1000");
    test_i::<i64>(-1000, 20, "-2a0");
    test_i::<i64>(-1000, 36, "-rs");
}

fn to_string_base_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::exact_from(100).to_string_base(0));
    assert_panic!(T::exact_from(100).to_string_base(1));
    assert_panic!(T::exact_from(100).to_string_base(37));
    assert_panic!(T::exact_from(100).to_string_base(100));
}

#[test]
fn to_string_base_fail() {
    apply_fn_to_primitive_ints!(to_string_base_fail_helper);
}

fn to_string_base_helper_unsigned<T: PrimitiveUnsigned>()
where
    BaseFmtWrapper<T>: Debug + Display,
    u8: WrappingFrom<T>,
{
    unsigned_pair_gen_var_8::<T, u64>().test_properties(|(x, base)| {
        let s = x.to_string_base(base);
        assert_eq!(_to_string_base_unsigned_naive(x, base), s);
        assert_eq!(format!("{}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(x, base)), s);
        //TODO from_string_base
        assert!(string_is_subset(&s, "0123456789abcdefghijklmnopqrstuvwxyz"));
        if x != T::ZERO {
            assert!(!s.starts_with('0'));
        }
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.to_string_base(10), x.to_string());
        assert_eq!(x.to_string_base(2), x.to_binary_string());
        assert_eq!(x.to_string_base(8), x.to_octal_string());
        assert_eq!(x.to_string_base(16), x.to_lower_hex_string());
    });

    unsigned_gen_var_8().test_properties(|base| {
        assert_eq!(T::ZERO.to_string_base(base), "0");
        assert_eq!(T::ONE.to_string_base(base), "1");
        assert_eq!(T::exact_from(base).to_string_base(base), "10");
    });
}

fn to_string_base_helper_signed<T: PrimitiveSigned>()
where
    BaseFmtWrapper<T>: Debug + Display,
    u8: WrappingFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    signed_unsigned_pair_gen_var_5::<T, u64>().test_properties(|(x, base)| {
        let s = x.to_string_base(base);
        assert_eq!(_to_string_base_signed_naive(x, base), s);
        assert_eq!(format!("{}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(x, base)), s);
        //TODO from_string_base
        assert!(string_is_subset(
            &s,
            "-0123456789abcdefghijklmnopqrstuvwxyz"
        ));
        assert_eq!(x < T::ZERO, s.starts_with('-'));
        assert!(!s.starts_with("-0"));
        if x != T::ZERO {
            assert!(!s.starts_with('0'));
        }
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.to_string_base(10), x.to_string());
    });

    signed_gen_var_2::<T>().test_properties(|x| {
        assert_eq!(x.to_string_base(2), x.to_binary_string());
        assert_eq!(x.to_string_base(8), x.to_octal_string());
        assert_eq!(x.to_string_base(16), x.to_lower_hex_string());
    });

    unsigned_gen_var_8().test_properties(|base| {
        assert_eq!(T::ZERO.to_string_base(base), "0");
        assert_eq!(T::ONE.to_string_base(base), "1");
        assert_eq!(T::NEGATIVE_ONE.to_string_base(base), "-1");
        assert_eq!(T::exact_from(base).to_string_base(base), "10");
    });
}

#[test]
fn to_string_base_properties() {
    apply_fn_to_unsigneds!(to_string_base_helper_unsigned);
    apply_fn_to_signeds!(to_string_base_helper_signed);
}

#[test]
pub fn test_to_string_base_upper() {
    fn test_u<T: PrimitiveUnsigned>(x: T, base: u64, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
    {
        assert_eq!(x.to_string_base_upper(base), out);
        assert_eq!(format!("{:#}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:#?}", BaseFmtWrapper::new(x, base)), out);
    };
    test_u::<u8>(0, 2, "0");
    test_u::<u8>(0, 3, "0");
    test_u::<u8>(0, 10, "0");
    test_u::<u8>(0, 16, "0");
    test_u::<u8>(0, 17, "0");
    test_u::<u16>(2, 3, "2");
    test_u::<u16>(2, 10, "2");
    test_u::<u16>(2, 16, "2");
    test_u::<u16>(2, 17, "2");
    test_u::<u32>(123, 8, "173");
    test_u::<u32>(1000000, 10, "1000000");
    test_u::<u32>(1000000, 20, "65000");
    test_u::<u32>(1000000, 36, "LFLS");
    test_u::<u64>(1000, 2, "1111101000");
    test_u::<u64>(1000, 3, "1101001");
    test_u::<u64>(1000, 4, "33220");
    test_u::<u64>(1000, 10, "1000");
    test_u::<u64>(1000, 20, "2A0");
    test_u::<u64>(1000, 36, "RS");

    fn test_i<T: PrimitiveSigned>(x: T, base: u64, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
    {
        assert_eq!(x.to_string_base_upper(base), out);
        assert_eq!(format!("{:#}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:#?}", BaseFmtWrapper::new(x, base)), out);
    };
    test_i::<i8>(0, 2, "0");
    test_i::<i8>(0, 3, "0");
    test_i::<i8>(0, 10, "0");
    test_i::<i8>(0, 16, "0");
    test_i::<i8>(0, 17, "0");
    test_i::<i16>(2, 3, "2");
    test_i::<i16>(2, 10, "2");
    test_i::<i16>(2, 16, "2");
    test_i::<i16>(2, 17, "2");
    test_i::<i32>(123, 8, "173");
    test_i::<i32>(1000000, 10, "1000000");
    test_i::<i32>(1000000, 20, "65000");
    test_i::<i32>(1000000, 36, "LFLS");
    test_i::<i64>(1000, 2, "1111101000");
    test_i::<i64>(1000, 3, "1101001");
    test_i::<i64>(1000, 4, "33220");
    test_i::<i64>(1000, 10, "1000");
    test_i::<i64>(1000, 20, "2A0");
    test_i::<i64>(1000, 36, "RS");

    test_i::<i16>(-2, 3, "-2");
    test_i::<i16>(-2, 10, "-2");
    test_i::<i16>(-2, 16, "-2");
    test_i::<i16>(-2, 17, "-2");
    test_i::<i32>(-123, 8, "-173");
    test_i::<i32>(-1000000, 10, "-1000000");
    test_i::<i32>(-1000000, 20, "-65000");
    test_i::<i32>(-1000000, 36, "-LFLS");
    test_i::<i64>(-1000, 2, "-1111101000");
    test_i::<i64>(-1000, 3, "-1101001");
    test_i::<i64>(-1000, 4, "-33220");
    test_i::<i64>(-1000, 10, "-1000");
    test_i::<i64>(-1000, 20, "-2A0");
    test_i::<i64>(-1000, 36, "-RS");
}

fn to_string_base_upper_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::exact_from(100).to_string_base_upper(0));
    assert_panic!(T::exact_from(100).to_string_base_upper(1));
    assert_panic!(T::exact_from(100).to_string_base_upper(37));
    assert_panic!(T::exact_from(100).to_string_base_upper(100));
}

#[test]
fn to_string_base_upper_fail() {
    apply_fn_to_primitive_ints!(to_string_base_upper_fail_helper);
}

fn to_string_base_upper_helper_unsigned<T: PrimitiveUnsigned>()
where
    BaseFmtWrapper<T>: Debug + Display,
{
    unsigned_pair_gen_var_8::<T, u64>().test_properties(|(x, base)| {
        let s = x.to_string_base_upper(base);
        assert_eq!(format!("{:#}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:#?}", BaseFmtWrapper::new(x, base)), s);
        //TODO from_string_base
        assert!(string_is_subset(&s, "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        if x != T::ZERO {
            assert!(!s.starts_with('0'));
        }
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.to_string_base_upper(10), x.to_string());
        assert_eq!(x.to_string_base_upper(2), x.to_binary_string());
        assert_eq!(x.to_string_base_upper(8), x.to_octal_string());
        assert_eq!(x.to_string_base_upper(16), x.to_upper_hex_string());
    });

    unsigned_gen_var_8().test_properties(|base| {
        assert_eq!(T::ZERO.to_string_base_upper(base), "0");
        assert_eq!(T::ONE.to_string_base_upper(base), "1");
        assert_eq!(T::exact_from(base).to_string_base_upper(base), "10");
    });
}

fn to_string_base_upper_helper_signed<T: PrimitiveSigned>()
where
    BaseFmtWrapper<T>: Debug + Display,
{
    signed_unsigned_pair_gen_var_5::<T, u64>().test_properties(|(x, base)| {
        let s = x.to_string_base_upper(base);
        assert_eq!(format!("{:#}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:#?}", BaseFmtWrapper::new(x, base)), s);
        //TODO from_string_base
        assert!(string_is_subset(
            &s,
            "-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        ));
        assert_eq!(x < T::ZERO, s.starts_with('-'));
        assert!(!s.starts_with("-0"));
        if x != T::ZERO {
            assert!(!s.starts_with('0'));
        }
    });

    signed_gen::<T>().test_properties(|x| {
        assert_eq!(x.to_string_base_upper(10), x.to_string());
    });

    signed_gen_var_2::<T>().test_properties(|x| {
        assert_eq!(x.to_string_base_upper(2), x.to_binary_string());
        assert_eq!(x.to_string_base_upper(8), x.to_octal_string());
        assert_eq!(x.to_string_base_upper(16), x.to_upper_hex_string());
    });

    unsigned_gen_var_8().test_properties(|base| {
        assert_eq!(T::ZERO.to_string_base_upper(base), "0");
        assert_eq!(T::ONE.to_string_base_upper(base), "1");
        assert_eq!(T::NEGATIVE_ONE.to_string_base_upper(base), "-1");
        assert_eq!(T::exact_from(base).to_string_base_upper(base), "10");
    });
}

#[test]
fn to_string_base_upper_properties() {
    apply_fn_to_unsigneds!(to_string_base_upper_helper_unsigned);
    apply_fn_to_signeds!(to_string_base_upper_helper_signed);
}
