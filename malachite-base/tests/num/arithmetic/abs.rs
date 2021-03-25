use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::generators::signed_gen_var_1;

fn abs_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.abs(), out);

        let mut n = n;
        n.abs_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::ONE);
    test(T::exact_from(100), T::exact_from(100));
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
}

#[test]
fn test_abs() {
    apply_fn_to_signeds!(abs_helper);
}

fn abs_assign_properties_helper<U, S: ExactFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>>() {
    signed_gen_var_1::<S>().test_properties(|n| {
        let mut abs = n;
        abs.abs_assign();
        assert_eq!(abs, n.abs());
        assert_eq!(abs.abs(), abs);
        assert_eq!(abs == n, n >= S::ZERO);
        assert_eq!(S::exact_from(n.unsigned_abs()), abs)
    });
}

#[test]
fn abs_assign_properties() {
    apply_fn_to_unsigned_signed_pairs!(abs_assign_properties_helper);
}
