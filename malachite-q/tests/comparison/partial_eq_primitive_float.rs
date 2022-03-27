use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q_test_util::common::rational_to_rug_rational;
use malachite_q_test_util::generators::rational_primitive_float_pair_gen;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_eq() {
    let test = |u, v: f32, out| {
        assert_eq!(Rational::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Rational::from_str(u).unwrap() == v, out);

        assert_eq!(v == Rational::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Rational::from_str(u).unwrap(), out);

        let v = v as f64;
        assert_eq!(Rational::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Rational::from_str(u).unwrap() == v, out);

        assert_eq!(v == Rational::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Rational::from_str(u).unwrap(), out);
    };
    test("2/3", f32::NAN, false);
    test("2/3", f32::POSITIVE_INFINITY, false);
    test("2/3", f32::NEGATIVE_INFINITY, false);
    test("-2/3", f32::NAN, false);
    test("-2/3", f32::POSITIVE_INFINITY, false);
    test("-2/3", f32::NEGATIVE_INFINITY, false);

    test("0", 0.0, true);
    test("0", -0.0, true);
    test("0", 5.0, false);
    test("0", -5.0, false);
    test("3/2", 1.5, true);
    test("3/2", 5.0, false);
    test("3/2", -1.5, false);
    test("-3/2", 1.5, false);
    test("-3/2", 5.0, false);
    test("-3/2", -1.5, true);
}

fn partial_eq_primitive_float_properties_helper<
    T: PartialEq<Rational> + PartialEq<Natural> + PartialEq<rug::Rational> + PrimitiveFloat,
>()
where
    Rational: From<T> + PartialEq<T> + PartialOrd<T>,
    Natural: PartialEq<T>,
    rug::Rational: PartialEq<T>,
{
    rational_primitive_float_pair_gen::<T>().test_properties(|(n, f)| {
        let eq = n == f;
        assert_eq!(rational_to_rug_rational(&n) == f, eq);
        assert_eq!(f == n, eq);
        assert_eq!(f == rational_to_rug_rational(&n), eq);
        assert_eq!(n.partial_cmp(&f) == Some(Ordering::Equal), eq);
        if f.is_finite() {
            assert_eq!(PartialEq::<Rational>::eq(&n, &Rational::from(f)), eq);
        }
    });
}

#[test]
fn partial_eq_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_eq_primitive_float_properties_helper);
}
