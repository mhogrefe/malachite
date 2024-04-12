// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_pair_gen_var_33, unsigned_pair_gen_var_34, unsigned_triple_gen_var_19,
};
use std::panic::catch_unwind;

#[test]
fn test_lcm() {
    fn test<T: PrimitiveUnsigned>(x: T, y: T, out: T) {
        assert_eq!(x.lcm(y), out);

        let mut x = x;
        x.lcm_assign(y);
        assert_eq!(x, out);
    }
    test::<u8>(0, 0, 0);
    test::<u16>(0, 6, 0);
    test::<u32>(6, 0, 0);
    test::<u64>(1, 6, 6);
    test::<u128>(6, 1, 6);
    test::<usize>(8, 12, 24);
    test::<u8>(54, 24, 216);
    test::<u16>(42, 56, 168);
    test::<u32>(48, 18, 144);
    test::<u64>(3, 5, 15);
    test::<u128>(12, 60, 60);
    test::<usize>(12, 90, 180);
}

fn lcm_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::MAX.lcm(T::TWO));
    assert_panic!({
        let mut x = T::MAX;
        x.lcm_assign(T::TWO);
    });
}

#[test]
fn lcm_fail() {
    apply_fn_to_unsigneds!(lcm_fail_helper);
}

#[test]
fn test_checked_lcm() {
    fn test<T: PrimitiveUnsigned>(x: T, y: T, out: Option<T>) {
        assert_eq!(x.checked_lcm(y), out);
    }
    test::<u8>(0, 0, Some(0));
    test::<u16>(0, 6, Some(0));
    test::<u32>(6, 0, Some(0));
    test::<u64>(1, 6, Some(6));
    test::<u128>(6, 1, Some(6));
    test::<usize>(8, 12, Some(24));
    test::<u8>(54, 24, Some(216));
    test::<u16>(42, 56, Some(168));
    test::<u32>(48, 18, Some(144));
    test::<u64>(3, 5, Some(15));
    test::<u128>(12, 60, Some(60));
    test::<usize>(12, 90, Some(180));
    test::<usize>(usize::MAX, 2, None);
}

fn lcm_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_34::<T>().test_properties(|(x, y)| {
        let lcm = x.lcm(y);

        let mut x_mut = x;
        x_mut.lcm_assign(y);
        assert_eq!(x_mut, lcm);

        assert_eq!(y.lcm(x), lcm);
        assert!(lcm.divisible_by(x));
        assert!(lcm.divisible_by(y));
        let gcd = x.gcd(y);
        if x != T::ZERO {
            assert_eq!(lcm / x * gcd, y);
        }
        if y != T::ZERO {
            assert_eq!(lcm / y * gcd, x);
        }
        if gcd != T::ZERO {
            assert_eq!(x / gcd * y, lcm);
        }
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.lcm(x), x);
        assert_eq!(x.lcm(T::ONE), x);
        assert_eq!(x.lcm(T::ZERO), T::ZERO);
    });
}

#[test]
fn lcm_properties() {
    apply_fn_to_unsigneds!(lcm_properties_helper);
}

fn checked_lcm_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_33::<T>().test_properties(|(x, y)| {
        let lcm = x.checked_lcm(y);
        assert_eq!(y.checked_lcm(x), lcm);
        if let Some(lcm) = lcm {
            assert_eq!(x.lcm(y), lcm);
        }
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.checked_lcm(x), Some(x));
        assert_eq!(x.checked_lcm(T::ONE), Some(x));
        assert_eq!(x.checked_lcm(T::ZERO), Some(T::ZERO));
    });

    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        if x != T::ZERO && y != T::ZERO && z != T::ZERO {
            assert_eq!(
                x.checked_lcm(y).and_then(|n| n.checked_lcm(z)),
                y.checked_lcm(z).and_then(|n| x.checked_lcm(n))
            );
        }
    });
}

#[test]
fn checked_lcm_properties() {
    apply_fn_to_unsigneds!(checked_lcm_properties_helper);
}
