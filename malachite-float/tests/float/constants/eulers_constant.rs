// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::EulersConstant;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::float::constants::eulers_constant::*;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_eulers_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::eulers_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) =
        rug_eulers_constant_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_eulers_constant_prec() {
    // - precision 1 makes the Ziv loop retry in every rounding mode
    test_eulers_constant_prec_helper(1, "0.50", "0x0.8#1", Less);
    test_eulers_constant_prec_helper(2, "0.50", "0x0.8#2", Less);
    test_eulers_constant_prec_helper(3, "0.62", "0x0.a#3", Greater);
    test_eulers_constant_prec_helper(4, "0.562", "0x0.9#4", Less);
    // - precision 5 with Nearest makes the Ziv loop retry
    test_eulers_constant_prec_helper(5, "0.562", "0x0.90#5", Less);
    test_eulers_constant_prec_helper(6, "0.578", "0x0.94#6", Greater);
    test_eulers_constant_prec_helper(7, "0.5781", "0x0.94#7", Greater);
    test_eulers_constant_prec_helper(8, "0.5781", "0x0.94#8", Greater);
    test_eulers_constant_prec_helper(9, "0.5781", "0x0.940#9", Greater);
    test_eulers_constant_prec_helper(10, "0.57715", "0x0.93c#10", Less);
    test_eulers_constant_prec_helper(
        100,
        "0.57721566490153286060651209008234",
        "0x0.93c467e37db0c7a4d1be3f810#100",
        Less,
    );
    test_eulers_constant_prec_helper(
        1000,
        "0.577215664901532860606512090082402431042159335939923598805767234884867726777664670936947\
        063291746749514631447249807082480960504014486542836224173997644923536253500333742937337737\
        673942792595258247094916008735203948165670853233151776611528621199501507984793745085705740\
        029921354786146694029604325421519037",
        "0x0.93c467e37db0c7a4d1be3f810152cb56a1cecc3af65cc0190c03df34709affbd8e4b59fa03a9f0eed0649\
        ccb621057d11056ae9132135a08e43b4673d74bafea58deb878cc86d733dbe7bf38154b36cf8a96d1567899aaa\
        e0c09d4c8b6b7b86fd2a1ea1de62ff8643ec7c271827977225e6ac2f0bd61c746961542a3ce#1000",
        Less,
    );
    test_eulers_constant_prec_helper(
        10000,
        "0.577215664901532860606512090082402431042159335939923598805767234884867726777664670936947\
        063291746749514631447249807082480960504014486542836224173997644923536253500333742937337737\
        673942792595258247094916008735203948165670853233151776611528621199501507984793745085705740\
        029921354786146694029604325421519058775535267331399254012967420513754139549111685102807984\
        234877587205038431093997361372553060889331267600172479537836759271351577226102734929139407\
        984301034177717780881549570661075010161916633401522789358679654972520362128792265559536696\
        281763887927268013243101047650596370394739495763890657296792960100901512519595092224350140\
        934987122824794974719564697631850667612906381105182419744486783638086174945516989279230187\
        739107294578155431600500218284409605377243420328547836701517739439870030237033951832869000\
        155819398804270741154222781971652301107356583396734871765049194181230004065469314299929777\
        956930310050308630341856980323108369164002589297089098548682577736428825395492587362959613\
        329857473930237343884707037028441292016641785024873337908056275499843459076164316710314671\
        072237002181074504441866475913480366902553245862544222534518138791243457350136129778227828\
        814894590986384600629316947188714958752549236649352047324364109726827616087759508809512620\
        840454447799229915724829251625127842765965708321461029821461795195795909592270420898962797\
        125536321794887376421066060706598256199010288075612519913751167821764361905705844078357350\
        158005607745793421314498850078641517161519456570617043245075008168705230789093704614306684\
        817916496842549150496724312183783875356489495086845410234060162250851558386723494418788044\
        094077010688379511130787202342639522692097160885690838251137871283682049117892594478486199\
        118529391029309905925526691727446892044386971114717457157457320393520912231608508682755889\
        010945168118101687497547096936667121020630482716589504932731486087494020700674259091824875\
        962137384231144265313502923031751722572216283248838112458957438623987037576628551303314392\
        999540185313414158621278864807611003015211965780068117773763501681838973389663986895793299\
        145638864431037060807817448995795832457941896202604984104392250786046036252772602291968299\
        586098833901378717142269178838195298445607916051972797360475910251099577913351579177225150\
        254929324632502874767794842158405075992904018557645990186269267764372660571176813365590881\
        554810747000062336372528894955463697143301200791308555263959549782302314403914974049474682\
        594732084618524605877669488287953010406349172292185800870677069042792674328444696851497182\
        567809584165449185145753319640633119937382157345087498832556088887352801901915508968855468\
        259245444527728173057301080606177011363773182462924660081277162101867744684959514281790145\
        111948934228834482530753118701860976122462317674977556412461983856401484123587177249554224\
        820161517657994080629683424289057259473926963863383874380547131967642926837249076087507378\
        528370230468650349051203422721743668979284862972908892678977703262462391226188876530057786\
        274360609444360392809770813383693423550858419",
        "0x0.93c467e37db0c7a4d1be3f810152cb56a1cecc3af65cc0190c03df34709affbd8e4b59fa03a9f0eed0649\
        ccb621057d11056ae9132135a08e43b4673d74bafea58deb878cc86d733dbe7bf38154b36cf8a96d1567899aaa\
        e0c09d4c8b6b7b86fd2a1ea1de62ff8643ec7c271827977225e6ac2f0bd61c746961542a3ce3bea5db54fe70e6\
        3e6d09f8fc28658e80567a47cfde60ee741e5d85a7bd46931ced8220365594964b839896fcaabccc9b31959c08\
        3f22ad3ee591c32fab2c7448f2a057db2db49ee52e0182741e53865f004cc8e704b7c5c40bf304c4d8c4f13edf\
        6047c555302d2238d8ce11df2424f1b66c2c5d238d0744db679af2890487031f9c0aea1c4bb6fe9554ee528fdf\
        1b05e5b256223b2f09215f3719f9c7ccc69ddf172d0d6234217fcc0037f18b93ef5389130b7a661e5c26e54214\
        068bbcafea32a67818bd3075ad1f5c7e9cc3d1737fb28171baf84dbb6612b7881c1a48e439cd03a92bf52225a2\
        b38e6542e9f722bce15a381b5753ea842763381ccae83512b30511b32e5e8d80362149ad030aaba5f3a5798bb2\
        2aa7ec1b6d0f17903f4e1f3a06731072b10e04218380c3f5be7d44c6937b6e79cf67655f07230456f98340336e\
        1166330fbef5f3cdbe29b7929c3bfbcf4298c94ecfa77dbb06ab26c11890ea9e63440b10921fb25361b34c7b93\
        42a13e3fb6a91c35f67b95163a91f916aa7b253cc82ee520ec006866584424e7cdce3c3dd186ce34a330da9ee7\
        b7082ea72531701cc5965f761e1ceadaf207f9827b4505add7b66ebb244716840db4b294b8d99d0ca6d322da79\
        05c98033652dc225eb36b89901c13c9c15380ccb048b15be0a613d075ec0fbc158250e9bd629d799c0e15a205c\
        aeead2c1a2aa22b0e2c64fa51a68ed7f17d46555745acbd84a03cbaf369df9900a8755a7b92ab850232acb7641\
        db7b5c77161f3c043d2d05cbf9ea7b79fb513153cdfe5aa6f2b5f81b3fb9dbf4381b493981ce589f4ca5a73055\
        92647909dfd95feeeb0fcbe2adfbd5e3efe4997aee382c6bac9124b87d32ec33e16eeb45be4ee4f90ab666fbdb\
        cf99671af23dc68b6ae362b362a64b2ae8549ca937b743a8758575b0bdedd6a46a78962ef8b4e1369ca581fa02\
        2577752f2c2ece38d34a71698e114d7dea29660ac422c5959744adc8b56aac32c422ff6c41c4a60c1825e4ad88\
        dd5cdac37a8e22aed95e82cb52cd4e753ae38e1880bc1e8c174ca7b8b9cd9556e6553c4dc59812942ca1a60fb1\
        f30f8ac1b0d1212ec32bb8e1351c29ae3872d9035d70f65cf9b0ce2e7917c79cea3d8fe9eab290c504ebdeec14\
        cac9419838543547083c37eddcaaf79043821d8b08c5ef1d180e7239f5061704d51fed0ab5fbc2402ee3f91e01\
        1a17155f9e4c0c0d5a797dea26198df2127915f3a71883c116e1169ec775d7f36778fb170521ccde7887b82873\
        a318dbac628ebd3715902aa0d19ddee5afbcd16b78ec3eb186933bcb20d7dd8d17fe0b1e5bb292e7bbc0774614\
        de12fd66eaefb59458510a82781686d3c7e3d760e8aaccaec24e874911b4ffd7e8398a8512959c2b53a907621d\
        247c096475825e8ce92b5826a3899243364e18d85b12972642bb1574349155e63a983a499a7fa5b957347a7453\
        95676785aa98b9da5e9e560e4009284500c36d2083f60b876171976b6b6a43f4fe377b5436caa3095c8fcaf262\
        67dc9a4a3bfe9ec962622611ab47eca95a7569829d4470a915ab2e115d89425530b3a83cb87#10000",
        Greater,
    );

    let g_f32 = Float::eulers_constant_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(g_f32.to_string(), "0.577215672");
    assert_eq!(to_hex_string(&g_f32), "0x0.93c468#24");
    assert_eq!(g_f32, f32::EULERS_CONSTANT);

    let g_f64 = Float::eulers_constant_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(g_f64.to_string(), "0.57721566490153287");
    assert_eq!(to_hex_string(&g_f64), "0x0.93c467e37db0c8#53");
    assert_eq!(g_f64, f64::EULERS_CONSTANT);
}

#[test]
#[should_panic]
fn eulers_constant_prec_fail_1() {
    Float::eulers_constant_prec(0);
}

fn test_eulers_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::eulers_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_eulers_constant_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_eulers_constant_prec_round() {
    test_eulers_constant_prec_round_helper(1, Floor, "0.50", "0x0.8#1", Less);
    test_eulers_constant_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_eulers_constant_prec_round_helper(1, Down, "0.50", "0x0.8#1", Less);
    test_eulers_constant_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_eulers_constant_prec_round_helper(1, Nearest, "0.50", "0x0.8#1", Less);
    test_eulers_constant_prec_round_helper(2, Floor, "0.50", "0x0.8#2", Less);
    test_eulers_constant_prec_round_helper(2, Ceiling, "0.75", "0x0.c#2", Greater);
    test_eulers_constant_prec_round_helper(2, Down, "0.50", "0x0.8#2", Less);
    test_eulers_constant_prec_round_helper(2, Up, "0.75", "0x0.c#2", Greater);
    test_eulers_constant_prec_round_helper(2, Nearest, "0.50", "0x0.8#2", Less);
    test_eulers_constant_prec_round_helper(3, Floor, "0.50", "0x0.8#3", Less);
    test_eulers_constant_prec_round_helper(3, Ceiling, "0.62", "0x0.a#3", Greater);
    test_eulers_constant_prec_round_helper(3, Down, "0.50", "0x0.8#3", Less);
    test_eulers_constant_prec_round_helper(3, Up, "0.62", "0x0.a#3", Greater);
    test_eulers_constant_prec_round_helper(3, Nearest, "0.62", "0x0.a#3", Greater);
    test_eulers_constant_prec_round_helper(4, Floor, "0.562", "0x0.9#4", Less);
    test_eulers_constant_prec_round_helper(4, Ceiling, "0.625", "0x0.a#4", Greater);
    test_eulers_constant_prec_round_helper(4, Down, "0.562", "0x0.9#4", Less);
    test_eulers_constant_prec_round_helper(4, Up, "0.625", "0x0.a#4", Greater);
    test_eulers_constant_prec_round_helper(4, Nearest, "0.562", "0x0.9#4", Less);
    test_eulers_constant_prec_round_helper(5, Floor, "0.562", "0x0.90#5", Less);
    test_eulers_constant_prec_round_helper(5, Ceiling, "0.594", "0x0.98#5", Greater);
    test_eulers_constant_prec_round_helper(5, Down, "0.562", "0x0.90#5", Less);
    test_eulers_constant_prec_round_helper(5, Up, "0.594", "0x0.98#5", Greater);
    test_eulers_constant_prec_round_helper(5, Nearest, "0.562", "0x0.90#5", Less);
    test_eulers_constant_prec_round_helper(6, Floor, "0.562", "0x0.90#6", Less);
    test_eulers_constant_prec_round_helper(6, Ceiling, "0.578", "0x0.94#6", Greater);
    test_eulers_constant_prec_round_helper(6, Down, "0.562", "0x0.90#6", Less);
    test_eulers_constant_prec_round_helper(6, Up, "0.578", "0x0.94#6", Greater);
    test_eulers_constant_prec_round_helper(6, Nearest, "0.578", "0x0.94#6", Greater);
    test_eulers_constant_prec_round_helper(7, Floor, "0.5703", "0x0.92#7", Less);
    test_eulers_constant_prec_round_helper(7, Ceiling, "0.5781", "0x0.94#7", Greater);
    test_eulers_constant_prec_round_helper(7, Down, "0.5703", "0x0.92#7", Less);
    test_eulers_constant_prec_round_helper(7, Up, "0.5781", "0x0.94#7", Greater);
    test_eulers_constant_prec_round_helper(7, Nearest, "0.5781", "0x0.94#7", Greater);
    test_eulers_constant_prec_round_helper(8, Floor, "0.5742", "0x0.93#8", Less);
    test_eulers_constant_prec_round_helper(8, Ceiling, "0.5781", "0x0.94#8", Greater);
    test_eulers_constant_prec_round_helper(8, Down, "0.5742", "0x0.93#8", Less);
    test_eulers_constant_prec_round_helper(8, Up, "0.5781", "0x0.94#8", Greater);
    test_eulers_constant_prec_round_helper(8, Nearest, "0.5781", "0x0.94#8", Greater);
    test_eulers_constant_prec_round_helper(9, Floor, "0.5762", "0x0.938#9", Less);
    test_eulers_constant_prec_round_helper(9, Ceiling, "0.5781", "0x0.940#9", Greater);
    test_eulers_constant_prec_round_helper(9, Down, "0.5762", "0x0.938#9", Less);
    test_eulers_constant_prec_round_helper(9, Up, "0.5781", "0x0.940#9", Greater);
    test_eulers_constant_prec_round_helper(9, Nearest, "0.5781", "0x0.940#9", Greater);
    test_eulers_constant_prec_round_helper(10, Floor, "0.57715", "0x0.93c#10", Less);
    test_eulers_constant_prec_round_helper(10, Ceiling, "0.57812", "0x0.940#10", Greater);
    test_eulers_constant_prec_round_helper(10, Down, "0.57715", "0x0.93c#10", Less);
    test_eulers_constant_prec_round_helper(10, Up, "0.57812", "0x0.940#10", Greater);
    test_eulers_constant_prec_round_helper(10, Nearest, "0.57715", "0x0.93c#10", Less);
    test_eulers_constant_prec_round_helper(
        100,
        Floor,
        "0.57721566490153286060651209008234",
        "0x0.93c467e37db0c7a4d1be3f810#100",
        Less,
    );
    test_eulers_constant_prec_round_helper(
        100,
        Ceiling,
        "0.57721566490153286060651209008313",
        "0x0.93c467e37db0c7a4d1be3f811#100",
        Greater,
    );
    test_eulers_constant_prec_round_helper(
        100,
        Down,
        "0.57721566490153286060651209008234",
        "0x0.93c467e37db0c7a4d1be3f810#100",
        Less,
    );
    test_eulers_constant_prec_round_helper(
        100,
        Up,
        "0.57721566490153286060651209008313",
        "0x0.93c467e37db0c7a4d1be3f811#100",
        Greater,
    );
    test_eulers_constant_prec_round_helper(
        100,
        Nearest,
        "0.57721566490153286060651209008234",
        "0x0.93c467e37db0c7a4d1be3f810#100",
        Less,
    );
}

// Precisions near long runs of identical bits in gamma's binary expansion make the Ziv loop retry
// at higher working precisions (found by instrumented sweep; unlike const_catalan.c, const_euler.c
// records no worst-case table). The results are too long to compare as strings, so they are checked
// against MPFR via rug.
#[test]
fn test_eulers_constant_ziv_retry() {
    for prec in [20, 177, 239] {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (x, o) = Float::eulers_constant_prec_round(prec, rm);
            assert!(x.is_valid());
            let (rug_x, rug_o) =
                rug_eulers_constant_prec_round(prec, rug_round_try_from_rounding_mode(rm).unwrap());
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_x)),
                ComparableFloatRef(&x)
            );
            assert_eq!(rug_o, o);
        }
    }
}

#[test]
#[should_panic]
fn eulers_constant_prec_round_fail_1() {
    Float::eulers_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn eulers_constant_prec_round_fail_2() {
    Float::eulers_constant_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn eulers_constant_prec_round_fail_3() {
    Float::eulers_constant_prec_round(1000, Exact);
}

#[test]
fn eulers_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (g, o) = Float::eulers_constant_prec(prec);
        assert!(g.is_valid());
        assert_eq!(g.get_prec(), Some(prec));
        assert_eq!(g.get_exponent(), Some(0));
        assert_ne!(o, Equal);
        if o == Less {
            let (g_alt, o_alt) = Float::eulers_constant_prec_round(prec, Ceiling);
            let mut next_upper = g.clone();
            next_upper.increment();
            assert_eq!(ComparableFloat(g_alt), ComparableFloat(next_upper));
            assert_eq!(o_alt, Greater);
        } else {
            let (g_alt, o_alt) = Float::eulers_constant_prec_round(prec, Floor);
            let mut next_lower = g.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(g_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (g_alt, o_alt) = Float::eulers_constant_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&g_alt), ComparableFloatRef(&g));
        assert_eq!(o_alt, o);

        let (rug_g, rug_o) = rug_eulers_constant_prec_round(
            prec,
            rug_round_try_from_rounding_mode(Nearest).unwrap(),
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_g)),
            ComparableFloatRef(&g)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn eulers_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (g, o) = Float::eulers_constant_prec_round(prec, rm);
        assert!(g.is_valid());
        assert_eq!(g.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up) => 1,
            _ => 0,
        };
        assert_eq!(g.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (g_alt, o_alt) = Float::eulers_constant_prec_round(prec, Ceiling);
            let mut next_upper = g.clone();
            next_upper.increment();
            assert_eq!(ComparableFloat(g_alt), ComparableFloat(next_upper));
            assert_eq!(o_alt, Greater);
        } else {
            let (g_alt, o_alt) = Float::eulers_constant_prec_round(prec, Floor);
            let mut next_lower = g.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(g_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_g, rug_o) = rug_eulers_constant_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_g)),
                ComparableFloatRef(&g)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::eulers_constant_prec_round(prec, Exact));
    });

    test_constant(Float::eulers_constant_prec_round, 10000);
}
