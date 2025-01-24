// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::Primes;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_27, unsigned_gen_var_28,
};
use malachite_base::test_util::num::arithmetic::primorial::{
    checked_primorial_naive, checked_product_of_first_n_primes_naive,
};
use std::panic::catch_unwind;

#[test]
fn test_primorial() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: T) {
        assert_eq!(T::primorial(n), out);
    }
    test::<u8>(0, 1);
    test::<u8>(1, 1);
    test::<u8>(2, 2);
    test::<u8>(3, 6);
    test::<u8>(4, 6);
    test::<u8>(5, 30);
    test::<u32>(20, 9699690);
}

fn primorial_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::primorial(200));
}

#[test]
fn primorial_fail() {
    apply_fn_to_unsigneds!(primorial_fail_helper);
}

#[test]
fn test_checked_primorial() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: Option<T>) {
        assert_eq!(T::checked_primorial(n), out);
        assert_eq!(checked_primorial_naive(n), out);
    }
    test::<u8>(0, Some(1));
    test::<u8>(1, Some(1));
    test::<u8>(2, Some(2));
    test::<u8>(3, Some(6));
    test::<u8>(4, Some(6));
    test::<u8>(5, Some(30));
    test::<u32>(20, Some(9699690));

    test::<u8>(11, None);
    test::<u32>(200, None);
}

#[test]
fn test_product_of_first_n_primes() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: T) {
        assert_eq!(T::product_of_first_n_primes(n), out);
    }
    test::<u8>(0, 1);
    test::<u8>(1, 2);
    test::<u8>(2, 6);
    test::<u8>(3, 30);
    test::<u8>(4, 210);
    test::<u32>(9, 223092870);
}

fn product_of_first_n_primes_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::product_of_first_n_primes(100));
}

#[test]
fn product_of_first_n_primes_fail() {
    apply_fn_to_unsigneds!(product_of_first_n_primes_fail_helper);
}

#[test]
fn test_checked_product_of_first_n_primes() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: Option<T>) {
        assert_eq!(T::checked_product_of_first_n_primes(n), out);
        assert_eq!(checked_product_of_first_n_primes_naive(n), out);
    }
    test::<u8>(0, Some(1));
    test::<u8>(1, Some(2));
    test::<u8>(2, Some(6));
    test::<u8>(3, Some(30));
    test::<u8>(4, Some(210));
    test::<u32>(9, Some(223092870));

    test::<u8>(5, None);
    test::<u32>(100, None);
}

fn primorial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_27::<T>().test_properties(|n| {
        let f = T::primorial(n);
        assert_eq!(T::checked_primorial(n), Some(f));
        assert_ne!(f, T::ZERO);
        // TODO compare with primorial(n - 1) depending on whether n is prime
    });
}

#[test]
fn primorial_properties() {
    apply_fn_to_unsigneds!(primorial_properties_helper);
}

fn checked_primorial_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen().test_properties(|n| {
        let of = T::checked_primorial(n);
        assert_eq!(checked_primorial_naive(n), of);
        assert_ne!(of, Some(T::ZERO));
        if let Some(f) = of {
            assert_eq!(T::primorial(n), f);
        }
        if n != u64::MAX && of.is_none() {
            assert!(T::checked_primorial(n + 1).is_none());
        }
    });
}

#[test]
fn checked_primorial_properties() {
    apply_fn_to_unsigneds!(checked_primorial_properties_helper);
}

fn product_of_first_n_primes_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_28::<T>().test_properties(|n| {
        let f = T::product_of_first_n_primes(n);
        assert_eq!(T::checked_product_of_first_n_primes(n), Some(f));
        assert_ne!(f, T::ZERO);
        if n != 0 {
            let p = u64::primes().nth(usize::exact_from(n) - 1).unwrap();
            assert_eq!(T::primorial(p), f);
            assert_eq!(f / T::product_of_first_n_primes(n - 1), T::exact_from(p));
        }
    });
}

#[test]
fn product_of_first_n_primes_properties() {
    apply_fn_to_unsigneds!(product_of_first_n_primes_properties_helper);
}

fn checked_product_of_first_n_primes_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen().test_properties(|n| {
        let of = T::checked_product_of_first_n_primes(n);
        assert_eq!(checked_product_of_first_n_primes_naive(n), of);
        assert_ne!(of, Some(T::ZERO));
        if let Some(f) = of {
            assert_eq!(T::product_of_first_n_primes(n), f);
        }
        if n != u64::MAX && of.is_none() {
            assert!(T::checked_product_of_first_n_primes(n + 1).is_none());
        }
    });
}

#[test]
fn checked_product_of_first_n_primes_properties() {
    apply_fn_to_unsigneds!(checked_product_of_first_n_primes_properties_helper);
}
