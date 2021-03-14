use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;

fn abs_negative_zeros_helper<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(NiceFloat(n.abs_negative_zeros()), out);

        let mut n = n;
        n.abs_negative_zeros_assign();
        assert_eq!(NiceFloat(n), out);
    };
    test(T::ZERO, T::ZERO);
    test(T::NEGATIVE_ZERO, T::ZERO);
    test(T::NAN, T::NAN);
    test(T::POSITIVE_INFINITY, T::POSITIVE_INFINITY);
    test(T::NEGATIVE_INFINITY, T::NEGATIVE_INFINITY);
    test(T::ONE, T::ONE);
    test(T::NEGATIVE_ONE, T::NEGATIVE_ONE);
    test(T::from(1.234), T::from(1.234));
    test(T::from(-1.234), T::from(-1.234));
}

#[test]
fn test_abs_negative_zeros() {
    apply_fn_to_primitive_floats!(abs_negative_zeros_helper);
}
