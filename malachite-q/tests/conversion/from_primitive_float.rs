// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::{primitive_float_gen, primitive_float_gen_var_8};
use malachite_q::Rational;

#[test]
fn test_try_from_f32() {
    let test = |f: f32, out| {
        let x = Rational::try_from(f);
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
    test(123.1, "Ok(16134963/131072)");
    test(-123.1, "Ok(-16134963/131072)");
    test(123.9, "Ok(16239821/131072)");
    test(-123.9, "Ok(-16239821/131072)");
    test(123.5, "Ok(247/2)");
    test(-123.5, "Ok(-247/2)");
    test(124.5, "Ok(249/2)");
    test(-124.5, "Ok(-249/2)");
    test(-0.499, "Ok(-8371831/16777216)");
    test(-0.5, "Ok(-1/2)");
    test(0.1, "Ok(13421773/134217728)");
    test(
        f32::MIN_POSITIVE_SUBNORMAL,
        "Ok(1/713623846352979940529142984724747568191373312)",
    );
    test(
        -f32::MIN_POSITIVE_SUBNORMAL,
        "Ok(-1/713623846352979940529142984724747568191373312)",
    );
    test(
        f32::MAX_SUBNORMAL,
        "Ok(8388607/713623846352979940529142984724747568191373312)",
    );
    test(
        -f32::MAX_SUBNORMAL,
        "Ok(-8388607/713623846352979940529142984724747568191373312)",
    );
    test(
        f32::MIN_POSITIVE_NORMAL,
        "Ok(1/85070591730234615865843651857942052864)",
    );
    test(
        -f32::MIN_POSITIVE_NORMAL,
        "Ok(-1/85070591730234615865843651857942052864)",
    );
    test(
        f32::MAX_FINITE,
        "Ok(340282346638528859811704183484516925440)",
    );
    test(
        -f32::MAX_FINITE,
        "Ok(-340282346638528859811704183484516925440)",
    );

    test(std::f32::consts::SQRT_2, "Ok(11863283/8388608)");
    test(std::f32::consts::PI, "Ok(13176795/4194304)");
    test(std::f32::consts::E, "Ok(2850325/1048576)");
}

#[test]
fn test_try_from_f64() {
    let test = |f: f64, out| {
        let x = Rational::try_from(f);
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
    test(123.1, "Ok(4331196204135219/35184372088832)");
    test(-123.1, "Ok(-4331196204135219/35184372088832)");
    test(123.9, "Ok(4359343701806285/35184372088832)");
    test(-123.9, "Ok(-4359343701806285/35184372088832)");
    test(123.5, "Ok(247/2)");
    test(-123.5, "Ok(-247/2)");
    test(124.5, "Ok(249/2)");
    test(-124.5, "Ok(-249/2)");
    test(-0.499, "Ok(-4494592428115755/9007199254740992)");
    test(-0.5, "Ok(-1/2)");
    test(
        f64::MIN_POSITIVE_SUBNORMAL,
        "Ok(1/202402253307310618352495346718917307049556649764142118356901358027430339567995346891\
        960383701437124495187077864316811911389808737385793476867013399940738509921517424276566361\
        364466907742093216341239767678472745068562007483424692698618103355649159556340810056512358\
        769552333414615230502532186327508646006263307707741093494784)",
    );
    test(
        -f64::MIN_POSITIVE_SUBNORMAL,
        "Ok(-1/20240225330731061835249534671891730704955664976414211835690135802743033956799534689\
        196038370143712449518707786431681191138980873738579347686701339994073850992151742427656636\
        136446690774209321634123976767847274506856200748342469269861810335564915955634081005651235\
        8769552333414615230502532186327508646006263307707741093494784)",
    );
    test(
        f64::MAX_SUBNORMAL,
        "Ok(4503599627370495/202402253307310618352495346718917307049556649764142118356901358027430\
        339567995346891960383701437124495187077864316811911389808737385793476867013399940738509921\
        517424276566361364466907742093216341239767678472745068562007483424692698618103355649159556\
        340810056512358769552333414615230502532186327508646006263307707741093494784)",
    );
    test(
        -f64::MAX_SUBNORMAL,
        "Ok(-4503599627370495/20240225330731061835249534671891730704955664976414211835690135802743\
        033956799534689196038370143712449518707786431681191138980873738579347686701339994073850992\
        151742427656636136446690774209321634123976767847274506856200748342469269861810335564915955\
        6340810056512358769552333414615230502532186327508646006263307707741093494784)",
    );
    test(
        f64::MIN_POSITIVE_NORMAL,
        "Ok(1/449423283715578976932326297697256183404494244735576643183575202894331689513752407831\
        771193306018840052800284699678483394146974422036041556232118576598685310944419733562163713\
        190755549003115235298632707380212514422095376705856157203684782776352068092908376276711465\
        74559986811484619929076208839082406056034304)",
    );
    test(
        -f64::MIN_POSITIVE_NORMAL,
        "Ok(-1/44942328371557897693232629769725618340449424473557664318357520289433168951375240783\
        177119330601884005280028469967848339414697442203604155623211857659868531094441973356216371\
        319075554900311523529863270738021251442209537670585615720368478277635206809290837627671146\
        574559986811484619929076208839082406056034304)",
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

    test(
        std::f64::consts::SQRT_2,
        "Ok(6369051672525773/4503599627370496)",
    );
    test(std::f64::consts::PI, "Ok(884279719003555/281474976710656)");
    test(std::f64::consts::E, "Ok(6121026514868073/2251799813685248)");
}

#[test]
fn test_rational_convertible_from_f32() {
    let test = |f: f32, out| {
        assert_eq!(Rational::convertible_from(f), out);
    };
    test(0.0, true);
    test(1.0, true);
    test(1.5, true);
    test(0.1, true);

    test(f32::NAN, false);
    test(f32::INFINITY, false);
    test(f32::NEGATIVE_INFINITY, false);
}

#[test]
fn test_rational_convertible_from_f64() {
    let test = |f: f64, out| {
        assert_eq!(Rational::convertible_from(f), out);
    };
    test(0.0, true);
    test(1.0, true);
    test(1.5, true);
    test(0.1, true);

    test(f64::NAN, false);
    test(f64::INFINITY, false);
    test(f64::NEGATIVE_INFINITY, false);
}

fn try_from_float_properties_helper<T: ExactFrom<Rational> + PrimitiveFloat>()
where
    Rational: ConvertibleFrom<T> + TryFrom<T>,
{
    primitive_float_gen::<T>().test_properties(|f| {
        let n = Rational::try_from(f);
        assert_eq!(n.is_ok(), Rational::convertible_from(f));
        if let Ok(n) = n {
            assert!(n.is_valid());
            assert_eq!(Rational::exact_from(-f), -&n);
            assert_eq!(
                NiceFloat(T::exact_from(n)),
                NiceFloat(f.abs_negative_zero())
            );
        }
    });
}

#[test]
fn try_from_float_properties() {
    apply_fn_to_primitive_floats!(try_from_float_properties_helper);

    primitive_float_gen_var_8::<f32>().test_properties(|f| {
        assert_eq!(
            Rational::exact_from(f),
            Rational::from(&rug::Rational::from_f32(f).unwrap())
        );
    });

    primitive_float_gen_var_8::<f64>().test_properties(|f| {
        assert_eq!(
            Rational::exact_from(f),
            Rational::from(&rug::Rational::from_f64(f).unwrap())
        );
    });
}

fn rational_convertible_from_primitive_float_properties_helper<T: PrimitiveFloat>()
where
    Rational: ConvertibleFrom<T>,
{
    primitive_float_gen().test_properties(|f| {
        assert_eq!(
            Rational::convertible_from(f),
            Rational::convertible_from(-f)
        );
    });
}

#[test]
fn rational_convertible_from_primitive_float_properties() {
    apply_fn_to_primitive_floats!(rational_convertible_from_primitive_float_properties_helper);
}
