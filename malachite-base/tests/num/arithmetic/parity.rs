// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};

fn even_primitive_helper<T: PrimitiveInt>() {
    let test = |n: T, out| {
        assert_eq!(n.even(), out);
    };
    test(T::ZERO, true);
    test(T::ONE, false);
    test(T::TWO, true);
    test(T::exact_from(123), false);
    test(T::exact_from(124), true);
    test(T::MAX, false);
}

fn even_signed_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.even(), out);
    };
    test(T::NEGATIVE_ONE, false);
    test(T::exact_from(-123), false);
    test(T::exact_from(-124), true);
    test(T::MIN, true);
}

#[test]
fn test_even() {
    apply_fn_to_primitive_ints!(even_primitive_helper);
    apply_fn_to_signeds!(even_signed_helper);
}

fn odd_primitive_helper<T: PrimitiveInt>() {
    let test = |n: T, out| {
        assert_eq!(n.odd(), out);
    };
    test(T::ZERO, false);
    test(T::ONE, true);
    test(T::TWO, false);
    test(T::exact_from(123), true);
    test(T::exact_from(124), false);
    test(T::MAX, true);
}

fn odd_signed_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.odd(), out);
    };
    test(T::NEGATIVE_ONE, true);
    test(T::exact_from(-123), true);
    test(T::exact_from(-124), false);
    test(T::MIN, false);
}

#[test]
fn test_odd() {
    apply_fn_to_primitive_ints!(odd_primitive_helper);
    apply_fn_to_signeds!(odd_signed_helper);
}

fn even_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        let even = x.even();
        assert_eq!(x.divisible_by(T::TWO), even);
        assert_eq!(!x.odd(), even);
        if x != T::MAX {
            assert_eq!((x + T::ONE).odd(), even);
        }
        if x != T::ZERO {
            assert_eq!((x - T::ONE).odd(), even);
        }
    });
}

fn even_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        let even = x.even();
        assert_eq!(x.divisible_by(T::TWO), even);
        assert_eq!(!x.odd(), even);
        if x != T::MAX {
            assert_eq!((x + T::ONE).odd(), even);
        }
        if x != T::MIN {
            assert_eq!((x - T::ONE).odd(), even);
            assert_eq!((-x).even(), even);
        }
    });
}

#[test]
fn even_properties() {
    apply_fn_to_unsigneds!(even_properties_helper_unsigned);
    apply_fn_to_signeds!(even_properties_helper_signed);
}

fn odd_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        let odd = x.odd();
        assert_ne!(x.divisible_by(T::TWO), odd);
        assert_eq!(!x.even(), odd);
        if x != T::MAX {
            assert_eq!((x + T::ONE).even(), odd);
        }
        if x != T::ZERO {
            assert_eq!((x - T::ONE).even(), odd);
        }
    });
}

fn odd_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        let odd = x.odd();
        assert_ne!(x.divisible_by(T::TWO), odd);
        assert_eq!(!x.even(), odd);
        if x != T::MAX {
            assert_eq!((x + T::ONE).even(), odd);
        }
        if x != T::MIN {
            assert_eq!((x - T::ONE).even(), odd);
            assert_eq!((-x).odd(), odd);
        }
    });
}

#[test]
fn odd_properties() {
    apply_fn_to_unsigneds!(odd_properties_helper_unsigned);
    apply_fn_to_signeds!(odd_properties_helper_signed);
}
