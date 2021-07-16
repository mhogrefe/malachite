use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base_test_util::generators::signed_gen;

fn saturating_abs_assign_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.saturating_abs(), out);

        let mut n = n;
        n.saturating_abs_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::ONE);
    test(T::exact_from(100), T::exact_from(100));
    test(T::MAX, T::MAX);
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
    test(T::MIN, T::MAX);
}

#[test]
fn test_saturating_abs_assign() {
    apply_fn_to_signeds!(saturating_abs_assign_helper);
}

fn saturating_abs_properties_helper<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|n| {
        let mut abs = n;
        abs.saturating_abs_assign();
        assert_eq!(abs, n.saturating_abs());
        assert_eq!(abs.saturating_abs(), abs);
        if n != T::MIN {
            assert_eq!(n.abs(), abs);
        }
        assert_eq!(abs == n, n >= T::ZERO);
    });
}

#[test]
fn saturating_abs_properties() {
    apply_fn_to_signeds!(saturating_abs_properties_helper);
}
