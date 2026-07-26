// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::GelfondSchneiderConstant;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_gelfond_schneider_constant_prec_helper(
    prec: u64,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::gelfond_schneider_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_gelfond_schneider_constant_prec() {
    test_gelfond_schneider_constant_prec_helper(1, "2.0", "0x2.0#1", Less);
    test_gelfond_schneider_constant_prec_helper(2, "3.0", "0x3.0#2", Greater);
    test_gelfond_schneider_constant_prec_helper(3, "2.5", "0x2.8#3", Less);
    test_gelfond_schneider_constant_prec_helper(4, "2.75", "0x2.c#4", Greater);
    test_gelfond_schneider_constant_prec_helper(5, "2.62", "0x2.a#5", Less);
    test_gelfond_schneider_constant_prec_helper(6, "2.69", "0x2.b#6", Greater);
    test_gelfond_schneider_constant_prec_helper(7, "2.656", "0x2.a8#7", Less);
    test_gelfond_schneider_constant_prec_helper(8, "2.672", "0x2.ac#8", Greater);
    test_gelfond_schneider_constant_prec_helper(9, "2.664", "0x2.aa#9", Less);
    test_gelfond_schneider_constant_prec_helper(10, "2.6641", "0x2.aa#10", Less);
    test_gelfond_schneider_constant_prec_helper(
        100,
        "2.6651441426902251886502972498731",
        "0x2.aa46e2f3fb0062e316c62ede4#100",
        Less,
    );

    test_gelfond_schneider_constant_prec_helper(
        1000,
        "2.665144142690225188650297249873139848274211313714659492835979593364920446178705954867609\
        180005196416941989363854235387514674242031438367407818698505487574895083114783962858356183\
        608346126643179409148910053401437395034287083311904527116973731595652905657632845729798177\
        43463728483308628193495285499275856",
        "0x2.aa46e2f3fb0062e316c62ede41c824f90d2439ed0d4d1f3bc1f20e16c27a0fe0f243a95fd8c2c8976f7af\
        38db43c1da1f0ae28e55701957848fdaaa8eced4490c6f84e85a2b9cea35c975e9dff6efb8d2da9c7926b2e46b\
        f27a37ca31c29f35e379fada862412a3968fe0be25b46a6960039190f4f55ecdd032e397f2c#1000",
        Greater,
    );
    test_gelfond_schneider_constant_prec_helper(
        10000,
        "2.665144142690225188650297249873139848274211313714659492835979593364920446178705954867609\
        180005196416941989363854235387514674242031438367407818698505487574895083114783962858356183\
        608346126643179409148910053401437395034287083311904527116973731595652905657632845729798177\
        434637284833086281934952854992758377356318883069338323445961180508097687908126127491072897\
        674297842663763250236960169562488171163970292690385990355562846011560523202446500663180639\
        152994795928010274550035284740862868569774849177514574499658837297574572589990638828000350\
        803690373387909046718280538365967679522829468189601804934461532880842378994716838254056869\
        371997377362354372632671308208566935024781324958444126617043118338099825819036311337895768\
        509523273079711578060767773767255995779629976943938497541384621068371403791894057204607551\
        548248746311581327005527774058011748869440565219731790935058137067162054457599985969390694\
        987697876924728772303790339115891556642026760609711356815249085312455545659935574176122815\
        155168089933928414216177023206262684289687628110835489787056747835344799408333416428022100\
        593966581426809950665673169968208570085761290443718664538367504689623258272137011205074050\
        238625263038953326832569734719894323455047764141774749747325298755907302639587601781187963\
        441361530376350538576265981519673484757079739663760846872004734411958077095794063432362705\
        307795709329626047819932555414957358875268708162162423650658993074677855698797575344650855\
        972069975021998309107760898026886248980145289991123668487117987427995441163710588060211017\
        148578107063294966496638321186149548587724059060809250474865776383287725329543206828925677\
        664910188365897895999784971881965264463195191015987923286182096176308673196873565829876207\
        927437147170315130872577718599525814039757993675876078248632448943849212689894722653644563\
        854920947236777823293255687867653244969152269793768600300688848014765402847052644683267253\
        836590681467543191284450755808909632583781118359959284761481527412843228044442794375127044\
        458933359591161055212766247607147729959544721123790485170852990713585665205265807790440016\
        941775726732574505028438459318735121496728201653430650870487508810708159253966188070785287\
        478144522904279332909993248786243484114169309502413921590954472053025936479130577297537497\
        779028383895508504863293011861509282683907442181160852035062397562127646716215347018214444\
        721315197092555435026155033514221748082461366505420579047634615200980498052316720074985640\
        949420854690269276052209007431303887651889533171306582984351198327547934971253401461236877\
        876127076015082871807907788825471510960219855100184979987473688597413062062410004799427000\
        964205911186535896846553106095609391355583096773066364182110639781860855534923878548677946\
        379592781567978115618427429933119768525048215239797968385305159653918996029776743888940900\
        151965975676591186311513433856455412091544630404911484375984677352083125576901150199053369\
        483012445999873563341879637702935455372122165063961654450644062817413229749577671271071361\
        03621690447654252724488546576235753707369249",
        "0x2.aa46e2f3fb0062e316c62ede41c824f90d2439ed0d4d1f3bc1f20e16c27a0fe0f243a95fd8c2c8976f7af\
        38db43c1da1f0ae28e55701957848fdaaa8eced4490c6f84e85a2b9cea35c975e9dff6efb8d2da9c7926b2e46b\
        f27a37ca31c29f35e379fada862412a3968fe0be25b46a6960039190f4f55ecdd032e397f2a016e1508077a1d3\
        0724b5d6edb5a761119fbde24e49c2ffd08dbf6f2663ebf57c86ae73d687a7c7cae0c323b6e1f8d85bf025e098\
        ab52398c546ebe9f13bf8d3d3a17798bcfc3c9d6dc1be732467661cd1097cbd9f56ff23cdaf6a8257f58fbbe2f\
        8306e5114ee8c731c62711954219161186566be0a03acf91b0a89bd1e280b2e3555ea1e364068763b12ea0180e\
        b66b42a888b2d785bc10002fed52ef198b6c1debee575b38003b1e68dda99d2029580e7e29779f921fddde3a03\
        0c4e84cb0aa1b0dcbcf4606380fed56179aade0f1d68e67ea2a2dc5096f313f84495e756330f665e8be0b2e3f8\
        f86972fb9fee6a6ef0c6d9cb078c66c127e5ab4d7a6e5afdadb02b5b3f17899699fde4ba745d30b3060dd0ba33\
        c8555a890744c404b1b3795269050389e06cd3ba0f2eb6fb79ba0c482b10ad897524145975059fe2190f44b119\
        1ce0475932f76ba61a3a8b6cd039df538ddff39342e00772a6617a7e987f6121055acc950b69d01b70689087b6\
        8cdd7d791bec85383ed73e2f702ce37f414339a189bc9eb8dd33899ae743896cf05291148cccaed0d8472ef947\
        ab30763e06bd512734ef86d540c913fb6ef569fca5b641e25b684e2c10202a1f21c3ea9cfc7acae27854f5796e\
        f62349f7394b197e23baddf04a801fecfe03ff411c6ed97c2897f79a53e19b9bf65f8b94b3b7a1e106672ad29b\
        fcf0ba9f9db727e466e2640cd29a47902d38de268347e59e99aef401a5b9a6dda37daec9302dee24295c472ea3\
        2488332ac26761f6d50c313c24b584ca1838cc85b2e40888eacc487a32e68c4a761a19fb00d206b22a0cbaa1a4\
        95c0c9ed1c0b194305311b70e423ff173f152bbe681e03b412da51135c9c4868d5f9136b58630e60b3c2c87ed3\
        b64cacdeeb70473933b67b8060d7d84769ea8224fbcbbeba13ebb374cc17df457bddd672202233d70b83a524a1\
        b4849507ee693dcf3d6ef95e0db4a155fdd7640f4ca9982a2f9456562d6f2c26199caf3edfb58af251de111c35\
        45ee09f3a257906e09613c4cfd3fb72d82a6ad36f60000b8a38457de873404b2ea314d9284280b9b5fa301101a\
        907bdc5193f2da5c8d9fc60e60c2f22c3cb0335c8f30d68a97fd00495f26320a37e841cf8623c06f8677c12206\
        b24086eab70cab264f93dfb79d11a3e6f9f553a6898cfa10b709826411dc78eb6e863f190a5b8d6aaeb24bdd15\
        6a5179201a4198aaba2ab979fc447ef0733035ea8c0ce461000873def672a12d00f9b9d6f7399e0c385930f383\
        bccb1a0df73a07547357996d5b9f807ae1fe7c979efe8aa0a5560baa967c5d1e14d0ec476e58881699288ed3e1\
        1c00fb66f7d515df24c8e2a2a2c6eb93973136d64058329a7208d0338b7e565a220ef47639ccf44182dc350c1b\
        add28276d5f1b9b0e75890d1ee46e04126696a73d1eb01340a319c6b230d80c5067046d088694b572354238052\
        f44b79ca1e2e8dd494962dc40276a723cd6a25202d3908c7e9a00776525ae0b23c10e6b317e818962e0a41d5e1\
        41f623545db9926c97ccc4b5ceb39102508f52b129c4809c3c2b33e76ec825dcdc9b8981abc#10000",
        Greater,
    );
    let gs_f32 = Float::gelfond_schneider_constant_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(gs_f32.to_string(), "2.66514421");
    assert_eq!(to_hex_string(&gs_f32), "0x2.aa46e4#24");
    assert_eq!(gs_f32, f32::GELFOND_SCHNEIDER_CONSTANT);

    let gs_f64 = Float::gelfond_schneider_constant_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(gs_f64.to_string(), "2.6651441426902251");
    assert_eq!(to_hex_string(&gs_f64), "0x2.aa46e2f3fb006#53");
    assert_eq!(gs_f64, f64::GELFOND_SCHNEIDER_CONSTANT);
}

#[test]
#[should_panic]
fn gelfond_schneider_constant_prec_fail_1() {
    Float::gelfond_schneider_constant_prec(0);
}

fn test_gelfond_schneider_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::gelfond_schneider_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_gelfond_schneider_constant_prec_round() {
    test_gelfond_schneider_constant_prec_round_helper(1, Floor, "2.0", "0x2.0#1", Less);
    test_gelfond_schneider_constant_prec_round_helper(1, Ceiling, "4.0", "0x4.0#1", Greater);
    test_gelfond_schneider_constant_prec_round_helper(1, Down, "2.0", "0x2.0#1", Less);
    test_gelfond_schneider_constant_prec_round_helper(1, Up, "4.0", "0x4.0#1", Greater);
    test_gelfond_schneider_constant_prec_round_helper(1, Nearest, "2.0", "0x2.0#1", Less);

    test_gelfond_schneider_constant_prec_round_helper(2, Floor, "2.0", "0x2.0#2", Less);
    test_gelfond_schneider_constant_prec_round_helper(2, Ceiling, "3.0", "0x3.0#2", Greater);
    test_gelfond_schneider_constant_prec_round_helper(2, Down, "2.0", "0x2.0#2", Less);
    test_gelfond_schneider_constant_prec_round_helper(2, Up, "3.0", "0x3.0#2", Greater);
    test_gelfond_schneider_constant_prec_round_helper(2, Nearest, "3.0", "0x3.0#2", Greater);

    test_gelfond_schneider_constant_prec_round_helper(3, Floor, "2.5", "0x2.8#3", Less);
    test_gelfond_schneider_constant_prec_round_helper(3, Ceiling, "3.0", "0x3.0#3", Greater);
    test_gelfond_schneider_constant_prec_round_helper(3, Down, "2.5", "0x2.8#3", Less);
    test_gelfond_schneider_constant_prec_round_helper(3, Up, "3.0", "0x3.0#3", Greater);
    test_gelfond_schneider_constant_prec_round_helper(3, Nearest, "2.5", "0x2.8#3", Less);

    test_gelfond_schneider_constant_prec_round_helper(4, Floor, "2.50", "0x2.8#4", Less);
    test_gelfond_schneider_constant_prec_round_helper(4, Ceiling, "2.75", "0x2.c#4", Greater);
    test_gelfond_schneider_constant_prec_round_helper(4, Down, "2.50", "0x2.8#4", Less);
    test_gelfond_schneider_constant_prec_round_helper(4, Up, "2.75", "0x2.c#4", Greater);
    test_gelfond_schneider_constant_prec_round_helper(4, Nearest, "2.75", "0x2.c#4", Greater);

    test_gelfond_schneider_constant_prec_round_helper(5, Floor, "2.62", "0x2.a#5", Less);
    test_gelfond_schneider_constant_prec_round_helper(5, Ceiling, "2.75", "0x2.c#5", Greater);
    test_gelfond_schneider_constant_prec_round_helper(5, Down, "2.62", "0x2.a#5", Less);
    test_gelfond_schneider_constant_prec_round_helper(5, Up, "2.75", "0x2.c#5", Greater);
    test_gelfond_schneider_constant_prec_round_helper(5, Nearest, "2.62", "0x2.a#5", Less);

    test_gelfond_schneider_constant_prec_round_helper(6, Floor, "2.62", "0x2.a#6", Less);
    test_gelfond_schneider_constant_prec_round_helper(6, Ceiling, "2.69", "0x2.b#6", Greater);
    test_gelfond_schneider_constant_prec_round_helper(6, Down, "2.62", "0x2.a#6", Less);
    test_gelfond_schneider_constant_prec_round_helper(6, Up, "2.69", "0x2.b#6", Greater);
    test_gelfond_schneider_constant_prec_round_helper(6, Nearest, "2.69", "0x2.b#6", Greater);

    test_gelfond_schneider_constant_prec_round_helper(7, Floor, "2.656", "0x2.a8#7", Less);
    test_gelfond_schneider_constant_prec_round_helper(7, Ceiling, "2.688", "0x2.b0#7", Greater);
    test_gelfond_schneider_constant_prec_round_helper(7, Down, "2.656", "0x2.a8#7", Less);
    test_gelfond_schneider_constant_prec_round_helper(7, Up, "2.688", "0x2.b0#7", Greater);
    test_gelfond_schneider_constant_prec_round_helper(7, Nearest, "2.656", "0x2.a8#7", Less);

    test_gelfond_schneider_constant_prec_round_helper(8, Floor, "2.656", "0x2.a8#8", Less);
    test_gelfond_schneider_constant_prec_round_helper(8, Ceiling, "2.672", "0x2.ac#8", Greater);
    test_gelfond_schneider_constant_prec_round_helper(8, Down, "2.656", "0x2.a8#8", Less);
    test_gelfond_schneider_constant_prec_round_helper(8, Up, "2.672", "0x2.ac#8", Greater);
    test_gelfond_schneider_constant_prec_round_helper(8, Nearest, "2.672", "0x2.ac#8", Greater);

    test_gelfond_schneider_constant_prec_round_helper(9, Floor, "2.664", "0x2.aa#9", Less);
    test_gelfond_schneider_constant_prec_round_helper(9, Ceiling, "2.672", "0x2.ac#9", Greater);
    test_gelfond_schneider_constant_prec_round_helper(9, Down, "2.664", "0x2.aa#9", Less);
    test_gelfond_schneider_constant_prec_round_helper(9, Up, "2.672", "0x2.ac#9", Greater);
    test_gelfond_schneider_constant_prec_round_helper(9, Nearest, "2.664", "0x2.aa#9", Less);

    test_gelfond_schneider_constant_prec_round_helper(10, Floor, "2.6641", "0x2.aa#10", Less);
    test_gelfond_schneider_constant_prec_round_helper(10, Ceiling, "2.6680", "0x2.ab#10", Greater);
    test_gelfond_schneider_constant_prec_round_helper(10, Down, "2.6641", "0x2.aa#10", Less);
    test_gelfond_schneider_constant_prec_round_helper(10, Up, "2.6680", "0x2.ab#10", Greater);
    test_gelfond_schneider_constant_prec_round_helper(10, Nearest, "2.6641", "0x2.aa#10", Less);

    test_gelfond_schneider_constant_prec_round_helper(
        100,
        Floor,
        "2.6651441426902251886502972498731",
        "0x2.aa46e2f3fb0062e316c62ede4#100",
        Less,
    );
    test_gelfond_schneider_constant_prec_round_helper(
        100,
        Ceiling,
        "2.6651441426902251886502972498762",
        "0x2.aa46e2f3fb0062e316c62ede8#100",
        Greater,
    );
    test_gelfond_schneider_constant_prec_round_helper(
        100,
        Down,
        "2.6651441426902251886502972498731",
        "0x2.aa46e2f3fb0062e316c62ede4#100",
        Less,
    );
    test_gelfond_schneider_constant_prec_round_helper(
        100,
        Up,
        "2.6651441426902251886502972498762",
        "0x2.aa46e2f3fb0062e316c62ede8#100",
        Greater,
    );
    test_gelfond_schneider_constant_prec_round_helper(
        100,
        Nearest,
        "2.6651441426902251886502972498731",
        "0x2.aa46e2f3fb0062e316c62ede4#100",
        Less,
    );
}

#[test]
#[should_panic]
fn gelfond_schneider_constant_prec_round_fail_1() {
    Float::gelfond_schneider_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn gelfond_schneider_constant_prec_round_fail_2() {
    Float::gelfond_schneider_constant_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn gelfond_schneider_constant_prec_round_fail_3() {
    Float::gelfond_schneider_constant_prec_round(1000, Exact);
}

#[test]
fn gelfond_schneider_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (gelfond_schneider_constant, o) = Float::gelfond_schneider_constant_prec(prec);
        assert!(gelfond_schneider_constant.is_valid());
        assert_eq!(gelfond_schneider_constant.get_prec(), Some(prec));
        assert_eq!(gelfond_schneider_constant.get_exponent(), Some(2));
        assert_ne!(o, Equal);
        if o == Less {
            let (gelfond_schneider_constant_alt, o_alt) =
                Float::gelfond_schneider_constant_prec_round(prec, Ceiling);
            let mut next_upper = gelfond_schneider_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(gelfond_schneider_constant_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !gelfond_schneider_constant.is_power_of_2() {
            let (gelfond_schneider_constant_alt, o_alt) =
                Float::gelfond_schneider_constant_prec_round(prec, Floor);
            let mut next_lower = gelfond_schneider_constant.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(gelfond_schneider_constant_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (gelfond_schneider_constant_alt, o_alt) =
            Float::gelfond_schneider_constant_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&gelfond_schneider_constant_alt),
            ComparableFloatRef(&gelfond_schneider_constant)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn gelfond_schneider_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (gelfond_schneider_constant, o) =
            Float::gelfond_schneider_constant_prec_round(prec, rm);
        assert!(gelfond_schneider_constant.is_valid());
        assert_eq!(gelfond_schneider_constant.get_prec(), Some(prec));
        // 2^sqrt(2) is in [2, 4), so the result has exponent 2 unless it rounds up to 4.
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up) => 3,
            _ => 2,
        };
        assert_eq!(
            gelfond_schneider_constant.get_exponent(),
            Some(expected_exponent)
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (gelfond_schneider_constant_alt, o_alt) =
                Float::gelfond_schneider_constant_prec_round(prec, Ceiling);
            let mut next_upper = gelfond_schneider_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(gelfond_schneider_constant_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !gelfond_schneider_constant.is_power_of_2() {
            let (gelfond_schneider_constant_alt, o_alt) =
                Float::gelfond_schneider_constant_prec_round(prec, Floor);
            let mut next_lower = gelfond_schneider_constant.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(gelfond_schneider_constant_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::gelfond_schneider_constant_prec_round(prec, Exact));
    });

    test_constant(Float::gelfond_schneider_constant_prec_round, 10000);
}
