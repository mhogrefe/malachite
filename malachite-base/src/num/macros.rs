// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_unsigneds {
    ($m: tt) => {
        $m!(u8);
        $m!(u16);
        $m!(u32);
        $m!(u64);
        $m!(u128);
        $m!(usize);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_signeds {
    ($m: tt) => {
        $m!(i8);
        $m!(i16);
        $m!(i32);
        $m!(i64);
        $m!(i128);
        $m!(isize);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_primitive_ints {
    ($m: tt) => {
        apply_to_unsigneds!($m);
        apply_to_signeds!($m);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_unsigned_signed_pairs {
    ($m: tt) => {
        $m!(u8, i8);
        $m!(u16, i16);
        $m!(u32, i32);
        $m!(u64, i64);
        $m!(u128, i128);
        $m!(usize, isize);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds {
    ($f: ident) => {
        $f::<u8>();
        $f::<u16>();
        $f::<u32>();
        $f::<u64>();
        $f::<u128>();
        $f::<usize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_signeds {
    ($f: ident) => {
        $f::<i8>();
        $f::<i16>();
        $f::<i32>();
        $f::<i64>();
        $f::<i128>();
        $f::<isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_ints {
    ($f: ident) => {
        apply_fn_to_unsigneds!($f);
        apply_fn_to_signeds!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigned_signed_pairs {
    ($f: ident) => {
        $f::<u8, i8>();
        $f::<u16, i16>();
        $f::<u32, i32>();
        $f::<u64, i64>();
        $f::<u128, i128>();
        $f::<usize, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds_and_unsigneds {
    ($f: ident) => {
        $f::<u8, u8>();
        $f::<u8, u16>();
        $f::<u8, u32>();
        $f::<u8, u64>();
        $f::<u8, u128>();
        $f::<u8, usize>();
        $f::<u16, u8>();
        $f::<u16, u16>();
        $f::<u16, u32>();
        $f::<u16, u64>();
        $f::<u16, u128>();
        $f::<u16, usize>();
        $f::<u32, u8>();
        $f::<u32, u16>();
        $f::<u32, u32>();
        $f::<u32, u64>();
        $f::<u32, u128>();
        $f::<u32, usize>();
        $f::<u64, u8>();
        $f::<u64, u16>();
        $f::<u64, u32>();
        $f::<u64, u64>();
        $f::<u64, u128>();
        $f::<u64, usize>();
        $f::<u128, u8>();
        $f::<u128, u16>();
        $f::<u128, u32>();
        $f::<u128, u64>();
        $f::<u128, u128>();
        $f::<u128, usize>();
        $f::<usize, u8>();
        $f::<usize, u16>();
        $f::<usize, u32>();
        $f::<usize, u64>();
        $f::<usize, u128>();
        $f::<usize, usize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds_and_signeds {
    ($f: ident) => {
        $f::<u8, i8>();
        $f::<u8, i16>();
        $f::<u8, i32>();
        $f::<u8, i64>();
        $f::<u8, i128>();
        $f::<u8, isize>();
        $f::<u16, i8>();
        $f::<u16, i16>();
        $f::<u16, i32>();
        $f::<u16, i64>();
        $f::<u16, i128>();
        $f::<u16, isize>();
        $f::<u32, i8>();
        $f::<u32, i16>();
        $f::<u32, i32>();
        $f::<u32, i64>();
        $f::<u32, i128>();
        $f::<u32, isize>();
        $f::<u64, i8>();
        $f::<u64, i16>();
        $f::<u64, i32>();
        $f::<u64, i64>();
        $f::<u64, i128>();
        $f::<u64, isize>();
        $f::<u128, i8>();
        $f::<u128, i16>();
        $f::<u128, i32>();
        $f::<u128, i64>();
        $f::<u128, i128>();
        $f::<u128, isize>();
        $f::<usize, i8>();
        $f::<usize, i16>();
        $f::<usize, i32>();
        $f::<usize, i64>();
        $f::<usize, i128>();
        $f::<usize, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds_and_primitive_ints {
    ($f: ident) => {
        apply_fn_to_unsigneds_and_unsigneds!($f);
        apply_fn_to_unsigneds_and_signeds!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds_and_unsigned_signed_pairs {
    ($f: ident) => {
        $f::<u8, u8, i8>();
        $f::<u8, u16, i16>();
        $f::<u8, u32, i32>();
        $f::<u8, u64, i64>();
        $f::<u8, u128, i128>();
        $f::<u8, usize, isize>();
        $f::<u16, u8, i8>();
        $f::<u16, u16, i16>();
        $f::<u16, u32, i32>();
        $f::<u16, u64, i64>();
        $f::<u16, u128, i128>();
        $f::<u16, usize, isize>();
        $f::<u32, u8, i8>();
        $f::<u32, u16, i16>();
        $f::<u32, u32, i32>();
        $f::<u32, u64, i64>();
        $f::<u32, u128, i128>();
        $f::<u32, usize, isize>();
        $f::<u64, u8, i8>();
        $f::<u64, u16, i16>();
        $f::<u64, u32, i32>();
        $f::<u64, u64, i64>();
        $f::<u64, u128, i128>();
        $f::<u64, usize, isize>();
        $f::<u128, u8, i8>();
        $f::<u128, u16, i16>();
        $f::<u128, u32, i32>();
        $f::<u128, u64, i64>();
        $f::<u128, u128, i128>();
        $f::<u128, usize, isize>();
        $f::<usize, u8, i8>();
        $f::<usize, u16, i16>();
        $f::<usize, u32, i32>();
        $f::<usize, u64, i64>();
        $f::<usize, u128, i128>();
        $f::<usize, usize, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_unsigneds_and_primitive_floats {
    ($f: ident) => {
        $f::<u8, f32>();
        $f::<u8, f64>();
        $f::<u16, f32>();
        $f::<u16, f64>();
        $f::<u32, f32>();
        $f::<u32, f64>();
        $f::<u64, f32>();
        $f::<u64, f64>();
        $f::<u128, f32>();
        $f::<u128, f64>();
        $f::<usize, f32>();
        $f::<usize, f64>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_signeds_and_primitive_floats {
    ($f: ident) => {
        $f::<i8, f32>();
        $f::<i8, f64>();
        $f::<i16, f32>();
        $f::<i16, f64>();
        $f::<i32, f32>();
        $f::<i32, f64>();
        $f::<i64, f32>();
        $f::<i64, f64>();
        $f::<i128, f32>();
        $f::<i128, f64>();
        $f::<isize, f32>();
        $f::<isize, f64>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_ints_and_primitive_floats {
    ($f: ident) => {
        apply_fn_to_unsigneds_and_primitive_floats!($f);
        apply_fn_to_signeds_and_primitive_floats!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_floats_and_unsigneds {
    ($f: ident) => {
        $f::<f32, u8>();
        $f::<f32, u16>();
        $f::<f32, u32>();
        $f::<f32, u64>();
        $f::<f32, u128>();
        $f::<f32, usize>();
        $f::<f64, u8>();
        $f::<f64, u16>();
        $f::<f64, u32>();
        $f::<f64, u64>();
        $f::<f64, u128>();
        $f::<f64, usize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_floats_and_signeds {
    ($f: ident) => {
        $f::<f32, i8>();
        $f::<f32, i16>();
        $f::<f32, i32>();
        $f::<f32, i64>();
        $f::<f32, i128>();
        $f::<f32, isize>();
        $f::<f64, i8>();
        $f::<f64, i16>();
        $f::<f64, i32>();
        $f::<f64, i64>();
        $f::<f64, i128>();
        $f::<f64, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_floats_and_unsigned_signed_pairs {
    ($f: ident) => {
        $f::<f32, u8, i8>();
        $f::<f32, u16, i16>();
        $f::<f32, u32, i32>();
        $f::<f32, u64, i64>();
        $f::<f32, u128, i128>();
        $f::<f32, usize, isize>();
        $f::<f64, u8, i8>();
        $f::<f64, u16, i16>();
        $f::<f64, u32, i32>();
        $f::<f64, u64, i64>();
        $f::<f64, u128, i128>();
        $f::<f64, usize, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_signeds_and_unsigneds {
    ($f: ident) => {
        $f::<i8, u8>();
        $f::<i8, u16>();
        $f::<i8, u32>();
        $f::<i8, u64>();
        $f::<i8, u128>();
        $f::<i8, usize>();
        $f::<i16, u8>();
        $f::<i16, u16>();
        $f::<i16, u32>();
        $f::<i16, u64>();
        $f::<i16, u128>();
        $f::<i16, usize>();
        $f::<i32, u8>();
        $f::<i32, u16>();
        $f::<i32, u32>();
        $f::<i32, u64>();
        $f::<i32, u128>();
        $f::<i32, usize>();
        $f::<i64, u8>();
        $f::<i64, u16>();
        $f::<i64, u32>();
        $f::<i64, u64>();
        $f::<i64, u128>();
        $f::<i64, usize>();
        $f::<i128, u8>();
        $f::<i128, u16>();
        $f::<i128, u32>();
        $f::<i128, u64>();
        $f::<i128, u128>();
        $f::<i128, usize>();
        $f::<isize, u8>();
        $f::<isize, u16>();
        $f::<isize, u32>();
        $f::<isize, u64>();
        $f::<isize, u128>();
        $f::<isize, usize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_signeds_and_signeds {
    ($f: ident) => {
        $f::<i8, i8>();
        $f::<i8, i16>();
        $f::<i8, i32>();
        $f::<i8, i64>();
        $f::<i8, i128>();
        $f::<i8, isize>();
        $f::<i16, i8>();
        $f::<i16, i16>();
        $f::<i16, i32>();
        $f::<i16, i64>();
        $f::<i16, i128>();
        $f::<i16, isize>();
        $f::<i32, i8>();
        $f::<i32, i16>();
        $f::<i32, i32>();
        $f::<i32, i64>();
        $f::<i32, i128>();
        $f::<i32, isize>();
        $f::<i64, i8>();
        $f::<i64, i16>();
        $f::<i64, i32>();
        $f::<i64, i64>();
        $f::<i64, i128>();
        $f::<i64, isize>();
        $f::<i128, i8>();
        $f::<i128, i16>();
        $f::<i128, i32>();
        $f::<i128, i64>();
        $f::<i128, i128>();
        $f::<i128, isize>();
        $f::<isize, i8>();
        $f::<isize, i16>();
        $f::<isize, i32>();
        $f::<isize, i64>();
        $f::<isize, i128>();
        $f::<isize, isize>();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_ints_and_unsigneds {
    ($f: ident) => {
        apply_fn_to_unsigneds_and_unsigneds!($f);
        apply_fn_to_signeds_and_unsigneds!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_ints_and_signeds {
    ($f: ident) => {
        apply_fn_to_unsigneds_and_signeds!($f);
        apply_fn_to_signeds_and_signeds!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_ints_and_primitive_ints {
    ($f: ident) => {
        apply_fn_to_primitive_ints_and_unsigneds!($f);
        apply_fn_to_primitive_ints_and_signeds!($f);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_primitive_floats {
    ($m: tt) => {
        $m!(f32);
        $m!(f64);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_primitive_float_unsigned_pairs {
    ($m: tt) => {
        $m!(f32, u32);
        $m!(f64, u64);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! apply_fn_to_primitive_floats {
    ($f: ident) => {
        $f::<f32>();
        $f::<f64>();
    };
}
