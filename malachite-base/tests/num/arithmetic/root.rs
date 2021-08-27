use malachite_base::num::arithmetic::root::{
    _ceiling_root_binary, _checked_root_binary, _floor_root_binary, _root_rem_binary,
};
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{
    signed_gen, signed_gen_var_2, signed_unsigned_pair_gen_var_18, unsigned_gen,
    unsigned_pair_gen_var_32,
};
use std::ops::Neg;
use std::panic::catch_unwind;

#[test]
fn test_floor_root() {
    fn test_u<T: PrimitiveUnsigned>(n: T, exp: u64, out: T) {
        assert_eq!(n.floor_root(exp), out);
        assert_eq!(_floor_root_binary(n, exp), out);

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
        assert_eq!(_ceiling_root_binary(n, exp), out);

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
        assert_eq!(_checked_root_binary(n, exp), out);
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
        assert_eq!(_root_rem_binary(n, exp), (out_root, out_rem));

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
        assert_eq!(_floor_root_binary(n, exp), root);
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
    });

    signed_gen_var_2::<T>().test_properties(|n| {
        assert_eq!(n.floor_root(2), n.floor_sqrt());
    });
}

#[test]
fn floor_root_properties() {
    apply_fn_to_unsigneds!(floor_root_properties_helper_unsigned);
    apply_fn_to_signeds!(floor_root_properties_helper_signed);
}

fn ceiling_root_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_32::<T, u64>().test_properties(|(n, exp)| {
        let root = n.ceiling_root(exp);
        let mut n_alt = n;
        n_alt.ceiling_root_assign(exp);
        assert_eq!(n_alt, root);
        assert_eq!(_ceiling_root_binary(n, exp), root);
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
    });

    signed_gen_var_2::<T>().test_properties(|n| {
        assert_eq!(n.ceiling_root(2), n.ceiling_sqrt());
    });
}

#[test]
fn ceiling_root_properties() {
    apply_fn_to_unsigneds!(ceiling_root_properties_helper_unsigned);
    apply_fn_to_signeds!(ceiling_root_properties_helper_signed);
}

fn checked_root_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_32::<T, u64>().test_properties(|(n, exp)| {
        let root = n.checked_root(exp);
        assert_eq!(_checked_root_binary(n, exp), root);
        if let Some(root) = root {
            assert_eq!(root.pow(exp), n);
            assert_eq!(n.floor_root(exp), root);
            assert_eq!(n.ceiling_root(exp), root);
        }
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(n.checked_root(2), n.checked_sqrt());
        assert_eq!(n.checked_root(1), Some(n));
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
    });

    signed_gen_var_2::<T>().test_properties(|n| {
        assert_eq!(n.checked_root(2), n.checked_sqrt());
    });
}

#[test]
fn checked_root_properties() {
    apply_fn_to_unsigneds!(checked_root_properties_helper_unsigned);
    apply_fn_to_signeds!(checked_root_properties_helper_signed);
}

fn root_rem_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_32::<T, u64>().test_properties(|(n, exp)| {
        let (root, rem) = n.root_rem(exp);
        let mut n_alt = n;
        assert_eq!(n_alt.root_assign_rem(exp), rem);
        assert_eq!(n_alt, root);
        assert_eq!(_root_rem_binary(n, exp), (root, rem));
        assert_eq!(n.floor_root(exp), root);
        assert_eq!(root.pow(exp) + rem, n);
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert_eq!(n.root_rem(2), n.sqrt_rem());
        assert_eq!(n.root_rem(1), (n, T::ZERO));
    });
}

#[test]
fn root_rem_properties() {
    apply_fn_to_unsigneds!(root_rem_properties_helper);
}
