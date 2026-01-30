// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::PiOver2;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_pi_over_2_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::pi_over_2_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_pi_over_2_prec() {
    test_pi_over_2_prec_helper(1, "2.0", "0x2.0#1", Greater);
    test_pi_over_2_prec_helper(2, "1.5", "0x1.8#2", Less);
    test_pi_over_2_prec_helper(3, "1.5", "0x1.8#3", Less);
    test_pi_over_2_prec_helper(4, "1.6", "0x1.a#4", Greater);
    test_pi_over_2_prec_helper(5, "1.56", "0x1.9#5", Less);
    test_pi_over_2_prec_helper(6, "1.56", "0x1.90#6", Less);
    test_pi_over_2_prec_helper(7, "1.58", "0x1.94#7", Greater);
    test_pi_over_2_prec_helper(8, "1.57", "0x1.92#8", Less);
    test_pi_over_2_prec_helper(9, "1.57", "0x1.92#9", Less);
    test_pi_over_2_prec_helper(10, "1.57", "0x1.920#10", Less);
    test_pi_over_2_prec_helper(
        100,
        "1.57079632679489661923132169164",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test_pi_over_2_prec_helper(
        1000,
        "1.570796326794896619231321691639751442098584699687552910487472296153908203143104499314017\
        412671058533991074043256641153323546922304775291115862679704064240558725142051350969260552\
        779822311474477465190982214405487832966723064237824116893391582635600954572824283461730174\
        3052271633241066968036301245706368",
        "0x1.921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c76273644a29410f31c6809bbdf2a3\
        3679a748636605614dbe4be286e9fc26adadaa3848bc90b6aecc4bcfd8de89885d34c6fdad617feb96de80d6fd\
        bdc70d7f6b5133f4b5d3e4822f8963fcc9250cca3d9c8b67b8400f97142c77e0b31b4906c38#1000",
        Less,
    );
    test_pi_over_2_prec_helper(
        10000,
        "1.570796326794896619231321691639751442098584699687552910487472296153908203143104499314017\
        412671058533991074043256641153323546922304775291115862679704064240558725142051350969260552\
        779822311474477465190982214405487832966723064237824116893391582635600954572824283461730174\
        305227163324106696803630124570636862293503303157794087440760460481414627045857682183946295\
        180005665265274410233260692073475970755804716528635182879795976546093058690966305896552559\
        274037231189981374783675942876362445613969091505974564916836681220328321543010697473197612\
        368595351089930471851385269608588146588376192337409233834702566000284063572631780413892885\
        671378894804586818589360734220450612476715073274792685525396139844629461771009978056064510\
        980432017209079906814887385654980259353605674999999186489024975529865866408048159297512229\
        727673454151321261154126672342517630965594085505001568919376443293766604190710308588834573\
        651799126745214377734365579781431941176893796875978890928890266085613403306500963938305597\
        954608210099469047628600532742931639432968076690913984115150976017650926484497886811299706\
        945624860887641739565757787428621227075347975414766558430863927944537549190877318732469659\
        627530200463850835569504924412006429180801781853830052355090971477798099473383918724724127\
        689887363423552023767323104023342129534745646656838514494576052376081028483012029019075096\
        755626691215017793820123748236631957099636302134961398391177390818004670860820609962293157\
        515143091487277853374919252747294293463497845463605398754651477660582672493601377980118240\
        332749559940917398876783184903713271263931275909208787336445488886396900040823530008072624\
        596086608607386175070720986784274080680578676276066737870924734219261661953697071667273881\
        208431259491784742781049609611092136275127128443835895247300826733402494313616395893042892\
        191913983988340727050476941893180475340032112562602558696492448042064244313472802120982642\
        511105330593153372139311019597472523561856893480478182185958643733882328786981206945432916\
        322997906695239013795049732882039475634734199176297854912911310261244703863359739134241300\
        738495451320068197218727652534101748126225874699825715714904595329625468610848230757854929\
        193705298942979886487749465080876964234069134341934471387077995927962622976979715524986262\
        340422993636822347924326918368111313049562304025621942195225622068274881390398857845717998\
        850064808044720847434277924203176711036112914244324079228014253008421369726133733839447626\
        069261274977333363911993228298058177443115288728249017796817284087162056257538034739725548\
        298047012614439855446572834568433614374470280050751654308964340460437380458912469294504857\
        454837992630682774890946564892410841499474361329402428782007135238777566189820725761873117\
        182271429222397632933910525570677367869761556713583051067984768115721476242468593555072882\
        701795139967201871003655289269531099193723904239244841660722856934375971753215109226595524\
        240502685307340337459639095598969976030709831714377220321872561859096089999195507959780907\
        337571345619874470453593247115980783972604",
        "0x1.921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c76273644a29410f31c6809bbdf2a3\
        3679a748636605614dbe4be286e9fc26adadaa3848bc90b6aecc4bcfd8de89885d34c6fdad617feb96de80d6fd\
        bdc70d7f6b5133f4b5d3e4822f8963fcc9250cca3d9c8b67b8400f97142c77e0b31b4906c38aba734d22c7f51f\
        a499ebf06caba47b9475b2c38c5e6ac410aa5773daa520ee12d2cdace186a9c95793009e2e8d811943042f8652\
        0bc8c5c6d9c77c73cee58301d0c07364f0745d80f451f6b8abbe0de98a593bc5797ed2ab02e30732a92f9d52ad\
        5ca2ba44c3131f40a202ae51cb51555885b5a662e1a08a0f46750aa4357be3974c9d9f70a08b1b7de1515d4e2a\
        eba0c18fb672e1f0b4dc3c98f57eb5d19b61267ae3d1929c0944ac33b9dc7a44c35a5dcd7e25ff40db31410c9b\
        0ec04e67d90d4c8a43e56302ef6401977c22eaef4c2bad8ee13118175b28dc411c49f40e9cb566287b6b7f9c1f\
        a211c9705a2415242100234e478254f0fcdaf10e334217b74b64d33864e30d5e9c4783528d0696c2a17b44b07d\
        39455a899d1b77785b609bd1df25d1df8283f7d954c50f8b28e9cd780bb33652c9f412187444677430ca2b7cfd\
        a3ec252e19dc5af5f7037baec42e09039a00d224fab60b5532769d5311b1fbb830dff6fb9214d811e9be86b926\
        80509246d87f569a4f8e04d83a9b964c04c8dbd92ea3cec7b746f7bf1ff280d5b3ca61dcbb6705e8260035d60d\
        4a7db204fb0622f2e4f610cb51231b47db7d79f3629da899cd9759da97637b6fe288fcd984a966640a2a257af5\
        e84df71e8026f19a57eb30794038c9725d9e065d42ba2e43a07e905af9cdce9fdedaabce05e8d3019056b50806\
        32016393cb3cf92ff7d8fd1e64752f4fc6d99117c1e3a8b6ffeb0b58a97a80f645682a955991edafd7e91c3b02\
        998bda41f006fc14f2e2bdde537c6500d43ab176f8bb4edeaa1547b143f7fe1d63399634627aab9b4ad93d85de\
        52c6470ffd1aedc7808d0087d1ecc7e90c1dc257e5ab616e8e9adcd29f23cdb7c22b2e94724de25fdcbc870eef\
        96d5265bf19b17d89a0e77263747790656d1b3ba600e83f4f7f15f88fda4aeded26d74848cc7556c738b5c9ead\
        0684768e857e392f0471e2d97c73aca5bc7fb717df90915b244445c094806f80e27d6af503447e18e68e7f8c8d\
        9d460d69797910c5f070bbbf53a96ff45810fd0f2d06607dab7ba740c5679eb6744f14cda5427f07e89f05bbe6\
        21dc0e956d46c8b2fd133404abb82c9e6398a108d0a3bf3569032bbdafd4363aa217afdce9ae7f5e6d7863d9f4\
        4d06b208de9d70f3f24801287169038d9af1134005dabdc705792321b4df804dc8f2ab1c88eacefd3553c60a1c\
        4ecad29bf903eadd10172dce2c19301bb314ae7d488e40cb42739a520d9a396e53d8a54a50da880294d29948ae\
        b07ab9fde4de37215b0523b40f33a00045d37daab8df48ff94b76359506ec8adb31b290f3dcfcdb7f9a029762c\
        2ab3229d816aed4cfc7d0845d23ccb74283b525bd3874dad994a26dba8497620c9311d6b7535824d3efbece773\
        05c47f6d93376554638b4cd0bffab3229366158cf708c9b0152ba84a614d02c89a0720c1d1f1faa4c4d2da14eb\
        2b5c7f26b4cfb9feb50e94e03f7f4187aa6969c737812aee0a66e90434238759331c174e3010f662f04b4359f9\
        f5d77e49e4b8c0a35b53850b43f9ac2295071435bce2982d528039b9f03c20e3fef572e473e#10000",
        Less,
    );

    let pi_over_2_f32 = Float::pi_over_2_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(pi_over_2_f32.to_string(), "1.5707964");
    assert_eq!(to_hex_string(&pi_over_2_f32), "0x1.921fb6#24");
    assert_eq!(pi_over_2_f32, f32::PI_OVER_2);

    let pi_over_2_f64 = Float::pi_over_2_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(pi_over_2_f64.to_string(), "1.5707963267948966");
    assert_eq!(to_hex_string(&pi_over_2_f64), "0x1.921fb54442d18#53");
    assert_eq!(pi_over_2_f64, f64::PI_OVER_2);
}

#[test]
#[should_panic]
fn pi_over_2_prec_fail_1() {
    Float::pi_over_2_prec(0);
}

fn test_pi_over_2_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::pi_over_2_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_pi_over_2_prec_round() {
    test_pi_over_2_prec_round_helper(1, Floor, "1.0", "0x1.0#1", Less);
    test_pi_over_2_prec_round_helper(1, Ceiling, "2.0", "0x2.0#1", Greater);
    test_pi_over_2_prec_round_helper(1, Down, "1.0", "0x1.0#1", Less);
    test_pi_over_2_prec_round_helper(1, Up, "2.0", "0x2.0#1", Greater);
    test_pi_over_2_prec_round_helper(1, Nearest, "2.0", "0x2.0#1", Greater);

    test_pi_over_2_prec_round_helper(2, Floor, "1.5", "0x1.8#2", Less);
    test_pi_over_2_prec_round_helper(2, Ceiling, "2.0", "0x2.0#2", Greater);
    test_pi_over_2_prec_round_helper(2, Down, "1.5", "0x1.8#2", Less);
    test_pi_over_2_prec_round_helper(2, Up, "2.0", "0x2.0#2", Greater);
    test_pi_over_2_prec_round_helper(2, Nearest, "1.5", "0x1.8#2", Less);

    test_pi_over_2_prec_round_helper(3, Floor, "1.5", "0x1.8#3", Less);
    test_pi_over_2_prec_round_helper(3, Ceiling, "1.8", "0x1.c#3", Greater);
    test_pi_over_2_prec_round_helper(3, Down, "1.5", "0x1.8#3", Less);
    test_pi_over_2_prec_round_helper(3, Up, "1.8", "0x1.c#3", Greater);
    test_pi_over_2_prec_round_helper(3, Nearest, "1.5", "0x1.8#3", Less);

    test_pi_over_2_prec_round_helper(4, Floor, "1.5", "0x1.8#4", Less);
    test_pi_over_2_prec_round_helper(4, Ceiling, "1.6", "0x1.a#4", Greater);
    test_pi_over_2_prec_round_helper(4, Down, "1.5", "0x1.8#4", Less);
    test_pi_over_2_prec_round_helper(4, Up, "1.6", "0x1.a#4", Greater);
    test_pi_over_2_prec_round_helper(4, Nearest, "1.6", "0x1.a#4", Greater);

    test_pi_over_2_prec_round_helper(5, Floor, "1.56", "0x1.9#5", Less);
    test_pi_over_2_prec_round_helper(5, Ceiling, "1.62", "0x1.a#5", Greater);
    test_pi_over_2_prec_round_helper(5, Down, "1.56", "0x1.9#5", Less);
    test_pi_over_2_prec_round_helper(5, Up, "1.62", "0x1.a#5", Greater);
    test_pi_over_2_prec_round_helper(5, Nearest, "1.56", "0x1.9#5", Less);

    test_pi_over_2_prec_round_helper(6, Floor, "1.56", "0x1.90#6", Less);
    test_pi_over_2_prec_round_helper(6, Ceiling, "1.59", "0x1.98#6", Greater);
    test_pi_over_2_prec_round_helper(6, Down, "1.56", "0x1.90#6", Less);
    test_pi_over_2_prec_round_helper(6, Up, "1.59", "0x1.98#6", Greater);
    test_pi_over_2_prec_round_helper(6, Nearest, "1.56", "0x1.90#6", Less);

    test_pi_over_2_prec_round_helper(7, Floor, "1.56", "0x1.90#7", Less);
    test_pi_over_2_prec_round_helper(7, Ceiling, "1.58", "0x1.94#7", Greater);
    test_pi_over_2_prec_round_helper(7, Down, "1.56", "0x1.90#7", Less);
    test_pi_over_2_prec_round_helper(7, Up, "1.58", "0x1.94#7", Greater);
    test_pi_over_2_prec_round_helper(7, Nearest, "1.58", "0x1.94#7", Greater);

    test_pi_over_2_prec_round_helper(8, Floor, "1.57", "0x1.92#8", Less);
    test_pi_over_2_prec_round_helper(8, Ceiling, "1.58", "0x1.94#8", Greater);
    test_pi_over_2_prec_round_helper(8, Down, "1.57", "0x1.92#8", Less);
    test_pi_over_2_prec_round_helper(8, Up, "1.58", "0x1.94#8", Greater);
    test_pi_over_2_prec_round_helper(8, Nearest, "1.57", "0x1.92#8", Less);

    test_pi_over_2_prec_round_helper(9, Floor, "1.57", "0x1.92#9", Less);
    test_pi_over_2_prec_round_helper(9, Ceiling, "1.574", "0x1.93#9", Greater);
    test_pi_over_2_prec_round_helper(9, Down, "1.57", "0x1.92#9", Less);
    test_pi_over_2_prec_round_helper(9, Up, "1.574", "0x1.93#9", Greater);
    test_pi_over_2_prec_round_helper(9, Nearest, "1.57", "0x1.92#9", Less);

    test_pi_over_2_prec_round_helper(10, Floor, "1.57", "0x1.920#10", Less);
    test_pi_over_2_prec_round_helper(10, Ceiling, "1.572", "0x1.928#10", Greater);
    test_pi_over_2_prec_round_helper(10, Down, "1.57", "0x1.920#10", Less);
    test_pi_over_2_prec_round_helper(10, Up, "1.572", "0x1.928#10", Greater);
    test_pi_over_2_prec_round_helper(10, Nearest, "1.57", "0x1.920#10", Less);

    test_pi_over_2_prec_round_helper(
        100,
        Floor,
        "1.57079632679489661923132169164",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test_pi_over_2_prec_round_helper(
        100,
        Ceiling,
        "1.570796326794896619231321691641",
        "0x1.921fb54442d18469898cc5172#100",
        Greater,
    );
    test_pi_over_2_prec_round_helper(
        100,
        Down,
        "1.57079632679489661923132169164",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test_pi_over_2_prec_round_helper(
        100,
        Up,
        "1.570796326794896619231321691641",
        "0x1.921fb54442d18469898cc5172#100",
        Greater,
    );
    test_pi_over_2_prec_round_helper(
        100,
        Nearest,
        "1.57079632679489661923132169164",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
}

#[test]
#[should_panic]
fn pi_over_2_prec_round_fail_1() {
    Float::pi_over_2_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn pi_over_2_prec_round_fail_2() {
    Float::pi_over_2_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn pi_over_2_prec_round_fail_3() {
    Float::pi_over_2_prec_round(1000, Exact);
}

#[test]
fn pi_over_2_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (pi_over_2, o) = Float::pi_over_2_prec(prec);
        assert!(pi_over_2.is_valid());
        assert_eq!(pi_over_2.get_prec(), Some(prec));
        assert_eq!(
            pi_over_2.get_exponent(),
            Some(if prec == 1 { 2 } else { 1 })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_over_2_alt, o_alt) = Float::pi_over_2_prec_round(prec, Ceiling);
            let mut next_upper = pi_over_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_over_2_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi_over_2.is_power_of_2() {
            let (pi_over_2_alt, o_alt) = Float::pi_over_2_prec_round(prec, Floor);
            let mut next_lower = pi_over_2.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_over_2_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (pi_over_2_alt, o_alt) = Float::pi_over_2_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&pi_over_2_alt),
            ComparableFloatRef(&pi_over_2)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn pi_over_2_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (pi_over_2, o) = Float::pi_over_2_prec_round(prec, rm);
        assert!(pi_over_2.is_valid());
        assert_eq!(pi_over_2.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 2,
            _ => 1,
        };
        assert_eq!(pi_over_2.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_over_2_alt, o_alt) = Float::pi_over_2_prec_round(prec, Ceiling);
            let mut next_upper = pi_over_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_over_2_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi_over_2.is_power_of_2() {
            let (pi_over_2_alt, o_alt) = Float::pi_over_2_prec_round(prec, Floor);
            let mut next_lower = pi_over_2.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_over_2_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::pi_over_2_prec_round(prec, Exact));
    });

    test_constant(Float::pi_over_2_prec_round, 10000);
}
