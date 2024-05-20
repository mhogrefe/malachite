// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::primitive_float_gen_var_8;
use malachite_q::conversion::from_primitive_float::RationalFromPrimitiveFloatError;
use malachite_q::test_util::generators::rational_gen_var_7;
use malachite_q::Rational;

#[test]
fn test_try_from_f32_simplest() {
    let test = |f: f32, out| {
        let x = Rational::try_from_float_simplest(f);
        assert_eq!(x.to_debug_string(), out);
        if let Ok(x) = x {
            assert!(x.is_valid());
        }
    };
    test(f32::NAN, "Err(RationalFromPrimitiveFloatError)");
    test(f32::INFINITY, "Err(RationalFromPrimitiveFloatError)");
    test(
        f32::NEGATIVE_INFINITY,
        "Err(RationalFromPrimitiveFloatError)",
    );
    test(0.0, "Ok(0)");
    test(-0.0, "Ok(0)");
    test(123.0, "Ok(123)");
    test(-123.0, "Ok(-123)");
    test(1.0e9, "Ok(1000000000)");
    test(-1.0e9, "Ok(-1000000000)");
    test(4294967295.0, "Ok(4294967296)");
    test(-4294967295.0, "Ok(-4294967296)");
    test(4294967296.0, "Ok(4294967296)");
    test(-4294967296.0, "Ok(-4294967296)");
    test(18446744073709551615.0, "Ok(18446744073709551616)");
    test(-18446744073709551615.0, "Ok(-18446744073709551616)");
    test(18446744073709551616.0, "Ok(18446744073709551616)");
    test(-18446744073709551616.0, "Ok(-18446744073709551616)");
    test(1.0e20, "Ok(100000002004087734272)");
    test(-1.0e20, "Ok(-100000002004087734272)");
    test(1.23e20, "Ok(122999999650278146048)");
    test(-1.23e20, "Ok(-122999999650278146048)");
    test(123.1, "Ok(1231/10)");
    test(-123.1, "Ok(-1231/10)");
    test(123.9, "Ok(1239/10)");
    test(-123.9, "Ok(-1239/10)");
    test(123.5, "Ok(247/2)");
    test(-123.5, "Ok(-247/2)");
    test(124.5, "Ok(249/2)");
    test(-124.5, "Ok(-249/2)");
    test(-0.499, "Ok(-499/1000)");
    test(-0.5, "Ok(-1/2)");
    test(0.1, "Ok(1/10)");
    test(
        f32::MIN_POSITIVE_SUBNORMAL,
        "Ok(1/475749230901986627019428656483165045460915542)",
    );
    test(
        -f32::MIN_POSITIVE_SUBNORMAL,
        "Ok(-1/475749230901986627019428656483165045460915542)",
    );
    test(
        f32::MAX_SUBNORMAL,
        "Ok(1/85070596800837319010234175901631774785)",
    );
    test(
        -f32::MAX_SUBNORMAL,
        "Ok(-1/85070596800837319010234175901631774785)",
    );
    test(
        f32::MIN_POSITIVE_NORMAL,
        "Ok(1/85070586659632517184362935130987167681)",
    );
    test(
        -f32::MIN_POSITIVE_NORMAL,
        "Ok(-1/85070586659632517184362935130987167681)",
    );
    test(
        f32::MAX_FINITE,
        "Ok(340282346638528859811704183484516925440)",
    );
    test(
        -f32::MAX_FINITE,
        "Ok(-340282346638528859811704183484516925440)",
    );

    test(std::f32::consts::SQRT_2, "Ok(4756/3363)");
    test(std::f32::consts::PI, "Ok(93343/29712)");
    test(std::f32::consts::E, "Ok(2721/1001)");

    test(0.33333334, "Ok(1/3)");
    test(0.3333333, "Ok(3195660/9586981)");
}

#[test]
fn test_try_from_f64_simplest() {
    let test = |f: f64, out| {
        let x = Rational::try_from_float_simplest(f);
        assert_eq!(x.to_debug_string(), out);
        if let Ok(x) = x {
            assert!(x.is_valid());
        }
    };
    test(f64::NAN, "Err(RationalFromPrimitiveFloatError)");
    test(f64::INFINITY, "Err(RationalFromPrimitiveFloatError)");
    test(
        f64::NEGATIVE_INFINITY,
        "Err(RationalFromPrimitiveFloatError)",
    );
    test(0.0, "Ok(0)");
    test(-0.0, "Ok(0)");
    test(123.0, "Ok(123)");
    test(-123.0, "Ok(-123)");
    test(1.0e9, "Ok(1000000000)");
    test(-1.0e9, "Ok(-1000000000)");
    test(4294967295.0, "Ok(4294967295)");
    test(-4294967295.0, "Ok(-4294967295)");
    test(4294967296.0, "Ok(4294967296)");
    test(-4294967296.0, "Ok(-4294967296)");
    test(18446744073709551615.0, "Ok(18446744073709551616)");
    test(-18446744073709551615.0, "Ok(-18446744073709551616)");
    test(18446744073709551616.0, "Ok(18446744073709551616)");
    test(-18446744073709551616.0, "Ok(-18446744073709551616)");
    test(1.0e20, "Ok(100000000000000000000)");
    test(-1.0e20, "Ok(-100000000000000000000)");
    test(1.23e20, "Ok(123000000000000000000)");
    test(-1.23e20, "Ok(-123000000000000000000)");
    test(
        1.0e100,
        "Ok(10000000000000000159028911097599180468360808563945281389781327557747838772170381060813\
        469985856815104)",
    );
    test(
        -1.0e100,
        "Ok(-1000000000000000015902891109759918046836080856394528138978132755774783877217038106081\
        3469985856815104)",
    );
    test(
        1.23e100,
        "Ok(12300000000000000836686295084537585379506223785413935301425289783235883702867663918638\
        982200322686976)",
    );
    test(
        -1.23e100,
        "Ok(-1230000000000000083668629508453758537950622378541393530142528978323588370286766391863\
        8982200322686976)",
    );
    test(123.1, "Ok(1231/10)");
    test(-123.1, "Ok(-1231/10)");
    test(123.9, "Ok(1239/10)");
    test(-123.9, "Ok(-1239/10)");
    test(123.5, "Ok(247/2)");
    test(-123.5, "Ok(-247/2)");
    test(124.5, "Ok(249/2)");
    test(-124.5, "Ok(-249/2)");
    test(-0.499, "Ok(-499/1000)");
    test(-0.5, "Ok(-1/2)");
    test(
        f64::MIN_POSITIVE_SUBNORMAL,
        "Ok(1/134934835538207078901663564479278204699704433176094745571267572018286893045330231261\
        306922467624749663458051909544541274259872491590528984578008933293825673281011616184377574\
        242977938494728810894159845118981830045708004988949795132412068903766106370893873371008239\
        179701555609743487001688124218339097337508871805160728996523)",
    );
    test(
        -f64::MIN_POSITIVE_SUBNORMAL,
        "Ok(-1/13493483553820707890166356447927820469970443317609474557126757201828689304533023126\
        130692246762474966345805190954454127425987249159052898457800893329382567328101161618437757\
        424297793849472881089415984511898183004570800498894979513241206890376610637089387337100823\
        9179701555609743487001688124218339097337508871805160728996523)",
    );
    test(
        f64::MAX_SUBNORMAL,
        "Ok(1/449423283715579026828334036065257014383474871770369402158895481105061903177076758952\
        991447760350212414247530205607522865754608382719974023705572587637923509973673766132988527\
        495573581641607121816936686895147170329503865639571478615835487722625286251243883901055723\
        06631238281271786009918606677660346752204801)",
    );
    test(
        -f64::MAX_SUBNORMAL,
        "Ok(-1/44942328371557902682833403606525701438347487177036940215889548110506190317707675895\
        299144776035021241424753020560752286575460838271997402370557258763792350997367376613298852\
        749557358164160712181693668689514717032950386563957147861583548772262528625124388390105572\
        306631238281271786009918606677660346752204801)",
    );
    test(
        f64::MIN_POSITIVE_NORMAL,
        "Ok(1/449423283715578927036318559329266431564839219927211067229101097246506353572882385286\
        930040496229077692647131520826525095187415228669222594373726590254689108839132120159189741\
        421503549517982068519788656012202297150024869750306233834035420571989467252315767391654946\
        28061906253051009157224240331719237716377601)",
    );
    test(
        -f64::MIN_POSITIVE_NORMAL,
        "Ok(-1/44942328371557892703631855932926643156483921992721106722910109724650635357288238528\
        693004049622907769264713152082652509518741522866922259437372659025468910883913212015918974\
        142150354951798206851978865601220229715002486975030623383403542057198946725231576739165494\
        628061906253051009157224240331719237716377601)",
    );
    test(
        f64::MAX_FINITE,
        "Ok(17976931348623157081452742373170435679807056752584499659891747680315726078002853876058\
        955863276687817154045895351438246423432132688946418276846754670353751698604991057655128207\
        624549009038932894407586850845513394230458323690322294816580855933212334827479782620414472\
        3168738177180919299881250404026184124858368)",
    );
    test(
        -f64::MAX_FINITE,
        "Ok(-1797693134862315708145274237317043567980705675258449965989174768031572607800285387605\
        895586327668781715404589535143824642343213268894641827684675467035375169860499105765512820\
        762454900903893289440758685084551339423045832369032229481658085593321233482747978262041447\
        23168738177180919299881250404026184124858368)",
    );

    test(std::f64::consts::SQRT_2, "Ok(131836323/93222358)");
    test(std::f64::consts::PI, "Ok(245850922/78256779)");
    test(std::f64::consts::E, "Ok(268876667/98914198)");

    test(0.3333333333333333, "Ok(1/3)");
    test(0.3333333333333337, "Ok(279293000147008/837879000441023)");
}

#[allow(clippy::trait_duplication_in_bounds)]
fn try_from_float_simplest_properties_helper<
    T: for<'a> RoundingFrom<&'a Rational> + for<'a> TryFrom<&'a Rational> + PrimitiveFloat,
>()
where
    Rational: TryFrom<T, Error = RationalFromPrimitiveFloatError>,
{
    primitive_float_gen_var_8::<T>().test_properties(|f| {
        let q = Rational::try_from_float_simplest(f);
        if let Ok(q) = q {
            assert!(q.is_valid());
            assert_eq!(Rational::try_from_float_simplest(-f), Ok(-&q));
        }
    });

    rational_gen_var_7().test_properties(|q| {
        // This only works for simple `Rational`s, i.e. those `Rational`s q that round to a float x
        // such that no simpler `Rational` rounds to x.
        assert_eq!(
            Rational::try_from_float_simplest(T::rounding_from(&q, Nearest).0),
            Ok(q)
        );
    });
}

#[test]
fn try_from_float_simplest_properties() {
    apply_fn_to_primitive_floats!(try_from_float_simplest_properties_helper);
}
