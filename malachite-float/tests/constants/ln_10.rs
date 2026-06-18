// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Ln10;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::ln_10::rug_ln_10_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_ln_10_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::ln_10_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = Float::ln_prec(Float::from(10), prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (rug_x, rug_o) =
        rug_ln_10_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_ln_10_prec() {
    test_ln_10_prec_helper(1, "2.0", "0x2.0#1", Less);
    test_ln_10_prec_helper(2, "2.0", "0x2.0#2", Less);
    test_ln_10_prec_helper(3, "2.5", "0x2.8#3", Greater);
    test_ln_10_prec_helper(4, "2.2", "0x2.4#4", Less);
    test_ln_10_prec_helper(5, "2.2", "0x2.4#5", Less);
    test_ln_10_prec_helper(6, "2.3", "0x2.5#6", Greater);
    test_ln_10_prec_helper(7, "2.31", "0x2.50#7", Greater);
    test_ln_10_prec_helper(8, "2.3", "0x2.4c#8", Less);
    test_ln_10_prec_helper(9, "2.305", "0x2.4e#9", Greater);
    test_ln_10_prec_helper(10, "2.301", "0x2.4d#10", Less);
    test_ln_10_prec_helper(
        100,
        "2.302585092994045684017991454684",
        "0x2.4d763776aaa2b05ba95b58ae0#100",
        Less,
    );
    test_ln_10_prec_helper(
        1000,
        "2.302585092994045684017991454684364207601101488628772976033327900967572609677352480235997\
        205089598298341967784042286248633409525465082806756666287369098781689482907208325554680843\
        799894826233198528393505308965377732628846163366222287698219886746543667474404243274365155\
        0489343149393914796194044002221051",
        "0x2.4d763776aaa2b05ba95b58ae0b4c28a38a3fb3e76977e43a0f187a0807c0b5ca58bc0b5ec6a0417331c32\
        f00b17c35a0b1889061042f8b6bee3de2100b945b59e0b3e28a2a324479d96a9b0ec360c7efbd9b3ac12acf1be\
        94586ed2748671eef299ecd6c8d8142163a4cda3511e2713d6c22c15f57b7883d1a7a963a4c#1000",
        Less,
    );
    test_ln_10_prec_helper(
        10000,
        "2.302585092994045684017991454684364207601101488628772976033327900967572609677352480235997\
        205089598298341967784042286248633409525465082806756666287369098781689482907208325554680843\
        799894826233198528393505308965377732628846163366222287698219886746543667474404243274365155\
        048934314939391479619404400222105101714174800368808401264708068556774321622835522011480466\
        371565912137345074785694768346361679210180644507064800027750268491674655058685693567342067\
        058113642922455440575892572420824131469568901675894025677631135691929203337658714166023010\
        570308963457207544037084746994016826928280848118428931484852494864487192780967627127577539\
        702766860595249671667418348570442250719796500471495105049221477656763693866297697952211071\
        826454973477266242570942932258279850258550978526538320760672631716430950599508780752371033\
        310119785754733154142180842754386359177811705430982748238504564801909561029929182431823752\
        535770975053956518769751037497088869218020518933950723853920514463419726528728696511086257\
        149219884997874887377134568620916705849807828059751193854445009978131146915934666241071846\
        692310107598438319191292230792503747298650929009880391941702654416816335727555703151596113\
        564846546190897042819763365836983716328982174407366009162177850541779276367731145041782137\
        660111010731042397832521894898817597921798666394319523936855916447118246753245630912528778\
        330963604262982153040874560927760726641354787576616262926568298704957954913954918049209069\
        438580790032763017941503117866862092408537949861264933479354871737451675809537088281067452\
        440105892444976479686075120275724181874989395971643105518848195288330746699317814634930000\
        321200327765654130472621883970596794457943468343218395304414844803701305753674262153675579\
        814770458031413637793236291560128185336498466942261465206459942072917119370602444929358037\
        007718981097362533224548366988505528285966192805098447175198503666680874970496982273220244\
        823343097169111136813588418696549323714996941979687803008850408979618598756579894836445212\
        043698216415292987811742973332588607915912510967187510929248475023930572665446276200923068\
        791518135803477701295593646298412366497023355174586195564772461857717369368404676577047874\
        319780573853271810933883496338813069945569399346101090745616033312247949360455361849123333\
        063704751724871276379140924398331810164737823379692265637682071706935846394531616949411701\
        841938119405416449466111274712819705817783293841742231409930022911502362192186723337268385\
        688273533371925103412930705632544426611429765388301822384091026198582888433587455960453004\
        548370789052578473166283701953392231047527564998119228742789713715713228319641003422124210\
        082180679525276689858180956119208391760721080919923461516952599099473782780648128058792731\
        993893453415320185969711021407542282796298237068941764740642225757212455392526179373652434\
        440560595336591539160312524480149313234572453879524389036839236450507881731359711238145323\
        701508413491122324390927681724749607955799151363982881058285740538000653371655553014196332\
        2419180876210182049194926514838926922937079",
        "0x2.4d763776aaa2b05ba95b58ae0b4c28a38a3fb3e76977e43a0f187a0807c0b5ca58bc0b5ec6a0417331c32\
        f00b17c35a0b1889061042f8b6bee3de2100b945b59e0b3e28a2a324479d96a9b0ec360c7efbd9b3ac12acf1be\
        94586ed2748671eef299ecd6c8d8142163a4cda3511e2713d6c22c15f57b7883d1a7a963a4c17a607891e3f2ab\
        4ebba627356d0b9a89c586691fb2c9a5e31753f6c74a3a95f53f703902fcf30785049a915d973789a0ce76fd1f\
        ea5b7ac9c4182be2121baa6dd0078f7f4f145d239b5b8e12323497ebc6f2ba2011fc5ec366d42a527aaab7da7a\
        297ddf8dd813a50e5838e295c03ff78b6c6b5afefff6086e82932c119b586e9923bbe672397da5d3cd8e60a9e3\
        291777f20250985e06449e9b8edc3f368b5ccb51ed792c7230396842aa9981294c93b0f72f193aa01b86615987\
        c74d9b08198e0d1357a10fc8190ae5c298b46391e3def563f3420c929ecca9b7be16817ad58c8e9087bd782c01\
        0428283670981e52a5dbfdbfcd8f6c02daccfd0b1637be28f14b7b5a6c4f70680dcc2d94937063059fa1a675b4\
        83a8b7bf4af401be2d2f85168ab7cf32ecc62769276f0498722cf936ab5a8db3b32cb56b96e26fdd4bed77ecd4\
        ffcc1971033f3bb206f96a6a59ff7715f12e0cc0401afc1892d03b5e95b1b37368b5bfa44e23a78f922a4fd7d7\
        8edbad2e071296d4f527a9a399a2baaea0f1fe3ed7c1b0d62b23dc3ea763166315d140aebd5325b212447c0120\
        b4254682bc08aaeeac203b25fbf4d9c7938394bc59adb357348983719de54b878d28fde659dfcb07523c4a512d\
        b14ae84c1f9c1359fe47040d7913a49f18c0f3c4aff8b34b8c7e3b9210cd13546c3cdf690ae7c2c3f644d3ecb6\
        7e05792436598a0fc8e589d9bb3aac793ced7f2191a049f8ad81b4434e917d2daddd840b4bf6ed0820323ea134\
        3fe7d17162bbce36c9043f47c73aeb5ccec8a054b297cdb77ca76a9510cd2e37babd96539189c4c7b5216d1a82\
        08b84a36d1c7294e4e2a1130efbdfcd485722ce0af138f931bc558a17d5305ead023b32fdfd7b45aacfdb3b4a1\
        1b7d15f853eed17671591ab525eb1a5d25051778cba4e41c4a3498f5485e7bed0935919475baf0327c2ed60e91\
        31904b21216eca69934c68b364a5511a68cac6af7dfd480b292a9b24cba6d4f3ce16ac4d89d07cd5977a701958\
        026407222aef761d3bf3dccfdd86b50d7cf020ff5ae3b802fbc471f154aa275633928875c7953c9055fdb1f518\
        88c5ba6255b5d14009f7b2f4e63612e62cda81c9309736c8ddd83a4c92da1d48f941a3b6b64ce4865fc02e14e8\
        c9accc31111944a4a58d2f019c385de01b78ce36b1154dd24ab866b760f3d14003d6d08ef39925f48278c38628\
        4f2fc1a1d9e5cc40f65214d9a0541aee8ba4ea691f75674ffc5f35d1a3081d6f954a90d1ed298e3bf5268b998f\
        bc9f6662248c16f06726cad067922d8edd8dd44de9d5d974d01776cf3f691b60b6bd1fde73078d842bddea90b5\
        11e377b981d1cdc1894f6c380fef3b5db0f01a01beddafddfa71638f69433735401b6a7528f4f0dbec0deaeafd\
        34b393687e22a367d21f6235f3a65bbadf92ae501a8bdccad6b74920635824d87dec9f7d5eeb9741ef2927e784\
        f2d046a2590534ce89d40f002d20326db3fbee3fd1a38e75f4e30b46d9b5ad26d57ef69bb1a89b1968a6615822\
        c0de10860a16843c1d621f35437df133ee2f231efb832a496ea0ff1a1314bafcf66f80de058#10000",
        Greater,
    );

    let ln_10_f32 = Float::ln_10_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(ln_10_f32.to_string(), "2.3025851");
    assert_eq!(to_hex_string(&ln_10_f32), "0x2.4d7638#24");
    assert_eq!(ln_10_f32, f32::LN_10);

    let ln_10_f64 = Float::ln_10_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(ln_10_f64.to_string(), "2.3025850929940459");
    assert_eq!(to_hex_string(&ln_10_f64), "0x2.4d763776aaa2c#53");
    assert_eq!(ln_10_f64, f64::LN_10);
}

#[test]
#[should_panic]
fn ln_10_prec_fail_1() {
    Float::ln_10_prec(0);
}

fn test_ln_10_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::ln_10_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_ln_10_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_ln_10_prec_round() {
    test_ln_10_prec_round_helper(1, Floor, "2.0", "0x2.0#1", Less);
    test_ln_10_prec_round_helper(1, Ceiling, "4.0", "0x4.0#1", Greater);
    test_ln_10_prec_round_helper(1, Down, "2.0", "0x2.0#1", Less);
    test_ln_10_prec_round_helper(1, Up, "4.0", "0x4.0#1", Greater);
    test_ln_10_prec_round_helper(1, Nearest, "2.0", "0x2.0#1", Less);

    test_ln_10_prec_round_helper(2, Floor, "2.0", "0x2.0#2", Less);
    test_ln_10_prec_round_helper(2, Ceiling, "3.0", "0x3.0#2", Greater);
    test_ln_10_prec_round_helper(2, Down, "2.0", "0x2.0#2", Less);
    test_ln_10_prec_round_helper(2, Up, "3.0", "0x3.0#2", Greater);
    test_ln_10_prec_round_helper(2, Nearest, "2.0", "0x2.0#2", Less);

    test_ln_10_prec_round_helper(3, Floor, "2.0", "0x2.0#3", Less);
    test_ln_10_prec_round_helper(3, Ceiling, "2.5", "0x2.8#3", Greater);
    test_ln_10_prec_round_helper(3, Down, "2.0", "0x2.0#3", Less);
    test_ln_10_prec_round_helper(3, Up, "2.5", "0x2.8#3", Greater);
    test_ln_10_prec_round_helper(3, Nearest, "2.5", "0x2.8#3", Greater);

    test_ln_10_prec_round_helper(4, Floor, "2.2", "0x2.4#4", Less);
    test_ln_10_prec_round_helper(4, Ceiling, "2.5", "0x2.8#4", Greater);
    test_ln_10_prec_round_helper(4, Down, "2.2", "0x2.4#4", Less);
    test_ln_10_prec_round_helper(4, Up, "2.5", "0x2.8#4", Greater);
    test_ln_10_prec_round_helper(4, Nearest, "2.2", "0x2.4#4", Less);

    test_ln_10_prec_round_helper(5, Floor, "2.2", "0x2.4#5", Less);
    test_ln_10_prec_round_helper(5, Ceiling, "2.4", "0x2.6#5", Greater);
    test_ln_10_prec_round_helper(5, Down, "2.2", "0x2.4#5", Less);
    test_ln_10_prec_round_helper(5, Up, "2.4", "0x2.6#5", Greater);
    test_ln_10_prec_round_helper(5, Nearest, "2.2", "0x2.4#5", Less);

    test_ln_10_prec_round_helper(6, Floor, "2.25", "0x2.4#6", Less);
    test_ln_10_prec_round_helper(6, Ceiling, "2.3", "0x2.5#6", Greater);
    test_ln_10_prec_round_helper(6, Down, "2.25", "0x2.4#6", Less);
    test_ln_10_prec_round_helper(6, Up, "2.3", "0x2.5#6", Greater);
    test_ln_10_prec_round_helper(6, Nearest, "2.3", "0x2.5#6", Greater);

    test_ln_10_prec_round_helper(7, Floor, "2.28", "0x2.48#7", Less);
    test_ln_10_prec_round_helper(7, Ceiling, "2.31", "0x2.50#7", Greater);
    test_ln_10_prec_round_helper(7, Down, "2.28", "0x2.48#7", Less);
    test_ln_10_prec_round_helper(7, Up, "2.31", "0x2.50#7", Greater);
    test_ln_10_prec_round_helper(7, Nearest, "2.31", "0x2.50#7", Greater);

    test_ln_10_prec_round_helper(8, Floor, "2.3", "0x2.4c#8", Less);
    test_ln_10_prec_round_helper(8, Ceiling, "2.31", "0x2.50#8", Greater);
    test_ln_10_prec_round_helper(8, Down, "2.3", "0x2.4c#8", Less);
    test_ln_10_prec_round_helper(8, Up, "2.31", "0x2.50#8", Greater);
    test_ln_10_prec_round_helper(8, Nearest, "2.3", "0x2.4c#8", Less);

    test_ln_10_prec_round_helper(9, Floor, "2.297", "0x2.4c#9", Less);
    test_ln_10_prec_round_helper(9, Ceiling, "2.305", "0x2.4e#9", Greater);
    test_ln_10_prec_round_helper(9, Down, "2.297", "0x2.4c#9", Less);
    test_ln_10_prec_round_helper(9, Up, "2.305", "0x2.4e#9", Greater);
    test_ln_10_prec_round_helper(9, Nearest, "2.305", "0x2.4e#9", Greater);

    test_ln_10_prec_round_helper(10, Floor, "2.301", "0x2.4d#10", Less);
    test_ln_10_prec_round_helper(10, Ceiling, "2.305", "0x2.4e#10", Greater);
    test_ln_10_prec_round_helper(10, Down, "2.301", "0x2.4d#10", Less);
    test_ln_10_prec_round_helper(10, Up, "2.305", "0x2.4e#10", Greater);
    test_ln_10_prec_round_helper(10, Nearest, "2.301", "0x2.4d#10", Less);

    test_ln_10_prec_round_helper(
        100,
        Floor,
        "2.302585092994045684017991454684",
        "0x2.4d763776aaa2b05ba95b58ae0#100",
        Less,
    );
    test_ln_10_prec_round_helper(
        100,
        Ceiling,
        "2.302585092994045684017991454687",
        "0x2.4d763776aaa2b05ba95b58ae4#100",
        Greater,
    );
    test_ln_10_prec_round_helper(
        100,
        Down,
        "2.302585092994045684017991454684",
        "0x2.4d763776aaa2b05ba95b58ae0#100",
        Less,
    );
    test_ln_10_prec_round_helper(
        100,
        Up,
        "2.302585092994045684017991454687",
        "0x2.4d763776aaa2b05ba95b58ae4#100",
        Greater,
    );
    test_ln_10_prec_round_helper(
        100,
        Nearest,
        "2.302585092994045684017991454684",
        "0x2.4d763776aaa2b05ba95b58ae0#100",
        Less,
    );
}

#[test]
#[should_panic]
fn ln_10_prec_round_fail_1() {
    Float::ln_10_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn ln_10_prec_round_fail_2() {
    Float::ln_10_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn ln_10_prec_round_fail_3() {
    Float::ln_10_prec_round(1000, Exact);
}

#[test]
fn ln_10_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (ln_10, o) = Float::ln_10_prec(prec);
        assert!(ln_10.is_valid());
        assert_eq!(ln_10.get_prec(), Some(prec));
        assert_eq!(ln_10.get_exponent(), Some(2));
        assert_ne!(o, Equal);
        if o == Less {
            let (ln_10_alt, o_alt) = Float::ln_10_prec_round(prec, Ceiling);
            let mut next_upper = ln_10.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(ln_10_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !ln_10.is_power_of_2() {
            let (ln_10_alt, o_alt) = Float::ln_10_prec_round(prec, Floor);
            let mut next_lower = ln_10.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(ln_10_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (ln_10_alt, o_alt) = Float::ln_10_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&ln_10_alt), ComparableFloatRef(&ln_10));
        assert_eq!(o_alt, o);

        let (ln_10_alt, o_alt) = Float::ln_prec(Float::from(10), prec);
        assert_eq!(ComparableFloatRef(&ln_10_alt), ComparableFloatRef(&ln_10));
        assert_eq!(o_alt, o);

        let (rug_ln_10, rug_o) =
            rug_ln_10_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln_10)),
            ComparableFloatRef(&ln_10)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn ln_10_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (ln_10, o) = Float::ln_10_prec_round(prec, rm);
        assert!(ln_10.is_valid());
        assert_eq!(ln_10.get_prec(), Some(prec));
        assert_eq!(
            ln_10.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                3
            } else {
                2
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (ln_10_alt, o_alt) = Float::ln_10_prec_round(prec, Ceiling);
            let mut next_upper = ln_10.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(ln_10_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !ln_10.is_power_of_2() {
            let (ln_10_alt, o_alt) = Float::ln_10_prec_round(prec, Floor);
            let mut next_lower = ln_10.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(ln_10_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_ln_10, rug_o) = rug_ln_10_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_ln_10)),
                ComparableFloatRef(&ln_10)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::ln_10_prec_round(prec, Exact));
    });

    test_constant(Float::ln_10_prec_round, 10000);
}
