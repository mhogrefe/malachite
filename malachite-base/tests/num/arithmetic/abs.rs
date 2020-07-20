use malachite_base_test_util::generators::{signed_gen, signed_gen_var_1};

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::conversion::traits::ExactFrom;

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

fn test<U: PrimitiveUnsigned, S: PrimitiveSigned>(n: S, out: U)
where
    S: UnsignedAbs<Output = U>,
{
    assert_eq!(n.unsigned_abs(), out);
}

fn unsigned_abs_helper<U: PrimitiveUnsigned, S: PrimitiveSigned>()
where
    S: UnsignedAbs<Output = U>,
    U: CheckedFrom<S>,
{
    test(S::ZERO, U::ZERO);
    test(S::ONE, U::ONE);
    test(S::exact_from(100), U::exact_from(100));
    test(S::NEGATIVE_ONE, U::ONE);
    test(S::exact_from(-100), U::exact_from(100));
    test(S::MIN, U::exact_from(S::MAX) + U::ONE);
}

#[test]
fn test_unsigned_abs() {
    apply_fn_to_unsigned_signed_pair!(unsigned_abs_helper);
}

fn abs_assign_properties_helper<T: PrimitiveSigned>()
where
    T: ExactFrom<<T as UnsignedAbs>::Output>,
{
    signed_gen_var_1::<T>().test_properties(|n| {
        let mut abs = n;
        abs.abs_assign();
        assert_eq!(abs, n.abs());
        assert_eq!(abs.abs(), abs);
        assert_eq!(abs == n, n >= T::ZERO);
        assert_eq!(T::exact_from(n.unsigned_abs()), abs)
    });
}

#[test]
fn abs_assign_properties() {
    apply_fn_to_signeds!(abs_assign_properties_helper);
}

fn unsigned_abs_properties_helper<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|n| {
        n.unsigned_abs();
    });
}

#[test]
fn unsigned_abs_properties() {
    apply_fn_to_signeds!(unsigned_abs_properties_helper);
}
