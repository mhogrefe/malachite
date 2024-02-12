use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{Ceiling, Floor};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::OneHalf;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, IsInteger, RoundingFrom,
};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_nz::integer::Integer;
use malachite_q::conversion::primitive_int_from_rational::{
    SignedFromRationalError, UnsignedFromRationalError,
};
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_3, rational_rounding_mode_pair_gen_var_3,
};
use malachite_q::Rational;
use std::cmp::Ordering;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_u32_try_from_rational() {
    let test = |s, out: Result<u32, UnsignedFromRationalError>| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(u32::try_from(&u), out);
    };
    test("0", Ok(0));
    test("123", Ok(123));
    test("-123", Err(UnsignedFromRationalError));
    test("1000000000000", Err(UnsignedFromRationalError));
    test("-1000000000000", Err(UnsignedFromRationalError));
    test("22/7", Err(UnsignedFromRationalError));
    test("-22/7", Err(UnsignedFromRationalError));
}

#[test]
fn test_i32_try_from_rational() {
    let test = |s, out: Result<i32, SignedFromRationalError>| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(i32::try_from(&u), out);
    };
    test("0", Ok(0));
    test("123", Ok(123));
    test("-123", Ok(-123));
    test("1000000000000", Err(SignedFromRationalError));
    test("-1000000000000", Err(SignedFromRationalError));
    test("22/7", Err(SignedFromRationalError));
    test("-22/7", Err(SignedFromRationalError));
}

#[test]
fn test_u32_convertible_from_rational() {
    let test = |s, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(u32::convertible_from(&u), out);
    };
    test("0", true);
    test("123", true);
    test("-123", false);
    test("1000000000000", false);
    test("-1000000000000", false);
    test("22/7", false);
    test("-22/7", false);
}

#[test]
fn test_i32_convertible_from_rational() {
    let test = |s, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(i32::convertible_from(&u), out);
    };
    test("0", true);
    test("123", true);
    test("-123", true);
    test("1000000000000", false);
    test("-1000000000000", false);
    test("22/7", false);
    test("-22/7", false);
}

#[test]
fn test_u32_rounding_from_rational() {
    let test = |s, rm, out: u32, o_out| {
        let r = Rational::from_str(s).unwrap();
        let (u, o) = u32::rounding_from(&r, rm);
        assert_eq!(u, out);
        assert_eq!(o, o_out);
    };
    test("123", RoundingMode::Floor, 123, Ordering::Equal);
    test("123", RoundingMode::Ceiling, 123, Ordering::Equal);
    test("123", RoundingMode::Down, 123, Ordering::Equal);
    test("123", RoundingMode::Up, 123, Ordering::Equal);
    test("123", RoundingMode::Nearest, 123, Ordering::Equal);
    test("123", RoundingMode::Exact, 123, Ordering::Equal);

    test("22/7", RoundingMode::Floor, 3, Ordering::Less);
    test("22/7", RoundingMode::Ceiling, 4, Ordering::Greater);
    test("22/7", RoundingMode::Down, 3, Ordering::Less);
    test("22/7", RoundingMode::Up, 4, Ordering::Greater);
    test("22/7", RoundingMode::Nearest, 3, Ordering::Less);

    test("7/2", RoundingMode::Floor, 3, Ordering::Less);
    test("7/2", RoundingMode::Ceiling, 4, Ordering::Greater);
    test("7/2", RoundingMode::Down, 3, Ordering::Less);
    test("7/2", RoundingMode::Up, 4, Ordering::Greater);
    test("7/2", RoundingMode::Nearest, 4, Ordering::Greater);

    test("9/2", RoundingMode::Floor, 4, Ordering::Less);
    test("9/2", RoundingMode::Ceiling, 5, Ordering::Greater);
    test("9/2", RoundingMode::Down, 4, Ordering::Less);
    test("9/2", RoundingMode::Up, 5, Ordering::Greater);
    test("9/2", RoundingMode::Nearest, 4, Ordering::Less);

    test("-123", RoundingMode::Ceiling, 0, Ordering::Greater);
    test("-123", RoundingMode::Down, 0, Ordering::Greater);
    test("-123", RoundingMode::Nearest, 0, Ordering::Greater);

    test(
        "1000000000000",
        RoundingMode::Floor,
        u32::MAX,
        Ordering::Less,
    );
    test(
        "1000000000000",
        RoundingMode::Down,
        u32::MAX,
        Ordering::Less,
    );
    test(
        "1000000000000",
        RoundingMode::Nearest,
        u32::MAX,
        Ordering::Less,
    );
}

#[test]
fn test_i32_rounding_from_rational() {
    let test = |s, rm, out: i32, o_out| {
        let r = Rational::from_str(s).unwrap();
        let (i, o) = i32::rounding_from(&r, rm);
        assert_eq!(i, out);
        assert_eq!(o, o_out);
    };
    test("123", RoundingMode::Floor, 123, Ordering::Equal);
    test("123", RoundingMode::Ceiling, 123, Ordering::Equal);
    test("123", RoundingMode::Down, 123, Ordering::Equal);
    test("123", RoundingMode::Up, 123, Ordering::Equal);
    test("123", RoundingMode::Nearest, 123, Ordering::Equal);
    test("123", RoundingMode::Exact, 123, Ordering::Equal);

    test("22/7", RoundingMode::Floor, 3, Ordering::Less);
    test("22/7", RoundingMode::Ceiling, 4, Ordering::Greater);
    test("22/7", RoundingMode::Down, 3, Ordering::Less);
    test("22/7", RoundingMode::Up, 4, Ordering::Greater);
    test("22/7", RoundingMode::Nearest, 3, Ordering::Less);

    test("-22/7", RoundingMode::Floor, -4, Ordering::Less);
    test("-22/7", RoundingMode::Ceiling, -3, Ordering::Greater);
    test("-22/7", RoundingMode::Down, -3, Ordering::Greater);
    test("-22/7", RoundingMode::Up, -4, Ordering::Less);
    test("-22/7", RoundingMode::Nearest, -3, Ordering::Greater);

    test("7/2", RoundingMode::Floor, 3, Ordering::Less);
    test("7/2", RoundingMode::Ceiling, 4, Ordering::Greater);
    test("7/2", RoundingMode::Down, 3, Ordering::Less);
    test("7/2", RoundingMode::Up, 4, Ordering::Greater);
    test("7/2", RoundingMode::Nearest, 4, Ordering::Greater);

    test("9/2", RoundingMode::Floor, 4, Ordering::Less);
    test("9/2", RoundingMode::Ceiling, 5, Ordering::Greater);
    test("9/2", RoundingMode::Down, 4, Ordering::Less);
    test("9/2", RoundingMode::Up, 5, Ordering::Greater);
    test("9/2", RoundingMode::Nearest, 4, Ordering::Less);

    test(
        "-1000000000000",
        RoundingMode::Ceiling,
        i32::MIN,
        Ordering::Greater,
    );
    test(
        "-1000000000000",
        RoundingMode::Down,
        i32::MIN,
        Ordering::Greater,
    );
    test(
        "-1000000000000",
        RoundingMode::Nearest,
        i32::MIN,
        Ordering::Greater,
    );

    test(
        "1000000000000",
        RoundingMode::Floor,
        i32::MAX,
        Ordering::Less,
    );
    test(
        "1000000000000",
        RoundingMode::Down,
        i32::MAX,
        Ordering::Less,
    );
    test(
        "1000000000000",
        RoundingMode::Nearest,
        i32::MAX,
        Ordering::Less,
    );
}

#[test]
fn rounding_from_rational_fail() {
    let x = Rational::from_str("22/7").unwrap();
    assert_panic!(u32::rounding_from(&x, RoundingMode::Exact));

    let x = Rational::from_str("-123").unwrap();
    assert_panic!(u32::rounding_from(&x, RoundingMode::Floor));
    assert_panic!(u32::rounding_from(&x, RoundingMode::Up));
    assert_panic!(u32::rounding_from(&x, RoundingMode::Exact));

    let x = Rational::from_str("1000000000000").unwrap();
    assert_panic!(u32::rounding_from(&x, RoundingMode::Ceiling));
    assert_panic!(u32::rounding_from(&x, RoundingMode::Up));
    assert_panic!(u32::rounding_from(&x, RoundingMode::Exact));

    let x = Rational::from_str("22/7").unwrap();
    assert_panic!(i32::rounding_from(&x, RoundingMode::Exact));

    let x = Rational::from_str("-1000000000000").unwrap();
    assert_panic!(i32::rounding_from(&x, RoundingMode::Floor));
    assert_panic!(i32::rounding_from(&x, RoundingMode::Up));
    assert_panic!(i32::rounding_from(&x, RoundingMode::Exact));

    let x = Rational::from_str("1000000000000").unwrap();
    assert_panic!(i32::rounding_from(&x, RoundingMode::Ceiling));
    assert_panic!(i32::rounding_from(&x, RoundingMode::Up));
    assert_panic!(i32::rounding_from(&x, RoundingMode::Exact));
}

fn try_from_rational_properties_helper<
    T: for<'a> TryFrom<&'a Rational> + for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>()
where
    Rational: TryFrom<T> + PartialOrd<T>,
{
    rational_gen().test_properties(|x| {
        let p_x = T::try_from(&x);
        assert_eq!(p_x.is_ok(), x >= T::MIN && x <= T::MAX && x.is_integer());
        assert_eq!(p_x.is_ok(), T::convertible_from(&x));
        if let Ok(n) = p_x {
            assert_eq!(n.to_string(), x.to_string());
            assert_eq!(T::exact_from(&x), n);
            assert!(PartialEq::<Rational>::eq(&Rational::exact_from(n), &x));
        }
    });
}

#[test]
fn try_from_rational_properties() {
    apply_fn_to_primitive_ints!(try_from_rational_properties_helper);
}

fn convertible_from_rational_properties_helper<
    T: for<'a> ConvertibleFrom<&'a Rational> + PrimitiveInt,
>()
where
    Rational: PartialOrd<T>,
{
    rational_gen().test_properties(|x| {
        let convertible = T::convertible_from(&x);
        assert_eq!(convertible, x >= T::MIN && x <= T::MAX && x.is_integer());
    });
}

#[test]
fn convertible_from_rational_properties() {
    apply_fn_to_primitive_ints!(convertible_from_rational_properties_helper);
}

fn rounding_from_rational_properties_helper<
    T: for<'a> ConvertibleFrom<&'a Rational>
        + PartialEq<Integer>
        + PartialOrd<Rational>
        + PrimitiveInt
        + for<'a> RoundingFrom<&'a Rational>,
>()
where
    Rational: From<T> + PartialOrd<T>,
{
    rational_rounding_mode_pair_gen_var_3::<T>().test_properties(|(x, rm)| {
        let (n, o) = T::rounding_from(&x, rm);
        if x >= T::MIN && x <= T::MAX {
            assert!((Rational::from(n) - &x).lt_abs(&1));
        }

        assert_eq!(n.partial_cmp(&x), Some(o));
        match (x >= T::ZERO, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    // TODO use range
    rational_gen_var_3().test_properties(|x| {
        if x < T::MIN || x > T::MAX {
            return;
        }
        let floor = T::rounding_from(&x, RoundingMode::Floor);
        assert_eq!(floor.0, (&x).floor());
        assert!(floor.0 <= x);
        if floor.0 < T::MAX {
            assert!(floor.0 + T::ONE > x);
        }
        let ceiling = T::rounding_from(&x, RoundingMode::Ceiling);
        assert_eq!(ceiling.0, (&x).ceiling());
        assert!(ceiling.0 >= x);
        if ceiling.0 > T::MIN {
            assert!(ceiling.0 - T::ONE < x);
        }

        if x >= T::ZERO {
            assert_eq!(T::rounding_from(&x, RoundingMode::Down), floor);
            assert_eq!(T::rounding_from(&x, RoundingMode::Up), ceiling);
        } else {
            assert_eq!(T::rounding_from(&x, RoundingMode::Down), ceiling);
            assert_eq!(T::rounding_from(&x, RoundingMode::Up), floor);
        }

        let nearest = T::rounding_from(&x, RoundingMode::Nearest);
        assert!(nearest == floor || nearest == ceiling);
        assert!((Rational::from(nearest.0) - x).le_abs(&Rational::ONE_HALF));
    });
}

fn rounding_from_rational_properties_unsigned_helper<
    T: PrimitiveUnsigned + for<'a> RoundingFrom<&'a Rational>,
>()
where
    Rational: From<T>,
{
    unsigned_gen::<T>().test_properties(|n| {
        let no = (n, Ordering::Equal);
        let x = Rational::from(n);
        assert_eq!(T::rounding_from(&x, RoundingMode::Floor), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Down), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Ceiling), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Up), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Nearest), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Exact), no);
    });
}

fn rounding_from_rational_properties_signed_helper<
    T: PrimitiveSigned + for<'a> RoundingFrom<&'a Rational>,
>()
where
    Rational: From<T>,
{
    signed_gen::<T>().test_properties(|n| {
        let no = (n, Ordering::Equal);
        let x = Rational::from(n);
        assert_eq!(T::rounding_from(&x, RoundingMode::Floor), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Down), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Ceiling), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Up), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Nearest), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Exact), no);
    });
}

#[test]
fn rounding_from_rational_properties() {
    apply_fn_to_primitive_ints!(rounding_from_rational_properties_helper);
    apply_fn_to_unsigneds!(rounding_from_rational_properties_unsigned_helper);
    apply_fn_to_signeds!(rounding_from_rational_properties_signed_helper);
}
