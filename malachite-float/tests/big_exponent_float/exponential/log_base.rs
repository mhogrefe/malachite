// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_4,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_zero() || o == Equal,
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

fn log_base_prec_round_all(
    x: &BigExponentFloat,
    b: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().log_base_prec_round(b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.log_base_prec_round_ref(b, prec, rm), &r));
    let mut z = x.clone();
    let o = z.log_base_prec_round_assign(b, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().log_base_prec(b, prec), &r));
        assert!(same_pair(&x.log_base_prec_ref(b, prec), &r));
        let mut z = x.clone();
        let o = z.log_base_prec_assign(b, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn log_base_rational_prec_round_all(
    x: &Rational,
    b: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::log_base_rational_prec_round(x.clone(), b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::log_base_rational_prec_round_ref(x, b, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::log_base_rational_prec(x.clone(), b, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::log_base_rational_prec_ref(x, b, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_log_base_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let cases: &[(&str, &str, u64, RoundingMode, &str, Ordering)] = &[
        ("0x3.243f6a8885a30", "3", 1, Nearest, "0x1.0", Less),
        ("0x3.243f6a8885a30", "3", 10, Floor, "0x1.0a8", Less),
        ("0x3.243f6a8885a30", "3", 10, Ceiling, "0x1.0b", Greater),
        ("0x3.243f6a8885a30", "3", 10, Down, "0x1.0a8", Less),
        ("0x3.243f6a8885a30", "3", 10, Up, "0x1.0b", Greater),
        ("0x3.243f6a8885a30", "3", 10, Nearest, "0x1.0a8", Less),
        (
            "0x3.243f6a8885a30",
            "3",
            100,
            Nearest,
            "0x1.0abf12beafb95a5373307e18c",
            Less,
        ),
        ("0x0.8", "3", 1, Nearest, "-0x0.8", Greater),
        ("0x0.8", "3", 10, Floor, "-0x0.a1c", Less),
        ("0x0.8", "3", 10, Ceiling, "-0x0.a18", Greater),
        ("0x0.8", "3", 10, Down, "-0x0.a18", Greater),
        ("0x0.8", "3", 10, Up, "-0x0.a1c", Less),
        ("0x0.8", "3", 10, Nearest, "-0x0.a18", Greater),
        (
            "0x0.8",
            "3",
            100,
            Nearest,
            "-0x0.a1849cc1a9a9e94e043eaf779",
            Greater,
        ),
        ("0x3", "3", 1, Nearest, "0x1.0", Equal),
        ("0x3", "3", 10, Floor, "0x1.0", Equal),
        ("0x3", "3", 10, Ceiling, "0x1.0", Equal),
        ("0x3", "3", 10, Down, "0x1.0", Equal),
        ("0x3", "3", 10, Up, "0x1.0", Equal),
        ("0x3", "3", 10, Nearest, "0x1.0", Equal),
        ("0x3", "3", 100, Nearest, "0x1.0", Equal),
        ("0x3", "3", 300, Exact, "0x1.0", Equal),
        ("0x9", "3", 1, Nearest, "0x2.0", Equal),
        ("0x9", "3", 10, Floor, "0x2.0", Equal),
        ("0x9", "3", 10, Ceiling, "0x2.0", Equal),
        ("0x9", "3", 10, Down, "0x2.0", Equal),
        ("0x9", "3", 10, Up, "0x2.0", Equal),
        ("0x9", "3", 10, Nearest, "0x2.0", Equal),
        ("0x9", "3", 100, Nearest, "0x2.0", Equal),
        ("0x9", "3", 300, Exact, "0x2.0", Equal),
        ("0x1b", "3", 1, Nearest, "0x4.0", Greater),
        ("0x1b", "3", 10, Floor, "0x3.0", Equal),
        ("0x1b", "3", 10, Ceiling, "0x3.0", Equal),
        ("0x1b", "3", 10, Down, "0x3.0", Equal),
        ("0x1b", "3", 10, Up, "0x3.0", Equal),
        ("0x1b", "3", 10, Nearest, "0x3.0", Equal),
        ("0x1b", "3", 100, Nearest, "0x3.0", Equal),
        ("0x1b", "3", 300, Exact, "0x3.0", Equal),
        ("0x2.0E+20", "3", 1, Nearest, "0x4.0E+1", Greater),
        ("0x2.0E+20", "3", 10, Floor, "0x33.1", Less),
        ("0x2.0E+20", "3", 10, Ceiling, "0x33.2", Greater),
        ("0x2.0E+20", "3", 10, Down, "0x33.1", Less),
        ("0x2.0E+20", "3", 10, Up, "0x33.2", Greater),
        ("0x2.0E+20", "3", 10, Nearest, "0x33.2", Greater),
        (
            "0x2.0E+20",
            "3",
            100,
            Nearest,
            "0x33.1af59946aec2d1af57d584d4",
            Less,
        ),
        ("0x1.0E+300000000", "3", 1, Nearest, "0x2.0E+7", Less),
        ("0x1.0E+300000000", "3", 10, Floor, "0x2.d2E+7", Less),
        ("0x1.0E+300000000", "3", 10, Ceiling, "0x2.d3E+7", Greater),
        ("0x1.0E+300000000", "3", 10, Down, "0x2.d2E+7", Less),
        ("0x1.0E+300000000", "3", 10, Up, "0x2.d3E+7", Greater),
        ("0x1.0E+300000000", "3", 10, Nearest, "0x2.d2E+7", Less),
        (
            "0x1.0E+300000000",
            "3",
            100,
            Nearest,
            "0x2d20ab38.4926d76dad9a45cc4c",
            Greater,
        ),
        ("0x1.8E-300000000", "3", 1, Nearest, "-0x2.0E+7", Greater),
        ("0x1.8E-300000000", "3", 10, Floor, "-0x2.d3E+7", Less),
        ("0x1.8E-300000000", "3", 10, Ceiling, "-0x2.d2E+7", Greater),
        ("0x1.8E-300000000", "3", 10, Down, "-0x2.d2E+7", Greater),
        ("0x1.8E-300000000", "3", 10, Up, "-0x2.d3E+7", Less),
        ("0x1.8E-300000000", "3", 10, Nearest, "-0x2.d2E+7", Greater),
        (
            "0x1.8E-300000000",
            "3",
            100,
            Nearest,
            "-0x2d20ab37.eaab742f57442f1a5",
            Less,
        ),
        ("-0x3", "3", 1, Nearest, "NaN", Equal),
        ("-0x3", "3", 10, Floor, "NaN", Equal),
        ("-0x3", "3", 10, Ceiling, "NaN", Equal),
        ("-0x3", "3", 10, Down, "NaN", Equal),
        ("-0x3", "3", 10, Up, "NaN", Equal),
        ("-0x3", "3", 10, Nearest, "NaN", Equal),
        ("-0x3", "3", 100, Nearest, "NaN", Equal),
        ("-0x3", "3", 300, Exact, "NaN", Equal),
        ("0x1.0", "3", 1, Nearest, "0x0.0", Equal),
        ("0x1.0", "3", 10, Floor, "0x0.0", Equal),
        ("0x1.0", "3", 10, Ceiling, "0x0.0", Equal),
        ("0x1.0", "3", 10, Down, "0x0.0", Equal),
        ("0x1.0", "3", 10, Up, "0x0.0", Equal),
        ("0x1.0", "3", 10, Nearest, "0x0.0", Equal),
        ("0x1.0", "3", 100, Nearest, "0x0.0", Equal),
        ("0x1.0", "3", 300, Exact, "0x0.0", Equal),
        ("0x0.ffffffff", "3", 1, Nearest, "-0x1.0E-8", Less),
        ("0x0.ffffffff", "3", 10, Floor, "-0xe.94E-9", Less),
        ("0x0.ffffffff", "3", 10, Ceiling, "-0xe.9E-9", Greater),
        ("0x0.ffffffff", "3", 10, Down, "-0xe.9E-9", Greater),
        ("0x0.ffffffff", "3", 10, Up, "-0xe.94E-9", Less),
        ("0x0.ffffffff", "3", 10, Nearest, "-0xe.9E-9", Greater),
        (
            "0x0.ffffffff",
            "3",
            100,
            Nearest,
            "-0xe.905701e5a8d4d9ad73367453E-9",
            Greater,
        ),
        ("0x2540be400.0", "3", 1, Nearest, "0x1.0E+1", Less),
        ("0x2540be400.0", "3", 10, Floor, "0x14.f", Less),
        ("0x2540be400.0", "3", 10, Ceiling, "0x14.f8", Greater),
        ("0x2540be400.0", "3", 10, Down, "0x14.f", Less),
        ("0x2540be400.0", "3", 10, Up, "0x14.f8", Greater),
        ("0x2540be400.0", "3", 10, Nearest, "0x14.f8", Greater),
        (
            "0x2540be400.0",
            "3",
            100,
            Nearest,
            "0x14.f5832b7a85b1dbc1c4d0f94c",
            Greater,
        ),
        (
            "0x3.c2e44f14928E+385781034",
            "3",
            10,
            Nearest,
            "0x3.a1E+7",
            Greater,
        ),
        (
            "0x4.40fc7d4464E-385781035",
            "3",
            10,
            Nearest,
            "-0x3.a1E+7",
            Less,
        ),
        (
            "0x6.f2ba546e9E+383703500",
            "3",
            10,
            Nearest,
            "0x3.9bE+7",
            Less,
        ),
        (
            "0x2.4d824b004f8E-383703501",
            "3",
            10,
            Nearest,
            "-0x3.9bE+7",
            Greater,
        ),
        (
            "0x7.6f8b7486146E+410294807",
            "3",
            10,
            Nearest,
            "0x3.dcE+7",
            Greater,
        ),
        (
            "0x2.26dab9fb138E-410294808",
            "3",
            10,
            Nearest,
            "-0x3.dcE+7",
            Less,
        ),
        (
            "0x3.26d7404860dE+373316333",
            "3",
            10,
            Nearest,
            "0x3.82E+7",
            Less,
        ),
        (
            "0x5.139b65afbe3E-373316334",
            "3",
            10,
            Nearest,
            "-0x3.82E+7",
            Greater,
        ),
        (
            "0x1.9692eb744c01E+399907595",
            "3",
            10,
            Nearest,
            "0x3.c3E+7",
            Greater,
        ),
        (
            "0xa.130dc9d1199E-399907596",
            "3",
            10,
            Nearest,
            "-0x3.c3E+7",
            Less,
        ),
        (
            "0x5.232776a7d89E+377471219",
            "3",
            10,
            Nearest,
            "0x3.8cE+7",
            Less,
        ),
        (
            "0x3.1d4d7fd56046E-377471220",
            "3",
            10,
            Nearest,
            "-0x3.8cE+7",
            Greater,
        ),
        (
            "0x2.eb7e706e5180eE+379548662",
            "3",
            10,
            Nearest,
            "0x3.92E+7",
            Greater,
        ),
        (
            "0x5.7ac9e83255c5E-379548663",
            "3",
            10,
            Nearest,
            "-0x3.92E+7",
            Less,
        ),
        (
            "0xc.818e63d73381E+422759456",
            "3",
            10,
            Nearest,
            "0x3.f9E+7",
            Less,
        ),
        (
            "0x1.47854e0063788E-422759457",
            "3",
            10,
            Nearest,
            "-0x3.f9E+7",
            Greater,
        ),
        (
            "0x3.82e1d3cf4aE+419227855",
            "3",
            10,
            Floor,
            "0x3.f1E+7",
            Less,
        ),
        (
            "0x4.8e888a3691E-419227856",
            "3",
            10,
            Floor,
            "-0x3.f2E+7",
            Less,
        ),
        (
            "0x3.82e1d3cf4aE+419227855",
            "3",
            10,
            Ceiling,
            "0x3.f2E+7",
            Greater,
        ),
        (
            "0x4.8e888a3691E-419227856",
            "3",
            10,
            Ceiling,
            "-0x3.f1E+7",
            Greater,
        ),
        (
            "0x2.038da1368a8E+405101151",
            "3",
            10,
            Floor,
            "0x3.ceE+7",
            Less,
        ),
        (
            "0x7.f1e28edcf6E-405101152",
            "3",
            10,
            Floor,
            "-0x3.cfE+7",
            Less,
        ),
        (
            "0x2.038da1368a8E+405101151",
            "3",
            10,
            Ceiling,
            "0x3.cfE+7",
            Greater,
        ),
        (
            "0x7.f1e28edcf6E-405101152",
            "3",
            10,
            Ceiling,
            "-0x3.ceE+7",
            Greater,
        ),
        (
            "0x1.0145d6810ef4E+393467527",
            "3",
            10,
            Floor,
            "0x3.b3E+7",
            Less,
        ),
        (
            "0xf.ebbc62c06f2E-393467528",
            "3",
            10,
            Floor,
            "-0x3.b4E+7",
            Less,
        ),
        (
            "0x1.0145d6810ef4E+393467527",
            "3",
            10,
            Ceiling,
            "0x3.b4E+7",
            Greater,
        ),
        (
            "0xf.ebbc62c06f2E-393467528",
            "3",
            10,
            Ceiling,
            "-0x3.b3E+7",
            Greater,
        ),
        (
            "0xb.a2b6bcecdacE+390143613",
            "3",
            10,
            Floor,
            "0x3.aaE+7",
            Less,
        ),
        (
            "0x1.6005e84663dE-390143614",
            "3",
            10,
            Floor,
            "-0x3.abE+7",
            Less,
        ),
        (
            "0xb.a2b6bcecdacE+390143613",
            "3",
            10,
            Ceiling,
            "0x3.abE+7",
            Greater,
        ),
        (
            "0x1.6005e84663dE-390143614",
            "3",
            10,
            Ceiling,
            "-0x3.aaE+7",
            Greater,
        ),
        (
            "0x2.4a4f88ae1be6E+405101200",
            "3",
            10,
            Floor,
            "0x3.cfE+7",
            Less,
        ),
        (
            "0x6.fc6e40459304E-405101201",
            "3",
            10,
            Floor,
            "-0x3.dE+7",
            Less,
        ),
        (
            "0x2.4a4f88ae1be6E+405101200",
            "3",
            10,
            Ceiling,
            "0x3.dE+7",
            Greater,
        ),
        (
            "0x6.fc6e40459304E-405101201",
            "3",
            10,
            Ceiling,
            "-0x3.cfE+7",
            Greater,
        ),
        (
            "0x1.3428b1ab8663E+381002871",
            "3",
            10,
            Floor,
            "0x3.94E+7",
            Less,
        ),
        (
            "0xd.4ab61c3efefE-381002872",
            "3",
            10,
            Floor,
            "-0x3.95E+7",
            Less,
        ),
        (
            "0x1.3428b1ab8663E+381002871",
            "3",
            10,
            Ceiling,
            "0x3.95E+7",
            Greater,
        ),
        (
            "0xd.4ab61c3efefE-381002872",
            "3",
            10,
            Ceiling,
            "-0x3.94E+7",
            Greater,
        ),
        (
            "0xe.02e796dccbb68E+403439245",
            "3",
            10,
            Floor,
            "0x3.cbE+7",
            Less,
        ),
        (
            "0x1.2455a23a46664E-403439246",
            "3",
            10,
            Floor,
            "-0x3.ccE+7",
            Less,
        ),
        (
            "0xe.02e796dccbb68E+403439245",
            "3",
            10,
            Ceiling,
            "0x3.ccE+7",
            Greater,
        ),
        (
            "0x1.2455a23a46664E-403439246",
            "3",
            10,
            Ceiling,
            "-0x3.cbE+7",
            Greater,
        ),
        (
            "0x5.d0051b12e6f3E+376847987",
            "3",
            10,
            Floor,
            "0x3.8aE+7",
            Less,
        ),
        (
            "0x2.c0adc10bd95fE-376847988",
            "3",
            10,
            Floor,
            "-0x3.8bE+7",
            Less,
        ),
        (
            "0x5.d0051b12e6f3E+376847987",
            "3",
            10,
            Ceiling,
            "0x3.8bE+7",
            Greater,
        ),
        (
            "0x2.c0adc10bd95fE-376847988",
            "3",
            10,
            Ceiling,
            "-0x3.8aE+7",
            Greater,
        ),
        (
            "0x2.a330a884e786d5bf3827f767160d076bb4E+400857223",
            "3",
            100,
            Nearest,
            "0x3c4c91b4.94e90088fa7a4d7d84",
            Greater,
        ),
        (
            "0x6.110234d2421443db77d11b855703fc19f8E-400857224",
            "3",
            100,
            Nearest,
            "-0x3c4c91b4.94e90088fa7a4d7d84",
            Less,
        ),
        (
            "0x1.a945d7bfab488e9b6801173f6e310f28a2E+410102862",
            "3",
            100,
            Nearest,
            "0x3db09bc7.1abc1a7ba3c5133c6",
            Less,
        ),
        (
            "0x9.a1a7a45875aace6d0b6a9755f7e052a84E-410102863",
            "3",
            100,
            Nearest,
            "-0x3db09bc7.1abc1a7ba3c5133c6",
            Greater,
        ),
        (
            "0x8.d91ffa2631b9e836c07bea3ed8627eaed5E+422482619",
            "3",
            100,
            Nearest,
            "0x3f8d56dc.c29505464cf635dc84",
            Greater,
        ),
        (
            "0x1.ceec03bd16e32427dab050c70da2168c224E-422482620",
            "3",
            100,
            Nearest,
            "-0x3f8d56dc.c29505464cf635dc84",
            Less,
        ),
        (
            "0xc.cb71685d5dde295f31ba122c61f123d9a9E+411776077",
            "3",
            100,
            Nearest,
            "0x3df10acd.79209d33fbf7a865a4",
            Less,
        ),
        (
            "0x1.4021f067fd660ac433b3ff3dc03473d1838E-411776078",
            "3",
            100,
            Nearest,
            "-0x3df10acd.79209d33fbf7a865a4",
            Greater,
        ),
        (
            "0x6.9f3bcc11148ea2895adc569194a36a1e122E+404450990",
            "3",
            100,
            Nearest,
            "0x3cd6f60f.87cb1861c94892df5",
            Greater,
        ),
        (
            "0x2.6a8b2de48c966a9258e38b1c4fa10dbbf36E-404450991",
            "3",
            100,
            Nearest,
            "-0x3cd6f60f.87cb1861c94892df5",
            Less,
        ),
        (
            "0x8.3d78940562081bce483000ff8e0a8c6432E+401790115",
            "3",
            100,
            Nearest,
            "0x3c707e6a.e5b559121225f20a3c",
            Less,
        ),
        (
            "0x1.f1147f927face5a309359d137511e82544b8E-401790116",
            "3",
            100,
            Nearest,
            "-0x3c707e6a.e5b559121225f20a3c",
            Greater,
        ),
        (
            "0xc.33964e92b247c3d6bb35b3065c71d08d0bd4E+403310299",
            "3",
            100,
            Nearest,
            "0x3cab08d0.85476e94f7ed5d3564",
            Greater,
        ),
        (
            "0x1.4fb2333231f7fc111e4c82c0510a6770c68cE-403310300",
            "3",
            100,
            Nearest,
            "-0x3cab08d0.85476e94f7ed5d3564",
            Less,
        ),
        (
            "0x1.41494b8f6cd0ffb8c14ed32d3596c5fe40ecE+402613448",
            "3",
            100,
            Nearest,
            "0x3c90330e.54e01df9459d143d6",
            Less,
        ),
        (
            "0xc.bfae51571fa5157533fd892ca8435dd0929cE-402613449",
            "3",
            100,
            Nearest,
            "-0x3c90330e.54e01df9459d143d6",
            Greater,
        ),
        (
            "0x2.93f906ed5db9e0e68e133533e2144ef484E+404872214",
            "3",
            100,
            Floor,
            "0x3ce72e99.b097ddb06f4373113c",
            Less,
        ),
        (
            "0x6.34d12c800e6576986922e6c362db57cd3E-404872215",
            "3",
            100,
            Floor,
            "-0x3ce72e99.b097ddb06f4373114",
            Less,
        ),
        (
            "0x2.93f906ed5db9e0e68e133533e2144ef484E+404872214",
            "3",
            100,
            Ceiling,
            "0x3ce72e99.b097ddb06f4373114",
            Greater,
        ),
        (
            "0x6.34d12c800e6576986922e6c362db57cd3E-404872215",
            "3",
            100,
            Ceiling,
            "-0x3ce72e99.b097ddb06f4373113c",
            Greater,
        ),
        (
            "0x1.250b843c5a3895be3f008838f578247624E+408353308",
            "3",
            100,
            Floor,
            "0x3d6d3c30.11b2dcca920538bd08",
            Less,
        ),
        (
            "0xd.fa35527aaa11dfc56e359cf922f25b89eE-408353309",
            "3",
            100,
            Floor,
            "-0x3d6d3c30.11b2dcca920538bd0c",
            Less,
        ),
        (
            "0x1.250b843c5a3895be3f008838f578247624E+408353308",
            "3",
            100,
            Ceiling,
            "0x3d6d3c30.11b2dcca920538bd0c",
            Greater,
        ),
        (
            "0xd.fa35527aaa11dfc56e359cf922f25b89eE-408353309",
            "3",
            100,
            Ceiling,
            "-0x3d6d3c30.11b2dcca920538bd08",
            Greater,
        ),
        (
            "0x8.e7a7870cf9164bb6fd59a572420fe8d681E+381676719",
            "3",
            100,
            Floor,
            "0x3969f2fb.0a75d15b095494d3a4",
            Less,
        ),
        (
            "0x1.cbf8b29df039b367c1249e9cbdda84845aE-381676720",
            "3",
            100,
            Floor,
            "-0x3969f2fb.0a75d15b095494d3a8",
            Less,
        ),
        (
            "0x8.e7a7870cf9164bb6fd59a572420fe8d681E+381676719",
            "3",
            100,
            Ceiling,
            "0x3969f2fb.0a75d15b095494d3a8",
            Greater,
        ),
        (
            "0x1.cbf8b29df039b367c1249e9cbdda84845aE-381676720",
            "3",
            100,
            Ceiling,
            "-0x3969f2fb.0a75d15b095494d3a4",
            Greater,
        ),
        (
            "0x6.eb0b30e42d93a09390025ff3bbad17c9628E+425427013",
            "3",
            100,
            Floor,
            "0x3ffeb983.a8d2739687d43e5e6",
            Less,
        ),
        (
            "0x2.501112f6c9646d83ab39f31fc3e212c526cE-425427014",
            "3",
            100,
            Floor,
            "-0x3ffeb983.a8d2739687d43e5e64",
            Less,
        ),
        (
            "0x6.eb0b30e42d93a09390025ff3bbad17c9628E+425427013",
            "3",
            100,
            Ceiling,
            "0x3ffeb983.a8d2739687d43e5e64",
            Greater,
        ),
        (
            "0x2.501112f6c9646d83ab39f31fc3e212c526cE-425427014",
            "3",
            100,
            Ceiling,
            "-0x3ffeb983.a8d2739687d43e5e6",
            Greater,
        ),
        (
            "0x5.cd1c0284f903d3cc43a11ba2aaf047e6996E+375557533",
            "3",
            100,
            Floor,
            "0x387e4e58.97341beb09f8a65c0c",
            Less,
        ),
        (
            "0x2.c20f4c9b482f926a729abeb28eee8e230baE-375557534",
            "3",
            100,
            Floor,
            "-0x387e4e58.97341beb09f8a65c1",
            Less,
        ),
        (
            "0x5.cd1c0284f903d3cc43a11ba2aaf047e6996E+375557533",
            "3",
            100,
            Ceiling,
            "0x387e4e58.97341beb09f8a65c1",
            Greater,
        ),
        (
            "0x2.c20f4c9b482f926a729abeb28eee8e230baE-375557534",
            "3",
            100,
            Ceiling,
            "-0x387e4e58.97341beb09f8a65c0c",
            Greater,
        ),
        (
            "0x3.2263b338e20e7e6ce72eaa0e03b140dc427E+411108982",
            "3",
            100,
            Floor,
            "0x3dd75a63.db61f395defdb8c908",
            Less,
        ),
        (
            "0x5.1ad118572536310d0e0041d8d69832a78acE-411108983",
            "3",
            100,
            Floor,
            "-0x3dd75a63.db61f395defdb8c90c",
            Less,
        ),
        (
            "0x3.2263b338e20e7e6ce72eaa0e03b140dc427E+411108982",
            "3",
            100,
            Ceiling,
            "0x3dd75a63.db61f395defdb8c90c",
            Greater,
        ),
        (
            "0x5.1ad118572536310d0e0041d8d69832a78acE-411108983",
            "3",
            100,
            Ceiling,
            "-0x3dd75a63.db61f395defdb8c908",
            Greater,
        ),
        (
            "0x7.3a23b464857c90009040f30127a7b03cbcc6E+388433245",
            "3",
            100,
            Floor,
            "0x3a6e22af.fcd041525d046087b4",
            Less,
        ),
        (
            "0x2.36c14ee7557f88ceda6ddcd88b18fb0b465cE-388433246",
            "3",
            100,
            Floor,
            "-0x3a6e22af.fcd041525d046087b8",
            Less,
        ),
        (
            "0x7.3a23b464857c90009040f30127a7b03cbcc6E+388433245",
            "3",
            100,
            Ceiling,
            "0x3a6e22af.fcd041525d046087b8",
            Greater,
        ),
        (
            "0x2.36c14ee7557f88ceda6ddcd88b18fb0b465cE-388433246",
            "3",
            100,
            Ceiling,
            "-0x3a6e22af.fcd041525d046087b4",
            Greater,
        ),
        (
            "0x1.b11c39504871018b3a74adf5783f2146962dE+389903146",
            "3",
            100,
            Floor,
            "0x3aa6bd5f.c4c84be66bc3e901f8",
            Less,
        ),
        (
            "0x9.75098207edbde76efee3e8ae080cf4669ce8E-389903147",
            "3",
            100,
            Floor,
            "-0x3aa6bd5f.c4c84be66bc3e901fc",
            Less,
        ),
        (
            "0x1.b11c39504871018b3a74adf5783f2146962dE+389903146",
            "3",
            100,
            Ceiling,
            "0x3aa6bd5f.c4c84be66bc3e901fc",
            Greater,
        ),
        (
            "0x9.75098207edbde76efee3e8ae080cf4669ce8E-389903147",
            "3",
            100,
            Ceiling,
            "-0x3aa6bd5f.c4c84be66bc3e901f8",
            Greater,
        ),
    ];
    for &(x, b, prec, rm, out, o) in cases {
        let r = log_base_prec_round_all(&hex(x), b.parse::<u64>().unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out, "{x} {b} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn log_base_prec_round_fail_1() {
    BigExponentFloat::from(5).log_base_prec_round(3, 0, Floor);
}

#[test]
#[should_panic]
fn log_base_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).log_base_prec_round(3, 10, Exact);
}

#[test]
#[should_panic]
fn log_base_prec_round_fail_3() {
    BigExponentFloat::from(5).log_base_prec_round(1, 10, Floor);
}

fn log_base_prec_round_properties_helper(
    x: &BigExponentFloat,
    b: u64,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&log_base_prec_round_all(x, b, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // The base-2 logarithm.
    assert!(same_pair(
        &x.log_base_prec_round_ref(2, prec, rm),
        &x.log_base_2_prec_round_ref(prec, rm)
    ));
}

#[test]
fn log_base_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_4().test_properties(
        |(x, b, prec, rm)| {
            log_base_prec_round_properties_helper(&x, b, prec, rm);
        },
    );
}

#[test]
fn log_base_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.log_base_prec_round_ref(b, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = log_base_prec_round_all(&float_to_big_exponent_float(&x), b, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}

#[test]
fn test_log_base_rational_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let cases: &[(&str, &str, u64, RoundingMode, &str, Ordering)] = &[
        ("3/5", "3", 1, Nearest, "-0x0.8", Less),
        ("3/5", "3", 10, Floor, "-0x0.772", Less),
        ("3/5", "3", 10, Ceiling, "-0x0.77", Greater),
        ("3/5", "3", 10, Down, "-0x0.77", Greater),
        ("3/5", "3", 10, Up, "-0x0.772", Less),
        ("3/5", "3", 10, Nearest, "-0x0.77", Greater),
        (
            "3/5",
            "3",
            100,
            Nearest,
            "-0x0.77088130fd4e46455c3c9ca988",
            Greater,
        ),
        ("9", "3", 1, Nearest, "0x2.0", Equal),
        ("9", "3", 10, Floor, "0x2.0", Equal),
        ("9", "3", 10, Ceiling, "0x2.0", Equal),
        ("9", "3", 10, Down, "0x2.0", Equal),
        ("9", "3", 10, Up, "0x2.0", Equal),
        ("9", "3", 10, Nearest, "0x2.0", Equal),
        ("9", "3", 100, Nearest, "0x2.0", Equal),
        ("9", "3", 300, Exact, "0x2.0", Equal),
        ("1/9", "3", 1, Nearest, "-0x2.0", Equal),
        ("1/9", "3", 10, Floor, "-0x2.0", Equal),
        ("1/9", "3", 10, Ceiling, "-0x2.0", Equal),
        ("1/9", "3", 10, Down, "-0x2.0", Equal),
        ("1/9", "3", 10, Up, "-0x2.0", Equal),
        ("1/9", "3", 10, Nearest, "-0x2.0", Equal),
        ("1/9", "3", 100, Nearest, "-0x2.0", Equal),
        ("1/9", "3", 300, Exact, "-0x2.0", Equal),
        ("1/3", "3", 1, Nearest, "-0x1.0", Equal),
        ("1/3", "3", 10, Floor, "-0x1.0", Equal),
        ("1/3", "3", 10, Ceiling, "-0x1.0", Equal),
        ("1/3", "3", 10, Down, "-0x1.0", Equal),
        ("1/3", "3", 10, Up, "-0x1.0", Equal),
        ("1/3", "3", 10, Nearest, "-0x1.0", Equal),
        ("1/3", "3", 100, Nearest, "-0x1.0", Equal),
        ("1/3", "3", 300, Exact, "-0x1.0", Equal),
        ("10000000000/3", "3", 1, Nearest, "0x1.0E+1", Less),
        ("10000000000/3", "3", 10, Floor, "0x13.f", Less),
        ("10000000000/3", "3", 10, Ceiling, "0x13.f8", Greater),
        ("10000000000/3", "3", 10, Down, "0x13.f", Less),
        ("10000000000/3", "3", 10, Up, "0x13.f8", Greater),
        ("10000000000/3", "3", 10, Nearest, "0x13.f8", Greater),
        (
            "10000000000/3",
            "3",
            100,
            Nearest,
            "0x13.f5832b7a85b1dbc1c4d0f94c",
            Greater,
        ),
        ("1", "3", 1, Nearest, "0x0.0", Equal),
        ("1", "3", 10, Floor, "0x0.0", Equal),
        ("1", "3", 10, Ceiling, "0x0.0", Equal),
        ("1", "3", 10, Down, "0x0.0", Equal),
        ("1", "3", 10, Up, "0x0.0", Equal),
        ("1", "3", 10, Nearest, "0x0.0", Equal),
        ("1", "3", 100, Nearest, "0x0.0", Equal),
        ("1", "3", 300, Exact, "0x0.0", Equal),
        ("0", "3", 1, Nearest, "-Infinity", Equal),
        ("0", "3", 10, Floor, "-Infinity", Equal),
        ("0", "3", 10, Ceiling, "-Infinity", Equal),
        ("0", "3", 10, Down, "-Infinity", Equal),
        ("0", "3", 10, Up, "-Infinity", Equal),
        ("0", "3", 10, Nearest, "-Infinity", Equal),
        ("0", "3", 100, Nearest, "-Infinity", Equal),
        ("0", "3", 300, Exact, "-Infinity", Equal),
        ("-1", "3", 1, Nearest, "NaN", Equal),
        ("-1", "3", 10, Floor, "NaN", Equal),
        ("-1", "3", 10, Ceiling, "NaN", Equal),
        ("-1", "3", 10, Down, "NaN", Equal),
        ("-1", "3", 10, Up, "NaN", Equal),
        ("-1", "3", 10, Nearest, "NaN", Equal),
        ("-1", "3", 100, Nearest, "NaN", Equal),
        ("-1", "3", 300, Exact, "NaN", Equal),
    ];
    for &(x, b, prec, rm, out, o) in cases {
        let r = log_base_rational_prec_round_all(
            &Rational::from_str(x).unwrap(),
            b.parse::<u64>().unwrap(),
            prec,
            rm,
        );
        assert_eq!(format!("{:?}", r.0), out, "{x} {b} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn log_base_rational_prec_round_fail_1() {
    BigExponentFloat::log_base_rational_prec_round(Rational::from_unsigneds(1u8, 3), 3, 0, Floor);
}

#[test]
#[should_panic]
fn log_base_rational_prec_round_fail_2() {
    BigExponentFloat::log_base_rational_prec_round(Rational::from_unsigneds(1u8, 5), 3, 10, Exact);
}

#[test]
#[should_panic]
fn log_base_rational_prec_round_fail_3() {
    BigExponentFloat::log_base_rational_prec_round(Rational::from_unsigneds(1u8, 3), 1, 10, Floor);
}

fn log_base_rational_prec_round_properties_helper(
    x: &Rational,
    b: u64,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_rational_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_rational_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_rational_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_rational_prec_round_all(x, b, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_rational_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A dyadic `Rational` gives the same result as the equal `BigExponentFloat`.
    if let Ok(y) = BigExponentFloat::try_from(x) {
        assert!(same_pair(&y.log_base_prec_round_ref(b, prec, rm), &r));
    }
}

#[test]
fn log_base_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, b, prec, rm)| {
            log_base_rational_prec_round_properties_helper(&x, b, prec, rm);
        },
    );
}

#[test]
fn log_base_rational_prec_round_float_agreement_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = Float::log_base_rational_prec_round_ref(&x, b, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = log_base_rational_prec_round_all(&x.clone(), b, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
