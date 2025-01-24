// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::repeat_n;
use malachite_base::num::arithmetic::traits::{SaturatingSubAssign, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::string::to_string::{
    digit_to_display_byte_lower, digit_to_display_byte_upper, BaseFmtWrapper,
};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::strings::{
    string_is_subset, ToBinaryString, ToLowerHexString, ToOctalString, ToUpperHexString,
};
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_2, signed_unsigned_pair_gen_var_5, signed_unsigned_pair_gen_var_6,
    signed_unsigned_unsigned_triple_gen_var_3, unsigned_gen, unsigned_gen_var_7,
    unsigned_gen_var_8, unsigned_pair_gen_var_2, unsigned_pair_gen_var_8, unsigned_pair_gen_var_9,
    unsigned_triple_gen_var_6,
};
use malachite_base::test_util::num::conversion::string::to_string::{
    to_string_base_signed_naive, to_string_base_unsigned_naive,
};
use std::cmp::max;
use std::fmt::{Debug, Display};
use std::panic::catch_unwind;

fn test_padding_unsigned(s: &str, s_padded: &str, width: usize) {
    assert!(s_padded.ends_with(&s));
    assert!(s_padded.len() >= width);
    assert_eq!(s.len() >= width, s == s_padded);
    if s.len() < width {
        let diff = s_padded.len() - s.len();
        assert!(s_padded[..diff].chars().all(|c| c == '0'));
        assert_eq!(&s_padded[diff..], s);
    }
}

fn test_padding_signed(mut s: &str, mut s_padded: &str, mut width: usize) {
    assert!(s_padded.len() >= width);
    assert_eq!(s.len() >= width, s == s_padded);
    let negative = s.starts_with('-');
    assert_eq!(s_padded.starts_with('-'), negative);
    if negative {
        s = &s[1..];
        s_padded = &s_padded[1..];
        width.saturating_sub_assign(1);
    }
    test_padding_unsigned(s, s_padded, width);
}

#[test]
fn test_digit_to_display_byte_lower() {
    let test_ok = |x, y| {
        assert_eq!(digit_to_display_byte_lower(x).unwrap(), y);
    };
    test_ok(0, b'0');
    test_ok(1, b'1');
    test_ok(2, b'2');
    test_ok(3, b'3');
    test_ok(4, b'4');
    test_ok(5, b'5');
    test_ok(6, b'6');
    test_ok(7, b'7');
    test_ok(8, b'8');
    test_ok(9, b'9');
    test_ok(10, b'a');
    test_ok(11, b'b');
    test_ok(12, b'c');
    test_ok(33, b'x');
    test_ok(34, b'y');
    test_ok(35, b'z');

    let test_err = |x| {
        assert!(digit_to_display_byte_lower(x).is_none());
    };
    test_err(36);
    test_err(100);
}

#[test]
fn digit_to_display_byte_lower_properties() {
    unsigned_gen().test_properties(|b| {
        assert_eq!(digit_to_display_byte_lower(b).is_some(), b < 36);
    });

    unsigned_gen_var_7().test_properties(|b| {
        let display_byte = digit_to_display_byte_lower(b).unwrap();
        assert!(display_byte.is_ascii_digit() || display_byte.is_ascii_lowercase());
        let display_byte_upper = digit_to_display_byte_upper(b).unwrap();
        assert_eq!(display_byte == display_byte_upper, b < 10);
        assert_eq!(
            char::from(display_byte).to_ascii_uppercase(),
            char::from(display_byte_upper)
        );
    });
}

#[test]
fn test_digit_to_display_byte_upper() {
    let test_ok = |x, y| {
        assert_eq!(digit_to_display_byte_upper(x).unwrap(), y);
    };
    test_ok(0, b'0');
    test_ok(1, b'1');
    test_ok(2, b'2');
    test_ok(3, b'3');
    test_ok(4, b'4');
    test_ok(5, b'5');
    test_ok(6, b'6');
    test_ok(7, b'7');
    test_ok(8, b'8');
    test_ok(9, b'9');
    test_ok(10, b'A');
    test_ok(11, b'B');
    test_ok(12, b'C');
    test_ok(33, b'X');
    test_ok(34, b'Y');
    test_ok(35, b'Z');

    let test_err = |x| {
        assert!(digit_to_display_byte_upper(x).is_none());
    };
    test_err(36);
    test_err(100);
}

#[test]
fn digit_to_display_byte_upper_properties() {
    unsigned_gen().test_properties(|b| {
        assert_eq!(
            digit_to_display_byte_upper(b).is_some(),
            (0..36).contains(&b)
        );
    });

    unsigned_gen_var_7().test_properties(|b| {
        let display_byte = digit_to_display_byte_upper(b).unwrap();
        assert!(display_byte.is_ascii_digit() || display_byte.is_ascii_uppercase());
        let display_byte_lower = digit_to_display_byte_lower(b).unwrap();
        assert_eq!(display_byte == display_byte_lower, b < 10);
        assert_eq!(
            char::from(display_byte).to_ascii_lowercase(),
            char::from(display_byte_lower)
        );
    });
}

#[test]
pub fn test_to_string_base() {
    fn test_u<T: PrimitiveUnsigned>(x: T, base: u8, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
        u8: WrappingFrom<T>,
    {
        assert_eq!(x.to_string_base(base), out);
        assert_eq!(to_string_base_unsigned_naive(x, base), out);
        assert_eq!(format!("{}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:00}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:00?}", BaseFmtWrapper::new(x, base)), out);
    }
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

    fn test_u_width<T: PrimitiveUnsigned>(x: T, base: u8, width: usize, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
        u8: WrappingFrom<T>,
    {
        let s = x.to_string_base(base);
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(x, base), width = width),
            out
        );
        assert_eq!(
            format!("{:0width$?}", BaseFmtWrapper::new(x, base), width = width),
            out
        );
        test_padding_unsigned(&s, out, width);
    }
    test_u_width::<u8>(0, 2, 0, "0");
    test_u_width::<u8>(0, 2, 1, "0");
    test_u_width::<u8>(0, 2, 2, "00");
    test_u_width::<u8>(0, 2, 5, "00000");
    test_u_width::<u32>(1000000, 36, 0, "lfls");
    test_u_width::<u32>(1000000, 36, 1, "lfls");
    test_u_width::<u32>(1000000, 36, 2, "lfls");
    test_u_width::<u32>(1000000, 36, 3, "lfls");
    test_u_width::<u32>(1000000, 36, 4, "lfls");
    test_u_width::<u32>(1000000, 36, 5, "0lfls");
    test_u_width::<u32>(1000000, 36, 6, "00lfls");

    fn test_i<T: PrimitiveSigned>(x: T, base: u8, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
        u8: WrappingFrom<<T as UnsignedAbs>::Output>,
        <T as UnsignedAbs>::Output: PrimitiveUnsigned,
    {
        assert_eq!(x.to_string_base(base), out);
        assert_eq!(to_string_base_signed_naive(x, base), out);
        assert_eq!(format!("{}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:00}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:00?}", BaseFmtWrapper::new(x, base)), out);
    }
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

    fn test_i_width<T: PrimitiveSigned>(x: T, base: u8, width: usize, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
        u8: WrappingFrom<T>,
    {
        let s = x.to_string_base(base);
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(x, base), width = width),
            out
        );
        assert_eq!(
            format!("{:0width$?}", BaseFmtWrapper::new(x, base), width = width),
            out
        );
        test_padding_signed(&s, out, width);
    }
    test_i_width::<i8>(0, 2, 0, "0");
    test_i_width::<i8>(0, 2, 1, "0");
    test_i_width::<i8>(0, 2, 2, "00");
    test_i_width::<i8>(0, 2, 5, "00000");
    test_i_width::<i32>(1000000, 36, 0, "lfls");
    test_i_width::<i32>(1000000, 36, 1, "lfls");
    test_i_width::<i32>(1000000, 36, 2, "lfls");
    test_i_width::<i32>(1000000, 36, 3, "lfls");
    test_i_width::<i32>(1000000, 36, 4, "lfls");
    test_i_width::<i32>(1000000, 36, 5, "0lfls");
    test_i_width::<i32>(1000000, 36, 6, "00lfls");
    test_i_width::<i32>(-1000000, 36, 0, "-lfls");
    test_i_width::<i32>(-1000000, 36, 1, "-lfls");
    test_i_width::<i32>(-1000000, 36, 2, "-lfls");
    test_i_width::<i32>(-1000000, 36, 3, "-lfls");
    test_i_width::<i32>(-1000000, 36, 4, "-lfls");
    test_i_width::<i32>(-1000000, 36, 5, "-lfls");
    test_i_width::<i32>(-1000000, 36, 6, "-0lfls");
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
    unsigned_pair_gen_var_8::<T, u8>().test_properties(|(x, base)| {
        let s = x.to_string_base(base);
        assert_eq!(to_string_base_unsigned_naive(x, base), s);
        assert_eq!(format!("{}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:00}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:00?}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(x.to_string_base_upper(base), s.to_uppercase());
        assert_eq!(T::from_string_base(base, &s).unwrap(), x);
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

    unsigned_triple_gen_var_6::<T, u8, usize>().test_properties(|(x, base, width)| {
        let fx = BaseFmtWrapper::new(x, base);
        let s = x.to_string_base(base);
        let s_padded = format!("{fx:0width$}");
        assert_eq!(format!("{fx:0width$?}"), s_padded);
        assert_eq!(T::from_string_base(base, &s).unwrap(), x);
        assert!(string_is_subset(
            &s_padded,
            "0123456789abcdefghijklmnopqrstuvwxyz"
        ));
        test_padding_unsigned(&s, &s_padded, width);
    });

    unsigned_pair_gen_var_2::<T, usize>().test_properties(|(x, width)| {
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(x, 10), width = width),
            format!("{x:0width$}")
        );
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(x, 2), width = width),
            format!("{x:0width$b}")
        );
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(x, 8), width = width),
            format!("{x:0width$o}")
        );
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(x, 16), width = width),
            format!("{x:0width$x}")
        );
    });

    unsigned_pair_gen_var_9::<usize, u8>().test_properties(|(width, base)| {
        let s = format!(
            "{:0width$}",
            BaseFmtWrapper::new(T::ZERO, base),
            width = width
        );
        assert_eq!(repeat_n('0', max(1, width)).collect::<String>(), s);
    });
}

fn to_string_base_helper_signed<T: PrimitiveSigned>()
where
    BaseFmtWrapper<T>: Debug + Display,
    u8: WrappingFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    signed_unsigned_pair_gen_var_5::<T, u8>().test_properties(|(x, base)| {
        let s = x.to_string_base(base);
        assert_eq!(to_string_base_signed_naive(x, base), s);
        assert_eq!(format!("{}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:?}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:00}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:00?}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(x.to_string_base_upper(base), s.to_uppercase());
        assert_eq!(T::from_string_base(base, &s).unwrap(), x);
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

    signed_unsigned_unsigned_triple_gen_var_3::<T, u8, usize>().test_properties(
        |(x, base, width)| {
            let fx = BaseFmtWrapper::new(x, base);
            let s = x.to_string_base(base);
            let s_padded = format!("{fx:0width$}");
            assert!(s_padded.len() >= width);
            assert_eq!(s.len() >= width, s == s_padded);
            assert_eq!(format!("{fx:0width$?}"), s_padded);
            assert_eq!(T::from_string_base(base, &s).unwrap(), x);
            assert!(string_is_subset(
                &s_padded,
                "-0123456789abcdefghijklmnopqrstuvwxyz"
            ));
            test_padding_signed(&s, &s_padded, width);
        },
    );

    signed_unsigned_pair_gen_var_6::<T, usize>().test_properties(|(x, width)| {
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(x, 10), width = width),
            format!("{x:0width$}")
        );
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(x, 2), width = width),
            format!("{x:0width$b}")
        );
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(x, 8), width = width),
            format!("{x:0width$o}")
        );
        assert_eq!(
            format!("{:0width$}", BaseFmtWrapper::new(x, 16), width = width),
            format!("{x:0width$x}")
        );
    });

    unsigned_pair_gen_var_9::<usize, u8>().test_properties(|(width, base)| {
        let s = format!(
            "{:0width$}",
            BaseFmtWrapper::new(T::ZERO, base),
            width = width
        );
        assert_eq!(repeat_n('0', max(1, width)).collect::<String>(), s);
    });
}

#[test]
fn to_string_base_properties() {
    apply_fn_to_unsigneds!(to_string_base_helper_unsigned);
    apply_fn_to_signeds!(to_string_base_helper_signed);
}

#[test]
pub fn test_to_string_base_upper() {
    fn test_u<T: PrimitiveUnsigned>(x: T, base: u8, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
    {
        assert_eq!(x.to_string_base_upper(base), out);
        assert_eq!(format!("{:#}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:#?}", BaseFmtWrapper::new(x, base)), out);
    }
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

    fn test_u_width<T: PrimitiveUnsigned>(x: T, base: u8, width: usize, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
        u8: WrappingFrom<T>,
    {
        let s = x.to_string_base_upper(base);
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(x, base), width = width),
            out
        );
        assert_eq!(
            format!("{:#0width$?}", BaseFmtWrapper::new(x, base), width = width),
            out
        );
        test_padding_unsigned(&s, out, width);
    }
    test_u_width::<u8>(0, 2, 0, "0");
    test_u_width::<u8>(0, 2, 1, "0");
    test_u_width::<u8>(0, 2, 2, "00");
    test_u_width::<u8>(0, 2, 5, "00000");
    test_u_width::<u32>(1000000, 36, 0, "LFLS");
    test_u_width::<u32>(1000000, 36, 1, "LFLS");
    test_u_width::<u32>(1000000, 36, 2, "LFLS");
    test_u_width::<u32>(1000000, 36, 3, "LFLS");
    test_u_width::<u32>(1000000, 36, 4, "LFLS");
    test_u_width::<u32>(1000000, 36, 5, "0LFLS");
    test_u_width::<u32>(1000000, 36, 6, "00LFLS");

    fn test_i<T: PrimitiveSigned>(x: T, base: u8, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
    {
        assert_eq!(x.to_string_base_upper(base), out);
        assert_eq!(format!("{:#}", BaseFmtWrapper::new(x, base)), out);
        assert_eq!(format!("{:#?}", BaseFmtWrapper::new(x, base)), out);
    }
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

    fn test_i_width<T: PrimitiveSigned>(x: T, base: u8, width: usize, out: &str)
    where
        BaseFmtWrapper<T>: Debug + Display,
        u8: WrappingFrom<T>,
    {
        let s = x.to_string_base_upper(base);
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(x, base), width = width),
            out
        );
        assert_eq!(
            format!("{:#0width$?}", BaseFmtWrapper::new(x, base), width = width),
            out
        );
        test_padding_signed(&s, out, width);
    }
    test_i_width::<i8>(0, 2, 0, "0");
    test_i_width::<i8>(0, 2, 1, "0");
    test_i_width::<i8>(0, 2, 2, "00");
    test_i_width::<i8>(0, 2, 5, "00000");
    test_i_width::<i32>(1000000, 36, 0, "LFLS");
    test_i_width::<i32>(1000000, 36, 1, "LFLS");
    test_i_width::<i32>(1000000, 36, 2, "LFLS");
    test_i_width::<i32>(1000000, 36, 3, "LFLS");
    test_i_width::<i32>(1000000, 36, 4, "LFLS");
    test_i_width::<i32>(1000000, 36, 5, "0LFLS");
    test_i_width::<i32>(1000000, 36, 6, "00LFLS");
    test_i_width::<i32>(-1000000, 36, 0, "-LFLS");
    test_i_width::<i32>(-1000000, 36, 1, "-LFLS");
    test_i_width::<i32>(-1000000, 36, 2, "-LFLS");
    test_i_width::<i32>(-1000000, 36, 3, "-LFLS");
    test_i_width::<i32>(-1000000, 36, 4, "-LFLS");
    test_i_width::<i32>(-1000000, 36, 5, "-LFLS");
    test_i_width::<i32>(-1000000, 36, 6, "-0LFLS");
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
    unsigned_pair_gen_var_8::<T, u8>().test_properties(|(x, base)| {
        let s = x.to_string_base_upper(base);
        assert_eq!(format!("{:#}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:#?}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:#00}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(x.to_string_base(base), s.to_lowercase());
        assert_eq!(T::from_string_base(base, &s).unwrap(), x);
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

    unsigned_triple_gen_var_6::<T, u8, usize>().test_properties(|(x, base, width)| {
        let fx = BaseFmtWrapper::new(x, base);
        let s = x.to_string_base_upper(base);
        let s_padded = format!("{fx:#0width$}");
        assert_eq!(format!("{fx:#0width$?}"), s_padded);
        assert_eq!(T::from_string_base(base, &s).unwrap(), x);
        assert!(string_is_subset(
            &s_padded,
            "01234567890123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        ));
        test_padding_unsigned(&s, &s_padded, width);
    });

    unsigned_pair_gen_var_2::<T, usize>().test_properties(|(x, width)| {
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(x, 10), width = width),
            format!("{x:0width$}")
        );
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(x, 2), width = width),
            format!("{x:0width$b}")
        );
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(x, 8), width = width),
            format!("{x:0width$o}")
        );
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(x, 16), width = width),
            format!("{x:0width$X}")
        );
    });

    unsigned_pair_gen_var_9::<usize, u8>().test_properties(|(width, base)| {
        let s = format!(
            "{:#0width$}",
            BaseFmtWrapper::new(T::ZERO, base),
            width = width
        );
        assert_eq!(repeat_n('0', max(1, width)).collect::<String>(), s);
    });
}

fn to_string_base_upper_helper_signed<T: PrimitiveSigned>()
where
    BaseFmtWrapper<T>: Debug + Display,
{
    signed_unsigned_pair_gen_var_5::<T, u8>().test_properties(|(x, base)| {
        let s = x.to_string_base_upper(base);
        assert_eq!(format!("{:#}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(format!("{:#?}", BaseFmtWrapper::new(x, base)), s);
        assert_eq!(x.to_string_base(base), s.to_lowercase());
        assert_eq!(T::from_string_base(base, &s).unwrap(), x);
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

    signed_unsigned_unsigned_triple_gen_var_3::<T, u8, usize>().test_properties(
        |(x, base, width)| {
            let fx = BaseFmtWrapper::new(x, base);
            let s = x.to_string_base_upper(base);
            let s_padded = format!("{fx:#0width$}");
            assert!(s_padded.len() >= width);
            assert_eq!(s.len() >= width, s == s_padded);
            assert_eq!(format!("{fx:#0width$?}"), s_padded);
            assert_eq!(T::from_string_base(base, &s).unwrap(), x);
            assert!(string_is_subset(
                &s_padded,
                "-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            ));
            test_padding_signed(&s, &s_padded, width);
        },
    );

    signed_unsigned_pair_gen_var_6::<T, usize>().test_properties(|(x, width)| {
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(x, 10), width = width),
            format!("{x:0width$}")
        );
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(x, 2), width = width),
            format!("{x:0width$b}")
        );
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(x, 8), width = width),
            format!("{x:0width$o}")
        );
        assert_eq!(
            format!("{:#0width$}", BaseFmtWrapper::new(x, 16), width = width),
            format!("{x:0width$X}")
        );
    });

    unsigned_pair_gen_var_9::<usize, u8>().test_properties(|(width, base)| {
        let s = format!(
            "{:#0width$}",
            BaseFmtWrapper::new(T::ZERO, base),
            width = width
        );
        assert_eq!(repeat_n('0', max(1, width)).collect::<String>(), s);
    });
}

#[test]
fn to_string_base_upper_properties() {
    apply_fn_to_unsigneds!(to_string_base_upper_helper_unsigned);
    apply_fn_to_signeds!(to_string_base_upper_helper_signed);
}
