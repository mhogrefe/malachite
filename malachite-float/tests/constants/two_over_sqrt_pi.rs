// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::TwoOverSqrtPi;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_two_over_sqrt_pi_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::two_over_sqrt_pi_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_two_over_sqrt_pi_prec() {
    test_two_over_sqrt_pi_prec_helper(1, "1.0", "0x1.0#1", Less);
    test_two_over_sqrt_pi_prec_helper(2, "1.0", "0x1.0#2", Less);
    test_two_over_sqrt_pi_prec_helper(3, "1.2", "0x1.4#3", Greater);
    test_two_over_sqrt_pi_prec_helper(4, "1.1", "0x1.2#4", Less);
    test_two_over_sqrt_pi_prec_helper(5, "1.12", "0x1.2#5", Less);
    test_two_over_sqrt_pi_prec_helper(6, "1.12", "0x1.20#6", Less);
    test_two_over_sqrt_pi_prec_helper(7, "1.12", "0x1.20#7", Less);
    test_two_over_sqrt_pi_prec_helper(8, "1.125", "0x1.20#8", Less);
    test_two_over_sqrt_pi_prec_helper(9, "1.129", "0x1.21#9", Greater);
    test_two_over_sqrt_pi_prec_helper(10, "1.129", "0x1.210#10", Greater);
    test_two_over_sqrt_pi_prec_helper(
        100,
        "1.128379167095512573896158903122",
        "0x1.20dd750429b6d11ae3a914fee#100",
        Greater,
    );
    test_two_over_sqrt_pi_prec_helper(
        1000,
        "1.128379167095512573896158903121545171688101258657997713688171443421284936882986828973487\
        320404214726886056695812723414703379862989652325732730979040035537986585675274119196879520\
        704928700435945142423160491545640441109017054346433244416926616222799025526908972046136475\
        3818374903174932317026021327967155",
        "0x1.20dd750429b6d11ae3a914fed7fd8688281341d7587cea2e7342b06199cc416180eb39f0b24e1e2281806\
        c12d98f35d77a3e9ddc91c394f0e9eedf0efffd84a2a4ac3b98489b8cbd3845e8fef6ff6af92a45e5f27c2d654\
        7a4f46505b5b4e62dd73fd6486de5e4e5585911777503638ea0ea96813d29a65a31a7d235ba#1000",
        Greater,
    );
    test_two_over_sqrt_pi_prec_helper(
        10000,
        "1.128379167095512573896158903121545171688101258657997713688171443421284936882986828973487\
        320404214726886056695812723414703379862989652325732730979040035537986585675274119196879520\
        704928700435945142423160491545640441109017054346433244416926616222799025526908972046136475\
        381837490317493231702602132796715543998754668320715597752333488152466078760432701203287243\
        392470100916625063893758913312576651631043248869097731406379754861763556365896778950217001\
        836917068443263565178670503666024049245124447449894540067794862528599318852700856608980726\
        631607875391971216318675658441114765847576463158466211523929554936506180343123616119044459\
        235264930718080170688589725005789478432836238548619548451139757591558099749638273874479384\
        145721266849535939897219177526087267452911757503086161868839476966576982758350723791327018\
        482697850617660789930811682114508296549646950349484018793976683355429771178335667478997183\
        163300275371977372408792825814573857927614754653462236857357604231049732437943817793615062\
        990240294910543518259630544244126468693514830520512369964555789973905060333899393713277246\
        184408849722866262122203479408633108870729232600533621778609544108260549664286731729420583\
        709898167677538495392748117508206316946634152433906289780994634796224277050525789933163762\
        144944610034611200146111014760538140478418650258730371308816054606091217117786637305172610\
        517669441962183230451102680326896192154681258135304184955488915831319872573827621301394798\
        442321270036762403269655890511176091794390559806184597712231091256394888644636818979096451\
        397846378693640734006412458420064879125851437050838091157673662671192611457760297346542852\
        808312236531894105842976564219243112231755769151738741538633137951466072661211616085355310\
        857774493679173076256383508892008233851964641402029759551974793310482402462041711945161548\
        326464873039374115874373991373282632741414920329349124653295705526587225112073460916295019\
        351673851763550205032343583177087022638899559022604128931488470928339695796081675002149402\
        705503760956452458569391936454552087427059271324878358397026924524406044511662789633794884\
        312566085791783949166112065856075232662140149368936155086704776086186500364186886553943513\
        209170716292378586966001695043767126312269700601051262635023004805630507146960562075511746\
        193744116099678233570896735643276617479233229464677226737161287382607246540458402419168410\
        369080032442595204449841004182771241558536904090276184130382096457426893023177519552932669\
        910328559505543058012609185253324264204948243252385345305151663497890783280043012055166242\
        868647524427377318531773354446540102614567355680535177054052252524092466096556675461869194\
        502097950414221238045052862533170581314105649117929016110826071463050044902113986948490263\
        880543969640100358021024367307867000301735766682570676407886519783729177203642562064756387\
        522705452837219505239146678435510613803649417614863894095412079053170504045514553464488107\
        056711353135654973323610571363687790680289636128613293872689308014945837435447842138998905\
        2458651564368105546071376972806302466513398",
        "0x1.20dd750429b6d11ae3a914fed7fd8688281341d7587cea2e7342b06199cc416180eb39f0b24e1e2281806\
        c12d98f35d77a3e9ddc91c394f0e9eedf0efffd84a2a4ac3b98489b8cbd3845e8fef6ff6af92a45e5f27c2d654\
        7a4f46505b5b4e62dd73fd6486de5e4e5585911777503638ea0ea96813d29a65a31a7d235b9fed80735a739f0c\
        dba12c519a4d0308fceb248f76d1a1b6642fad251a58a530d43df05fad7bc33a90c79b03bcd7729e27b629a758\
        4a30de9e46e24a71a8c3d52f6003ce62af51cbfa1129bfaf37d42219a1910d82dc1985ae3e4662f05fc75ed5e7\
        1b2bd75e97b95e80fe24ff9dc42273cac1e83311f7493092145e08d7abb2500c1067f03dfdff9363e8edcef4b8\
        ac1a2da635fe6ab630cb1893bfff7e5649d7b0aae1d49c6aa3cc08ba146db963c8dfd22432b1cbc010b48c3a5c\
        d3dfa2509f6be88b407d1059307b923d91ffb1db6618afd15a92a5ba06cc83afaa446091eaacf54b522f1945d1\
        91e4dacda113387974af3a343f7ce369957efdc384258530fcf9037413bb07fa3705b934bac814576820d20740\
        aa64c532696cc455b79c64ad5cce5fff52d9aa114ec9ff1074edc6309a26029b70a9232e59eaaa501aa0aca4a2\
        7728273c272ec6bb3470b095b7c747a2ef9e3c620e65f0f648cc2d13a4b4302e197a71e8ecdbc01118580a1c11\
        a341c7934d311d73f344b85d072c47b21e844d6b92cb6738ad4aeef5149c0f4beedcbb58ba8d1a4138068ddfc4\
        afdf301a3bd5f02b99abc0cc348add7d5e09ccd55b1dac8c54e22fc24f30c8c4ce5a86e55205efb686835d5904\
        a675e2c8135ba596719fbaa19ba789266db7b1ab4db36b1cb412b4d5ee13fddf1768dbc347d2a6d4c536bac90f\
        fffe356c775ee06458e42d744d6a16ed51ac40e9e2033422963d77a545f8b5aee2ea3b37c23db4f504d47b986c\
        a2d543ab7f158f26d46f44ba838e2cea777469249aff72afd71755af1ceac1fa7ad0c8c536ed4f2b2425458583\
        3d07df62553eb1ffffc1d0f3df7714180ace2618d12251cfea9e6a60a5ff086d8cd2d7a0f42e90b054aeadc826\
        803b731044178f676b13f2294d97b99ed318e0aa55fa0d4f2b6ab619c577093cce0852386de4f542e39dcfd5cb\
        4c8d73693cb51fccd163f7d0f8553874137d7a9d03886c41b7a347a8831e1add4f861d48e93b7bdb34a69871d4\
        e8af6148204ec26c6d1a154b3331efde63e122929a6dceb7786262d41694e2ba47da460a8a6e130b982445ea0e\
        45b400ea6ea6a7044e25163551bc27945081d344524130f3f1e8d0fc0a21a5b5a2dca8624b0501a0a00737ea60\
        4328ee18c6a983887ec3a2325eaf3f288194a7ef259177ff866265e4030896fd1b5f757135db4ad9164768e3bc\
        05755d775c9466c11c1a9cd9d3805fd322c9abff54269ff248d5c01e998dacac195aa549174422e94b1775e758\
        33cc9ef905054e08c1488ba83a5f846f053cd01762f2b68f4c9978ec41a53f8ee6c5b90ed2c27e83fa68f02bca\
        516823862007222eee333565e467ceafd23310af018dd80ba829818b82c9e9625c711e87079fca694ddbd3fbbf\
        6d0113ee672fc425c1bf33fe92da8753e6b576bc07f16f5381ad615fc5d82278d2308edc3ec6853ec1c7220860\
        f5d5e1d380d8a3e93c6395f420f4ad28f7da5583282ac819cdb68a6e5a6761e1ab90f8d78981c1fa1503b46dfb\
        78235f460132e2eb2877c1fca3e7a1644373b339635421e388ec3d17c19ef2417f476d13aa6#10000",
        Less,
    );

    let two_over_sqrt_pi_f32 = Float::two_over_sqrt_pi_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(two_over_sqrt_pi_f32.to_string(), "1.1283792");
    assert_eq!(to_hex_string(&two_over_sqrt_pi_f32), "0x1.20dd76#24");
    assert_eq!(two_over_sqrt_pi_f32, f32::TWO_OVER_SQRT_PI);

    let two_over_sqrt_pi_f64 = Float::two_over_sqrt_pi_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(two_over_sqrt_pi_f64.to_string(), "1.1283791670955126");
    assert_eq!(to_hex_string(&two_over_sqrt_pi_f64), "0x1.20dd750429b6d#53");
    assert_eq!(two_over_sqrt_pi_f64, f64::TWO_OVER_SQRT_PI);
}

#[test]
#[should_panic]
fn two_over_sqrt_pi_prec_fail_1() {
    Float::two_over_sqrt_pi_prec(0);
}

fn test_two_over_sqrt_pi_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::two_over_sqrt_pi_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_two_over_sqrt_pi_prec_round() {
    test_two_over_sqrt_pi_prec_round_helper(1, Floor, "1.0", "0x1.0#1", Less);
    test_two_over_sqrt_pi_prec_round_helper(1, Ceiling, "2.0", "0x2.0#1", Greater);
    test_two_over_sqrt_pi_prec_round_helper(1, Down, "1.0", "0x1.0#1", Less);
    test_two_over_sqrt_pi_prec_round_helper(1, Up, "2.0", "0x2.0#1", Greater);
    test_two_over_sqrt_pi_prec_round_helper(1, Nearest, "1.0", "0x1.0#1", Less);

    test_two_over_sqrt_pi_prec_round_helper(2, Floor, "1.0", "0x1.0#2", Less);
    test_two_over_sqrt_pi_prec_round_helper(2, Ceiling, "1.5", "0x1.8#2", Greater);
    test_two_over_sqrt_pi_prec_round_helper(2, Down, "1.0", "0x1.0#2", Less);
    test_two_over_sqrt_pi_prec_round_helper(2, Up, "1.5", "0x1.8#2", Greater);
    test_two_over_sqrt_pi_prec_round_helper(2, Nearest, "1.0", "0x1.0#2", Less);

    test_two_over_sqrt_pi_prec_round_helper(3, Floor, "1.0", "0x1.0#3", Less);
    test_two_over_sqrt_pi_prec_round_helper(3, Ceiling, "1.2", "0x1.4#3", Greater);
    test_two_over_sqrt_pi_prec_round_helper(3, Down, "1.0", "0x1.0#3", Less);
    test_two_over_sqrt_pi_prec_round_helper(3, Up, "1.2", "0x1.4#3", Greater);
    test_two_over_sqrt_pi_prec_round_helper(3, Nearest, "1.2", "0x1.4#3", Greater);

    test_two_over_sqrt_pi_prec_round_helper(4, Floor, "1.1", "0x1.2#4", Less);
    test_two_over_sqrt_pi_prec_round_helper(4, Ceiling, "1.2", "0x1.4#4", Greater);
    test_two_over_sqrt_pi_prec_round_helper(4, Down, "1.1", "0x1.2#4", Less);
    test_two_over_sqrt_pi_prec_round_helper(4, Up, "1.2", "0x1.4#4", Greater);
    test_two_over_sqrt_pi_prec_round_helper(4, Nearest, "1.1", "0x1.2#4", Less);

    test_two_over_sqrt_pi_prec_round_helper(5, Floor, "1.12", "0x1.2#5", Less);
    test_two_over_sqrt_pi_prec_round_helper(5, Ceiling, "1.19", "0x1.3#5", Greater);
    test_two_over_sqrt_pi_prec_round_helper(5, Down, "1.12", "0x1.2#5", Less);
    test_two_over_sqrt_pi_prec_round_helper(5, Up, "1.19", "0x1.3#5", Greater);
    test_two_over_sqrt_pi_prec_round_helper(5, Nearest, "1.12", "0x1.2#5", Less);

    test_two_over_sqrt_pi_prec_round_helper(6, Floor, "1.12", "0x1.20#6", Less);
    test_two_over_sqrt_pi_prec_round_helper(6, Ceiling, "1.16", "0x1.28#6", Greater);
    test_two_over_sqrt_pi_prec_round_helper(6, Down, "1.12", "0x1.20#6", Less);
    test_two_over_sqrt_pi_prec_round_helper(6, Up, "1.16", "0x1.28#6", Greater);
    test_two_over_sqrt_pi_prec_round_helper(6, Nearest, "1.12", "0x1.20#6", Less);

    test_two_over_sqrt_pi_prec_round_helper(7, Floor, "1.12", "0x1.20#7", Less);
    test_two_over_sqrt_pi_prec_round_helper(7, Ceiling, "1.14", "0x1.24#7", Greater);
    test_two_over_sqrt_pi_prec_round_helper(7, Down, "1.12", "0x1.20#7", Less);
    test_two_over_sqrt_pi_prec_round_helper(7, Up, "1.14", "0x1.24#7", Greater);
    test_two_over_sqrt_pi_prec_round_helper(7, Nearest, "1.12", "0x1.20#7", Less);

    test_two_over_sqrt_pi_prec_round_helper(8, Floor, "1.125", "0x1.20#8", Less);
    test_two_over_sqrt_pi_prec_round_helper(8, Ceiling, "1.13", "0x1.22#8", Greater);
    test_two_over_sqrt_pi_prec_round_helper(8, Down, "1.125", "0x1.20#8", Less);
    test_two_over_sqrt_pi_prec_round_helper(8, Up, "1.13", "0x1.22#8", Greater);
    test_two_over_sqrt_pi_prec_round_helper(8, Nearest, "1.125", "0x1.20#8", Less);

    test_two_over_sqrt_pi_prec_round_helper(9, Floor, "1.125", "0x1.20#9", Less);
    test_two_over_sqrt_pi_prec_round_helper(9, Ceiling, "1.129", "0x1.21#9", Greater);
    test_two_over_sqrt_pi_prec_round_helper(9, Down, "1.125", "0x1.20#9", Less);
    test_two_over_sqrt_pi_prec_round_helper(9, Up, "1.129", "0x1.21#9", Greater);
    test_two_over_sqrt_pi_prec_round_helper(9, Nearest, "1.129", "0x1.21#9", Greater);

    test_two_over_sqrt_pi_prec_round_helper(10, Floor, "1.127", "0x1.208#10", Less);
    test_two_over_sqrt_pi_prec_round_helper(10, Ceiling, "1.129", "0x1.210#10", Greater);
    test_two_over_sqrt_pi_prec_round_helper(10, Down, "1.127", "0x1.208#10", Less);
    test_two_over_sqrt_pi_prec_round_helper(10, Up, "1.129", "0x1.210#10", Greater);
    test_two_over_sqrt_pi_prec_round_helper(10, Nearest, "1.129", "0x1.210#10", Greater);

    test_two_over_sqrt_pi_prec_round_helper(
        100,
        Floor,
        "1.12837916709551257389615890312",
        "0x1.20dd750429b6d11ae3a914fec#100",
        Less,
    );
    test_two_over_sqrt_pi_prec_round_helper(
        100,
        Ceiling,
        "1.128379167095512573896158903122",
        "0x1.20dd750429b6d11ae3a914fee#100",
        Greater,
    );
    test_two_over_sqrt_pi_prec_round_helper(
        100,
        Down,
        "1.12837916709551257389615890312",
        "0x1.20dd750429b6d11ae3a914fec#100",
        Less,
    );
    test_two_over_sqrt_pi_prec_round_helper(
        100,
        Up,
        "1.128379167095512573896158903122",
        "0x1.20dd750429b6d11ae3a914fee#100",
        Greater,
    );
    test_two_over_sqrt_pi_prec_round_helper(
        100,
        Nearest,
        "1.128379167095512573896158903122",
        "0x1.20dd750429b6d11ae3a914fee#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn two_over_sqrt_pi_prec_round_fail_1() {
    Float::two_over_sqrt_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn two_over_sqrt_pi_prec_round_fail_2() {
    Float::two_over_sqrt_pi_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn two_over_sqrt_pi_prec_round_fail_3() {
    Float::two_over_sqrt_pi_prec_round(1000, Exact);
}

#[test]
fn two_over_sqrt_pi_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (two_over_sqrt_pi, o) = Float::two_over_sqrt_pi_prec(prec);
        assert!(two_over_sqrt_pi.is_valid());
        assert_eq!(two_over_sqrt_pi.get_prec(), Some(prec));
        assert_eq!(two_over_sqrt_pi.get_exponent(), Some(1));
        assert_ne!(o, Equal);
        if o == Less {
            let (two_over_sqrt_pi_alt, o_alt) = Float::two_over_sqrt_pi_prec_round(prec, Ceiling);
            let mut next_upper = two_over_sqrt_pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(two_over_sqrt_pi_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !two_over_sqrt_pi.is_power_of_2() {
            let (two_over_sqrt_pi_alt, o_alt) = Float::two_over_sqrt_pi_prec_round(prec, Floor);
            let mut next_lower = two_over_sqrt_pi.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(two_over_sqrt_pi_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (two_over_sqrt_pi_alt, o_alt) = Float::two_over_sqrt_pi_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&two_over_sqrt_pi_alt),
            ComparableFloatRef(&two_over_sqrt_pi)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn two_over_sqrt_pi_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (two_over_sqrt_pi, o) = Float::two_over_sqrt_pi_prec_round(prec, rm);
        assert!(two_over_sqrt_pi.is_valid());
        assert_eq!(two_over_sqrt_pi.get_prec(), Some(prec));
        assert_eq!(
            two_over_sqrt_pi.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                2
            } else {
                1
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (two_over_sqrt_pi_alt, o_alt) = Float::two_over_sqrt_pi_prec_round(prec, Ceiling);
            let mut next_upper = two_over_sqrt_pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(two_over_sqrt_pi_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !two_over_sqrt_pi.is_power_of_2() {
            let (two_over_sqrt_pi_alt, o_alt) = Float::two_over_sqrt_pi_prec_round(prec, Floor);
            let mut next_lower = two_over_sqrt_pi.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(two_over_sqrt_pi_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::two_over_sqrt_pi_prec_round(prec, Exact));
    });

    test_constant(Float::two_over_sqrt_pi_prec_round, 10000);
}
