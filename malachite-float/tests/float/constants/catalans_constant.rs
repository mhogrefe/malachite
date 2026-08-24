// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::CatalansConstant;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::float::constants::catalans_constant::*;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_catalans_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::catalans_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) =
        rug_catalans_constant_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_catalans_constant_prec() {
    // - precision 1 makes the Ziv loop retry twice (G begins with a run of three 1 bits)
    test_catalans_constant_prec_helper(1, "1.0", "0x1.0#1", Greater);
    test_catalans_constant_prec_helper(2, "1.0", "0x1.0#2", Greater);
    test_catalans_constant_prec_helper(3, "0.88", "0x0.e#3", Less);
    test_catalans_constant_prec_helper(4, "0.938", "0x0.f#4", Greater);
    test_catalans_constant_prec_helper(5, "0.906", "0x0.e8#5", Less);
    test_catalans_constant_prec_helper(6, "0.922", "0x0.ec#6", Greater);
    test_catalans_constant_prec_helper(7, "0.9141", "0x0.ea#7", Less);
    // - precision 8 with Nearest makes the Ziv loop retry (a run of five 1 bits at position 9)
    test_catalans_constant_prec_helper(8, "0.9141", "0x0.ea#8", Less);
    test_catalans_constant_prec_helper(9, "0.9160", "0x0.ea8#9", Greater);
    test_catalans_constant_prec_helper(10, "0.91602", "0x0.ea8#10", Greater);
    test_catalans_constant_prec_helper(
        100,
        "0.91596559417721901505460351493252",
        "0x0.ea7cb89f409ae845215822e38#100",
        Greater,
    );
    test_catalans_constant_prec_helper(
        1000,
        "0.915965594177219015054603514932384110774149374281672134266498119621763019776254769479356\
        512926115106248574422619196199579035898803325859059431594737481158406995332028773319460519\
        038727478164087865909024706484152163000228727640942388259957741508816397470252482011560707\
        644883807873370489900864775113226027",
        "0x0.ea7cb89f409ae845215822e37d32d0c63ec43e1381c2ff8094a263e5a3ccd76f94dc058a46eec5858f924\
        d663f739c42ec95f8da12f16bfc58bb20c7f2063a6c6f6de500cb94f358cfdec842b851f77bda255282ce0920a\
        17ff4ff46738f8e5a7c6e6898f5a0182d43759a75680d01c68c37800513b3a8dd76669af91a#1000",
        Greater,
    );
    test_catalans_constant_prec_helper(
        10000,
        "0.915965594177219015054603514932384110774149374281672134266498119621763019776254769479356\
        512926115106248574422619196199579035898803325859059431594737481158406995332028773319460519\
        038727478164087865909024706484152163000228727640942388259957741508816397470252482011560707\
        644883807873370489900864775113225997134340748540755323076856533576809583526021938232395080\
        072068035576104823573394231914982983618997706903640418086217941101917532743149978233976105\
        512247795303248753718786658280823605702255941948180975350971131571261580424272363643985001\
        738287597797653068370092980873887495610893659771940968726844441668046216243398648389162804\
        482815062730227420738843117221827219047225587053190868573542349853949830991911596738846450\
        861515249962423704374517773723517754407085384644013217483929999475724461997549619758706400\
        747487070149093767887304586997986064487497464387206238513712392736304998503539223928787979\
        063364403235478453585192777778727090608303199430133231671247615870979245547911909212620185\
        480396393424349565375967394943547300143851807050512507488613285641293449595022987229831628\
        948164616225739894762318195420066071881427594975599589836373037675338533813545031276817240\
        118140721534688316835681686393272936775866739258395406180333878306870649014334860172981069\
        921799565309581871579115539560366890369904939667538437758104931899553855162621962533168040\
        162737521301209406045387950760538271231974679008823691786155733891244172238339381481207759\
        942984917243976685756327180688082799829793788494327249346576074905438748195268130744370462\
        946358928102765317050765479744948399489594770927885911958487241278660840885545978238124922\
        605056100945844866989585768716111717866623368474099493855413210937552818155258815915022282\
        444544417186099465881517664960782236789705192697113125713754543701243296730572468450158193\
        130160877662156509575546796667866170823476825581335186819377456500145652617040960746889539\
        302347919806000842455621751084234717363878793695778784409337922198945753409616474245546224\
        787880029229148036907115270795545505414782688498185246005814466517868142315411487855409966\
        516738539727614697016904391511490089333079184574657620996775481231382015436010988527216297\
        701087615747817356416369857035534067264935196316955476721150777231590044833826051611638343\
        086513979722516174138538129324801194636251880084039819455390551821042460629218521756024654\
        860192976723974051103952645692429786421242403751892678729602717733787383799783266762086119\
        520679121512638211925232940406920599438642746932153388566711733082714240833265920326075316\
        592804231023099735840039594034263222768807011868196176780905631581597845376375783563735902\
        771648831310288769379505350732080180758102238230803176250432942472226839122971295535135510\
        431476188665547436769218412018877161799228562056352205470320069180868806612117420406099241\
        234876051540682022625595048124858941187358346822904230836155547694777708319408748124916748\
        929006593696164166234368370754396383894514401195564873813429212298200130210799619224249244\
        930519992358581580826035249799850591866972199",
        "0x0.ea7cb89f409ae845215822e37d32d0c63ec43e1381c2ff8094a263e5a3ccd76f94dc058a46eec5858f924\
        d663f739c42ec95f8da12f16bfc58bb20c7f2063a6c6f6de500cb94f358cfdec842b851f77bda255282ce0920a\
        17ff4ff46738f8e5a7c6e6898f5a0182d43759a75680d01c68c37800513b3a8dd76669af919aece8763ba76a87\
        7a39b6446d11a2a0cee24dd912b35d8a5d650d1e2d9689c760e1f3d35f3fe5c5f62f45b333b991ff2c977b1460\
        ded3847257a8ede354e1d1ad9cee17f63350ea8d3203e2bf626b2057fdc67f7921cdc86e6854484aeeabaa5163\
        dfde7ea092be912e1db310315a6293c127a01839550e46a9b11a7a8fb50b34791b853813268ef233dcb2c3fc3e\
        ae220797ebecb8f04d62cb5cb902f8278626f5150727c8ee590768006cc74f3db2950db6131ac4ce8f48c48767\
        d028e8d27fb550f60a488cd45562790747e070d0677188a23f46069a94084ab51e33c1a0d3c4e114472c1a23c2\
        e5ba7a1c6420f3f3c353a9e8c033adeda0f421a483b0068230725c1f3bc53ff2be9ceae394f13efe440975c193\
        3fb3ab734a47acde143c3adeb0d53109dced16da14c9794567da609e2b56427262940f0da3ac43a95391333dda\
        bb23fc8233c0cb8f99a69e33fedc6cbf5cf4a5f255dcc77c0709ab65fe92d90abe8f72e1556e4c8b64d4ee9677\
        561302d31453cc46294d65dfa36421e9dc6762460a932a68371394202639bf4452ea4897e7adfc74d41b59a205\
        5fb407f0522f3794be726a1bc82bfd41b53d37514aca27d09976c432fdbaa6204ac3cb7e94d3001a4f47996046\
        45ca5d010a37892fef162f9c498c34d22d1bcc39b35ec3c0145baaa961b94e64cb636549d70e43934e1bd3cddc\
        3edeba4085121bb046576ea8b94f000a75773a69001065b7d6b9adfa4e6e9f330459e3ddb9586d88bffc66e412\
        f47c700c8efe611805ed77bd52e22c0f8afe68e2fb0c2b00434967e04427f8c2d79bfe8241acb272d940aecebc\
        6e4f8cfbb74cab912471f38e326a503949f2c3d230ef06cdae66c6b6f4c4b039419916b81151eb4c7e117800ae\
        5c4d3e41a0518eadd538dc31daaa9d0bc722f97abd42a63dc43c0764f5e6f71feb335d8e26321bf35a468bb8f6\
        bcc4431ebddf5983d1822bbeb512c4f591eecb509443ce0cee8895829453857716139825749ae33bb68a1921e6\
        2287cc02a5a45cf4019c2aa7e3818db693aecd5249c6f75908d1b2bffa2f8d1dc715a2c885c78634aef2a53e4e\
        087c64553231e373406561bf320b0d24659aeee27360e47e70841ba0ef2f49dc50eaeee615f58fe38842535dee\
        0e30baa7c72dbdee91997f8544e205f7da3dbd6f175fd845b21657702fafe0de29e3d4362902dadf63b992b192\
        a4a18b22c47096ec5b7fe87de7ba920aaf98a827255e22e2c1b2e73a79d6830bd138b4495c52b05f8005017fe0\
        189282f84117edf29ca70a09d97bd3390cc6184f19d1eb9e2d06aaf50cb898e5a6bf445d37bd4d18b924fd981a\
        d5a2ad2d4056ef9683ca5ecb77b92853736688daa21eac23c4c62b353d711319995dcf380c874a1473577ed9f0\
        811c5048d874eda28a191ff02afb9257d47331a2b6a6510557a9d0fe78e28618bca1bd663cc2d05f18abd24af8\
        fd0e3264dcd535c9f5bf5495e2b1bf32e7eabe17f626ecc811f03a838370e06550d06ce5c2768deca02f246b87\
        6b0d5c97f0f8e703e3056c39dbadd6aec7791b960f616ca11d4a8b7e0069d47c368b053a3d9#10000",
        Less,
    );

    let g_f32 = Float::catalans_constant_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(g_f32.to_string(), "0.915965617");
    assert_eq!(to_hex_string(&g_f32), "0x0.ea7cb9#24");
    assert_eq!(g_f32, f32::CATALANS_CONSTANT);

    let g_f64 = Float::catalans_constant_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(g_f64.to_string(), "0.91596559417721901");
    assert_eq!(to_hex_string(&g_f64), "0x0.ea7cb89f409ae8#53");
    assert_eq!(g_f64, f64::CATALANS_CONSTANT);
}

#[test]
#[should_panic]
fn catalans_constant_prec_fail_1() {
    Float::catalans_constant_prec(0);
}

fn test_catalans_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::catalans_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_catalans_constant_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_catalans_constant_prec_round() {
    test_catalans_constant_prec_round_helper(1, Floor, "0.50", "0x0.8#1", Less);
    test_catalans_constant_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_catalans_constant_prec_round_helper(1, Down, "0.50", "0x0.8#1", Less);
    test_catalans_constant_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_catalans_constant_prec_round_helper(1, Nearest, "1.0", "0x1.0#1", Greater);
    test_catalans_constant_prec_round_helper(2, Floor, "0.75", "0x0.c#2", Less);
    test_catalans_constant_prec_round_helper(2, Ceiling, "1.0", "0x1.0#2", Greater);
    test_catalans_constant_prec_round_helper(2, Down, "0.75", "0x0.c#2", Less);
    test_catalans_constant_prec_round_helper(2, Up, "1.0", "0x1.0#2", Greater);
    test_catalans_constant_prec_round_helper(2, Nearest, "1.0", "0x1.0#2", Greater);
    test_catalans_constant_prec_round_helper(3, Floor, "0.88", "0x0.e#3", Less);
    test_catalans_constant_prec_round_helper(3, Ceiling, "1.0", "0x1.0#3", Greater);
    test_catalans_constant_prec_round_helper(3, Down, "0.88", "0x0.e#3", Less);
    test_catalans_constant_prec_round_helper(3, Up, "1.0", "0x1.0#3", Greater);
    test_catalans_constant_prec_round_helper(3, Nearest, "0.88", "0x0.e#3", Less);
    test_catalans_constant_prec_round_helper(4, Floor, "0.875", "0x0.e#4", Less);
    test_catalans_constant_prec_round_helper(4, Ceiling, "0.938", "0x0.f#4", Greater);
    test_catalans_constant_prec_round_helper(4, Down, "0.875", "0x0.e#4", Less);
    test_catalans_constant_prec_round_helper(4, Up, "0.938", "0x0.f#4", Greater);
    test_catalans_constant_prec_round_helper(4, Nearest, "0.938", "0x0.f#4", Greater);
    test_catalans_constant_prec_round_helper(5, Floor, "0.906", "0x0.e8#5", Less);
    test_catalans_constant_prec_round_helper(5, Ceiling, "0.938", "0x0.f0#5", Greater);
    test_catalans_constant_prec_round_helper(5, Down, "0.906", "0x0.e8#5", Less);
    test_catalans_constant_prec_round_helper(5, Up, "0.938", "0x0.f0#5", Greater);
    test_catalans_constant_prec_round_helper(5, Nearest, "0.906", "0x0.e8#5", Less);
    test_catalans_constant_prec_round_helper(6, Floor, "0.906", "0x0.e8#6", Less);
    test_catalans_constant_prec_round_helper(6, Ceiling, "0.922", "0x0.ec#6", Greater);
    test_catalans_constant_prec_round_helper(6, Down, "0.906", "0x0.e8#6", Less);
    test_catalans_constant_prec_round_helper(6, Up, "0.922", "0x0.ec#6", Greater);
    test_catalans_constant_prec_round_helper(6, Nearest, "0.922", "0x0.ec#6", Greater);
    test_catalans_constant_prec_round_helper(7, Floor, "0.9141", "0x0.ea#7", Less);
    test_catalans_constant_prec_round_helper(7, Ceiling, "0.9219", "0x0.ec#7", Greater);
    test_catalans_constant_prec_round_helper(7, Down, "0.9141", "0x0.ea#7", Less);
    test_catalans_constant_prec_round_helper(7, Up, "0.9219", "0x0.ec#7", Greater);
    test_catalans_constant_prec_round_helper(7, Nearest, "0.9141", "0x0.ea#7", Less);
    test_catalans_constant_prec_round_helper(8, Floor, "0.9141", "0x0.ea#8", Less);
    test_catalans_constant_prec_round_helper(8, Ceiling, "0.9180", "0x0.eb#8", Greater);
    test_catalans_constant_prec_round_helper(8, Down, "0.9141", "0x0.ea#8", Less);
    test_catalans_constant_prec_round_helper(8, Up, "0.9180", "0x0.eb#8", Greater);
    test_catalans_constant_prec_round_helper(8, Nearest, "0.9141", "0x0.ea#8", Less);
    test_catalans_constant_prec_round_helper(9, Floor, "0.9141", "0x0.ea0#9", Less);
    test_catalans_constant_prec_round_helper(9, Ceiling, "0.9160", "0x0.ea8#9", Greater);
    test_catalans_constant_prec_round_helper(9, Down, "0.9141", "0x0.ea0#9", Less);
    test_catalans_constant_prec_round_helper(9, Up, "0.9160", "0x0.ea8#9", Greater);
    test_catalans_constant_prec_round_helper(9, Nearest, "0.9160", "0x0.ea8#9", Greater);
    test_catalans_constant_prec_round_helper(10, Floor, "0.91504", "0x0.ea4#10", Less);
    test_catalans_constant_prec_round_helper(10, Ceiling, "0.91602", "0x0.ea8#10", Greater);
    test_catalans_constant_prec_round_helper(10, Down, "0.91504", "0x0.ea4#10", Less);
    test_catalans_constant_prec_round_helper(10, Up, "0.91602", "0x0.ea8#10", Greater);
    test_catalans_constant_prec_round_helper(10, Nearest, "0.91602", "0x0.ea8#10", Greater);
    test_catalans_constant_prec_round_helper(
        100,
        Floor,
        "0.91596559417721901505460351493173",
        "0x0.ea7cb89f409ae845215822e37#100",
        Less,
    );
    test_catalans_constant_prec_round_helper(
        100,
        Ceiling,
        "0.91596559417721901505460351493252",
        "0x0.ea7cb89f409ae845215822e38#100",
        Greater,
    );
    test_catalans_constant_prec_round_helper(
        100,
        Down,
        "0.91596559417721901505460351493173",
        "0x0.ea7cb89f409ae845215822e37#100",
        Less,
    );
    test_catalans_constant_prec_round_helper(
        100,
        Up,
        "0.91596559417721901505460351493252",
        "0x0.ea7cb89f409ae845215822e38#100",
        Greater,
    );
    test_catalans_constant_prec_round_helper(
        100,
        Nearest,
        "0.91596559417721901505460351493252",
        "0x0.ea7cb89f409ae845215822e38#100",
        Greater,
    );
}

// Precisions just below long runs of identical bits in G's binary expansion (from the worst-case
// table in MPFR's const_catalan.c) make the Ziv loop retry at higher working precisions. The
// results are too long to compare as strings, so they are checked against MPFR via rug.
#[test]
fn test_catalans_constant_ziv_retry() {
    for prec in [175, 704, 912, 913] {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (x, o) = Float::catalans_constant_prec_round(prec, rm);
            assert!(x.is_valid());
            let (rug_x, rug_o) = rug_catalans_constant_prec_round(
                prec,
                rug_round_try_from_rounding_mode(rm).unwrap(),
            );
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
fn catalans_constant_prec_round_fail_1() {
    Float::catalans_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn catalans_constant_prec_round_fail_2() {
    Float::catalans_constant_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn catalans_constant_prec_round_fail_3() {
    Float::catalans_constant_prec_round(1000, Exact);
}

#[test]
fn catalans_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (g, o) = Float::catalans_constant_prec(prec);
        assert!(g.is_valid());
        assert_eq!(g.get_prec(), Some(prec));
        assert_eq!(g.get_exponent(), Some(if prec <= 2 { 1 } else { 0 }));
        assert_ne!(o, Equal);
        if o == Less {
            let (g_alt, o_alt) = Float::catalans_constant_prec_round(prec, Ceiling);
            let mut next_upper = g.clone();
            next_upper.increment();
            assert_eq!(ComparableFloat(g_alt), ComparableFloat(next_upper));
            assert_eq!(o_alt, Greater);
        } else {
            let (g_alt, o_alt) = Float::catalans_constant_prec_round(prec, Floor);
            let mut next_lower = g.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(g_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (g_alt, o_alt) = Float::catalans_constant_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&g_alt), ComparableFloatRef(&g));
        assert_eq!(o_alt, o);

        let (rug_g, rug_o) = rug_catalans_constant_prec_round(
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
fn catalans_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (g, o) = Float::catalans_constant_prec_round(prec, rm);
        assert!(g.is_valid());
        assert_eq!(g.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1..=3, Ceiling | Up) | (1 | 2, Nearest) => 1,
            _ => 0,
        };
        assert_eq!(g.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (g_alt, o_alt) = Float::catalans_constant_prec_round(prec, Ceiling);
            let mut next_upper = g.clone();
            next_upper.increment();
            assert_eq!(ComparableFloat(g_alt), ComparableFloat(next_upper));
            assert_eq!(o_alt, Greater);
        } else {
            let (g_alt, o_alt) = Float::catalans_constant_prec_round(prec, Floor);
            let mut next_lower = g.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(g_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_g, rug_o) = rug_catalans_constant_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_g)),
                ComparableFloatRef(&g)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::catalans_constant_prec_round(prec, Exact));
    });

    test_constant(Float::catalans_constant_prec_round, 10000);
}
