// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::num::logic::bit_convertible::{
    to_bits_asc_alt, to_bits_asc_signed_naive, to_bits_asc_unsigned_naive, to_bits_desc_alt,
    to_bits_desc_signed_naive, to_bits_desc_unsigned_naive,
};

#[test]
pub fn test_to_bits_asc() {
    fn test_unsigned<T: PrimitiveUnsigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_asc(), out);
        assert_eq!(to_bits_asc_unsigned_naive(x), out);
        assert_eq!(to_bits_asc_alt(&x), out);
        assert_eq!(x.bits().collect_vec(), out);
    }
    test_unsigned(0u8, &[]);
    test_unsigned(1u16, &[true]);
    test_unsigned(2u32, &[false, true]);
    test_unsigned(3u64, &[true, true]);
    test_unsigned(123u16, &[true, true, false, true, true, true, true]);
    test_unsigned(u8::MAX, &[true; 8]);

    fn test_signed<T: PrimitiveSigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_asc(), out);
        assert_eq!(to_bits_asc_signed_naive(x), out);
        assert_eq!(to_bits_asc_alt(&x), out);
        assert_eq!(x.bits().collect_vec(), out);
    }
    test_signed(0i8, &[]);
    test_signed(1i16, &[true, false]);
    test_signed(2i32, &[false, true, false]);
    test_signed(3i64, &[true, true, false]);
    test_signed(-1i16, &[true]);
    test_signed(-2i32, &[false, true]);
    test_signed(-3i64, &[true, false, true]);
    test_signed(123i16, &[true, true, false, true, true, true, true, false]);
    test_signed(
        -123i16,
        &[true, false, true, false, false, false, false, true],
    );
    test_signed(i8::MAX, &[true, true, true, true, true, true, true, false]);
    test_signed(
        i8::MIN,
        &[false, false, false, false, false, false, false, true],
    );
}

#[test]
pub fn test_to_bits_desc() {
    fn test_unsigned<T: PrimitiveUnsigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_desc(), out);
        assert_eq!(to_bits_desc_unsigned_naive(x), out);
        assert_eq!(to_bits_desc_alt(&x), out);
        assert_eq!(x.bits().rev().collect_vec(), out);
    }
    test_unsigned(0u8, &[]);
    test_unsigned(1u16, &[true]);
    test_unsigned(2u32, &[true, false]);
    test_unsigned(3u64, &[true, true]);
    test_unsigned(123u16, &[true, true, true, true, false, true, true]);
    test_unsigned(u8::MAX, &[true; 8]);

    fn test_signed<T: PrimitiveSigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_desc(), out);
        assert_eq!(to_bits_desc_signed_naive(x), out);
        assert_eq!(to_bits_desc_alt(&x), out);
        assert_eq!(x.bits().rev().collect_vec(), out);
    }
    test_signed(0i8, &[]);
    test_signed(1i16, &[false, true]);
    test_signed(2i32, &[false, true, false]);
    test_signed(3i64, &[false, true, true]);
    test_signed(-1i16, &[true]);
    test_signed(-2i32, &[true, false]);
    test_signed(-3i64, &[true, false, true]);
    test_signed(123i16, &[false, true, true, true, true, false, true, true]);
    test_signed(
        -123i16,
        &[true, false, false, false, false, true, false, true],
    );
    test_signed(i8::MAX, &[false, true, true, true, true, true, true, true]);
    test_signed(
        i8::MIN,
        &[true, false, false, false, false, false, false, false],
    );
}

fn to_bits_asc_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|u| {
        let bits = u.to_bits_asc();
        assert_eq!(to_bits_asc_unsigned_naive(u), bits);
        assert_eq!(to_bits_asc_alt(&u), bits);
        assert_eq!(u.bits().collect::<Vec<bool>>(), bits);
        assert_eq!(
            bits.iter().copied().rev().collect::<Vec<bool>>(),
            u.to_bits_desc()
        );
        assert_eq!(T::from_bits_asc(bits.iter().copied()), u);
        if u != T::ZERO {
            assert_eq!(*bits.last().unwrap(), true);
        }
        assert_eq!(bits.len(), usize::exact_from(u.significant_bits()));
    });
}

fn to_bits_asc_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|i| {
        let bits = i.to_bits_asc();
        assert_eq!(to_bits_asc_signed_naive(i), bits);
        assert_eq!(to_bits_asc_alt(&i), bits);
        assert_eq!(i.bits().collect::<Vec<bool>>(), bits);
        assert_eq!(
            bits.iter().copied().rev().collect::<Vec<bool>>(),
            i.to_bits_desc()
        );
        assert_eq!(T::from_bits_asc(bits.iter().copied()), i);
        if i != T::ZERO {
            assert_eq!(*bits.last().unwrap(), i < T::ZERO);
        }
        let bit_len = bits.len();
        assert!(bit_len <= usize::exact_from(T::WIDTH));
        if bit_len > 1 {
            assert_ne!(bits[bit_len - 1], bits[bit_len - 2]);
        }
    });
}

#[test]
fn to_bits_asc_properties() {
    apply_fn_to_unsigneds!(to_bits_asc_properties_helper_unsigned);
    apply_fn_to_signeds!(to_bits_asc_properties_helper_signed);
}

fn to_bits_desc_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|u| {
        let bits = u.to_bits_desc();
        assert_eq!(to_bits_desc_unsigned_naive(u), bits);
        assert_eq!(to_bits_desc_alt(&u), bits);
        assert_eq!(u.bits().rev().collect::<Vec<bool>>(), bits);
        assert_eq!(
            bits.iter().copied().rev().collect::<Vec<bool>>(),
            u.to_bits_asc()
        );
        assert_eq!(T::from_bits_desc(bits.iter().copied()), u);
        if u != T::ZERO {
            assert_eq!(bits[0], true);
        }
        assert_eq!(bits.len(), usize::exact_from(u.significant_bits()));
    });
}

fn to_bits_desc_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|i| {
        let bits = i.to_bits_desc();
        assert_eq!(to_bits_desc_signed_naive(i), bits);
        assert_eq!(to_bits_desc_alt(&i), bits);
        assert_eq!(i.bits().rev().collect::<Vec<bool>>(), bits);
        assert_eq!(
            bits.iter().copied().rev().collect::<Vec<bool>>(),
            i.to_bits_asc()
        );
        assert_eq!(T::from_bits_desc(bits.iter().copied()), i);
        if i != T::ZERO {
            assert_eq!(bits[0], i < T::ZERO);
        }
        let bit_len = bits.len();
        assert!(bit_len <= usize::exact_from(T::WIDTH));
        if bit_len > 1 {
            assert_ne!(bits[0], bits[1]);
        }
    });
}

#[test]
fn to_bits_desc_properties() {
    apply_fn_to_unsigneds!(to_bits_desc_properties_helper_unsigned);
    apply_fn_to_signeds!(to_bits_desc_properties_helper_signed);
}
