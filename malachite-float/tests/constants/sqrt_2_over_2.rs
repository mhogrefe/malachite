// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Sqrt2Over2;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::sqrt_2_over_2::rug_sqrt_2_over_2_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_sqrt_2_over_2_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::sqrt_2_over_2_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) =
        rug_sqrt_2_over_2_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_sqrt_2_over_2_prec() {
    test_sqrt_2_over_2_prec_helper(1, "0.5", "0x0.8#1", Less);
    test_sqrt_2_over_2_prec_helper(2, "0.8", "0x0.c#2", Greater);
    test_sqrt_2_over_2_prec_helper(3, "0.8", "0x0.c#3", Greater);
    test_sqrt_2_over_2_prec_helper(4, "0.7", "0x0.b#4", Less);
    test_sqrt_2_over_2_prec_helper(5, "0.72", "0x0.b8#5", Greater);
    test_sqrt_2_over_2_prec_helper(6, "0.7", "0x0.b4#6", Less);
    test_sqrt_2_over_2_prec_helper(7, "0.71", "0x0.b6#7", Greater);
    test_sqrt_2_over_2_prec_helper(8, "0.707", "0x0.b5#8", Less);
    test_sqrt_2_over_2_prec_helper(9, "0.707", "0x0.b50#9", Less);
    test_sqrt_2_over_2_prec_helper(10, "0.707", "0x0.b50#10", Less);
    test_sqrt_2_over_2_prec_helper(
        100,
        "0.7071067811865475244008443621046",
        "0x0.b504f333f9de6484597d89b37#100",
        Less,
    );
    test_sqrt_2_over_2_prec_helper(
        1000,
        "0.707106781186547524400844362104849039284835937688474036588339868995366239231053519425193\
        767163820786367506923115456148512462418027925368606322060748549967915706611332963752796377\
        899975250576391030285735054779985802985137267298431007364258709320444599304776164615242154\
        3571607254198813018139976257039948",
        "0x0.b504f333f9de6484597d89b3754abe9f1d6f60ba893ba84ced17ac85833399154afc83043ab8a2c3a8b1f\
        e6fdc83db390f74a85e439c7b4a780487363dfa2768d2202e8742af1f4e53059c6011bc337bcab1bc911688458\
        a460abc722f7c4e33c6d5a8a38bb7e9dccb2a634331f3c84df52f120f836e582eeaa4a08990#1000",
        Less,
    );
    test_sqrt_2_over_2_prec_helper(
        10000,
        "0.707106781186547524400844362104849039284835937688474036588339868995366239231053519425193\
        767163820786367506923115456148512462418027925368606322060748549967915706611332963752796377\
        899975250576391030285735054779985802985137267298431007364258709320444599304776164615242154\
        357160725419881301813997625703994843626698273165904414820310307629176197527372875143879980\
        864917787610168765928505677187301704249423580193449985349502407515272013895158227123911534\
        246468459310790289231555798334356506507809284493618617644254632430624748857710916710214284\
        303007341236038571792743707782853483882686011324272350792940081037923746132861300104279223\
        326072919944697218546329590015569412323407854131505029742935200159324017109744863914532052\
        253631844065686992762805866102012254561385011347056378681364024786905448375200918493418422\
        536289968236453038149847069023782741186449859016340123721031463456242952609050222992107529\
        556012472067086426573905290180168553865459143465735508555584195829086344470987935829107606\
        411475924423604484731693144578144138297631757027113382661984730875564580120435775506757522\
        769064378002631573400856370132698473512015025874765943146281569259408173900078468458844092\
        618934202614391881469460715032793478434298229757775082236225491844801844366155719470778832\
        552044195714616905660302621681474265852495788587811427487071949959401088121548260328210591\
        365836312876979735862796731861931613074137131110433557791979996326058812634945877049407967\
        432004172854259073611590710203521325452826616669921822893289839825963364619993768330860799\
        128943013168180891374799710970188887684071310886939959727569861563703344916499494769336441\
        142818934887483125998329176288809946966142267236784739748147608444574274626945237791441726\
        304826204827144469726932331287246377819098220515848991653092600968969247002857816686027403\
        427028793399983506068611973791071315329256610870441619147364380869682373391871598000079609\
        440367392880862610593374521248868346460365554818486080446685433057836729266741664762733792\
        582235537892430123180041722455740929382777714322756165710996315566625898530421827985217642\
        820504395925038018050457973283533844180278587003837845254806835970066246780262009299955253\
        105408179886321569030273350514678498552121255289087476552862796749222556346139017245675331\
        878437388014158141480276621121347876726451441938422321458664138544415904351266992616906137\
        499540618594627036323768392515241079590094308355448643461460059879994035190927166626823010\
        554114963964653643589039994404958837088705449153040016315590821399411558577181934830851499\
        967080807439343009022752776993456557593005193187662502279093022402037512059759215283726684\
        180683729868721199427664258965448018694945758659793706721440892106251095847593779672219369\
        809465727499995305379352454513044175881811237487892942918401872896557866990104999331109347\
        496129795663821180970529605016401307493728329984443703397808369592978644432123673179294343\
        224841119300349167632139952814158280695697127882453103259301082363151668148753784893530330\
        34282490800463593546460765661841406784944688",
        "0x0.b504f333f9de6484597d89b3754abe9f1d6f60ba893ba84ced17ac85833399154afc83043ab8a2c3a8b1f\
        e6fdc83db390f74a85e439c7b4a780487363dfa2768d2202e8742af1f4e53059c6011bc337bcab1bc911688458\
        a460abc722f7c4e33c6d5a8a38bb7e9dccb2a634331f3c84df52f120f836e582eeaa4a0899040ca4a81394ab6d\
        8fd0efdf4d3a02cebc93e0c4264dabcd528b651b8cf341b6f8236c70104dc01fe32352f332a5e9f7bda1ebff6a\
        1be3fca221307dea06241f7aa81c2c1fcbddea2f7dc3318838a2eaff5f3b2d24f4a763facb882fdfe170fd3b1f\
        780f9acce41797f2805c246785e929570235fcf8f7bca3ea33b4d7c60a5e633e3e1485f3b494d82bc6085ac27d\
        a43e4927adb8fc16e69481b04ef744894c1ea75568775190fba44fa353f48185f107dbb4a77dac64cc266eb850\
        ed4822e1e899d034211eb71c181ec80dd4ed1a3b3423cb62e6acb96e07f9aa061a094a16b203080f7b7e36f488\
        a515a79246344e3005da0545ab5820feaef3706e86336a418ff3fffababf23884c066deae134242ed2f48d9f17\
        902db9392dcb8eb050fc44784505370806676e1672decc57738f21713469bd3039791011a309ffe11229a1cf54\
        bd4ccdb64f1e738fca6b04956709055c72a8706aa88b44318bbc67b01a86817f42f94f645f2e395c03d7abb8dc\
        12d985073c1bb548e046353f87c7991d9b140e912b44e05ad41023edcc4fb1d45327428cde06860f111402426c\
        a7a7cdd9ea598ea44eba918dae319e24064b5f2a4dfaecb33c5a69626ed433dec724014ff54640b9ab3e15d1e9\
        e74eff05e8da6ebb88bc02bdb4adbf578d82e11646aff75e83bff64b6dc7bbc7e0e15dde70da4f5fad7a230441\
        4ac56e80e53f8db5e05bf60de3705376de33fc2d93a70430d9d09bab8d8b2a4c39e908e355734ec00f2bca22de\
        3051f0527ec4b475bca5ee3816b4a2ed4a5825220655e4a1c3e1e937e879df457b182d29d0bb94456509fda036\
        4c148aec1dd069aac6c0ee88acf4b21f5513f5bbabd10294badb7a5444f1e8495d8e342a406e174cca3d9b96f6\
        d02f10c97cad8dc932895a026198c0eb251acd44db7c43279eaba98c9acd51c312bd49d2bd2cbbaa3fd07b0366\
        197754277b881e9696abdb60a9d2f9be163bff14946990d94a3875720ac59e76e3256f0f218d09c5dbdf3982da\
        37bff05e3aa7f9f60215ee42d992ccb291543dc321f1e8bc387c612dacbc0306aadc17ea769f9f0ac495ada91b\
        0c1659418fa26513d68d337eab7ee14d3e3d49f213cd5096a30506a4a4adb5f26e39eefcb774054efe116c69c2\
        f51766400eba060cc863df9422c240293ddc5d4084e93b861ffef090d3df21a7c1e8555fdef298fba5d7ede16d\
        7df6701732ddbbdc7ddc66da56c725cf780f16487c034e91f67d86471713eb23ed521bb15325ce3eba29a27eda\
        8b7e2b7d0f57354ac3a7da80fb4d8bdb35c0efdd08f6a920aebf71f29fcac2ca35d9040b0bc02ccb402dda7ced\
        3cb7b19e4e89fb60b961d7f1a00d1974ceeb5123f13cb0051fae04cc8d32aa51a8007ff04855fde16c525440a2\
        cf6893d2a66765b1e7b1b9d388e4e7c8ab361cbd9413c53f85dc5de228b6ee5587b74aaba247c9f75c07b4f9a5\
        a92e3b0799c077cf6dc56777d46d9e3e713721552cc26a6dcab09762945122fe7a1ed9babf85050fe127f10c7a\
        5b18390482022591979c6201978e1d5c4b8eea7bcb36ecbb0a5d746bcbd042458eae7ed89bf#10000",
        Greater,
    );

    let sqrt_2_over_2_f32 = Float::sqrt_2_over_2_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_2_over_2_f32.to_string(), "0.70710677");
    assert_eq!(to_hex_string(&sqrt_2_over_2_f32), "0x0.b504f3#24");
    assert_eq!(sqrt_2_over_2_f32, f32::SQRT_2_OVER_2);

    let sqrt_2_over_2_f64 = Float::sqrt_2_over_2_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_2_over_2_f64.to_string(), "0.7071067811865476");
    assert_eq!(to_hex_string(&sqrt_2_over_2_f64), "0x0.b504f333f9de68#53");
    assert_eq!(sqrt_2_over_2_f64, f64::SQRT_2_OVER_2);
}

fn test_sqrt_2_over_2_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::sqrt_2_over_2_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_sqrt_2_over_2_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_sqrt_2_over_2_prec_round() {
    test_sqrt_2_over_2_prec_round_helper(1, Floor, "0.5", "0x0.8#1", Less);
    test_sqrt_2_over_2_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_sqrt_2_over_2_prec_round_helper(1, Down, "0.5", "0x0.8#1", Less);
    test_sqrt_2_over_2_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_sqrt_2_over_2_prec_round_helper(1, Nearest, "0.5", "0x0.8#1", Less);

    test_sqrt_2_over_2_prec_round_helper(2, Floor, "0.5", "0x0.8#2", Less);
    test_sqrt_2_over_2_prec_round_helper(2, Ceiling, "0.8", "0x0.c#2", Greater);
    test_sqrt_2_over_2_prec_round_helper(2, Down, "0.5", "0x0.8#2", Less);
    test_sqrt_2_over_2_prec_round_helper(2, Up, "0.8", "0x0.c#2", Greater);
    test_sqrt_2_over_2_prec_round_helper(2, Nearest, "0.8", "0x0.c#2", Greater);

    test_sqrt_2_over_2_prec_round_helper(3, Floor, "0.6", "0x0.a#3", Less);
    test_sqrt_2_over_2_prec_round_helper(3, Ceiling, "0.8", "0x0.c#3", Greater);
    test_sqrt_2_over_2_prec_round_helper(3, Down, "0.6", "0x0.a#3", Less);
    test_sqrt_2_over_2_prec_round_helper(3, Up, "0.8", "0x0.c#3", Greater);
    test_sqrt_2_over_2_prec_round_helper(3, Nearest, "0.8", "0x0.c#3", Greater);

    test_sqrt_2_over_2_prec_round_helper(4, Floor, "0.7", "0x0.b#4", Less);
    test_sqrt_2_over_2_prec_round_helper(4, Ceiling, "0.75", "0x0.c#4", Greater);
    test_sqrt_2_over_2_prec_round_helper(4, Down, "0.7", "0x0.b#4", Less);
    test_sqrt_2_over_2_prec_round_helper(4, Up, "0.75", "0x0.c#4", Greater);
    test_sqrt_2_over_2_prec_round_helper(4, Nearest, "0.7", "0x0.b#4", Less);

    test_sqrt_2_over_2_prec_round_helper(5, Floor, "0.69", "0x0.b0#5", Less);
    test_sqrt_2_over_2_prec_round_helper(5, Ceiling, "0.72", "0x0.b8#5", Greater);
    test_sqrt_2_over_2_prec_round_helper(5, Down, "0.69", "0x0.b0#5", Less);
    test_sqrt_2_over_2_prec_round_helper(5, Up, "0.72", "0x0.b8#5", Greater);
    test_sqrt_2_over_2_prec_round_helper(5, Nearest, "0.72", "0x0.b8#5", Greater);

    test_sqrt_2_over_2_prec_round_helper(6, Floor, "0.7", "0x0.b4#6", Less);
    test_sqrt_2_over_2_prec_round_helper(6, Ceiling, "0.72", "0x0.b8#6", Greater);
    test_sqrt_2_over_2_prec_round_helper(6, Down, "0.7", "0x0.b4#6", Less);
    test_sqrt_2_over_2_prec_round_helper(6, Up, "0.72", "0x0.b8#6", Greater);
    test_sqrt_2_over_2_prec_round_helper(6, Nearest, "0.7", "0x0.b4#6", Less);

    test_sqrt_2_over_2_prec_round_helper(7, Floor, "0.703", "0x0.b4#7", Less);
    test_sqrt_2_over_2_prec_round_helper(7, Ceiling, "0.71", "0x0.b6#7", Greater);
    test_sqrt_2_over_2_prec_round_helper(7, Down, "0.703", "0x0.b4#7", Less);
    test_sqrt_2_over_2_prec_round_helper(7, Up, "0.71", "0x0.b6#7", Greater);
    test_sqrt_2_over_2_prec_round_helper(7, Nearest, "0.71", "0x0.b6#7", Greater);

    test_sqrt_2_over_2_prec_round_helper(8, Floor, "0.707", "0x0.b5#8", Less);
    test_sqrt_2_over_2_prec_round_helper(8, Ceiling, "0.711", "0x0.b6#8", Greater);
    test_sqrt_2_over_2_prec_round_helper(8, Down, "0.707", "0x0.b5#8", Less);
    test_sqrt_2_over_2_prec_round_helper(8, Up, "0.711", "0x0.b6#8", Greater);
    test_sqrt_2_over_2_prec_round_helper(8, Nearest, "0.707", "0x0.b5#8", Less);

    test_sqrt_2_over_2_prec_round_helper(9, Floor, "0.707", "0x0.b50#9", Less);
    test_sqrt_2_over_2_prec_round_helper(9, Ceiling, "0.709", "0x0.b58#9", Greater);
    test_sqrt_2_over_2_prec_round_helper(9, Down, "0.707", "0x0.b50#9", Less);
    test_sqrt_2_over_2_prec_round_helper(9, Up, "0.709", "0x0.b58#9", Greater);
    test_sqrt_2_over_2_prec_round_helper(9, Nearest, "0.707", "0x0.b50#9", Less);

    test_sqrt_2_over_2_prec_round_helper(10, Floor, "0.707", "0x0.b50#10", Less);
    test_sqrt_2_over_2_prec_round_helper(10, Ceiling, "0.708", "0x0.b54#10", Greater);
    test_sqrt_2_over_2_prec_round_helper(10, Down, "0.707", "0x0.b50#10", Less);
    test_sqrt_2_over_2_prec_round_helper(10, Up, "0.708", "0x0.b54#10", Greater);
    test_sqrt_2_over_2_prec_round_helper(10, Nearest, "0.707", "0x0.b50#10", Less);

    test_sqrt_2_over_2_prec_round_helper(
        100,
        Floor,
        "0.7071067811865475244008443621046",
        "0x0.b504f333f9de6484597d89b37#100",
        Less,
    );
    test_sqrt_2_over_2_prec_round_helper(
        100,
        Ceiling,
        "0.7071067811865475244008443621054",
        "0x0.b504f333f9de6484597d89b38#100",
        Greater,
    );
    test_sqrt_2_over_2_prec_round_helper(
        100,
        Down,
        "0.7071067811865475244008443621046",
        "0x0.b504f333f9de6484597d89b37#100",
        Less,
    );
    test_sqrt_2_over_2_prec_round_helper(
        100,
        Up,
        "0.7071067811865475244008443621054",
        "0x0.b504f333f9de6484597d89b38#100",
        Greater,
    );
    test_sqrt_2_over_2_prec_round_helper(
        100,
        Nearest,
        "0.7071067811865475244008443621046",
        "0x0.b504f333f9de6484597d89b37#100",
        Less,
    );
}

#[test]
#[should_panic]
fn sqrt_2_over_2_prec_round_fail_1() {
    Float::sqrt_2_over_2_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sqrt_2_over_2_prec_round_fail_2() {
    Float::sqrt_2_over_2_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn sqrt_2_over_2_prec_round_fail_3() {
    Float::sqrt_2_over_2_prec_round(1000, Exact);
}

#[test]
fn sqrt_2_over_2_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (sqrt_2_over_2, o) = Float::sqrt_2_over_2_prec(prec);
        assert!(sqrt_2_over_2.is_valid());
        assert_eq!(sqrt_2_over_2.get_prec(), Some(prec));
        assert_eq!(sqrt_2_over_2.get_exponent(), Some(0));
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_2_over_2_alt, o_alt) = Float::sqrt_2_over_2_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_2_over_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(sqrt_2_over_2_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_2_over_2.is_power_of_2() {
            let (sqrt_2_over_2_alt, o_alt) = Float::sqrt_2_over_2_prec_round(prec, Floor);
            let mut next_lower = sqrt_2_over_2.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(sqrt_2_over_2_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (sqrt_2_over_2_alt, o_alt) = Float::sqrt_2_over_2_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&sqrt_2_over_2_alt),
            ComparableFloatRef(&sqrt_2_over_2)
        );
        assert_eq!(o_alt, o);

        let (rug_sqrt_2_over_2, rug_o) =
            rug_sqrt_2_over_2_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sqrt_2_over_2)),
            ComparableFloatRef(&sqrt_2_over_2)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn sqrt_2_over_2_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (sqrt_2_over_2, o) = Float::sqrt_2_over_2_prec_round(prec, rm);
        assert!(sqrt_2_over_2.is_valid());
        assert_eq!(sqrt_2_over_2.get_prec(), Some(prec));
        assert_eq!(
            sqrt_2_over_2.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                1
            } else {
                0
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_2_over_2_alt, o_alt) = Float::sqrt_2_over_2_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_2_over_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(sqrt_2_over_2_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_2_over_2.is_power_of_2() {
            let (sqrt_2_over_2_alt, o_alt) = Float::sqrt_2_over_2_prec_round(prec, Floor);
            let mut next_lower = sqrt_2_over_2.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(sqrt_2_over_2_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sqrt_2_over_2, rug_o) = rug_sqrt_2_over_2_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sqrt_2_over_2)),
                ComparableFloatRef(&sqrt_2_over_2)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::sqrt_2_over_2_prec_round(prec, Exact));
    });

    test_constant(Float::sqrt_2_over_2_prec_round, 10000);
}
