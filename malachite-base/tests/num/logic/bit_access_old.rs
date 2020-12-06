use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use std::panic::catch_unwind;

fn set_bit_helper_unsigned<T: PrimitiveInt>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.set_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, 101);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, 1024);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1000000000000, 10, 1000000001024);
    }
}

fn set_bit_helper_signed<T: PrimitiveSigned>() {
    set_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.set_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -1);
    test(-1, 100, -1);
    test(-33, 5, -1);
    test(-32, 0, -31);

    if T::WIDTH >= u64::WIDTH {
        test(-1000000000000, 10, -999999998976);
        test(-1000000000000, 100, -1000000000000);
    }
}

#[test]
fn test_set_bit() {
    apply_fn_to_unsigneds!(set_bit_helper_unsigned);
    apply_fn_to_signeds!(set_bit_helper_signed);
}

fn set_bit_fail_helper<T: PrimitiveInt>() {
    assert_panic!({
        let mut n = T::exact_from(5);
        n.set_bit(200);
    });
}

#[test]
fn set_bit_fail() {
    apply_fn_to_primitive_ints!(set_bit_fail_helper);
}

fn clear_bit_helper_unsigned<T: PrimitiveInt>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.clear_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(0, 10, 0);
    test(0, 100, 0);
    test(101, 0, 100);
    if T::WIDTH >= u16::WIDTH {
        test(1024, 10, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1000000001024, 10, 1000000000000);
        test(1000000001024, 100, 1000000001024);
    }
}

fn clear_bit_helper_signed<T: PrimitiveSigned>() {
    clear_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.clear_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -33);
    test(-31, 0, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-999999998976, 10, -1000000000000);
    }
}

#[test]
fn test_clear_bit() {
    apply_fn_to_unsigneds!(clear_bit_helper_unsigned);
    apply_fn_to_signeds!(clear_bit_helper_signed);
}

fn clear_bit_fail_helper<T: PrimitiveSigned>() {
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.clear_bit(200);
    });
}

#[test]
fn clear_bit_fail() {
    apply_fn_to_signeds!(clear_bit_fail_helper);
}

fn assign_bit_helper_unsigned<T: PrimitiveInt>() {
    let test = |n: u64, index, bit, out: u64| {
        let mut n = T::exact_from(n);
        n.assign_bit(index, bit);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, true, 101);
    test(0, 10, false, 0);
    test(0, 100, false, 0);
    test(101, 0, false, 100);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, true, 1024);
        test(1024, 10, false, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1000000000000, 10, true, 1000000001024);
        test(1000000001024, 10, false, 1000000000000);
        test(1000000001024, 100, false, 1000000001024);
    }
}

fn assign_bit_helper_signed<T: PrimitiveSigned>() {
    assign_bit_helper_unsigned::<T>();

    let test = |n: i64, index, bit, out: i64| {
        let mut n = T::exact_from(n);
        n.assign_bit(index, bit);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, true, -1);
    test(-1, 100, true, -1);
    test(-33, 5, true, -1);
    test(-32, 0, true, -31);
    test(-1, 5, false, -33);
    test(-31, 0, false, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-1000000000000, 10, true, -999999998976);
        test(-1000000000000, 100, true, -1000000000000);
        test(-999999998976, 10, false, -1000000000000);
    }
}

#[test]
fn test_assign_bit() {
    apply_fn_to_unsigneds!(assign_bit_helper_unsigned);
    apply_fn_to_signeds!(assign_bit_helper_signed);
}

fn assign_bit_fail_helper<T: PrimitiveInt>() {
    assert_panic!({
        let mut n = T::exact_from(5);
        n.assign_bit(200, true);
    });
}

fn assign_bit_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.assign_bit(200, false);
    });
}

#[test]
fn assign_bit_fail() {
    apply_fn_to_primitive_ints!(assign_bit_fail_helper);
    apply_fn_to_signeds!(assign_bit_fail_helper_signed);
}
