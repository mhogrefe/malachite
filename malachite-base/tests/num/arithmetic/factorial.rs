// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::factorial::checked_multifactorial_naive;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_23, unsigned_gen_var_24, unsigned_gen_var_25,
    unsigned_pair_gen_var_12, unsigned_pair_gen_var_43,
};
use malachite_base::test_util::num::arithmetic::factorial::{
    checked_double_factorial_naive, checked_factorial_naive, checked_subfactorial_naive,
};
use std::panic::catch_unwind;

#[test]
fn test_factorial() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: T) {
        assert_eq!(T::factorial(n), out);
    }
    test::<u8>(0, 1);
    test::<u8>(1, 1);
    test::<u8>(2, 2);
    test::<u8>(3, 6);
    test::<u8>(4, 24);
    test::<u8>(5, 120);
    test::<u32>(10, 3628800);
}

fn factorial_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::factorial(100));
}

#[test]
fn factorial_fail() {
    apply_fn_to_unsigneds!(factorial_fail_helper);
}

#[test]
fn test_checked_factorial() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: Option<T>) {
        assert_eq!(T::checked_factorial(n), out);
        assert_eq!(checked_factorial_naive(n), out);
    }
    test::<u8>(0, Some(1));
    test::<u8>(1, Some(1));
    test::<u8>(2, Some(2));
    test::<u8>(3, Some(6));
    test::<u8>(4, Some(24));
    test::<u8>(5, Some(120));
    test::<u32>(10, Some(3628800));

    test::<u8>(6, None);
    test::<u32>(100, None);
}

#[test]
fn test_double_factorial() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: T) {
        assert_eq!(T::double_factorial(n), out);
    }
    test::<u8>(0, 1);
    test::<u8>(1, 1);
    test::<u8>(2, 2);
    test::<u8>(3, 3);
    test::<u8>(4, 8);
    test::<u8>(5, 15);
    test::<u8>(6, 48);
    test::<u8>(7, 105);
    test::<u32>(19, 654729075);
    test::<u32>(20, 3715891200);
}

fn double_factorial_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::double_factorial(100));
}

#[test]
fn double_factorial_fail() {
    apply_fn_to_unsigneds!(double_factorial_fail_helper);
}

#[test]
fn test_checked_double_factorial() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: Option<T>) {
        assert_eq!(T::checked_double_factorial(n), out);
        assert_eq!(checked_double_factorial_naive(n), out);
    }
    test::<u8>(0, Some(1));
    test::<u8>(1, Some(1));
    test::<u8>(2, Some(2));
    test::<u8>(3, Some(3));
    test::<u8>(4, Some(8));
    test::<u8>(5, Some(15));
    test::<u8>(6, Some(48));
    test::<u8>(7, Some(105));
    test::<u32>(19, Some(654729075));
    test::<u32>(20, Some(3715891200));

    test::<u8>(8, None);
    test::<u32>(100, None);
}

#[test]
fn test_multifactorial() {
    fn test<T: PrimitiveUnsigned>(n: u64, m: u64, out: T) {
        assert_eq!(T::multifactorial(n, m), out);
    }
    test::<u8>(0, 1, 1);
    test::<u8>(1, 1, 1);
    test::<u8>(2, 1, 2);
    test::<u8>(3, 1, 6);
    test::<u8>(4, 1, 24);
    test::<u8>(5, 1, 120);

    test::<u8>(0, 2, 1);
    test::<u8>(1, 2, 1);
    test::<u8>(2, 2, 2);
    test::<u8>(3, 2, 3);
    test::<u8>(4, 2, 8);
    test::<u8>(5, 2, 15);
    test::<u8>(6, 2, 48);
    test::<u8>(7, 2, 105);

    test::<u8>(0, 3, 1);
    test::<u8>(1, 3, 1);
    test::<u8>(2, 3, 2);
    test::<u8>(3, 3, 3);
    test::<u8>(4, 3, 4);
    test::<u8>(5, 3, 10);
    test::<u8>(6, 3, 18);
    test::<u8>(7, 3, 28);
    test::<u8>(8, 3, 80);
    test::<u8>(9, 3, 162);

    test::<u32>(10, 1, 3628800);
    test::<u32>(20, 2, 3715891200);
    test::<u32>(25, 3, 608608000);
}

fn multifactorial_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::multifactorial(1, 0));
    assert_panic!(T::multifactorial(100, 1));
}

#[test]
fn multifactorial_fail() {
    apply_fn_to_unsigneds!(multifactorial_fail_helper);
}

#[test]
fn test_checked_multifactorial() {
    fn test<T: PrimitiveUnsigned>(n: u64, m: u64, out: Option<T>) {
        assert_eq!(T::checked_multifactorial(n, m), out);
        assert_eq!(checked_multifactorial_naive(n, m), out);
    }
    test::<u8>(0, 1, Some(1));
    test::<u8>(1, 1, Some(1));
    test::<u8>(2, 1, Some(2));
    test::<u8>(3, 1, Some(6));
    test::<u8>(4, 1, Some(24));
    test::<u8>(5, 1, Some(120));

    test::<u8>(0, 2, Some(1));
    test::<u8>(1, 2, Some(1));
    test::<u8>(2, 2, Some(2));
    test::<u8>(3, 2, Some(3));
    test::<u8>(4, 2, Some(8));
    test::<u8>(5, 2, Some(15));
    test::<u8>(6, 2, Some(48));
    test::<u8>(7, 2, Some(105));

    test::<u8>(0, 3, Some(1));
    test::<u8>(1, 3, Some(1));
    test::<u8>(2, 3, Some(2));
    test::<u8>(3, 3, Some(3));
    test::<u8>(4, 3, Some(4));
    test::<u8>(5, 3, Some(10));
    test::<u8>(6, 3, Some(18));
    test::<u8>(7, 3, Some(28));
    test::<u8>(8, 3, Some(80));
    test::<u8>(9, 3, Some(162));

    test::<u32>(10, 1, Some(3628800));
    test::<u32>(20, 2, Some(3715891200));
    test::<u32>(25, 3, Some(608608000));

    test::<u8>(6, 1, None);
    test::<u8>(8, 2, None);
    test::<u8>(10, 3, None);
    test::<u32>(100, 1, None);
    test::<u32>(100, 2, None);
    test::<u32>(100, 3, None);
}

fn checked_multifactorial_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::checked_multifactorial(1, 0));
}

#[test]
fn checked_multifactorial_fail() {
    apply_fn_to_unsigneds!(checked_multifactorial_fail_helper);
}

#[test]
fn test_subfactorial() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: T) {
        assert_eq!(T::subfactorial(n), out);
    }
    test::<u8>(0, 1);
    test::<u8>(1, 0);
    test::<u8>(2, 1);
    test::<u8>(3, 2);
    test::<u8>(4, 9);
    test::<u8>(5, 44);
    test::<u32>(10, 1334961);
}

fn subfactorial_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::subfactorial(100));
}

#[test]
fn subfactorial_fail() {
    apply_fn_to_unsigneds!(subfactorial_fail_helper);
}

#[test]
fn test_checked_subfactorial() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: Option<T>) {
        assert_eq!(T::checked_subfactorial(n), out);
        assert_eq!(checked_subfactorial_naive(n), out);
    }
    test::<u8>(0, Some(1));
    test::<u8>(1, Some(0));
    test::<u8>(2, Some(1));
    test::<u8>(3, Some(2));
    test::<u8>(4, Some(9));
    test::<u8>(5, Some(44));
    test::<u32>(10, Some(1334961));

    test::<u8>(6, None);
    test::<u32>(100, None);
}

fn factorial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_23::<T>().test_properties(|n| {
        let f = T::factorial(n);
        assert_eq!(T::checked_factorial(n), Some(f));
        assert_eq!(T::multifactorial(n, 1), f);
        assert_ne!(f, T::ZERO);
        if n != 0 {
            assert_eq!(f / T::factorial(n - 1), T::exact_from(n));
        }
    });
}

#[test]
fn factorial_properties() {
    apply_fn_to_unsigneds!(factorial_properties_helper);
}

fn checked_factorial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen().test_properties(|n| {
        let of = T::checked_factorial(n);
        assert_eq!(checked_factorial_naive(n), of);
        assert_eq!(T::checked_multifactorial(n, 1), of);
        assert_ne!(of, Some(T::ZERO));
        if let Some(f) = of {
            assert_eq!(T::factorial(n), f);
        }
        if n != u64::MAX && of.is_none() {
            assert!(T::checked_factorial(n + 1).is_none());
        }
    });
}

#[test]
fn checked_factorial_properties() {
    apply_fn_to_unsigneds!(checked_factorial_properties_helper);
}

fn double_factorial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_24::<T>().test_properties(|n| {
        let f = T::double_factorial(n);
        assert_eq!(T::checked_double_factorial(n), Some(f));
        assert_eq!(T::multifactorial(n, 2), f);
        assert_ne!(f, T::ZERO);
        if n > 1 {
            assert_eq!(f / T::double_factorial(n - 2), T::exact_from(n));
        }
    });
}

#[test]
fn double_factorial_properties() {
    apply_fn_to_unsigneds!(double_factorial_properties_helper);
}

fn checked_double_factorial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen().test_properties(|n| {
        let of = T::checked_double_factorial(n);
        assert_eq!(checked_double_factorial_naive(n), of);
        assert_eq!(T::checked_multifactorial(n, 2), of);
        assert_ne!(of, Some(T::ZERO));
        if let Some(f) = of {
            assert_eq!(T::double_factorial(n), f);
        }
        if n != u64::MAX && of.is_none() {
            assert!(T::checked_double_factorial(n + 1).is_none());
        }
    });
}

#[test]
fn checked_double_factorial_properties() {
    apply_fn_to_unsigneds!(checked_double_factorial_properties_helper);
}

fn multifactorial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_43::<T>().test_properties(|(n, m)| {
        let f = T::multifactorial(n, m);
        assert_eq!(T::checked_multifactorial(n, m), Some(f));
        assert_ne!(f, T::ZERO);
        if n >= m {
            assert_eq!(f / T::multifactorial(n - m, m), T::exact_from(n));
        }
    });
}

#[test]
fn multifactorial_properties() {
    apply_fn_to_unsigneds!(multifactorial_properties_helper);
}

fn checked_multifactorial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_12::<u64, u64>().test_properties(|(n, m)| {
        let of = T::checked_multifactorial(n, m);
        assert_eq!(checked_multifactorial_naive(n, m), of);
        assert_ne!(of, Some(T::ZERO));
        if let Some(f) = of {
            assert_eq!(T::multifactorial(n, m), f);
        }
        if n != u64::MAX && of.is_none() {
            assert!(T::checked_multifactorial(n + 1, m).is_none());
        }
    });
}

#[test]
fn checked_multifactorial_properties() {
    apply_fn_to_unsigneds!(checked_multifactorial_properties_helper);
}

fn subfactorial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_25::<T>().test_properties(|n| {
        let f = T::subfactorial(n);
        assert_eq!(T::checked_subfactorial(n), Some(f));
        if n != 1 {
            assert_ne!(f, T::ZERO);
        }
        if n != 0 && n != 2 {
            let g = if n.even() { f - T::ONE } else { f + T::ONE };
            assert_eq!(g / T::subfactorial(n - 1), T::exact_from(n));
        }
    });
}

#[test]
fn subfactorial_properties() {
    apply_fn_to_unsigneds!(subfactorial_properties_helper);
}

fn checked_subfactorial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen().test_properties(|n| {
        let of = T::checked_subfactorial(n);
        assert_eq!(checked_subfactorial_naive(n), of);
        if n != 1 {
            assert_ne!(of, Some(T::ZERO));
        }
        if let Some(f) = of {
            assert_eq!(T::subfactorial(n), f);
        }
        if n != u64::MAX && of.is_none() {
            assert!(T::checked_subfactorial(n + 1).is_none());
        }
    });
}

#[test]
fn checked_subfactorial_properties() {
    apply_fn_to_unsigneds!(checked_subfactorial_properties_helper);
}
