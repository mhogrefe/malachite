// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::repeat_n;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    bool_vec_gen_var_1, bool_vec_gen_var_2, bool_vec_gen_var_3, bool_vec_gen_var_4,
    unsigned_gen_var_5,
};
use malachite_base::test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_asc_signed_naive, from_bits_asc_unsigned_naive, from_bits_desc_alt,
};
use std::iter::once;
use std::panic::catch_unwind;

#[test]
pub fn test_from_bits_asc() {
    fn test_unsigned<T: PrimitiveUnsigned>(bits: &[bool], out: T) {
        assert_eq!(T::from_bits_asc(bits.iter().copied()), out);
        assert_eq!(
            from_bits_asc_unsigned_naive::<T, _>(bits.iter().copied()),
            out
        );
        assert_eq!(from_bits_asc_alt::<T, _>(bits.iter().copied()), out);
    }
    test_unsigned(&[], 0u8);
    test_unsigned(&[false], 0u8);
    test_unsigned(&[false, false, false], 0u8);
    test_unsigned(&[false; 100], 0u8);
    test_unsigned(&[true], 1u16);
    test_unsigned(&[false, true], 2u32);
    test_unsigned(&[true, true], 3u64);
    test_unsigned(&[true, true, false, true, true, true, true], 123u16);
    test_unsigned(
        &[true, true, false, true, true, true, true, false, false, false],
        123u16,
    );
    test_unsigned(&[true; 8], u8::MAX);

    fn test_signed<T: PrimitiveSigned>(bits: &[bool], out: T) {
        assert_eq!(T::from_bits_asc(bits.iter().copied()), out);
        assert_eq!(
            from_bits_asc_signed_naive::<T, _>(bits.iter().copied()),
            out
        );
        assert_eq!(from_bits_asc_alt::<T, _>(bits.iter().copied()), out);
    }
    test_signed(&[], 0i8);
    test_signed(&[false], 0i8);
    test_signed(&[false, false, false], 0i8);
    test_signed(&[false; 100], 0i8);
    test_signed(&[true, false], 1i16);
    test_signed(&[false, true, false], 2i32);
    test_signed(&[true, true, false], 3i64);
    test_signed(&[true], -1i16);
    test_signed(&[true, true, true], -1i16);
    test_signed(&[true; 100], -1i16);
    test_signed(&[false, true], -2i32);
    test_signed(&[true, false, true], -3i64);
    test_signed(&[true, true, false, true, true, true, true, false], 123i16);
    test_signed(
        &[true, true, false, true, true, true, true, false, false, false, false],
        123i16,
    );
    test_signed(
        &[true, false, true, false, false, false, false, true],
        -123i16,
    );
    test_signed(
        &[true, false, true, false, false, false, false, true, true, true, true],
        -123i16,
    );
    test_signed(&[true, true, true, true, true, true, true, false], i8::MAX);
    test_signed(
        &[false, false, false, false, false, false, false, true],
        i8::MIN,
    );
}

fn from_bits_asc_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::from_bits_asc(repeat_n(true, 200)));
}

fn from_bits_asc_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::from_bits_asc(
        repeat_n(false, 200).chain([true, false].iter().copied())
    ));
    assert_panic!(T::from_bits_asc(repeat_n(false, 200).chain(once(true))));
}

#[test]
fn from_bits_asc_fail() {
    apply_fn_to_unsigneds!(from_bits_asc_fail_helper_unsigned);
    apply_fn_to_signeds!(from_bits_asc_fail_helper_signed);
}

#[test]
pub fn test_from_bits_desc() {
    fn test_unsigned<T: PrimitiveUnsigned>(bits: &[bool], out: T) {
        assert_eq!(T::from_bits_desc(bits.iter().copied()), out);
        assert_eq!(from_bits_desc_alt::<T, _>(bits.iter().copied()), out);
    }
    test_unsigned(&[], 0u8);
    test_unsigned(&[false], 0u8);
    test_unsigned(&[false, false, false], 0u8);
    test_unsigned(&[false; 100], 0u8);
    test_unsigned(&[true], 1u16);
    test_unsigned(&[true, false], 2u32);
    test_unsigned(&[true, true], 3u64);
    test_unsigned(&[true, true, true, true, false, true, true], 123u16);
    test_unsigned(
        &[false, false, false, true, true, true, true, false, true, true],
        123u16,
    );
    test_unsigned(&[true; 8], u8::MAX);

    fn test_signed<T: PrimitiveSigned>(bits: &[bool], out: T) {
        assert_eq!(T::from_bits_desc(bits.iter().copied()), out);
        assert_eq!(from_bits_desc_alt::<T, _>(bits.iter().copied()), out);
    }
    test_signed(&[], 0i8);
    test_signed(&[false], 0i8);
    test_signed(&[false, false, false], 0i8);
    test_signed(&[false; 100], 0i8);
    test_signed(&[false, true], 1i16);
    test_signed(&[false, true, false], 2i32);
    test_signed(&[false, true, true], 3i64);
    test_signed(&[true], -1i16);
    test_signed(&[true, true, true], -1i16);
    test_signed(&[true; 100], -1i16);
    test_signed(&[true, false], -2i32);
    test_signed(&[true, false, true], -3i64);
    test_signed(&[false, true, true, true, true, false, true, true], 123i16);
    test_signed(
        &[false, false, false, false, true, true, true, true, false, true, true],
        123i16,
    );
    test_signed(
        &[true, false, false, false, false, true, false, true],
        -123i16,
    );
    test_signed(
        &[true, true, true, true, false, false, false, false, true, false, true],
        -123i16,
    );
    test_signed(&[false, true, true, true, true, true, true, true], i8::MAX);
    test_signed(
        &[true, false, false, false, false, false, false, false],
        i8::MIN,
    );
}

fn from_bits_desc_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::from_bits_desc(repeat_n(true, 200)));
}

fn from_bits_desc_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::from_bits_desc(
        [false, true].iter().copied().chain(repeat_n(false, 200))
    ));
    assert_panic!(T::from_bits_desc(once(true).chain(repeat_n(false, 200))));
}

#[test]
fn from_bits_desc_fail() {
    apply_fn_to_unsigneds!(from_bits_desc_fail_helper_unsigned);
    apply_fn_to_signeds!(from_bits_desc_fail_helper_signed);
}

fn from_bits_asc_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    bool_vec_gen_var_1::<T>().test_properties(|bs| {
        let n = T::from_bits_asc(bs.iter().copied());
        assert_eq!(from_bits_asc_unsigned_naive::<T, _>(bs.iter().copied()), n);
        assert_eq!(from_bits_asc_alt::<T, _>(bs.iter().copied()), n);
        let trailing_falses = bs.iter().rev().take_while(|&&bit| !bit).count();
        let trimmed_bits = bs[..bs.len() - trailing_falses].to_vec();
        assert_eq!(n.to_bits_asc(), trimmed_bits);
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(T::from_bits_desc(repeat_n(false, u)), T::ZERO);
    });
}

fn from_bits_asc_properties_helper_signed<T: PrimitiveSigned>() {
    bool_vec_gen_var_2::<T>().test_properties(|bs| {
        let n = T::from_bits_asc(bs.iter().copied());
        assert_eq!(from_bits_asc_signed_naive::<T, _>(bs.iter().copied()), n);
        assert_eq!(from_bits_asc_alt::<T, _>(bs.iter().copied()), n);
        let trimmed_bits = if bs.iter().all(|&bit| !bit) {
            Vec::new()
        } else {
            let sign_bits = if *bs.last().unwrap() {
                bs.iter().rev().take_while(|&&bit| bit).count()
            } else {
                bs.iter().rev().take_while(|&&bit| !bit).count()
            };
            bs[..=bs.len() - sign_bits].to_vec()
        };
        assert_eq!(n.to_bits_asc(), trimmed_bits);
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(T::from_bits_asc(repeat_n(false, u)), T::ZERO);
        assert_eq!(T::from_bits_asc(repeat_n(true, u + 1)), T::NEGATIVE_ONE);
    });
}

#[test]
fn from_bits_asc_properties() {
    apply_fn_to_unsigneds!(from_bits_asc_properties_helper_unsigned);
    apply_fn_to_signeds!(from_bits_asc_properties_helper_signed);
}

fn from_bits_desc_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    bool_vec_gen_var_3::<T>().test_properties(|bs| {
        let n = T::from_bits_desc(bs.iter().copied());
        assert_eq!(from_bits_desc_alt::<T, _>(bs.iter().copied()), n);
        let leading_falses = bs.iter().take_while(|&&bit| !bit).count();
        let trimmed_bits = bs[leading_falses..].to_vec();
        assert_eq!(n.to_bits_desc(), trimmed_bits);
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(T::from_bits_desc(repeat_n(false, u)), T::ZERO);
    });
}

fn from_bits_desc_properties_helper_signed<T: PrimitiveSigned>() {
    bool_vec_gen_var_4::<T>().test_properties(|bs| {
        let n = T::from_bits_desc(bs.iter().copied());
        assert_eq!(from_bits_desc_alt::<T, _>(bs.iter().copied()), n);
        let trimmed_bits = if bs.iter().all(|&bit| !bit) {
            Vec::new()
        } else {
            let sign_bits = if bs[0] {
                bs.iter().take_while(|&&bit| bit).count()
            } else {
                bs.iter().take_while(|&&bit| !bit).count()
            };
            bs[sign_bits - 1..].to_vec()
        };
        assert_eq!(n.to_bits_desc(), trimmed_bits);
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(T::from_bits_desc(repeat_n(false, u)), T::ZERO);
        assert_eq!(T::from_bits_desc(repeat_n(true, u + 1)), T::NEGATIVE_ONE);
    });
}

#[test]
fn from_bits_desc_properties() {
    apply_fn_to_unsigneds!(from_bits_desc_properties_helper_unsigned);
    apply_fn_to_signeds!(from_bits_desc_properties_helper_signed);
}
