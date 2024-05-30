// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_gen_var_13, primitive_float_gen_var_2,
    primitive_float_gen_var_3, primitive_float_gen_var_4,
    primitive_float_rounding_mode_pair_gen_var_1,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::cmp::Ordering::*;

#[test]
fn test_rounding_from_f32() {
    let test = |f: f32, rm: RoundingMode, out, o_out| {
        let (x, o) = Natural::rounding_from(f, rm);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(o, o_out);
    };
    test(0.0, Exact, "0", Equal);
    test(-0.0, Exact, "0", Equal);
    test(123.0, Exact, "123", Equal);
    test(1.0e9, Exact, "1000000000", Equal);
    test(1.0e9, Exact, "1000000000", Equal);
    test(4294967295.0, Exact, "4294967296", Equal);
    test(4294967296.0, Exact, "4294967296", Equal);
    test(18446744073709551615.0, Exact, "18446744073709551616", Equal);
    test(18446744073709551616.0, Exact, "18446744073709551616", Equal);
    test(1.0e20, Exact, "100000002004087734272", Equal);
    test(1.23e20, Exact, "122999999650278146048", Equal);
    test(1.6777216e7, Exact, "16777216", Equal);
    test(1.6777218e7, Exact, "16777218", Equal);

    test(123.1, Floor, "123", Less);
    test(123.1, Down, "123", Less);
    test(123.1, Ceiling, "124", Greater);
    test(123.1, Up, "124", Greater);
    test(123.1, Nearest, "123", Less);

    test(123.9, Floor, "123", Less);
    test(123.9, Down, "123", Less);
    test(123.9, Ceiling, "124", Greater);
    test(123.9, Up, "124", Greater);
    test(123.9, Nearest, "124", Greater);

    test(123.5, Nearest, "124", Greater);
    test(124.5, Nearest, "124", Less);
    test(-0.99, Ceiling, "0", Greater);
    test(-0.99, Down, "0", Greater);
    test(-0.499, Nearest, "0", Greater);
    test(-0.5, Nearest, "0", Greater);

    test(f32::NEGATIVE_INFINITY, Nearest, "0", Greater);
    test(f32::NEGATIVE_INFINITY, Down, "0", Greater);
    test(f32::NEGATIVE_INFINITY, Ceiling, "0", Greater);
    test(-123.0, Nearest, "0", Greater);
    test(-123.0, Down, "0", Greater);
    test(-123.0, Ceiling, "0", Greater);
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_1() {
    Natural::rounding_from(f32::NAN, Floor);
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_2() {
    Natural::rounding_from(f32::INFINITY, Floor);
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_3() {
    Natural::rounding_from(f32::NEGATIVE_INFINITY, Floor);
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_4() {
    Natural::rounding_from(123.1, Exact);
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_5() {
    Natural::rounding_from(-123.0, Floor);
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_6() {
    Natural::rounding_from(-0.1, Floor);
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_7() {
    Natural::rounding_from(-0.1, Up);
}

#[test]
fn test_rounding_from_f64() {
    let test = |f: f64, rm: RoundingMode, out, o_out| {
        let (x, o) = Natural::rounding_from(f, rm);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(o, o_out);
    };
    test(0.0, Exact, "0", Equal);
    test(-0.0, Exact, "0", Equal);
    test(123.0, Exact, "123", Equal);
    test(1.0e9, Exact, "1000000000", Equal);
    test(1.0e9, Exact, "1000000000", Equal);
    test(4294967295.0, Exact, "4294967295", Equal);
    test(4294967296.0, Exact, "4294967296", Equal);
    test(18446744073709551615.0, Exact, "18446744073709551616", Equal);
    test(18446744073709551616.0, Exact, "18446744073709551616", Equal);
    test(1.0e20, Exact, "100000000000000000000", Equal);
    test(1.23e20, Exact, "123000000000000000000", Equal);
    test(
        1.0e100,
        Exact,
        "100000000000000001590289110975991804683608085639452813897813275577478387721703810608134699\
        85856815104",
        Equal,
    );
    test(
        1.23e100,
        Exact,
        "123000000000000008366862950845375853795062237854139353014252897832358837028676639186389822\
        00322686976",
        Equal,
    );
    test(9.007199254740992e15, Exact, "9007199254740992", Equal);
    test(9.007199254740994e15, Exact, "9007199254740994", Equal);

    test(123.1, Floor, "123", Less);
    test(123.1, Down, "123", Less);
    test(123.1, Ceiling, "124", Greater);
    test(123.1, Up, "124", Greater);
    test(123.1, Nearest, "123", Less);

    test(123.9, Floor, "123", Less);
    test(123.9, Down, "123", Less);
    test(123.9, Ceiling, "124", Greater);
    test(123.9, Up, "124", Greater);
    test(123.9, Nearest, "124", Greater);

    test(123.5, Nearest, "124", Greater);
    test(124.5, Nearest, "124", Less);
    test(-0.99, Ceiling, "0", Greater);
    test(-0.99, Down, "0", Greater);
    test(-0.499, Nearest, "0", Greater);
    test(-0.5, Nearest, "0", Greater);

    test(f64::NEGATIVE_INFINITY, Nearest, "0", Greater);
    test(f64::NEGATIVE_INFINITY, Down, "0", Greater);
    test(f64::NEGATIVE_INFINITY, Ceiling, "0", Greater);

    test(-123.0, Nearest, "0", Greater);
    test(-123.0, Down, "0", Greater);
    test(-123.0, Ceiling, "0", Greater);
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_1() {
    Natural::rounding_from(f64::NAN, Floor);
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_2() {
    Natural::rounding_from(f64::INFINITY, Floor);
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_3() {
    Natural::rounding_from(f64::NEGATIVE_INFINITY, Floor);
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_4() {
    Natural::rounding_from(123.1, Exact);
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_5() {
    Natural::rounding_from(-123.0, Floor);
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_6() {
    Natural::rounding_from(-0.1, Floor);
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_7() {
    Natural::rounding_from(-0.1, Up);
}

#[test]
fn test_try_from_f32() {
    let test = |f: f32, out| {
        let on = Natural::try_from(f);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test(f32::NAN, "Err(FloatInfiniteOrNan)");
    test(f32::INFINITY, "Err(FloatInfiniteOrNan)");
    test(f32::NEGATIVE_INFINITY, "Err(FloatInfiniteOrNan)");
    test(0.0, "Ok(0)");
    test(-0.0, "Ok(0)");
    test(123.0, "Ok(123)");
    test(1.0e9, "Ok(1000000000)");
    test(4294967295.0, "Ok(4294967296)");
    test(4294967296.0, "Ok(4294967296)");
    test(18446744073709551615.0, "Ok(18446744073709551616)");
    test(18446744073709551616.0, "Ok(18446744073709551616)");
    test(1.0e20, "Ok(100000002004087734272)");
    test(1.23e20, "Ok(122999999650278146048)");
    test(123.1, "Err(FloatNonIntegerOrOutOfRange)");
    test(123.5, "Err(FloatNonIntegerOrOutOfRange)");
    test(124.5, "Err(FloatNonIntegerOrOutOfRange)");
    test(-0.99, "Err(FloatNegative)");
    test(-0.499, "Err(FloatNegative)");
    test(-0.5, "Err(FloatNegative)");
    test(-123.0, "Err(FloatNegative)");
    test(f32::MIN_POSITIVE, "Err(FloatNonIntegerOrOutOfRange)");
    test(f32::MAX_SUBNORMAL, "Err(FloatNonIntegerOrOutOfRange)");
    test(f32::MIN_POSITIVE_NORMAL, "Err(FloatNonIntegerOrOutOfRange)");
    test(
        f32::MAX_FINITE,
        "Ok(340282346638528859811704183484516925440)",
    );
}

#[test]
fn test_try_from_f64() {
    let test = |f: f64, out| {
        let on = Natural::try_from(f);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test(f64::NAN, "Err(FloatInfiniteOrNan)");
    test(f64::INFINITY, "Err(FloatInfiniteOrNan)");
    test(f64::NEGATIVE_INFINITY, "Err(FloatInfiniteOrNan)");
    test(0.0, "Ok(0)");
    test(-0.0, "Ok(0)");
    test(123.0, "Ok(123)");
    test(1.0e9, "Ok(1000000000)");
    test(4294967295.0, "Ok(4294967295)");
    test(4294967296.0, "Ok(4294967296)");
    test(18446744073709551615.0, "Ok(18446744073709551616)");
    test(18446744073709551616.0, "Ok(18446744073709551616)");
    test(1.0e20, "Ok(100000000000000000000)");
    test(1.23e20, "Ok(123000000000000000000)");
    test(
        1.0e100,
        "Ok(10000000000000000159028911097599180468360808563945281389781327557747838772170381060813\
        469985856815104)",
    );
    test(
        1.23e100,
        "Ok(12300000000000000836686295084537585379506223785413935301425289783235883702867663918638\
        982200322686976)",
    );
    test(123.1, "Err(FloatNonIntegerOrOutOfRange)");
    test(123.5, "Err(FloatNonIntegerOrOutOfRange)");
    test(124.5, "Err(FloatNonIntegerOrOutOfRange)");
    test(-0.99, "Err(FloatNegative)");
    test(-0.499, "Err(FloatNegative)");
    test(-0.5, "Err(FloatNegative)");
    test(-123.0, "Err(FloatNegative)");
    test(f64::MIN_POSITIVE, "Err(FloatNonIntegerOrOutOfRange)");
    test(f64::MAX_SUBNORMAL, "Err(FloatNonIntegerOrOutOfRange)");
    test(f64::MIN_POSITIVE_NORMAL, "Err(FloatNonIntegerOrOutOfRange)");
    test(
        f64::MAX_FINITE,
        "Ok(1797693134862315708145274237317043567980705675258449965989174768031572607800285387605\
        8955863276687817154045895351438246423432132688946418276846754670353751698604991057655128207\
        6245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723\
        168738177180919299881250404026184124858368)",
    );
}

#[test]
fn test_exact_from_f32() {
    let test = |f: f32, out| {
        let x = Natural::exact_from(f);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(0.0, "0");
    test(-0.0, "0");
    test(123.0, "123");
    test(1.0e9, "1000000000");
    test(4294967295.0, "4294967296");
    test(4294967296.0, "4294967296");
    test(18446744073709551615.0, "18446744073709551616");
    test(18446744073709551616.0, "18446744073709551616");
    test(1.0e20, "100000002004087734272");
    test(1.23e20, "122999999650278146048");
    test(f32::MAX_FINITE, "340282346638528859811704183484516925440");
}

#[test]
#[should_panic]
fn exact_from_f32_fail_1() {
    Natural::exact_from(f32::NAN);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_2() {
    Natural::exact_from(f32::INFINITY);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_3() {
    Natural::exact_from(f32::NEGATIVE_INFINITY);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_4() {
    Natural::exact_from(123.1);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_5() {
    Natural::exact_from(123.5);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_6() {
    Natural::exact_from(124.5);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_7() {
    Natural::exact_from(-0.99);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_8() {
    Natural::exact_from(-0.499);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_9() {
    Natural::exact_from(-0.5);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_10() {
    Natural::exact_from(-123.0);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_11() {
    Natural::exact_from(f32::MIN_POSITIVE);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_12() {
    Natural::exact_from(f32::MAX_SUBNORMAL);
}

#[test]
#[should_panic]
fn exact_from_f32_fail_13() {
    Natural::exact_from(f32::MIN_POSITIVE_NORMAL);
}

#[test]
fn test_exact_from_f64() {
    let test = |f: f64, out| {
        let x = Natural::exact_from(f);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(0.0, "0");
    test(-0.0, "0");
    test(123.0, "123");
    test(1.0e9, "1000000000");
    test(4294967295.0, "4294967295");
    test(4294967296.0, "4294967296");
    test(18446744073709551615.0, "18446744073709551616");
    test(18446744073709551616.0, "18446744073709551616");
    test(1.0e20, "100000000000000000000");
    test(1.23e20, "123000000000000000000");
    test(
        1.0e100,
        "100000000000000001590289110975991804683608085639452813897813275577478387721703810608134699\
        85856815104",
    );
    test(
        1.23e100,
        "123000000000000008366862950845375853795062237854139353014252897832358837028676639186389822\
        00322686976",
    );
    test(
        f64::MAX_FINITE,
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368",
    );
}

#[test]
#[should_panic]
fn exact_from_f64_fail_1() {
    Natural::exact_from(f64::NAN);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_2() {
    Natural::exact_from(f64::INFINITY);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_3() {
    Natural::exact_from(f64::NEGATIVE_INFINITY);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_4() {
    Natural::exact_from(123.1);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_5() {
    Natural::exact_from(123.5);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_6() {
    Natural::exact_from(124.5);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_7() {
    Natural::exact_from(-0.99);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_8() {
    Natural::exact_from(-0.499);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_9() {
    Natural::exact_from(-0.5);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_10() {
    Natural::exact_from(-123.0);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_11() {
    Natural::exact_from(f64::MIN_POSITIVE);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_12() {
    Natural::exact_from(f64::MAX_SUBNORMAL);
}

#[test]
#[should_panic]
fn exact_from_f64_fail_13() {
    Natural::exact_from(f64::MIN_POSITIVE_NORMAL);
}

#[test]
fn test_convertible_from_f32() {
    let test = |f: f32, out| {
        assert_eq!(Natural::convertible_from(f), out);
    };
    test(f32::NAN, false);
    test(f32::INFINITY, false);
    test(f32::NEGATIVE_INFINITY, false);
    test(0.0, true);
    test(-0.0, true);
    test(123.0, true);
    test(1.0e9, true);
    test(4294967295.0, true);
    test(4294967296.0, true);
    test(18446744073709551615.0, true);
    test(18446744073709551616.0, true);
    test(1.0e20, true);
    test(1.23e20, true);
    test(123.1, false);
    test(123.5, false);
    test(124.5, false);
    test(-0.99, false);
    test(-0.499, false);
    test(-0.5, false);
    test(-123.0, false);
    test(f32::MIN_POSITIVE, false);
    test(f32::MAX_SUBNORMAL, false);
    test(f32::MIN_POSITIVE_NORMAL, false);
    test(f32::MAX_FINITE, true);
}

#[test]
fn test_convertible_from_f64() {
    let test = |f: f64, out| {
        assert_eq!(Natural::convertible_from(f), out);
    };
    test(f64::NAN, false);
    test(f64::INFINITY, false);
    test(f64::NEGATIVE_INFINITY, false);
    test(0.0, true);
    test(-0.0, true);
    test(123.0, true);
    test(1.0e9, true);
    test(4294967295.0, true);
    test(4294967296.0, true);
    test(18446744073709551615.0, true);
    test(18446744073709551616.0, true);
    test(1.0e20, true);
    test(1.23e20, true);
    test(1.0e100, true);
    test(1.23e100, true);
    test(123.1, false);
    test(123.5, false);
    test(124.5, false);
    test(-0.99, false);
    test(-0.499, false);
    test(-0.5, false);
    test(-123.0, false);
    test(f64::MIN_POSITIVE, false);
    test(f64::MAX_SUBNORMAL, false);
    test(f64::MIN_POSITIVE_NORMAL, false);
    test(f64::MAX_FINITE, true);
}

#[test]
fn test_convertible_from_i64() {
    let test = |i: i64, out| {
        assert_eq!(Natural::convertible_from(i), out);
    };
    test(0, true);
    test(123, true);
    test(-123, false);
    test(i64::MAX, true);
    test(i64::MIN, false);
}

fn rounding_from_float_properties_helper<T: PrimitiveFloat + for<'a> RoundingFrom<&'a Natural>>()
where
    Natural: PartialOrd<T> + RoundingFrom<T>,
{
    primitive_float_rounding_mode_pair_gen_var_1::<T>().test_properties(|(f, rm)| {
        let (n, o) = Natural::rounding_from(f, rm);
        assert!(n.is_valid());
        assert_eq!(n.partial_cmp(&f), Some(o));
        match (f.is_sign_positive(), rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }
    });

    primitive_float_gen_var_2::<T>().test_properties(|f| {
        let no = Natural::rounding_from(f, Exact);
        assert!(no.0.is_valid());
        assert_eq!(no.1, Equal);
        assert_eq!(no, Natural::rounding_from(f, Floor));
        assert_eq!(no, Natural::rounding_from(f, Ceiling));
        assert_eq!(no, Natural::rounding_from(f, Down));
        assert_eq!(no, Natural::rounding_from(f, Up));
        assert_eq!(no, Natural::rounding_from(f, Nearest));
        let (f_alt, o) = T::rounding_from(&no.0, Exact);
        assert_eq!(o, Equal);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
    });

    primitive_float_gen_var_3::<T>().test_properties(|f| {
        let n_floor = Natural::rounding_from(f, Floor);
        assert!(n_floor.0.is_valid());
        assert_eq!(n_floor.1, Less);
        let n_ceiling = (&n_floor.0 + Natural::ONE, Greater);
        assert_eq!(n_ceiling, Natural::rounding_from(f, Ceiling));
        assert_eq!(n_floor, Natural::rounding_from(f, Down));
        assert_eq!(n_ceiling, Natural::rounding_from(f, Up));
        let n_nearest = Natural::rounding_from(f, Nearest);
        assert!(n_nearest == n_floor || n_nearest == n_ceiling);
    });

    primitive_float_gen_var_4::<T>().test_properties(|f| {
        let floor = Natural::rounding_from(f, Floor);
        let ceiling = (&floor.0 + Natural::ONE, Greater);
        let nearest = Natural::rounding_from(f, Nearest);
        assert_eq!(nearest, if floor.0.even() { floor } else { ceiling });
    });
}

#[test]
fn rounding_from_float_properties() {
    apply_fn_to_primitive_floats!(rounding_from_float_properties_helper);
}

fn try_from_float_properties_helper<T: PrimitiveFloat + for<'a> RoundingFrom<&'a Natural>>()
where
    Limb: TryFrom<NiceFloat<T>>,
    Natural: TryFrom<T> + RoundingFrom<T>,
    NiceFloat<T>: TryFrom<Limb>,
{
    primitive_float_gen::<T>().test_properties(|f| {
        let on = Natural::try_from(f);
        assert!(on.map_or(true, |n| n.is_valid()));
        if let Ok(n) = Limb::try_from(NiceFloat(f)) {
            assert_eq!(n, Natural::exact_from(f));
        }
    });

    primitive_float_gen_var_2::<T>().test_properties(|f| {
        let n = Natural::exact_from(f);
        assert!(n.is_valid());
        assert_eq!(n, Natural::rounding_from(f, Exact).0);
        assert_eq!(NiceFloat(T::rounding_from(&n, Exact).0), NiceFloat(f));
    });

    primitive_float_gen_var_3::<T>().test_properties(|f| {
        assert!(Natural::try_from(f).is_err());
    });

    primitive_float_gen_var_4::<T>().test_properties(|f| {
        assert!(Natural::try_from(f).is_err());
    });

    primitive_float_gen_var_13::<T, Limb>().test_properties(|f| {
        assert_eq!(Limb::exact_from(NiceFloat(f)), Natural::exact_from(f));
    });
}

#[test]
fn try_from_float_properties() {
    apply_fn_to_primitive_floats!(try_from_float_properties_helper);
}

fn convertible_from_float_properties_helper<T: PrimitiveFloat>()
where
    Natural: ConvertibleFrom<T>,
    Limb: ConvertibleFrom<T>,
{
    primitive_float_gen::<T>().test_properties(|f| {
        let nc = Natural::convertible_from(f);
        if Limb::convertible_from(f) {
            assert!(nc);
        }
    });

    primitive_float_gen_var_2::<T>().test_properties(|f| {
        assert!(Natural::convertible_from(f));
    });

    primitive_float_gen_var_3::<T>().test_properties(|f| {
        assert!(!Natural::convertible_from(f));
    });

    primitive_float_gen_var_4::<T>().test_properties(|f| {
        assert!(!Natural::convertible_from(f));
    });
}

#[test]
fn convertible_from_float_properties() {
    apply_fn_to_primitive_floats!(convertible_from_float_properties_helper);
}
