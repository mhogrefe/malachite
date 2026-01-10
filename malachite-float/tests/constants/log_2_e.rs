// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Log2E;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::constants::log_2_e::log_2_e_prec_round_simple;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_log_2_e_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::log_2_e_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = log_2_e_prec_round_simple(prec, Nearest);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_log_2_e_prec() {
    test_log_2_e_prec_helper(1, "1.0", "0x1.0#1", Less);
    test_log_2_e_prec_helper(2, "1.5", "0x1.8#2", Greater);
    test_log_2_e_prec_helper(3, "1.5", "0x1.8#3", Greater);
    test_log_2_e_prec_helper(4, "1.5", "0x1.8#4", Greater);
    test_log_2_e_prec_helper(5, "1.44", "0x1.7#5", Less);
    test_log_2_e_prec_helper(6, "1.44", "0x1.70#6", Less);
    test_log_2_e_prec_helper(7, "1.44", "0x1.70#7", Less);
    test_log_2_e_prec_helper(8, "1.445", "0x1.72#8", Greater);
    test_log_2_e_prec_helper(9, "1.441", "0x1.71#9", Less);
    test_log_2_e_prec_helper(10, "1.443", "0x1.718#10", Greater);
    test_log_2_e_prec_helper(
        100,
        "1.442695040888963407359924681003",
        "0x1.71547652b82fe1777d0ffda0e#100",
        Greater,
    );
    test_log_2_e_prec_helper(
        1000,
        "1.442695040888963407359924681001892137426645954152985934135449406931109219181185079885526\
        622893506344496997518309652544255593101687168359642720662158223479336274537369884718493630\
        701387663532015533894318916664837643128615424047478422289497904795091530351338588054968865\
        8930969963680361105110756308441454",
        "0x1.71547652b82fe1777d0ffda0d23a7d11d6aef551bad2b4b1164a2cd9a342648fbc3887eeaa2ed9ac49b25\
        eeb82d7c167d52173cc1895213f897f5e06a7be73665fc529264c2fb3ab643687aaf3ab440c16bd777e75050a8\
        d1a39e8af56c64a7833352906deb692ce4f199e108cf392819cfc406b19abb71ec25e11f75c#1000",
        Less,
    );
    test_log_2_e_prec_helper(
        10000,
        "1.442695040888963407359924681001892137426645954152985934135449406931109219181185079885526\
        622893506344496997518309652544255593101687168359642720662158223479336274537369884718493630\
        701387663532015533894318916664837643128615424047478422289497904795091530351338588054968865\
        893096996368036110511075630844145427215828344941891933908577715790044171280246848341374522\
        695182369011239094034459968539906113421722886278029158010630061976762445652605995073753240\
        625655815475938178305239725510724813077156267545807578171330193573006168761937372982675897\
        415623817983567103443489750680705518088486561386832917732182934913968431059345402202518636\
        934526269215095597191002219679224321433424494179071455118499385921221675365311300774632767\
        206461233741108211913794433398480579310912877609670200375758998158851806126788099760956252\
        507841024847056900768768058461327865474782027808659462060910749015324819969730579015272324\
        787298740981254100033448687573822364716494544753706716759589942809981826783490131666633534\
        803678986944688709116660497353729258607212948697354540708098306748938341237186314008359796\
        188659758687452533054689212976641570420621259246313692421680590877408335813928666541584971\
        162587069556578588747699631296952500459372627389026805669355128729433837219131116650881001\
        587862655915637954055905677822368140030968843934808622848184791345633141193023840264097274\
        843644962195449224465222047176358607479658556660534098286098574027883743312688563354434306\
        978701896435826139118100252599020766184432984883184723915912701390457047735764831010211928\
        297085328960931680353919649869573264393791490308485470616433789856348238900004564261855622\
        496930913960312520223767376074153862116245551165086436799129389371225572752855358505388627\
        546928167550407303918984389641052039899021078907741074670715487187445927826480325745329406\
        836552544103465737320315138225129361437624142202250714370369730734609414850108603189323604\
        113311115744937702491468814553609722861672425272088889061517451052531559178316247029430178\
        095934252371975125612329569505926858901075573121447832714438655839592620356007499708416567\
        681679268721978983048302281782977385152293797381195278398266923467818982723138352442777865\
        647623134859901194028780732484171511058619349202546881837818357300094700147502951964817837\
        874039354216278848238941974695520862627419471357392597226651239427201166462692938707284017\
        956993398889202501277913459329094676020415764879790841607401359157889710773691716288172692\
        755182517960232474350173532606863738793763572044458313264353526509290061748888247033974920\
        459020557724020364994277699238470527177685203357040125910731736639056137520453197778773562\
        797180625921321743667984249874334562328228971247257945609197595215055835184523639531383497\
        657376260166981167685146145560638695562162638896323160272971094995069592801744350030798593\
        924157506631512149867563008065061831442109254227561277967430907171276820183902280303065724\
        252294800267075913436729080697447776799229453917140856561043148672020633912643384787632773\
        495535569020831968308365499869958364273149",
        "0x1.71547652b82fe1777d0ffda0d23a7d11d6aef551bad2b4b1164a2cd9a342648fbc3887eeaa2ed9ac49b25\
        eeb82d7c167d52173cc1895213f897f5e06a7be73665fc529264c2fb3ab643687aaf3ab440c16bd777e75050a8\
        d1a39e8af56c64a7833352906deb692ce4f199e108cf392819cfc406b19abb71ec25e11f75c6142e64ca16da20\
        b1d74a12c719098b4040cbe82351bd8bd422427231f9ee25bd0c470fe2464b892824c120f2d07db43448b6bdd3\
        58165f1a2b3d8675f9ee1d8d19ce22ece1d8e1cf2ed95025c73a0b608ecb0ac9cb843a1c53b021698c0eba2176\
        77f7d0b9c4b6e004ca5169aedd5b0fbf79220755e0827ae421af38c350dbfd200f5437ad88854a185bbc4bbb5f\
        9231797de27c1982e15c6f87a30b468c39b2a586e51fb9211a51bfd1e1075e545eff26d6bfc0b51b3db80dbcb2\
        4254977733864dadcb616d749e415eb60cbda42f3df5d4a527d762fb97efdb287e16c60c8e5cbd7fc5b97445f4\
        a50a628f826ca66964b903cfa5da099460790c09071c1e23caa41ed00d87cd3ea10639d896b14ff19af35259c5\
        375f02f650e4e4154a0f0ad4150c9f23c2ac41a153c154c13420555a1dd2359b43d861faa27020f89240cce172\
        8bc41b06c19c811d57a442f32964754bf5393c68fce7c1143becd02ebdb9d9d6cd6ce0772765a409b32d4e2ff5\
        e479ea9cf606f388b14bf1040fd7ac3c572fa96799afa88d9b908ee33cc144dd7b3a9034befd1786dffd1b374c\
        3ab057c78308382a2ecb4a8ae9ebfe2400db7e42eb1afaf630de1ee8d85808160092d7bfb276e839390f2cc6dd\
        801dd8d0791db4a9e602c37179d4d793f85e6b7de417c9d92c6cded0dec3a15568bd8c41b35d16b262f9c9b86f\
        8efb146d75817a5a20dd9f20c8b36649fc6713eb9a596b7b63c90b82f4efacb3afa37dbda4bd2259ec05bf0464\
        c73c98129349a177c1beac7023630d669ff199f5bb7b898c5f42057260d95232260b815e608b14add28858c21b\
        afcb7061a345db80ebbd88c1a8cd7d558c3e49fde7e67e61b47752cc51aa59a7a8909a7888a2a53158b35e487f\
        9ccf8914ce12c742a93e9cb6d7360a5e4fb2e4acdd70f3a7c5afb7233d9547378dbd9a0f8eee80c01446b265c0\
        30bbd0bba9aa245c1a351aa40c8e9a6c1a2f9259df8d78033b8e5b1fbb46c6ff0514ca6f17c72498ff937dc7ef\
        1270ea59b36d4b6f41f6d327e363e04463279e5a893d88566e171b92c0af10b9eb3015a189e17eb81288143e5c\
        ea089a45ca8b5c69aeb433e4dc420589191a767f85e5aa15108ac58de430f24ddc986f1a18377639dcc819b7fe\
        b92041aeb238c812adf284474f81d729d962149f69e3aa2000a51acde13f7d8c097cdde28b92b7323ee95f66b3\
        c11bb8a14c27dad07685f8ebe4195220ef186f13536485dd4d3f6d1a01a7f5dc63145fb5688742fa36da1ef616\
        c00760f0ca4ea16514fed4ceb9a0b1afafe73aaa7a69727855effa8662bb16794ee125dc16c57b422bf1ab01bf\
        576392dd8813dc17771e2e4956c8bf568126613e4814945f3583b02acc36249d1db915aecab476582cf6dc8646\
        02ba37849ddb40cc61052fdd5001e3ecd7ac0e1d9d011d01f5392aaaf33b9d2a91f01e5ded9a6ec3f5b1e01e45\
        e96fb5653006d8a86b8986eb4bfc9b73352350d82bbde9d247cbd4b56ca2d8fd78c0244b2003418eef32ffec82\
        a35b9c7c64ed6f67a012528ad9fcd8c71017143f043b22cace3bfa4730f9c22bc761a40f5ea#10000",
        Less,
    );

    let log_2_e_f32 = Float::log_2_e_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(log_2_e_f32.to_string(), "1.442695");
    assert_eq!(to_hex_string(&log_2_e_f32), "0x1.715476#24");
    assert_eq!(log_2_e_f32, f32::LOG_2_E);

    let log_2_e_f64 = Float::log_2_e_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(log_2_e_f64.to_string(), "1.4426950408889634");
    assert_eq!(to_hex_string(&log_2_e_f64), "0x1.71547652b82fe#53");
    assert_eq!(log_2_e_f64, f64::LOG_2_E);
}

fn test_log_2_e_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::log_2_e_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = log_2_e_prec_round_simple(prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_log_2_e_prec_round() {
    test_log_2_e_prec_round_helper(1, Floor, "1.0", "0x1.0#1", Less);
    test_log_2_e_prec_round_helper(1, Ceiling, "2.0", "0x2.0#1", Greater);
    test_log_2_e_prec_round_helper(1, Down, "1.0", "0x1.0#1", Less);
    test_log_2_e_prec_round_helper(1, Up, "2.0", "0x2.0#1", Greater);
    test_log_2_e_prec_round_helper(1, Nearest, "1.0", "0x1.0#1", Less);

    test_log_2_e_prec_round_helper(2, Floor, "1.0", "0x1.0#2", Less);
    test_log_2_e_prec_round_helper(2, Ceiling, "1.5", "0x1.8#2", Greater);
    test_log_2_e_prec_round_helper(2, Down, "1.0", "0x1.0#2", Less);
    test_log_2_e_prec_round_helper(2, Up, "1.5", "0x1.8#2", Greater);
    test_log_2_e_prec_round_helper(2, Nearest, "1.5", "0x1.8#2", Greater);

    test_log_2_e_prec_round_helper(3, Floor, "1.2", "0x1.4#3", Less);
    test_log_2_e_prec_round_helper(3, Ceiling, "1.5", "0x1.8#3", Greater);
    test_log_2_e_prec_round_helper(3, Down, "1.2", "0x1.4#3", Less);
    test_log_2_e_prec_round_helper(3, Up, "1.5", "0x1.8#3", Greater);
    test_log_2_e_prec_round_helper(3, Nearest, "1.5", "0x1.8#3", Greater);

    test_log_2_e_prec_round_helper(4, Floor, "1.4", "0x1.6#4", Less);
    test_log_2_e_prec_round_helper(4, Ceiling, "1.5", "0x1.8#4", Greater);
    test_log_2_e_prec_round_helper(4, Down, "1.4", "0x1.6#4", Less);
    test_log_2_e_prec_round_helper(4, Up, "1.5", "0x1.8#4", Greater);
    test_log_2_e_prec_round_helper(4, Nearest, "1.5", "0x1.8#4", Greater);

    test_log_2_e_prec_round_helper(5, Floor, "1.44", "0x1.7#5", Less);
    test_log_2_e_prec_round_helper(5, Ceiling, "1.5", "0x1.8#5", Greater);
    test_log_2_e_prec_round_helper(5, Down, "1.44", "0x1.7#5", Less);
    test_log_2_e_prec_round_helper(5, Up, "1.5", "0x1.8#5", Greater);
    test_log_2_e_prec_round_helper(5, Nearest, "1.44", "0x1.7#5", Less);

    test_log_2_e_prec_round_helper(6, Floor, "1.44", "0x1.70#6", Less);
    test_log_2_e_prec_round_helper(6, Ceiling, "1.47", "0x1.78#6", Greater);
    test_log_2_e_prec_round_helper(6, Down, "1.44", "0x1.70#6", Less);
    test_log_2_e_prec_round_helper(6, Up, "1.47", "0x1.78#6", Greater);
    test_log_2_e_prec_round_helper(6, Nearest, "1.44", "0x1.70#6", Less);

    test_log_2_e_prec_round_helper(7, Floor, "1.44", "0x1.70#7", Less);
    test_log_2_e_prec_round_helper(7, Ceiling, "1.45", "0x1.74#7", Greater);
    test_log_2_e_prec_round_helper(7, Down, "1.44", "0x1.70#7", Less);
    test_log_2_e_prec_round_helper(7, Up, "1.45", "0x1.74#7", Greater);
    test_log_2_e_prec_round_helper(7, Nearest, "1.44", "0x1.70#7", Less);

    test_log_2_e_prec_round_helper(8, Floor, "1.44", "0x1.70#8", Less);
    test_log_2_e_prec_round_helper(8, Ceiling, "1.445", "0x1.72#8", Greater);
    test_log_2_e_prec_round_helper(8, Down, "1.44", "0x1.70#8", Less);
    test_log_2_e_prec_round_helper(8, Up, "1.445", "0x1.72#8", Greater);
    test_log_2_e_prec_round_helper(8, Nearest, "1.445", "0x1.72#8", Greater);

    test_log_2_e_prec_round_helper(9, Floor, "1.441", "0x1.71#9", Less);
    test_log_2_e_prec_round_helper(9, Ceiling, "1.445", "0x1.72#9", Greater);
    test_log_2_e_prec_round_helper(9, Down, "1.441", "0x1.71#9", Less);
    test_log_2_e_prec_round_helper(9, Up, "1.445", "0x1.72#9", Greater);
    test_log_2_e_prec_round_helper(9, Nearest, "1.441", "0x1.71#9", Less);

    test_log_2_e_prec_round_helper(10, Floor, "1.441", "0x1.710#10", Less);
    test_log_2_e_prec_round_helper(10, Ceiling, "1.443", "0x1.718#10", Greater);
    test_log_2_e_prec_round_helper(10, Down, "1.441", "0x1.710#10", Less);
    test_log_2_e_prec_round_helper(10, Up, "1.443", "0x1.718#10", Greater);
    test_log_2_e_prec_round_helper(10, Nearest, "1.443", "0x1.718#10", Greater);

    test_log_2_e_prec_round_helper(
        100,
        Floor,
        "1.442695040888963407359924681001",
        "0x1.71547652b82fe1777d0ffda0c#100",
        Less,
    );
    test_log_2_e_prec_round_helper(
        100,
        Ceiling,
        "1.442695040888963407359924681003",
        "0x1.71547652b82fe1777d0ffda0e#100",
        Greater,
    );
    test_log_2_e_prec_round_helper(
        100,
        Down,
        "1.442695040888963407359924681001",
        "0x1.71547652b82fe1777d0ffda0c#100",
        Less,
    );
    test_log_2_e_prec_round_helper(
        100,
        Up,
        "1.442695040888963407359924681003",
        "0x1.71547652b82fe1777d0ffda0e#100",
        Greater,
    );
    test_log_2_e_prec_round_helper(
        100,
        Nearest,
        "1.442695040888963407359924681003",
        "0x1.71547652b82fe1777d0ffda0e#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn log_2_e_prec_round_fail_1() {
    Float::log_2_e_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn log_2_e_prec_round_fail_2() {
    Float::log_2_e_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn log_2_e_prec_round_fail_3() {
    Float::log_2_e_prec_round(1000, Exact);
}

#[test]
fn log_2_e_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (log_2_e, o) = Float::log_2_e_prec(prec);
        assert!(log_2_e.is_valid());
        assert_eq!(log_2_e.get_prec(), Some(prec));
        assert_eq!(log_2_e.get_exponent(), Some(1));
        assert_ne!(o, Equal);
        if o == Less {
            let (log_2_e_alt, o_alt) = Float::log_2_e_prec_round(prec, Ceiling);
            let mut next_upper = log_2_e.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(log_2_e_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !log_2_e.is_power_of_2() {
            let (log_2_e_alt, o_alt) = Float::log_2_e_prec_round(prec, Floor);
            let mut next_lower = log_2_e.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(log_2_e_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (log_2_e_alt, o_alt) = Float::log_2_e_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&log_2_e_alt),
            ComparableFloatRef(&log_2_e)
        );
        assert_eq!(o_alt, o);

        let (log_2_e_alt, o_alt) = log_2_e_prec_round_simple(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&log_2_e_alt),
            ComparableFloatRef(&log_2_e)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn log_2_e_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (log_2_e, o) = Float::log_2_e_prec_round(prec, rm);
        assert!(log_2_e.is_valid());
        assert_eq!(log_2_e.get_prec(), Some(prec));
        assert_eq!(
            log_2_e.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                2
            } else {
                1
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (log_2_e_alt, o_alt) = Float::log_2_e_prec_round(prec, Ceiling);
            let mut next_upper = log_2_e.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(log_2_e_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !log_2_e.is_power_of_2() {
            let (log_2_e_alt, o_alt) = Float::log_2_e_prec_round(prec, Floor);
            let mut next_lower = log_2_e.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(log_2_e_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        let (log_2_e_alt, o_alt) = log_2_e_prec_round_simple(prec, rm);
        assert_eq!(
            ComparableFloatRef(&log_2_e_alt),
            ComparableFloatRef(&log_2_e)
        );
        assert_eq!(o_alt, o);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::log_2_e_prec_round(prec, Exact));
    });

    test_constant(Float::log_2_e_prec_round, 10000);
}
