use malachite_base::num::arithmetic::sqrt::{
    _ceiling_sqrt_binary, _checked_sqrt_binary, _floor_sqrt_binary, _sqrt_rem_binary,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::unsigned_gen;

#[test]
fn test_floor_sqrt() {
    fn test<T: PrimitiveUnsigned>(n: T, out: T) {
        assert_eq!(n.floor_sqrt(), out);
        assert_eq!(_floor_sqrt_binary(n), out);

        let mut n = n;
        n.floor_sqrt_assign();
        assert_eq!(n, out);
    }
    test::<u8>(0, 0);
    test::<u8>(1, 1);
    test::<u8>(2, 1);
    test::<u8>(3, 1);
    test::<u8>(4, 2);
    test::<u8>(5, 2);
    test::<u8>(10, 3);
    test::<u8>(100, 10);
    test::<u32>(1000000000, 31622);
    test::<u64>(152415765279683, 12345677);
    test::<u64>(152415765279684, 12345678);
    test::<u64>(152415765279685, 12345678);
}

#[test]
fn test_ceiling_sqrt() {
    fn test<T: PrimitiveUnsigned>(n: T, out: T) {
        assert_eq!(n.ceiling_sqrt(), out);
        assert_eq!(_ceiling_sqrt_binary(n), out);

        let mut n = n;
        n.ceiling_sqrt_assign();
        assert_eq!(n, out);
    }
    test::<u8>(0, 0);
    test::<u8>(1, 1);
    test::<u8>(2, 2);
    test::<u8>(3, 2);
    test::<u8>(4, 2);
    test::<u8>(5, 3);
    test::<u8>(10, 4);
    test::<u8>(100, 10);
    test::<u32>(1000000000, 31623);
    test::<u64>(152415765279683, 12345678);
    test::<u64>(152415765279684, 12345678);
    test::<u64>(152415765279685, 12345679);
}

#[test]
fn test_checked_sqrt() {
    fn test<T: PrimitiveUnsigned>(n: T, out: Option<T>) {
        assert_eq!(n.checked_sqrt(), out);
        assert_eq!(_checked_sqrt_binary(n), out);
    }
    test::<u8>(0, Some(0));
    test::<u8>(1, Some(1));
    test::<u8>(2, None);
    test::<u8>(3, None);
    test::<u8>(4, Some(2));
    test::<u8>(5, None);
    test::<u8>(10, None);
    test::<u8>(100, Some(10));
    test::<u32>(1000000000, None);
    test::<u64>(152415765279683, None);
    test::<u64>(152415765279684, Some(12345678));
    test::<u64>(152415765279685, None);
}

#[test]
fn test_sqrt_rem() {
    fn test<T: PrimitiveUnsigned>(n: T, sqrt: T, rem: T) {
        let (actual_sqrt, actual_rem) = n.sqrt_rem();
        assert_eq!(actual_sqrt, sqrt);
        assert_eq!(actual_rem, rem);
        assert_eq!(_sqrt_rem_binary(n), (sqrt, rem));

        let mut n = n;
        assert_eq!(n.sqrt_rem_assign(), rem);
        assert_eq!(n, sqrt);
    }
    test::<u8>(0, 0, 0);
    test::<u8>(1, 1, 0);
    test::<u8>(2, 1, 1);
    test::<u8>(3, 1, 2);
    test::<u8>(4, 2, 0);
    test::<u8>(5, 2, 1);
    test::<u8>(10, 3, 1);
    test::<u8>(100, 10, 0);
    test::<u32>(1000000000, 31622, 49116);
    test::<u64>(152415765279683, 12345677, 24691354);
    test::<u64>(152415765279684, 12345678, 0);
    test::<u64>(152415765279685, 12345678, 1);
}

fn floor_sqrt_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let sqrt = n.floor_sqrt();
        let mut n_alt = n;
        n_alt.floor_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(_floor_sqrt_binary(n), sqrt);
        let square = sqrt.square();
        let ceiling_sqrt = n.ceiling_sqrt();
        if square == n {
            assert_eq!(ceiling_sqrt, sqrt);
        } else {
            assert_eq!(ceiling_sqrt, sqrt + T::ONE);
        }
        assert!(square <= n);
        if let Some(next_square) = (sqrt + T::ONE).checked_square() {
            assert!(next_square > n);
        }
    });
}

#[test]
fn floor_sqrt_properties() {
    apply_fn_to_unsigneds!(floor_sqrt_properties_helper);
}

fn ceiling_sqrt_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let sqrt = n.ceiling_sqrt();
        let mut n_alt = n;
        n_alt.ceiling_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(_ceiling_sqrt_binary(n), sqrt);
        if let Some(square) = sqrt.checked_square() {
            let floor_sqrt = n.floor_sqrt();
            if square == n {
                assert_eq!(floor_sqrt, sqrt);
            } else {
                assert_eq!(floor_sqrt, sqrt - T::ONE);
            }
            assert!(square >= n);
        }
        if n != T::ZERO {
            assert!((sqrt - T::ONE).square() < n);
        }
    });
}

#[test]
fn ceiling_sqrt_properties() {
    apply_fn_to_unsigneds!(ceiling_sqrt_properties_helper);
}

fn checked_sqrt_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let sqrt = n.checked_sqrt();
        assert_eq!(_checked_sqrt_binary(n), sqrt);
        if let Some(sqrt) = sqrt {
            assert_eq!(sqrt.square(), n);
            assert_eq!(n.floor_sqrt(), sqrt);
            assert_eq!(n.ceiling_sqrt(), sqrt);
        }
    });
}

#[test]
fn checked_sqrt_properties() {
    apply_fn_to_unsigneds!(checked_sqrt_properties_helper);
}

fn sqrt_rem_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|n| {
        let (sqrt, rem) = n.sqrt_rem();
        let mut n_alt = n;
        assert_eq!(n_alt.sqrt_rem_assign(), rem);
        assert_eq!(n_alt, sqrt);
        assert_eq!(_sqrt_rem_binary(n), (sqrt, rem));
        assert_eq!(n.floor_sqrt(), sqrt);
        assert!(rem <= sqrt << 1);
        assert_eq!(sqrt.square() + rem, n);
    });
}

#[test]
fn sqrt_rem_properties() {
    apply_fn_to_unsigneds!(sqrt_rem_properties_helper);
}
