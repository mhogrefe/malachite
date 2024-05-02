// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::mantissa_and_exponent::sci_mantissa_and_exponent_round;
use malachite_base::num::conversion::traits::SciMantissaAndExponent;
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{
    primitive_float_unsigned_pair_gen_var_1, primitive_float_unsigned_pair_gen_var_2,
    primitive_float_unsigned_rounding_mode_triple_gen_var_1,
    primitive_float_unsigned_rounding_mode_triple_gen_var_2, unsigned_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_1,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen_var_2, natural_rounding_mode_pair_gen_var_2,
};
use std::cmp::Ordering;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_sci_mantissa_and_exponent() {
    let test = |s: &str, mantissa: f32, exponent: u64| {
        let n = Natural::from_str(s).unwrap();
        let (actual_mantissa, actual_exponent) = n.sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(actual_mantissa), NiceFloat(mantissa));
        assert_eq!(actual_exponent, exponent);
        assert_eq!(NiceFloat(n.sci_mantissa()), NiceFloat(mantissa));
        assert_eq!(
            SciMantissaAndExponent::<f32, u64, Natural>::sci_exponent(&n),
            exponent
        );
    };
    test("3", 1.5, 1);
    test("123", 1.921875, 6);
    test("1000000000", 1.8626451, 29);

    test("16777216", 1.0, 24);
    test("16777218", 1.0000001, 24);
    test("16777217", 1.0, 24);

    test("33554432", 1.0, 25);
    test("33554436", 1.0000001, 25);
    test("33554440", 1.0000002, 25);

    test("33554433", 1.0, 25);
    test("33554434", 1.0, 25);
    test("33554435", 1.0000001, 25);
    test("33554437", 1.0000001, 25);
    test("33554438", 1.0000002, 25);
    test("33554439", 1.0000002, 25);
    test("340282346638528859811704183484516925439", 1.9999999, 127);
    test("340282346638528859811704183484516925440", 1.9999999, 127);
    test("340282346638528859811704183484516925441", 1.9999999, 127);
    test(
        "10000000000000000000000000000000000000000000000000000",
        1.670478,
        172,
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        1.8920966,
        458,
    );
}

#[test]
fn test_sci_mantissa_and_exponent_round() {
    let test = |n: &str, rm: RoundingMode, out: Option<(f32, u64, Ordering)>| {
        let actual_out = Natural::from_str(n)
            .unwrap()
            .sci_mantissa_and_exponent_round(rm);
        assert_eq!(
            actual_out.map(|(m, e, o)| (NiceFloat(m), e, o)),
            out.map(|(m, e, o)| (NiceFloat(m), e, o))
        )
    };
    test("3", RoundingMode::Floor, Some((1.5, 1, Ordering::Equal)));
    test("3", RoundingMode::Down, Some((1.5, 1, Ordering::Equal)));
    test("3", RoundingMode::Ceiling, Some((1.5, 1, Ordering::Equal)));
    test("3", RoundingMode::Up, Some((1.5, 1, Ordering::Equal)));
    test("3", RoundingMode::Nearest, Some((1.5, 1, Ordering::Equal)));
    test("3", RoundingMode::Exact, Some((1.5, 1, Ordering::Equal)));

    test(
        "123",
        RoundingMode::Floor,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "123",
        RoundingMode::Down,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "123",
        RoundingMode::Ceiling,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "123",
        RoundingMode::Up,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "123",
        RoundingMode::Nearest,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "123",
        RoundingMode::Exact,
        Some((1.921875, 6, Ordering::Equal)),
    );

    test(
        "1000000000",
        RoundingMode::Floor,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "1000000000",
        RoundingMode::Down,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "1000000000",
        RoundingMode::Ceiling,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "1000000000",
        RoundingMode::Up,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "1000000000",
        RoundingMode::Nearest,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "1000000000",
        RoundingMode::Exact,
        Some((1.8626451, 29, Ordering::Equal)),
    );

    test(
        "16777216",
        RoundingMode::Floor,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "16777216",
        RoundingMode::Down,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "16777216",
        RoundingMode::Ceiling,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "16777216",
        RoundingMode::Up,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "16777216",
        RoundingMode::Nearest,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "16777216",
        RoundingMode::Exact,
        Some((1.0, 24, Ordering::Equal)),
    );

    test(
        "16777218",
        RoundingMode::Floor,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "16777218",
        RoundingMode::Down,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "16777218",
        RoundingMode::Ceiling,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "16777218",
        RoundingMode::Up,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "16777218",
        RoundingMode::Nearest,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "16777218",
        RoundingMode::Exact,
        Some((1.0000001, 24, Ordering::Equal)),
    );

    test(
        "16777217",
        RoundingMode::Floor,
        Some((1.0, 24, Ordering::Less)),
    );
    test(
        "16777217",
        RoundingMode::Down,
        Some((1.0, 24, Ordering::Less)),
    );
    test(
        "16777217",
        RoundingMode::Ceiling,
        Some((1.0000001, 24, Ordering::Greater)),
    );
    test(
        "16777217",
        RoundingMode::Up,
        Some((1.0000001, 24, Ordering::Greater)),
    );
    test(
        "16777217",
        RoundingMode::Nearest,
        Some((1.0, 24, Ordering::Less)),
    );
    test("16777217", RoundingMode::Exact, None);

    test(
        "33554432",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "33554432",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "33554432",
        RoundingMode::Ceiling,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "33554432",
        RoundingMode::Up,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "33554432",
        RoundingMode::Nearest,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "33554432",
        RoundingMode::Exact,
        Some((1.0, 25, Ordering::Equal)),
    );

    test(
        "33554436",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "33554436",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "33554436",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "33554436",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "33554436",
        RoundingMode::Nearest,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "33554436",
        RoundingMode::Exact,
        Some((1.0000001, 25, Ordering::Equal)),
    );

    test(
        "33554440",
        RoundingMode::Floor,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "33554440",
        RoundingMode::Down,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "33554440",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "33554440",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "33554440",
        RoundingMode::Nearest,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "33554440",
        RoundingMode::Exact,
        Some((1.0000002, 25, Ordering::Equal)),
    );

    test(
        "33554433",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554433",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554433",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554433",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554433",
        RoundingMode::Nearest,
        Some((1.0, 25, Ordering::Less)),
    );
    test("33554433", RoundingMode::Exact, None);

    test(
        "33554434",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554434",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554434",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554434",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554434",
        RoundingMode::Nearest,
        Some((1.0, 25, Ordering::Less)),
    );
    test("33554434", RoundingMode::Exact, None);

    test(
        "33554435",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554435",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554435",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554435",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554435",
        RoundingMode::Nearest,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test("33554435", RoundingMode::Exact, None);

    test(
        "33554437",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554437",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554437",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554437",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554437",
        RoundingMode::Nearest,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test("33554437", RoundingMode::Exact, None);

    test(
        "33554438",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554438",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554438",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554438",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554438",
        RoundingMode::Nearest,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test("33554438", RoundingMode::Exact, None);

    test(
        "33554439",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554439",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554439",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554439",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554439",
        RoundingMode::Nearest,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test("33554439", RoundingMode::Exact, None);

    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Floor,
        Some((1.9999998, 127, Ordering::Less)),
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Down,
        Some((1.9999998, 127, Ordering::Less)),
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Ceiling,
        Some((1.9999999, 127, Ordering::Greater)),
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Up,
        Some((1.9999999, 127, Ordering::Greater)),
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Nearest,
        Some((1.9999999, 127, Ordering::Greater)),
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Exact,
        None,
    );

    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Floor,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Down,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Ceiling,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Up,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Nearest,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Exact,
        Some((1.9999999, 127, Ordering::Equal)),
    );

    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Floor,
        Some((1.9999999, 127, Ordering::Less)),
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Down,
        Some((1.9999999, 127, Ordering::Less)),
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Ceiling,
        Some((1.0, 128, Ordering::Greater)),
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Up,
        Some((1.0, 128, Ordering::Greater)),
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Nearest,
        Some((1.9999999, 127, Ordering::Less)),
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Exact,
        None,
    );

    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Floor,
        Some((1.6704779, 172, Ordering::Less)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Down,
        Some((1.6704779, 172, Ordering::Less)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        Some((1.670478, 172, Ordering::Greater)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Up,
        Some((1.670478, 172, Ordering::Greater)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        Some((1.670478, 172, Ordering::Greater)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Exact,
        None,
    );

    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Floor,
        Some((1.8920966, 458, Ordering::Less)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Down,
        Some((1.8920966, 458, Ordering::Less)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Ceiling,
        Some((1.8920968, 458, Ordering::Greater)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Up,
        Some((1.8920968, 458, Ordering::Greater)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Nearest,
        Some((1.8920966, 458, Ordering::Less)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Exact,
        None,
    );

    test(
        "115792089210356248957287600559322556832945588264647333956682042358504754249728",
        RoundingMode::Floor,
        Some((1.9999999, 255, Ordering::Less)),
    );
    test(
        "115792089210356248957287600559322556832945588264647333956682042358504754249728",
        RoundingMode::Ceiling,
        Some((1.0, 256, Ordering::Greater)),
    );
}

#[test]
fn test_from_sci_mantissa_and_exponent() {
    let test = |mantissa: f32, exponent: u64, out: Option<&str>| {
        assert_eq!(
            <&Natural as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
                mantissa, exponent
            ),
            out.map(|s| Natural::from_str(s).unwrap())
        );
    };
    test(1.5, 1, Some("3"));
    test(1.51, 1, Some("3"));
    test(1.921875, 6, Some("123"));
    test(
        1.670478,
        172,
        Some("10000000254586612611935772707803116801852191350456320"),
    );

    test(2.0, 1, None);
    test(10.0, 1, None);
    test(0.5, 1, None);
}

fn from_sci_mantissa_and_exponent_fail_helper<T: PrimitiveFloat>()
where
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
{
    assert_panic!(
        <&Natural as SciMantissaAndExponent<T, u64, _>>::from_sci_mantissa_and_exponent(T::ZERO, 0)
    );
}

#[test]
fn from_sci_mantissa_and_exponent_fail() {
    apply_fn_to_primitive_floats!(from_sci_mantissa_and_exponent_fail_helper);
}

#[test]
fn test_from_sci_mantissa_and_exponent_round() {
    let test = |mantissa: f32, exponent: u64, rm: RoundingMode, out: Option<(&str, Ordering)>| {
        assert_eq!(
            Natural::from_sci_mantissa_and_exponent_round(mantissa, exponent, rm),
            out.map(|(s, o)| (Natural::from_str(s).unwrap(), o))
        );
    };
    test(1.5, 1, RoundingMode::Floor, Some(("3", Ordering::Equal)));
    test(1.5, 1, RoundingMode::Down, Some(("3", Ordering::Equal)));
    test(1.5, 1, RoundingMode::Ceiling, Some(("3", Ordering::Equal)));
    test(1.5, 1, RoundingMode::Up, Some(("3", Ordering::Equal)));
    test(1.5, 1, RoundingMode::Nearest, Some(("3", Ordering::Equal)));
    test(1.5, 1, RoundingMode::Exact, Some(("3", Ordering::Equal)));

    test(1.51, 1, RoundingMode::Floor, Some(("3", Ordering::Less)));
    test(1.51, 1, RoundingMode::Down, Some(("3", Ordering::Less)));
    test(
        1.51,
        1,
        RoundingMode::Ceiling,
        Some(("4", Ordering::Greater)),
    );
    test(1.51, 1, RoundingMode::Up, Some(("4", Ordering::Greater)));
    test(1.51, 1, RoundingMode::Nearest, Some(("3", Ordering::Less)));
    test(1.51, 1, RoundingMode::Exact, None);

    test(
        1.921875,
        6,
        RoundingMode::Floor,
        Some(("123", Ordering::Equal)),
    );
    test(
        1.921875,
        6,
        RoundingMode::Down,
        Some(("123", Ordering::Equal)),
    );
    test(
        1.921875,
        6,
        RoundingMode::Ceiling,
        Some(("123", Ordering::Equal)),
    );
    test(
        1.921875,
        6,
        RoundingMode::Up,
        Some(("123", Ordering::Equal)),
    );
    test(
        1.921875,
        6,
        RoundingMode::Nearest,
        Some(("123", Ordering::Equal)),
    );
    test(
        1.921875,
        6,
        RoundingMode::Exact,
        Some(("123", Ordering::Equal)),
    );

    test(
        1.670478,
        172,
        RoundingMode::Floor,
        Some((
            "10000000254586612611935772707803116801852191350456320",
            Ordering::Equal,
        )),
    );
    test(
        1.670478,
        172,
        RoundingMode::Down,
        Some((
            "10000000254586612611935772707803116801852191350456320",
            Ordering::Equal,
        )),
    );
    test(
        1.670478,
        172,
        RoundingMode::Ceiling,
        Some((
            "10000000254586612611935772707803116801852191350456320",
            Ordering::Equal,
        )),
    );
    test(
        1.670478,
        172,
        RoundingMode::Up,
        Some((
            "10000000254586612611935772707803116801852191350456320",
            Ordering::Equal,
        )),
    );
    test(
        1.670478,
        172,
        RoundingMode::Nearest,
        Some((
            "10000000254586612611935772707803116801852191350456320",
            Ordering::Equal,
        )),
    );
    test(
        1.670478,
        172,
        RoundingMode::Exact,
        Some((
            "10000000254586612611935772707803116801852191350456320",
            Ordering::Equal,
        )),
    );

    test(2.0, 1, RoundingMode::Floor, None);
    test(2.0, 1, RoundingMode::Down, None);
    test(2.0, 1, RoundingMode::Ceiling, None);
    test(2.0, 1, RoundingMode::Up, None);
    test(2.0, 1, RoundingMode::Nearest, None);
    test(2.0, 1, RoundingMode::Exact, None);

    test(10.0, 1, RoundingMode::Floor, None);
    test(10.0, 1, RoundingMode::Down, None);
    test(10.0, 1, RoundingMode::Ceiling, None);
    test(10.0, 1, RoundingMode::Up, None);
    test(10.0, 1, RoundingMode::Nearest, None);
    test(10.0, 1, RoundingMode::Exact, None);

    test(0.5, 1, RoundingMode::Floor, None);
    test(0.5, 1, RoundingMode::Down, None);
    test(0.5, 1, RoundingMode::Ceiling, None);
    test(0.5, 1, RoundingMode::Up, None);
    test(0.5, 1, RoundingMode::Nearest, None);
    test(0.5, 1, RoundingMode::Exact, None);
}

fn sci_mantissa_and_exponent_properties_helper<T: PrimitiveFloat>()
where
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
    Limb: SciMantissaAndExponent<T, u64>,
{
    natural_gen_var_2().test_properties(|n| {
        let (mantissa, exponent) = n.sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(n.sci_mantissa()), NiceFloat(mantissa));
        assert_eq!(n.sci_exponent(), exponent);
        assert!(mantissa >= T::ONE);
        assert!(mantissa < T::TWO);
        assert_eq!(
            n.sci_mantissa_and_exponent_round(RoundingMode::Nearest)
                .map(|(m, e, _): (T, u64, Ordering)| (NiceFloat(m), e)),
            Some((NiceFloat(mantissa), exponent))
        );
    });

    unsigned_gen_var_1::<Limb>().test_properties(|x| {
        let (mantissa_1, exponent_1) = x.sci_mantissa_and_exponent();
        let (mantissa_2, exponent_2) = Natural::from(x).sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(mantissa_1), NiceFloat(mantissa_2));
        assert_eq!(exponent_1, exponent_2);
    });
}

#[test]
fn sci_mantissa_and_exponent_properties() {
    apply_fn_to_primitive_floats!(sci_mantissa_and_exponent_properties_helper);
}

fn sci_mantissa_and_exponent_round_properties_helper<T: PrimitiveFloat>() {
    natural_rounding_mode_pair_gen_var_2().test_properties(|(n, rm)| {
        if let Some((mantissa, exponent, o)) = n.sci_mantissa_and_exponent_round::<T>(rm) {
            assert!(mantissa >= T::ONE);
            assert!(mantissa < T::TWO);
            if rm == RoundingMode::Exact {
                let n_alt = Natural::from_sci_mantissa_and_exponent_round(mantissa, exponent, rm)
                    .unwrap()
                    .0;
                assert_eq!(n_alt, n);
            }
            match rm {
                RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
                RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
                RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
                _ => {}
            }
        }
    });

    natural_gen_var_2().test_properties(|n| {
        let (floor_mantissa, floor_exponent, floor_o) = n
            .sci_mantissa_and_exponent_round::<T>(RoundingMode::Floor)
            .unwrap();
        assert_eq!(
            n.sci_mantissa_and_exponent_round::<T>(RoundingMode::Down)
                .unwrap(),
            (floor_mantissa, floor_exponent, floor_o)
        );
        let (ceiling_mantissa, ceiling_exponent, ceiling_o) = n
            .sci_mantissa_and_exponent_round::<T>(RoundingMode::Ceiling)
            .unwrap();
        assert_eq!(
            n.sci_mantissa_and_exponent_round::<T>(RoundingMode::Up)
                .unwrap(),
            (ceiling_mantissa, ceiling_exponent, ceiling_o)
        );
        let (nearest_mantissa, nearest_exponent, nearest_o) = n
            .sci_mantissa_and_exponent_round::<T>(RoundingMode::Nearest)
            .unwrap();
        if let Some((mantissa, exponent, o)) =
            n.sci_mantissa_and_exponent_round::<T>(RoundingMode::Exact)
        {
            assert_eq!(floor_mantissa, mantissa);
            assert_eq!(ceiling_mantissa, mantissa);
            assert_eq!(nearest_mantissa, mantissa);
            assert_eq!(floor_exponent, exponent);
            assert_eq!(ceiling_exponent, exponent);
            assert_eq!(nearest_exponent, exponent);
            assert_eq!(o, Ordering::Equal);
            assert_eq!(floor_o, Ordering::Equal);
            assert_eq!(ceiling_o, Ordering::Equal);
            assert_eq!(nearest_o, Ordering::Equal);
        } else {
            assert_eq!(floor_o, Ordering::Less);
            assert_eq!(ceiling_o, Ordering::Greater);
            assert_ne!(
                (floor_mantissa, floor_exponent),
                (ceiling_mantissa, ceiling_exponent)
            );
            assert!(
                (nearest_mantissa, nearest_exponent) == (floor_mantissa, floor_exponent)
                    || (nearest_mantissa, nearest_exponent) == (ceiling_mantissa, ceiling_exponent)
            );
            if ceiling_mantissa == T::ONE {
                assert_eq!(floor_mantissa, T::TWO.next_lower());
                assert_eq!(floor_exponent, ceiling_exponent - 1);
            } else {
                assert_eq!(floor_mantissa, ceiling_mantissa.next_lower());
                assert_eq!(floor_exponent, ceiling_exponent);
            }
        }
    });

    unsigned_rounding_mode_pair_gen_var_1::<Limb>().test_properties(|(x, rm)| {
        assert_eq!(
            sci_mantissa_and_exponent_round::<Limb, T>(x, rm).map(|(m, e, o)| (NiceFloat(m), e, o)),
            Natural::from(x)
                .sci_mantissa_and_exponent_round(rm)
                .map(|(m, e, o)| (NiceFloat(m), e, o))
        );
    });
}

#[test]
fn sci_mantissa_and_exponent_round_properties() {
    apply_fn_to_primitive_floats!(sci_mantissa_and_exponent_round_properties_helper);
}

fn from_sci_mantissa_and_exponent_properties_helper<T: PrimitiveFloat>()
where
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(m, e)| {
        let on =
            <&Natural as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e);
        assert_eq!(on.is_some(), m >= T::ONE && m < T::TWO);
    });

    primitive_float_unsigned_pair_gen_var_2::<T>().test_properties(|(m, e)| {
        let n = <&Natural as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e)
            .unwrap();
        assert!(m >= T::ONE && m < T::TWO);
        assert_eq!(
            Natural::from_sci_mantissa_and_exponent_round(m, e, RoundingMode::Nearest)
                .unwrap()
                .0,
            n
        );
    });
}

#[test]
fn from_sci_mantissa_and_exponent_properties() {
    apply_fn_to_primitive_floats!(from_sci_mantissa_and_exponent_properties_helper);
}

fn from_sci_mantissa_and_exponent_round_properties_helper<T: PrimitiveFloat>() {
    primitive_float_unsigned_rounding_mode_triple_gen_var_1::<T, u64>().test_properties(
        |(m, e, rm)| {
            let on = Natural::from_sci_mantissa_and_exponent_round(m, e, rm);
            if let Some((_, o)) = on {
                assert!(m >= T::ONE && m < T::TWO);
                match rm {
                    RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
                    RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
                    RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
                    _ => {}
                }
            } else {
                assert!(m < T::ONE || m >= T::TWO || rm == RoundingMode::Exact);
            }
        },
    );

    primitive_float_unsigned_rounding_mode_triple_gen_var_2::<T>().test_properties(|(m, e, rm)| {
        assert!(m >= T::ONE && m < T::TWO);
        let on = Natural::from_sci_mantissa_and_exponent_round(m, e, rm);
        if on.is_none() {
            assert_eq!(rm, RoundingMode::Exact);
        }
    });

    primitive_float_unsigned_pair_gen_var_2::<T>().test_properties(|(m, e)| {
        let floor_n =
            Natural::from_sci_mantissa_and_exponent_round(m, e, RoundingMode::Floor).unwrap();
        assert_eq!(
            Natural::from_sci_mantissa_and_exponent_round(m, e, RoundingMode::Down).unwrap(),
            floor_n
        );
        let ceiling_n =
            Natural::from_sci_mantissa_and_exponent_round(m, e, RoundingMode::Ceiling).unwrap();
        assert_eq!(
            Natural::from_sci_mantissa_and_exponent_round(m, e, RoundingMode::Up).unwrap(),
            ceiling_n
        );
        let nearest_n =
            Natural::from_sci_mantissa_and_exponent_round(m, e, RoundingMode::Nearest).unwrap();
        if let Some(n) = Natural::from_sci_mantissa_and_exponent_round(m, e, RoundingMode::Exact) {
            assert_eq!(floor_n, n);
            assert_eq!(ceiling_n, n);
            assert_eq!(nearest_n, n);
        } else {
            assert!(nearest_n == floor_n || nearest_n == ceiling_n);
            assert_eq!(ceiling_n, (floor_n.0 + Natural::ONE, Ordering::Greater));
        }
    });
}

#[test]
fn from_sci_mantissa_and_exponent_round_properties() {
    apply_fn_to_primitive_floats!(from_sci_mantissa_and_exponent_round_properties_helper);
}
