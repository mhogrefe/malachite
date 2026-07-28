// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_32, unsigned_gen_var_33, unsigned_gen_var_34,
};
use malachite_base::test_util::num::arithmetic::fibonacci::{
    checked_fibonacci_naive, checked_fibonacci_pair_naive, checked_lucas_number_naive,
    checked_lucas_number_pair_naive,
};
use std::panic::catch_unwind;

#[test]
fn test_fibonacci() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: T) {
        assert_eq!(T::fibonacci(n), out);
    }
    test::<u8>(0, 0);
    test::<u8>(1, 1);
    test::<u8>(2, 1);
    test::<u8>(3, 2);
    test::<u8>(4, 3);
    test::<u8>(5, 5);
    test::<u8>(10, 55);
    test::<u8>(13, 233);
    test::<u32>(30, 832040);
    test::<u64>(93, 12200160415121876738);
    test::<u128>(186, 332825110087067562321196029789634457848);
}

fn fibonacci_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::fibonacci(1000));
}

#[test]
fn fibonacci_fail() {
    apply_fn_to_unsigneds!(fibonacci_fail_helper);
}

#[test]
fn test_checked_fibonacci() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: Option<T>) {
        assert_eq!(T::checked_fibonacci(n), out);
        assert_eq!(checked_fibonacci_naive(n), out);
    }
    test::<u8>(0, Some(0));
    test::<u8>(1, Some(1));
    test::<u8>(10, Some(55));
    test::<u8>(13, Some(233));
    test::<u8>(14, None);
    test::<u16>(24, Some(46368));
    test::<u16>(25, None);
    test::<u32>(47, Some(2971215073));
    test::<u32>(48, None);
    test::<u64>(93, Some(12200160415121876738));
    test::<u64>(94, None);
    test::<u128>(186, Some(332825110087067562321196029789634457848));
    test::<u128>(187, None);
    test::<u32>(100, None);
}

#[test]
fn test_fibonacci_pair() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: (T, T)) {
        assert_eq!(T::fibonacci_pair(n), out);
    }
    test::<u8>(0, (0, 1));
    test::<u8>(1, (1, 0));
    test::<u8>(2, (1, 1));
    test::<u8>(3, (2, 1));
    test::<u8>(10, (55, 34));
    test::<u8>(13, (233, 144));
    test::<u32>(30, (832040, 514229));
    test::<u64>(93, (12200160415121876738, 7540113804746346429));
    test::<u128>(
        186,
        (
            332825110087067562321196029789634457848,
            205697230343233228174223751303346572685,
        ),
    );
}

fn fibonacci_pair_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::fibonacci_pair(1000));
}

#[test]
fn fibonacci_pair_fail() {
    apply_fn_to_unsigneds!(fibonacci_pair_fail_helper);
}

#[test]
fn test_checked_fibonacci_pair() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: Option<(T, T)>) {
        assert_eq!(T::checked_fibonacci_pair(n), out);
        assert_eq!(checked_fibonacci_pair_naive(n), out);
    }
    test::<u8>(0, Some((0, 1)));
    test::<u8>(1, Some((1, 0)));
    test::<u8>(13, Some((233, 144)));
    test::<u8>(14, None);
    test::<u64>(93, Some((12200160415121876738, 7540113804746346429)));
    test::<u64>(94, None);
}

#[test]
fn test_lucas_number() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: T) {
        assert_eq!(T::lucas_number(n), out);
    }
    test::<u8>(0, 2);
    test::<u8>(1, 1);
    test::<u8>(2, 3);
    test::<u8>(3, 4);
    test::<u8>(4, 7);
    test::<u8>(5, 11);
    test::<u8>(10, 123);
    test::<u8>(11, 199);
    test::<u32>(30, 1860498);
    test::<u64>(92, 16860207025497407047);
    test::<u128>(184, 284266580942632122201475224120405260207);
}

fn lucas_number_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::lucas_number(1000));
}

#[test]
fn lucas_number_fail() {
    apply_fn_to_unsigneds!(lucas_number_fail_helper);
}

#[test]
fn test_checked_lucas_number() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: Option<T>) {
        assert_eq!(T::checked_lucas_number(n), out);
        assert_eq!(checked_lucas_number_naive(n), out);
    }
    test::<u8>(0, Some(2));
    test::<u8>(10, Some(123));
    test::<u8>(11, Some(199));
    test::<u8>(12, None);
    test::<u16>(23, Some(64079));
    test::<u16>(24, None);
    test::<u32>(46, Some(4106118243));
    test::<u32>(47, None);
    test::<u64>(92, Some(16860207025497407047));
    test::<u64>(93, None);
    test::<u128>(184, Some(284266580942632122201475224120405260207));
    test::<u128>(185, None);
}

#[test]
fn test_lucas_number_pair() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: (T, T)) {
        assert_eq!(T::lucas_number_pair(n), out);
    }
    test::<u8>(1, (1, 2));
    test::<u8>(2, (3, 1));
    test::<u8>(3, (4, 3));
    test::<u8>(10, (123, 76));
    test::<u8>(11, (199, 123));
    test::<u32>(30, (1860498, 1149851));
    test::<u64>(92, (16860207025497407047, 10420180999117162549));
    test::<u128>(
        184,
        (
            284266580942632122201475224120405260207,
            175686408888269774266693084155517082804,
        ),
    );
}

fn lucas_number_pair_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::lucas_number_pair(1000));
    // L(-1) = -1 cannot be represented
    assert_panic!(T::lucas_number_pair(0));
}

#[test]
fn lucas_number_pair_fail() {
    apply_fn_to_unsigneds!(lucas_number_pair_fail_helper);
}

#[test]
fn test_checked_lucas_number_pair() {
    fn test<T: PrimitiveUnsigned>(n: u64, out: Option<(T, T)>) {
        assert_eq!(T::checked_lucas_number_pair(n), out);
        assert_eq!(checked_lucas_number_pair_naive(n), out);
    }
    test::<u8>(0, None);
    test::<u8>(1, Some((1, 2)));
    test::<u8>(11, Some((199, 123)));
    test::<u8>(12, None);
    test::<u64>(92, Some((16860207025497407047, 10420180999117162549)));
    test::<u64>(93, None);
}

fn fibonacci_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_32::<T>().test_properties(|n| {
        let f = T::fibonacci(n);
        assert_eq!(T::checked_fibonacci(n), Some(f));
        assert_eq!(checked_fibonacci_naive(n), Some(f));
        let (f_n, f_n_minus_1) = T::fibonacci_pair(n);
        assert_eq!(f_n, f);
        if n != 0 {
            assert_eq!(T::fibonacci(n - 1), f_n_minus_1);
        }
        if n >= 2 {
            assert_eq!(f, T::fibonacci(n - 1) + T::fibonacci(n - 2));
        }
    });
}

#[test]
fn fibonacci_properties() {
    apply_fn_to_unsigneds!(fibonacci_properties_helper);
}

fn checked_fibonacci_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen().test_properties(|n| {
        let f = T::checked_fibonacci(n);
        assert_eq!(checked_fibonacci_naive::<T>(n), f);
        assert_eq!(
            T::checked_fibonacci_pair(n),
            checked_fibonacci_pair_naive::<T>(n)
        );
        // If F(n) is representable, so is the pair, since F(n - 1) <= F(n) for n >= 1.
        if f.is_some() {
            assert!(T::checked_fibonacci_pair(n).is_some());
        }
    });
}

#[test]
fn checked_fibonacci_properties() {
    apply_fn_to_unsigneds!(checked_fibonacci_properties_helper);
}

fn lucas_number_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_33::<T>().test_properties(|n| {
        let l = T::lucas_number(n);
        assert_eq!(T::checked_lucas_number(n), Some(l));
        assert_eq!(checked_lucas_number_naive(n), Some(l));
        // L(n) = F(n) + 2F(n - 1), using F(-1) = 1 for n = 0. The Fibonacci pair is always
        // representable when L(n) is, since F(n) <= L(n) and F(n - 1) < L(n).
        let (f_n, f_n_minus_1) = T::fibonacci_pair(n);
        assert_eq!(l, f_n + f_n_minus_1 + f_n_minus_1);
        if n >= 2 {
            assert_eq!(l, T::lucas_number(n - 1) + T::lucas_number(n - 2));
        }
    });

    unsigned_gen_var_34::<T>().test_properties(|n| {
        let (l_n, l_n_minus_1) = T::lucas_number_pair(n);
        assert_eq!(l_n, T::lucas_number(n));
        assert_eq!(l_n_minus_1, T::lucas_number(n - 1));
    });
}

#[test]
fn lucas_number_properties() {
    apply_fn_to_unsigneds!(lucas_number_properties_helper);
}

fn checked_lucas_number_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen().test_properties(|n| {
        assert_eq!(
            T::checked_lucas_number(n),
            checked_lucas_number_naive::<T>(n)
        );
        assert_eq!(
            T::checked_lucas_number_pair(n),
            checked_lucas_number_pair_naive::<T>(n)
        );
    });
}

#[test]
fn checked_lucas_number_properties() {
    apply_fn_to_unsigneds!(checked_lucas_number_properties_helper);
}
