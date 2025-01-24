// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::root::{
    cbrt_chebyshev_approx_u32, cbrt_chebyshev_approx_u64, fast_ceiling_root_u32,
    fast_ceiling_root_u64, fast_checked_root_u32, fast_checked_root_u64, fast_floor_cbrt_u32,
    fast_floor_cbrt_u64, fast_floor_root_u32, fast_floor_root_u64, fast_root_rem_u32,
    fast_root_rem_u64, floor_root_approx_and_refine,
};
use malachite_base::num::arithmetic::root::{
    ceiling_root_binary, checked_root_binary, floor_root_binary, root_rem_binary,
};
use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CheckedRoot, FloorRoot, Parity, RootRem,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_2, signed_unsigned_pair_gen_var_18, unsigned_gen,
    unsigned_gen_var_1, unsigned_pair_gen_var_32,
};
use std::ops::Neg;
use std::panic::catch_unwind;

#[test]
fn test_floor_root() {
    fn test_u<T: PrimitiveUnsigned>(n: T, exp: u64, out: T) {
        assert_eq!(n.floor_root(exp), out);
        assert_eq!(floor_root_binary(n, exp), out);

        let mut n = n;
        n.floor_root_assign(exp);
        assert_eq!(n, out);
    }
    test_u::<u8>(0, 1, 0);
    test_u::<u8>(1, 1, 1);
    test_u::<u8>(2, 1, 2);
    test_u::<u8>(100, 1, 100);

    test_u::<u8>(0, 2, 0);
    test_u::<u8>(1, 2, 1);
    test_u::<u8>(2, 2, 1);
    test_u::<u8>(3, 2, 1);
    test_u::<u8>(4, 2, 2);
    test_u::<u8>(5, 2, 2);
    test_u::<u8>(0, 3, 0);
    test_u::<u8>(1, 3, 1);
    test_u::<u8>(2, 3, 1);
    test_u::<u8>(7, 3, 1);
    test_u::<u8>(8, 3, 2);
    test_u::<u8>(9, 3, 2);
    test_u::<u8>(10, 2, 3);
    test_u::<u8>(100, 2, 10);
    test_u::<u8>(100, 3, 4);
    test_u::<u32>(1000000000, 2, 31622);
    test_u::<u32>(1000000000, 3, 1000);
    test_u::<u32>(1000000000, 4, 177);
    test_u::<u32>(1000000000, 5, 63);
    test_u::<u32>(1000000000, 6, 31);
    test_u::<u32>(1000000000, 7, 19);
    test_u::<u32>(1000000000, 8, 13);
    test_u::<u32>(1000000000, 9, 10);
    test_u::<u32>(1000000000, 10, 7);

    fn test_i<T: PrimitiveSigned>(n: T, exp: u64, out: T) {
        assert_eq!(n.floor_root(exp), out);

        let mut n = n;
        n.floor_root_assign(exp);
        assert_eq!(n, out);
    }
    test_i::<i8>(0, 1, 0);
    test_i::<i8>(1, 1, 1);
    test_i::<i8>(2, 1, 2);
    test_i::<i8>(100, 1, 100);

    test_i::<i8>(0, 2, 0);
    test_i::<i8>(0, 2, 0);
    test_i::<i8>(1, 2, 1);
    test_i::<i8>(2, 2, 1);
    test_i::<i8>(3, 2, 1);
    test_i::<i8>(4, 2, 2);
    test_i::<i8>(5, 2, 2);
    test_i::<i8>(0, 3, 0);
    test_i::<i8>(1, 3, 1);
    test_i::<i8>(2, 3, 1);
    test_i::<i8>(7, 3, 1);
    test_i::<i8>(8, 3, 2);
    test_i::<i8>(9, 3, 2);
    test_i::<i8>(10, 2, 3);
    test_i::<i8>(100, 2, 10);
    test_i::<i8>(100, 3, 4);
    test_i::<i32>(1000000000, 2, 31622);
    test_i::<i32>(1000000000, 3, 1000);
    test_i::<i32>(1000000000, 4, 177);
    test_i::<i32>(1000000000, 5, 63);
    test_i::<i32>(1000000000, 6, 31);
    test_i::<i32>(1000000000, 7, 19);
    test_i::<i32>(1000000000, 8, 13);
    test_i::<i32>(1000000000, 9, 10);
    test_i::<i32>(1000000000, 10, 7);

    test_i::<i8>(-1, 1, -1);
    test_i::<i8>(-2, 1, -2);
    test_i::<i8>(-100, 1, -100);

    test_i::<i8>(-1, 3, -1);
    test_i::<i8>(-2, 3, -2);
    test_i::<i8>(-7, 3, -2);
    test_i::<i8>(-8, 3, -2);
    test_i::<i8>(-9, 3, -3);
    test_i::<i8>(-100, 3, -5);
    test_i::<i32>(-1000000000, 3, -1000);
    test_i::<i32>(-1000000000, 5, -64);
    test_i::<i32>(-1000000000, 7, -20);
    test_i::<i32>(-1000000000, 9, -10);
}

fn floor_root_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.floor_root(0));
}

fn floor_root_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::ONE.floor_root(0));
    assert_panic!(T::NEGATIVE_ONE.floor_root(0));
    assert_panic!(T::NEGATIVE_ONE.floor_root(2));
    assert_panic!(T::NEGATIVE_ONE.floor_root(4));
    assert_panic!(T::NEGATIVE_ONE.floor_root(100));
}

#[test]
pub fn floor_root_fail() {
    apply_fn_to_unsigneds!(floor_root_fail_helper_unsigned);
    apply_fn_to_signeds!(floor_root_fail_helper_signed);
}

#[test]
fn test_ceiling_root() {
    fn test_u<T: PrimitiveUnsigned>(n: T, exp: u64, out: T) {
        assert_eq!(n.ceiling_root(exp), out);
        assert_eq!(ceiling_root_binary(n, exp), out);

        let mut n = n;
        n.ceiling_root_assign(exp);
        assert_eq!(n, out);
    }
    test_u::<u8>(0, 1, 0);
    test_u::<u8>(1, 1, 1);
    test_u::<u8>(2, 1, 2);
    test_u::<u8>(100, 1, 100);

    test_u::<u8>(0, 2, 0);
    test_u::<u8>(1, 2, 1);
    test_u::<u8>(2, 2, 2);
    test_u::<u8>(3, 2, 2);
    test_u::<u8>(4, 2, 2);
    test_u::<u8>(5, 2, 3);
    test_u::<u8>(0, 3, 0);
    test_u::<u8>(1, 3, 1);
    test_u::<u8>(2, 3, 2);
    test_u::<u8>(7, 3, 2);
    test_u::<u8>(8, 3, 2);
    test_u::<u8>(9, 3, 3);
    test_u::<u8>(10, 2, 4);
    test_u::<u8>(100, 2, 10);
    test_u::<u8>(100, 3, 5);
    test_u::<u32>(1000000000, 2, 31623);
    test_u::<u32>(1000000000, 3, 1000);
    test_u::<u32>(1000000000, 4, 178);
    test_u::<u32>(1000000000, 5, 64);
    test_u::<u32>(1000000000, 6, 32);
    test_u::<u32>(1000000000, 7, 20);
    test_u::<u32>(1000000000, 8, 14);
    test_u::<u32>(1000000000, 9, 10);
    test_u::<u32>(1000000000, 10, 8);

    fn test_i<T: PrimitiveSigned>(n: T, exp: u64, out: T) {
        assert_eq!(n.ceiling_root(exp), out);

        let mut n = n;
        n.ceiling_root_assign(exp);
        assert_eq!(n, out);
    }
    test_i::<i8>(0, 1, 0);
    test_i::<i8>(1, 1, 1);
    test_i::<i8>(2, 1, 2);
    test_i::<i8>(100, 1, 100);

    test_i::<i8>(0, 2, 0);
    test_i::<i8>(1, 2, 1);
    test_i::<i8>(2, 2, 2);
    test_i::<i8>(3, 2, 2);
    test_i::<i8>(4, 2, 2);
    test_i::<i8>(5, 2, 3);
    test_i::<i8>(0, 3, 0);
    test_i::<i8>(1, 3, 1);
    test_i::<i8>(2, 3, 2);
    test_i::<i8>(7, 3, 2);
    test_i::<i8>(8, 3, 2);
    test_i::<i8>(9, 3, 3);
    test_i::<i8>(10, 2, 4);
    test_i::<i8>(100, 2, 10);
    test_i::<i8>(100, 3, 5);
    test_i::<i32>(1000000000, 2, 31623);
    test_i::<i32>(1000000000, 3, 1000);
    test_i::<i32>(1000000000, 4, 178);
    test_i::<i32>(1000000000, 5, 64);
    test_i::<i32>(1000000000, 6, 32);
    test_i::<i32>(1000000000, 7, 20);
    test_i::<i32>(1000000000, 8, 14);
    test_i::<i32>(1000000000, 9, 10);
    test_i::<i32>(1000000000, 10, 8);

    test_i::<i8>(-1, 1, -1);
    test_i::<i8>(-2, 1, -2);
    test_i::<i8>(-100, 1, -100);

    test_i::<i8>(-1, 3, -1);
    test_i::<i8>(-2, 3, -1);
    test_i::<i8>(-7, 3, -1);
    test_i::<i8>(-8, 3, -2);
    test_i::<i8>(-9, 3, -2);
    test_i::<i8>(-100, 3, -4);
    test_i::<i32>(-1000000000, 3, -1000);
    test_i::<i32>(-1000000000, 5, -63);
    test_i::<i32>(-1000000000, 7, -19);
    test_i::<i32>(-1000000000, 9, -10);
}

fn ceiling_root_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.ceiling_root(0));
}

fn ceiling_root_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::ONE.ceiling_root(0));
    assert_panic!(T::NEGATIVE_ONE.ceiling_root(0));
    assert_panic!(T::NEGATIVE_ONE.ceiling_root(2));
    assert_panic!(T::NEGATIVE_ONE.ceiling_root(4));
    assert_panic!(T::NEGATIVE_ONE.ceiling_root(100));
}

#[test]
pub fn ceiling_root_fail() {
    apply_fn_to_unsigneds!(ceiling_root_fail_helper_unsigned);
    apply_fn_to_signeds!(ceiling_root_fail_helper_signed);
}

#[test]
fn test_checked_root() {
    fn test_u<T: PrimitiveUnsigned>(n: T, exp: u64, out: Option<T>) {
        assert_eq!(n.checked_root(exp), out);
        assert_eq!(checked_root_binary(n, exp), out);
    }
    test_u::<u8>(0, 1, Some(0));
    test_u::<u8>(1, 1, Some(1));
    test_u::<u8>(2, 1, Some(2));
    test_u::<u8>(100, 1, Some(100));

    test_u::<u8>(0, 2, Some(0));
    test_u::<u8>(1, 2, Some(1));
    test_u::<u8>(2, 2, None);
    test_u::<u8>(3, 2, None);
    test_u::<u8>(4, 2, Some(2));
    test_u::<u8>(5, 2, None);
    test_u::<u8>(0, 3, Some(0));
    test_u::<u8>(1, 3, Some(1));
    test_u::<u8>(2, 3, None);
    test_u::<u8>(7, 3, None);
    test_u::<u8>(8, 3, Some(2));
    test_u::<u8>(9, 3, None);
    test_u::<u8>(10, 2, None);
    test_u::<u8>(100, 2, Some(10));
    test_u::<u8>(100, 3, None);
    test_u::<u32>(1000000000, 2, None);
    test_u::<u32>(1000000000, 3, Some(1000));
    test_u::<u32>(1000000000, 4, None);
    test_u::<u32>(1000000000, 5, None);
    test_u::<u32>(1000000000, 6, None);
    test_u::<u32>(1000000000, 7, None);
    test_u::<u32>(1000000000, 8, None);
    test_u::<u32>(1000000000, 9, Some(10));
    test_u::<u32>(1000000000, 10, None);

    fn test_i<T: PrimitiveSigned>(n: T, exp: u64, out: Option<T>) {
        assert_eq!(n.checked_root(exp), out);
    }
    test_i::<i8>(0, 1, Some(0));
    test_i::<i8>(1, 1, Some(1));
    test_i::<i8>(2, 1, Some(2));
    test_i::<i8>(100, 1, Some(100));

    test_i::<i8>(0, 2, Some(0));
    test_i::<i8>(1, 2, Some(1));
    test_i::<i8>(2, 2, None);
    test_i::<i8>(3, 2, None);
    test_i::<i8>(4, 2, Some(2));
    test_i::<i8>(5, 2, None);
    test_i::<i8>(0, 3, Some(0));
    test_i::<i8>(1, 3, Some(1));
    test_i::<i8>(2, 3, None);
    test_i::<i8>(7, 3, None);
    test_i::<i8>(8, 3, Some(2));
    test_i::<i8>(9, 3, None);
    test_i::<i8>(10, 2, None);
    test_i::<i8>(100, 2, Some(10));
    test_i::<i8>(100, 3, None);
    test_i::<i32>(1000000000, 2, None);
    test_i::<i32>(1000000000, 3, Some(1000));
    test_i::<i32>(1000000000, 4, None);
    test_i::<i32>(1000000000, 5, None);
    test_i::<i32>(1000000000, 6, None);
    test_i::<i32>(1000000000, 7, None);
    test_i::<i32>(1000000000, 8, None);
    test_i::<i32>(1000000000, 9, Some(10));
    test_i::<i32>(1000000000, 10, None);

    test_i::<i8>(-1, 1, Some(-1));
    test_i::<i8>(-2, 1, Some(-2));
    test_i::<i8>(-100, 1, Some(-100));

    test_i::<i8>(-1, 3, Some(-1));
    test_i::<i8>(-2, 3, None);
    test_i::<i8>(-7, 3, None);
    test_i::<i8>(-8, 3, Some(-2));
    test_i::<i8>(-9, 3, None);
    test_i::<i8>(-100, 3, None);
    test_i::<i32>(-1000000000, 3, Some(-1000));
    test_i::<i32>(-1000000000, 5, None);
    test_i::<i32>(-1000000000, 7, None);
    test_i::<i32>(-1000000000, 9, Some(-10));
}

fn checked_root_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.checked_root(0));
}

fn checked_root_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::ONE.checked_root(0));
    assert_panic!(T::NEGATIVE_ONE.checked_root(0));
    assert_panic!(T::NEGATIVE_ONE.checked_root(2));
    assert_panic!(T::NEGATIVE_ONE.checked_root(4));
    assert_panic!(T::NEGATIVE_ONE.checked_root(100));
}

#[test]
pub fn checked_root_fail() {
    apply_fn_to_unsigneds!(checked_root_fail_helper_unsigned);
    apply_fn_to_signeds!(checked_root_fail_helper_signed);
}

#[test]
fn test_root_rem() {
    fn test<T: PrimitiveUnsigned>(n: T, exp: u64, out_root: T, out_rem: T) {
        assert_eq!(n.root_rem(exp), (out_root, out_rem));
        assert_eq!(root_rem_binary(n, exp), (out_root, out_rem));

        let mut n = n;
        assert_eq!(n.root_assign_rem(exp), out_rem);
        assert_eq!(n, out_root);
    }
    test::<u8>(0, 1, 0, 0);
    test::<u8>(1, 1, 1, 0);
    test::<u8>(2, 1, 2, 0);
    test::<u8>(100, 1, 100, 0);

    test::<u8>(0, 2, 0, 0);
    test::<u8>(1, 2, 1, 0);
    test::<u8>(2, 2, 1, 1);
    test::<u8>(3, 2, 1, 2);
    test::<u8>(4, 2, 2, 0);
    test::<u8>(5, 2, 2, 1);
    test::<u8>(0, 3, 0, 0);
    test::<u8>(1, 3, 1, 0);
    test::<u8>(2, 3, 1, 1);
    test::<u8>(7, 3, 1, 6);
    test::<u8>(8, 3, 2, 0);
    test::<u8>(9, 3, 2, 1);
    test::<u8>(10, 2, 3, 1);
    test::<u8>(100, 2, 10, 0);
    test::<u8>(100, 3, 4, 36);
    test::<u32>(1000000000, 2, 31622, 49116);
    test::<u32>(1000000000, 3, 1000, 0);
    test::<u32>(1000000000, 4, 177, 18493759);
    test::<u32>(1000000000, 5, 63, 7563457);
    test::<u32>(1000000000, 6, 31, 112496319);
    test::<u32>(1000000000, 7, 19, 106128261);
    test::<u32>(1000000000, 8, 13, 184269279);
    test::<u32>(1000000000, 9, 10, 0);
    test::<u32>(1000000000, 10, 7, 717524751);
}

fn root_rem_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.root_rem(0));
}

#[test]
pub fn root_rem_fail() {
    apply_fn_to_unsigneds!(root_rem_fail_helper);
}

fn floor_root_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_32::<T, u64>().test_properties(|(n, exp)| {
        let root = n.floor_root(exp);
        let mut n_alt = n;
        n_alt.floor_root_assign(exp);
        assert_eq!(n_alt, root);
        assert_eq!(floor_root_binary(n, exp), root);
        let pow = root.pow(exp);
        let ceiling_root = n.ceiling_root(exp);
        if pow == n {
            assert_eq!(ceiling_root, root);
        } else {
            assert_eq!(ceiling_root, root + T::ONE);
        }
        assert!(pow <= n);
        if exp != 1 {
            if let Some(next_pow) = (root + T::ONE).checked_pow(exp) {
                assert!(next_pow > n);
            }
        }
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(n.floor_root(2), n.floor_sqrt());
        assert_eq!(n.floor_root(1), n);

        let cbrt = n.floor_root(3);
        let mut n_alt = n;
        n_alt.floor_root_assign(3);
        assert_eq!(n_alt, cbrt);
        assert_eq!(floor_root_binary(n, 3), cbrt);
        let cube = cbrt.pow(3);
        let ceiling_cbrt = n.ceiling_root(3);
        if cube == n {
            assert_eq!(ceiling_cbrt, cbrt);
        } else {
            assert_eq!(ceiling_cbrt, cbrt + T::ONE);
        }
        assert!(cube <= n);
        if let Some(next_cube) = (cbrt + T::ONE).checked_pow(3) {
            assert!(next_cube > n);
        }
    });
}

fn floor_root_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_18::<T, u64>().test_properties(|(n, exp)| {
        let root = n.floor_root(exp);
        let mut n_alt = n;
        n_alt.floor_root_assign(exp);
        assert_eq!(n_alt, root);
        if let Some(pow) = root.checked_pow(exp) {
            let ceiling_root = n.ceiling_root(exp);
            if pow == n {
                assert_eq!(ceiling_root, root);
            } else {
                assert_eq!(ceiling_root, root + T::ONE);
            }
            assert!(pow <= n);
        }
        if exp.odd() && n != T::MIN {
            assert_eq!(-(-n).ceiling_root(exp), root);
        }
    });

    signed_gen::<T>().test_properties(|n| {
        assert_eq!(n.floor_root(1), n);

        let cbrt = n.floor_root(3);
        let mut n_alt = n;
        n_alt.floor_root_assign(3);
        assert_eq!(n_alt, cbrt);
        if let Some(cube) = cbrt.checked_pow(3) {
            let ceiling_cbrt = n.ceiling_root(3);
            if cube == n {
                assert_eq!(ceiling_cbrt, cbrt);
            } else {
                assert_eq!(ceiling_cbrt, cbrt + T::ONE);
            }
            assert!(cube <= n);
        }
        if n != T::MIN {
            assert_ne!((-n).ceiling_root(3), T::MIN);
            assert_eq!(-(-n).ceiling_root(3), cbrt);
        }
    });

    signed_gen_var_2::<T>().test_properties(|n| {
        assert_eq!(n.floor_root(2), n.floor_sqrt());
    });
}

macro_rules! floor_root_approx_and_refine_helper {
    ($t:ty) => {
        #[allow(clippy::cast_lossless)]
        unsigned_pair_gen_var_32::<$t, u64>().test_properties(|(n, exp)| {
            assert_eq!(
                floor_root_approx_and_refine(|x| x as f64, |f| f as $t, n, exp),
                n.floor_root(exp),
            );
        });

        #[allow(clippy::cast_lossless)]
        unsigned_gen::<$t>().test_properties(|n| {
            assert_eq!(
                floor_root_approx_and_refine(|x| x as f64, |f| f as $t, n, 3),
                n.floor_root(3),
            );
        });
    };
}

#[test]
fn floor_root_properties() {
    apply_fn_to_unsigneds!(floor_root_properties_helper_unsigned);
    apply_fn_to_signeds!(floor_root_properties_helper_signed);

    unsigned_gen_var_1::<u32>().test_properties(|n| {
        assert_eq!(cbrt_chebyshev_approx_u32(n), n.floor_root(3));
        assert_eq!(fast_floor_cbrt_u32(n), n.floor_root(3));
    });

    unsigned_gen_var_1::<u64>().test_properties(|n| {
        assert_eq!(cbrt_chebyshev_approx_u64(n), n.floor_root(3));
        assert_eq!(fast_floor_cbrt_u64(n), n.floor_root(3));
    });

    unsigned_pair_gen_var_32::<u32, u64>().test_properties(|(n, exp)| {
        assert_eq!(fast_floor_root_u32(n, exp), n.floor_root(exp));
    });

    unsigned_pair_gen_var_32::<u64, u64>().test_properties(|(n, exp)| {
        assert_eq!(fast_floor_root_u64(n, exp), n.floor_root(exp));
    });

    apply_to_unsigneds!(floor_root_approx_and_refine_helper);
}

fn ceiling_root_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_32::<T, u64>().test_properties(|(n, exp)| {
        let root = n.ceiling_root(exp);
        let mut n_alt = n;
        n_alt.ceiling_root_assign(exp);
        assert_eq!(n_alt, root);
        assert_eq!(ceiling_root_binary(n, exp), root);
        if let Some(pow) = root.checked_pow(exp) {
            let floor_root = n.floor_root(exp);
            if pow == n {
                assert_eq!(floor_root, root);
            } else {
                assert_eq!(floor_root, root - T::ONE);
            }
            assert!(pow >= n);
        }
        if exp != 1 && n != T::ZERO {
            assert!((root - T::ONE).pow(exp) < n);
        }
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(n.ceiling_root(2), n.ceiling_sqrt());
        assert_eq!(n.ceiling_root(1), n);

        let cbrt = n.ceiling_root(3);
        let mut n_alt = n;
        n_alt.ceiling_root_assign(3);
        assert_eq!(n_alt, cbrt);
        assert_eq!(ceiling_root_binary(n, 3), cbrt);
        if let Some(cube) = cbrt.checked_pow(3) {
            let floor_cbrt = n.floor_root(3);
            if cube == n {
                assert_eq!(floor_cbrt, cbrt);
            } else {
                assert_eq!(floor_cbrt, cbrt - T::ONE);
            }
            assert!(cube >= n);
        }
        if n != T::ZERO {
            assert!((cbrt - T::ONE).pow(3) < n);
        }
    });
}

fn ceiling_root_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_18::<T, u64>().test_properties(|(n, exp)| {
        let root = n.ceiling_root(exp);
        let mut n_alt = n;
        n_alt.ceiling_root_assign(exp);
        assert_eq!(n_alt, root);
        if let Some(pow) = root.checked_pow(exp) {
            let floor_root = n.floor_root(exp);
            if pow == n {
                assert_eq!(floor_root, root);
            } else {
                assert_eq!(floor_root, root - T::ONE);
            }
            assert!(pow >= n);
        }
        if exp.odd() && n != T::MIN {
            assert_eq!(-(-n).floor_root(exp), root);
        }
    });

    signed_gen::<T>().test_properties(|n| {
        assert_eq!(n.ceiling_root(1), n);

        let cbrt = n.ceiling_root(3);
        let mut n_alt = n;
        n_alt.ceiling_root_assign(3);
        assert_eq!(n_alt, cbrt);
        if let Some(cube) = cbrt.checked_pow(3) {
            let floor_cbrt = n.floor_root(3);
            if cube == n {
                assert_eq!(floor_cbrt, cbrt);
            } else {
                assert_eq!(floor_cbrt, cbrt - T::ONE);
            }
            assert!(cube >= n);
        }
        if n != T::MIN {
            assert_eq!(-(-n).floor_root(3), cbrt);
        }
    });

    signed_gen_var_2::<T>().test_properties(|n| {
        assert_eq!(n.ceiling_root(2), n.ceiling_sqrt());
    });
}

#[test]
fn ceiling_root_properties() {
    apply_fn_to_unsigneds!(ceiling_root_properties_helper_unsigned);
    apply_fn_to_signeds!(ceiling_root_properties_helper_signed);

    unsigned_pair_gen_var_32::<u32, u64>().test_properties(|(n, exp)| {
        assert_eq!(fast_ceiling_root_u32(n, exp), n.ceiling_root(exp));
    });

    unsigned_pair_gen_var_32::<u64, u64>().test_properties(|(n, exp)| {
        assert_eq!(fast_ceiling_root_u64(n, exp), n.ceiling_root(exp));
    });
}

fn checked_root_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_32::<T, u64>().test_properties(|(n, exp)| {
        let root = n.checked_root(exp);
        assert_eq!(checked_root_binary(n, exp), root);
        if let Some(root) = root {
            assert_eq!(root.pow(exp), n);
            assert_eq!(n.floor_root(exp), root);
            assert_eq!(n.ceiling_root(exp), root);
        }
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(n.checked_root(2), n.checked_sqrt());
        assert_eq!(n.checked_root(1), Some(n));

        let cbrt = n.checked_root(3);
        assert_eq!(checked_root_binary(n, 3), cbrt);
        if let Some(cbrt) = cbrt {
            assert_eq!(cbrt.pow(3), n);
            assert_eq!(n.floor_root(3), cbrt);
            assert_eq!(n.ceiling_root(3), cbrt);
        }
    });
}

fn checked_root_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_18::<T, u64>().test_properties(|(n, exp)| {
        let root = n.checked_root(exp);
        if let Some(root) = root {
            assert_eq!(root.pow(exp), n);
            assert_eq!(n.floor_root(exp), root);
            assert_eq!(n.ceiling_root(exp), root);
        }
        if exp.odd() && n != T::MIN {
            assert_eq!((-n).checked_root(exp).map(Neg::neg), root);
        }
    });

    signed_gen::<T>().test_properties(|n| {
        assert_eq!(n.checked_root(1), Some(n));

        let cbrt = n.checked_root(3);
        if let Some(cbrt) = cbrt {
            assert_eq!(cbrt.pow(3), n);
            assert_eq!(n.floor_root(3), cbrt);
            assert_eq!(n.ceiling_root(3), cbrt);
        }
        if n != T::MIN {
            assert_eq!((-n).checked_root(3).map(Neg::neg), cbrt);
        }
    });

    signed_gen_var_2::<T>().test_properties(|n| {
        assert_eq!(n.checked_root(2), n.checked_sqrt());
    });
}

#[test]
fn checked_root_properties() {
    apply_fn_to_unsigneds!(checked_root_properties_helper_unsigned);
    apply_fn_to_signeds!(checked_root_properties_helper_signed);

    unsigned_pair_gen_var_32::<u32, u64>().test_properties(|(n, exp)| {
        assert_eq!(fast_checked_root_u32(n, exp), n.checked_root(exp));
    });

    unsigned_pair_gen_var_32::<u64, u64>().test_properties(|(n, exp)| {
        assert_eq!(fast_checked_root_u64(n, exp), n.checked_root(exp));
    });
}

fn root_rem_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_32::<T, u64>().test_properties(|(n, exp)| {
        let (root, rem) = n.root_rem(exp);
        let mut n_alt = n;
        assert_eq!(n_alt.root_assign_rem(exp), rem);
        assert_eq!(n_alt, root);
        assert_eq!(root_rem_binary(n, exp), (root, rem));
        assert_eq!(n.floor_root(exp), root);
        assert_eq!(root.pow(exp) + rem, n);
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(n.root_rem(2), n.sqrt_rem());
        assert_eq!(n.root_rem(1), (n, T::ZERO));

        let (cbrt, rem) = n.root_rem(3);
        let mut n_alt = n;
        assert_eq!(n_alt.root_assign_rem(3), rem);
        assert_eq!(n_alt, cbrt);
        assert_eq!(root_rem_binary(n, 3), (cbrt, rem));
        assert_eq!(n.floor_root(3), cbrt);
        assert_eq!(cbrt.pow(3) + rem, n);
    });
}

#[test]
fn root_rem_properties() {
    apply_fn_to_unsigneds!(root_rem_properties_helper);

    unsigned_pair_gen_var_32::<u32, u64>().test_properties(|(n, exp)| {
        assert_eq!(fast_root_rem_u32(n, exp), n.root_rem(exp));
    });

    unsigned_pair_gen_var_32::<u64, u64>().test_properties(|(n, exp)| {
        assert_eq!(fast_root_rem_u64(n, exp), n.root_rem(exp));
    });
}
