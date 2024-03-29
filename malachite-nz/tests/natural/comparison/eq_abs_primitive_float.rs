use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::generators::natural_primitive_float_pair_gen;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_eq_abs_f32() {
    let test = |u, v: f32, out| {
        assert_eq!(Natural::from_str(u).unwrap().eq_abs(&v), out);
        assert_eq!(!Natural::from_str(u).unwrap().ne_abs(&v), out);
        assert_eq!(v.eq_abs(&Natural::from_str(u).unwrap()), out);
    };
    test("0", 0.0, true);
    test("0", -0.0, true);
    test("0", 5.0, false);
    test("0", -5.0, false);
    test("123", 123.0, true);
    test("123", -123.0, true);
    test("123", 5.0, false);
    test("123", -5.0, false);
    test("1000000000000", 123.0, false);
    test("1000000000000", -123.0, false);
    test("1", 0.5, false);
    test("1", -0.5, false);
    test("1", f32::INFINITY, false);
    test("1", f32::NEGATIVE_INFINITY, false);
    test("1", f32::NAN, false);
}

#[test]
fn test_eq_abs_f64() {
    let test = |u, v: f64, out| {
        assert_eq!(Natural::from_str(u).unwrap().eq_abs(&v), out);
        assert_eq!(!Natural::from_str(u).unwrap().ne_abs(&v), out);
        assert_eq!(v.eq_abs(&Natural::from_str(u).unwrap()), out);
    };
    test("0", 0.0, true);
    test("0", -0.0, true);
    test("0", 5.0, false);
    test("0", -5.0, false);
    test("123", 123.0, true);
    test("123", -123.0, true);
    test("123", 5.0, false);
    test("123", -5.0, false);
    test("1000000000000", 123.0, false);
    test("1000000000000", -123.0, false);
    test("1", 0.5, false);
    test("1", -0.5, false);
    test("1", f64::INFINITY, false);
    test("1", f64::NEGATIVE_INFINITY, false);
    test("1", f64::NAN, false);
}

fn eq_abs_primitive_float_properties_helper<
    T: EqAbs<Natural> + PartialEq<Natural> + PrimitiveFloat,
>()
where
    Natural: EqAbs<T> + PartialEq<T> + PartialOrdAbs<T>,
{
    natural_primitive_float_pair_gen::<T>().test_properties(|(n, x)| {
        let eq = n.eq_abs(&x);
        assert_ne!(n.ne_abs(&x), eq);

        assert_eq!(x.eq_abs(&n), eq);
        assert_eq!(n.partial_cmp_abs(&x) == Some(Ordering::Equal), eq);
    });

    natural_gen().test_properties(|n| {
        assert_ne!(n, T::NAN);
        assert_ne!(n, T::INFINITY);
        assert_ne!(n, T::NEGATIVE_INFINITY);
    });
}

#[test]
fn eq_abs_primitive_float_properties() {
    apply_fn_to_primitive_floats!(eq_abs_primitive_float_properties_helper);
}
