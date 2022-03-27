use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_q::Rational;
use malachite_q_test_util::common::rational_to_rug_rational;
use malachite_q_test_util::generators::{
    rational_gen, rational_primitive_float_pair_gen,
    rational_primitive_float_primitive_float_triple_gen,
    rational_rational_primitive_float_triple_gen,
};
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_cmp_primitive_float() {
    let test = |u, v: f32, out: Option<Ordering>| {
        let out_rev = out.map(Ordering::reverse);
        assert_eq!(Rational::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(rug::Rational::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(v.partial_cmp(&Rational::from_str(u).unwrap()), out_rev);
        assert_eq!(v.partial_cmp(&rug::Rational::from_str(u).unwrap()), out_rev);

        let v = v as f64;
        assert_eq!(Rational::from_str(u).unwrap().partial_cmp(&v), out);
        assert_eq!(rug::Rational::from_str(u).unwrap().partial_cmp(&v), out);

        assert_eq!(v.partial_cmp(&Rational::from_str(u).unwrap()), out_rev);
        assert_eq!(v.partial_cmp(&rug::Rational::from_str(u).unwrap()), out_rev);
    };
    test("2/3", f32::NAN, None);
    test("2/3", f32::POSITIVE_INFINITY, Some(Ordering::Less));
    test("2/3", f32::NEGATIVE_INFINITY, Some(Ordering::Greater));
    test("-2/3", f32::NAN, None);
    test("-2/3", f32::POSITIVE_INFINITY, Some(Ordering::Less));
    test("-2/3", f32::NEGATIVE_INFINITY, Some(Ordering::Greater));

    test("0", 0.0, Some(Ordering::Equal));
    test("0", -0.0, Some(Ordering::Equal));
    test("0", 5.0, Some(Ordering::Less));
    test("0", -5.0, Some(Ordering::Greater));
    test("3/2", 1.5, Some(Ordering::Equal));
    test("3/2", 5.0, Some(Ordering::Less));
    test("3/2", -5.0, Some(Ordering::Greater));
    test("-3/2", 5.0, Some(Ordering::Less));
    test("-3/2", -5.0, Some(Ordering::Greater));
    test("-3/2", -1.5, Some(Ordering::Equal));

    test("1/3", 0.333, Some(Ordering::Greater));
    test("1/3", 0.334, Some(Ordering::Less));
    test("1/3", -0.333, Some(Ordering::Greater));
    test("1/3", -0.334, Some(Ordering::Greater));
    test("-1/3", -0.334, Some(Ordering::Greater));
    test("-1/3", -0.333, Some(Ordering::Less));
    test("-1/3", 0.334, Some(Ordering::Less));
    test("-1/3", 0.333, Some(Ordering::Less));
}

fn partial_cmp_primitive_float_properties_helper<
    T: PartialOrd<Rational> + PartialOrd<rug::Rational> + PrimitiveFloat,
>()
where
    Rational: From<T> + PartialOrd<T>,
    rug::Rational: PartialOrd<T>,
{
    rational_primitive_float_pair_gen::<T>().test_properties(|(n, u)| {
        let cmp = n.partial_cmp(&u);
        assert_eq!(rational_to_rug_rational(&n).partial_cmp(&u), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(u.partial_cmp(&n), cmp_rev);
        assert_eq!(u.partial_cmp(&rational_to_rug_rational(&n)), cmp_rev);

        if u.is_finite() {
            assert_eq!(n.cmp(&Rational::from(u)), cmp.unwrap());
        }
    });

    rational_rational_primitive_float_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n < u && u < m {
            assert_eq!(n.cmp(&m), Ordering::Less);
        } else if n > u && u > m {
            assert_eq!(n.cmp(&m), Ordering::Greater);
        }
    });

    rational_primitive_float_primitive_float_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u < n && n < v {
            assert!(u < v);
        } else if u > n && n > v {
            assert!(u > v);
        }
    });

    rational_gen().test_properties(|x| {
        assert!(x > T::NEGATIVE_INFINITY);
        assert!(x < T::POSITIVE_INFINITY);
    });
}

#[test]
fn partial_cmp_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_cmp_primitive_float_properties_helper);
}
