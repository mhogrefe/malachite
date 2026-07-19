// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Sqrt5Over5;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::sqrt_5_over_5::rug_sqrt_5_over_5_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_sqrt_5_over_5_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::sqrt_5_over_5_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) =
        rug_sqrt_5_over_5_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_sqrt_5_over_5_prec() {
    test_sqrt_5_over_5_prec_helper(1, "0.50", "0x0.8#1", Greater);
    test_sqrt_5_over_5_prec_helper(2, "0.50", "0x0.8#2", Greater);
    test_sqrt_5_over_5_prec_helper(3, "0.44", "0x0.7#3", Less);
    test_sqrt_5_over_5_prec_helper(4, "0.438", "0x0.70#4", Less);
    test_sqrt_5_over_5_prec_helper(5, "0.453", "0x0.74#5", Greater);
    test_sqrt_5_over_5_prec_helper(6, "0.445", "0x0.72#6", Less);
    test_sqrt_5_over_5_prec_helper(7, "0.4453", "0x0.72#7", Less);
    test_sqrt_5_over_5_prec_helper(8, "0.4473", "0x0.728#8", Greater);
    test_sqrt_5_over_5_prec_helper(9, "0.4473", "0x0.728#9", Greater);
    test_sqrt_5_over_5_prec_helper(10, "0.44727", "0x0.728#10", Greater);
    test_sqrt_5_over_5_prec_helper(
        100,
        "0.44721359549995793928183473374639",
        "0x0.727c9716ffb764d594a519c028#100",
        Greater,
    );
    test_sqrt_5_over_5_prec_helper(
        1000,
        "0.447213595499957939281834733746255247088123671922305144854179449082104185127560979882882\
        881675756454993901635230154756700850653544889414772717272024306690541773355634638375833162\
        255329064527971316107152270083506757000684678482812888417286507819450518525445775259903480\
        488136322355181781899698474278145926",
        "0x0.727c9716ffb764d594a519c0252be9ae6d00dc9194a760ed9691c407204d6c3ba95cc6ad54ba97cc67b60\
        1be24c700ff9a3692d0768013344d88eb0e6c7af27c602abcac3c6fcfcc3567d892ccb2324f5b87a8ba4ee2d5c\
        deabed1eb98ccb18d5a927b16b9b7b47a4b2fb1ca056ec9721c71b77b2bb2344265e86960258#1000",
        Less,
    );
    test_sqrt_5_over_5_prec_helper(
        10000,
        "0.447213595499957939281834733746255247088123671922305144854179449082104185127560979882882\
        881675756454993901635230154756700850653544889414772717272024306690541773355634638375833162\
        255329064527971316107152270083506757000684678482812888417286507819450518525445775259903480\
        488136322355181781899698474278145945779696417728308537978819826338715403949735776885017950\
        826591236635384299995484960306086823007191533666502499763035627881600112484171048708447111\
        221261268564046818666396586791949270454240268349922840527180947577100877937412227132009151\
        427991319113391383512915644390500012107846246801001857352975105944411353250733214897170701\
        052466135698926684448463527455405326481536020888663165146701178619627245268639737294389397\
        994036163790485289192406904428238446582519639265162220834099161409624080691198988701371110\
        371114502477728331002052487262514204889923757884936580680894943223091144640347535318092183\
        705915120715596879610831076155812878727944605751212599896442770435469718490703024209269111\
        008141445574484306955750582242424236866357867038207601602223635800918123769249929420848849\
        661776025881362262939190655889579799786338315492158492361500135975424840969476100554721658\
        311982792489782988712136692505812881665588928536177797949209261670707575008412274951521376\
        680375817638511823594714892838049707574229238818038382737607022079527687208256211622075739\
        790370402939409128404352778578176889275565277178758488009205775081079692031212341044723018\
        077155082008438736997450854370075043111538663344600955653973332489242135692854497277054915\
        642682013597129061054224836119194569891039090262034461950174299305887256580508002409556064\
        831092897997741235599638006721312448777281927857550703453259194287645591261591229904603088\
        847003307783455728182608395879422271256427873491536234984413512421777563774734334325524524\
        675975422307901936596578136603651816280200779101944652301690566917578721469279223447335673\
        314396521584288057823801799116848304499142583664643348237995144027880757639545603057744683\
        733669083676573460054863064045921525722504952205728469392604022360538244047203162025525686\
        108372343523715028138020312325818352796253445193119256469813570924832371589116888531922571\
        787512970994960698022162711502833294924390366047105191377313899163270607239114907366447053\
        001544484516574733750680940148446532290347953303484134528895243923604844079596707365967004\
        932536061093537534893800373914419917178396632880503241839292370211488496548174459640821887\
        421984472350570630604424218352590241772713919433815892514520650179504459408086825761644664\
        306780239103133028158044351292330842591575608446276415710735381545333286263318639092413744\
        129290182974548816497625635530695033735814718372492885374928398299955787982625894402918399\
        935651524127897052500718856572804924511820757911269076566356471196782592502320073820262531\
        811439436400363448721191025515703996659978571277208917420938669903730780661685608436545207\
        277889083156048834914944682939459999262502189124549391948662781099560325775362130993512551\
        298676697769185396588326280294101828283590904",
        "0x0.727c9716ffb764d594a519c0252be9ae6d00dc9194a760ed9691c407204d6c3ba95cc6ad54ba97cc67b60\
        1be24c700ff9a3692d0768013344d88eb0e6c7af27c602abcac3c6fcfcc3567d892ccb2324f5b87a8ba4ee2d5c\
        deabed1eb98ccb18d5a927b16b9b7b47a4b2fb1ca056ec9721c71b77b2bb2344265e8696025b78f8f294aa4a0c\
        7ddb8adcfdf4ec7adf6db0fb1e3a2838e76c3c8a7c9a5e00aea568fb973b9ce70c05ddb349e1545717879fb502\
        493dd2c8e8210d254d250516ca4121e86ec71410beaa74111a905d5c2c1dcf5c906e8c8c89cd10b9debb1729e9\
        012ec9a014044c4a19aec2d2da269e68d77a4d845fa5e585fe4fa6e1f8602c5677fe323913db3d33ef59583844\
        778518145de0cb29a9b94481e4129aeb56a174a5dc9c21b4c8371affcfb445236f132364829874419ed09fad25\
        451325e177e173aecfe1c47efbccb34ccd6cf8d6298804e4163aa75f92c777be55c357d3be2276e66caa901f84\
        a95458beaf209bc922fe287780570a3c7c35c01a1060521a94e23c591454de96a55b7d4289c06af16f1c70abf7\
        d1f4361427b527a5570f0a27a737a41e80fc721bb91c9fa0e3b197b7821a36c23a07efe7bb4d93bddaea6d4cf9\
        af2ccf981789398055ae0b5ee972e9b7fb0b7fe56f28f527d7f7f1be03f0bbe396a0257002500a3b5654e8439d\
        46f84f0549e17424fa1e5d21a76f52a2db8014af61d11e4df3584462e31d55a78f5019736030a3dfe8343d53c6\
        71c5ef2dcb8dad62d08eee72f2dcfd66132b7f09b87b48b8ee9836b49b5c5a9c20e229b21b4ff51476230679bd\
        1da0aa0194b10e35b6e1166a986788d7b9e732dcd88bedd4ff0e8db7e4fe97d508dc6fdf876816f1b460ee4d66\
        1d4619a4af80976f1add5e1a8579bdfdd2978755a450d95322c05e233013f4cda997debb5ccc9e3301af9df960\
        7022f4853861a36ccdf25cfcfbc91c13f9e3da2e4d5e338ff080b3a9dbb383a547da57939340dd6927e95c0add\
        cae2ebe69234a28f0c06590afe1a580b179fb43d05520edbd35e595b75f757a23faabfcb69b1f6b50037714f91\
        6f3961a3a66d0277ed8135b99e8fd573743f2ea7b88113db3d10cb1de553f3d0310d305e594e87892cb0543b6b\
        d0300ef8de91ae5838437229f35df77eef7ea6ff2a7489f4e7e229400be5a8c02a94ed7615e085327dfb49ba73\
        cf541f122a6021e271f193e43dbb94f996410d8ed06c040f8b296292b2e682f0dfa29115e096d79f8d150568e4\
        d0633708fde904089dc485f98ecac3913ee11fdc41bc7b5d1a1619a0f44496a1746b79a20e8afc64091fd0be3a\
        f4fe49527567129cdb2084ce1b2cafd37aefc1a6f4dd22c3cef8b9c85f5fb3a5f3e258775c6b85a0de635d1900\
        9ba8c03957d14bd4cdbab0217c750434aa88d5a0609748cb67f54ddd531e2d5d51a772ca395361c40381985695\
        841e3b87d37e41a5c232da36de2c59f960201511eb4f981f3dfcc3dd399701fe737f08a2a7f0f191dcd737823d\
        6e980c8c5e609e091d5da1311f6e200a29e8a4d5fcb8a129dbd21c747bf00917e358d8440e5eddc0b80524b149\
        337d17b308f731af067b64b5b2eb58b7d7aa9e6fb784abe592ff22710ebffbd97b1250e1e16655531f18c4900e\
        6f76a5f1c22eeaabddb2255947716ddd6a9f9d1d22af3d6591d7c1435c2e54074a96abe2c08de3b0033c345a21\
        bc355f7509e5f4ab3a401668030501a913f9177132a1dc011378413f1e954ace3894efb1d088#10000",
        Less,
    );

    let sqrt_5_over_5_f32 = Float::sqrt_5_over_5_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_5_over_5_f32.to_string(), "0.447213590");
    assert_eq!(to_hex_string(&sqrt_5_over_5_f32), "0x0.727c970#24");
    assert_eq!(sqrt_5_over_5_f32, f32::SQRT_5_OVER_5);

    let sqrt_5_over_5_f64 = Float::sqrt_5_over_5_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_5_over_5_f64.to_string(), "0.44721359549995793");
    assert_eq!(to_hex_string(&sqrt_5_over_5_f64), "0x0.727c9716ffb764#53");
    assert_eq!(sqrt_5_over_5_f64, f64::SQRT_5_OVER_5);
}

#[test]
#[should_panic]
fn sqrt_5_over_5_prec_fail_1() {
    Float::sqrt_5_over_5_prec(0);
}

fn test_sqrt_5_over_5_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::sqrt_5_over_5_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_sqrt_5_over_5_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_sqrt_5_over_5_prec_round() {
    test_sqrt_5_over_5_prec_round_helper(1, Floor, "0.25", "0x0.4#1", Less);
    test_sqrt_5_over_5_prec_round_helper(1, Ceiling, "0.50", "0x0.8#1", Greater);
    test_sqrt_5_over_5_prec_round_helper(1, Down, "0.25", "0x0.4#1", Less);
    test_sqrt_5_over_5_prec_round_helper(1, Up, "0.50", "0x0.8#1", Greater);
    test_sqrt_5_over_5_prec_round_helper(1, Nearest, "0.50", "0x0.8#1", Greater);

    test_sqrt_5_over_5_prec_round_helper(2, Floor, "0.38", "0x0.6#2", Less);
    test_sqrt_5_over_5_prec_round_helper(2, Ceiling, "0.50", "0x0.8#2", Greater);
    test_sqrt_5_over_5_prec_round_helper(2, Down, "0.38", "0x0.6#2", Less);
    test_sqrt_5_over_5_prec_round_helper(2, Up, "0.50", "0x0.8#2", Greater);
    test_sqrt_5_over_5_prec_round_helper(2, Nearest, "0.50", "0x0.8#2", Greater);

    test_sqrt_5_over_5_prec_round_helper(3, Floor, "0.44", "0x0.7#3", Less);
    test_sqrt_5_over_5_prec_round_helper(3, Ceiling, "0.50", "0x0.8#3", Greater);
    test_sqrt_5_over_5_prec_round_helper(3, Down, "0.44", "0x0.7#3", Less);
    test_sqrt_5_over_5_prec_round_helper(3, Up, "0.50", "0x0.8#3", Greater);
    test_sqrt_5_over_5_prec_round_helper(3, Nearest, "0.44", "0x0.7#3", Less);

    test_sqrt_5_over_5_prec_round_helper(4, Floor, "0.438", "0x0.70#4", Less);
    test_sqrt_5_over_5_prec_round_helper(4, Ceiling, "0.469", "0x0.78#4", Greater);
    test_sqrt_5_over_5_prec_round_helper(4, Down, "0.438", "0x0.70#4", Less);
    test_sqrt_5_over_5_prec_round_helper(4, Up, "0.469", "0x0.78#4", Greater);
    test_sqrt_5_over_5_prec_round_helper(4, Nearest, "0.438", "0x0.70#4", Less);

    test_sqrt_5_over_5_prec_round_helper(5, Floor, "0.438", "0x0.70#5", Less);
    test_sqrt_5_over_5_prec_round_helper(5, Ceiling, "0.453", "0x0.74#5", Greater);
    test_sqrt_5_over_5_prec_round_helper(5, Down, "0.438", "0x0.70#5", Less);
    test_sqrt_5_over_5_prec_round_helper(5, Up, "0.453", "0x0.74#5", Greater);
    test_sqrt_5_over_5_prec_round_helper(5, Nearest, "0.453", "0x0.74#5", Greater);

    test_sqrt_5_over_5_prec_round_helper(6, Floor, "0.445", "0x0.72#6", Less);
    test_sqrt_5_over_5_prec_round_helper(6, Ceiling, "0.453", "0x0.74#6", Greater);
    test_sqrt_5_over_5_prec_round_helper(6, Down, "0.445", "0x0.72#6", Less);
    test_sqrt_5_over_5_prec_round_helper(6, Up, "0.453", "0x0.74#6", Greater);
    test_sqrt_5_over_5_prec_round_helper(6, Nearest, "0.445", "0x0.72#6", Less);

    test_sqrt_5_over_5_prec_round_helper(7, Floor, "0.4453", "0x0.72#7", Less);
    test_sqrt_5_over_5_prec_round_helper(7, Ceiling, "0.4492", "0x0.73#7", Greater);
    test_sqrt_5_over_5_prec_round_helper(7, Down, "0.4453", "0x0.72#7", Less);
    test_sqrt_5_over_5_prec_round_helper(7, Up, "0.4492", "0x0.73#7", Greater);
    test_sqrt_5_over_5_prec_round_helper(7, Nearest, "0.4453", "0x0.72#7", Less);

    test_sqrt_5_over_5_prec_round_helper(8, Floor, "0.4453", "0x0.720#8", Less);
    test_sqrt_5_over_5_prec_round_helper(8, Ceiling, "0.4473", "0x0.728#8", Greater);
    test_sqrt_5_over_5_prec_round_helper(8, Down, "0.4453", "0x0.720#8", Less);
    test_sqrt_5_over_5_prec_round_helper(8, Up, "0.4473", "0x0.728#8", Greater);
    test_sqrt_5_over_5_prec_round_helper(8, Nearest, "0.4473", "0x0.728#8", Greater);

    test_sqrt_5_over_5_prec_round_helper(9, Floor, "0.4463", "0x0.724#9", Less);
    test_sqrt_5_over_5_prec_round_helper(9, Ceiling, "0.4473", "0x0.728#9", Greater);
    test_sqrt_5_over_5_prec_round_helper(9, Down, "0.4463", "0x0.724#9", Less);
    test_sqrt_5_over_5_prec_round_helper(9, Up, "0.4473", "0x0.728#9", Greater);
    test_sqrt_5_over_5_prec_round_helper(9, Nearest, "0.4473", "0x0.728#9", Greater);

    test_sqrt_5_over_5_prec_round_helper(10, Floor, "0.44678", "0x0.726#10", Less);
    test_sqrt_5_over_5_prec_round_helper(10, Ceiling, "0.44727", "0x0.728#10", Greater);
    test_sqrt_5_over_5_prec_round_helper(10, Down, "0.44678", "0x0.726#10", Less);
    test_sqrt_5_over_5_prec_round_helper(10, Up, "0.44727", "0x0.728#10", Greater);
    test_sqrt_5_over_5_prec_round_helper(10, Nearest, "0.44727", "0x0.728#10", Greater);

    test_sqrt_5_over_5_prec_round_helper(
        100,
        Floor,
        "0.44721359549995793928183473374600",
        "0x0.727c9716ffb764d594a519c020#100",
        Less,
    );
    test_sqrt_5_over_5_prec_round_helper(
        100,
        Ceiling,
        "0.44721359549995793928183473374639",
        "0x0.727c9716ffb764d594a519c028#100",
        Greater,
    );
    test_sqrt_5_over_5_prec_round_helper(
        100,
        Down,
        "0.44721359549995793928183473374600",
        "0x0.727c9716ffb764d594a519c020#100",
        Less,
    );
    test_sqrt_5_over_5_prec_round_helper(
        100,
        Up,
        "0.44721359549995793928183473374639",
        "0x0.727c9716ffb764d594a519c028#100",
        Greater,
    );
    test_sqrt_5_over_5_prec_round_helper(
        100,
        Nearest,
        "0.44721359549995793928183473374639",
        "0x0.727c9716ffb764d594a519c028#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn sqrt_5_over_5_prec_round_fail_1() {
    Float::sqrt_5_over_5_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sqrt_5_over_5_prec_round_fail_2() {
    Float::sqrt_5_over_5_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn sqrt_5_over_5_prec_round_fail_3() {
    Float::sqrt_5_over_5_prec_round(1000, Exact);
}

#[test]
fn sqrt_5_over_5_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (sqrt_5_over_5, o) = Float::sqrt_5_over_5_prec(prec);
        assert!(sqrt_5_over_5.is_valid());
        assert_eq!(sqrt_5_over_5.get_prec(), Some(prec));
        assert_eq!(
            sqrt_5_over_5.get_exponent(),
            Some(if prec <= 2 { 0 } else { -1 })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_5_over_5_alt, o_alt) = Float::sqrt_5_over_5_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_5_over_5.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(sqrt_5_over_5_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_5_over_5.is_power_of_2() {
            let (sqrt_5_over_5_alt, o_alt) = Float::sqrt_5_over_5_prec_round(prec, Floor);
            let mut next_lower = sqrt_5_over_5.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(sqrt_5_over_5_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (sqrt_5_over_5_alt, o_alt) = Float::sqrt_5_over_5_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&sqrt_5_over_5_alt),
            ComparableFloatRef(&sqrt_5_over_5)
        );
        assert_eq!(o_alt, o);

        let (rug_sqrt_5_over_5, rug_o) =
            rug_sqrt_5_over_5_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sqrt_5_over_5)),
            ComparableFloatRef(&sqrt_5_over_5)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn sqrt_5_over_5_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (sqrt_5_over_5, o) = Float::sqrt_5_over_5_prec_round(prec, rm);
        assert!(sqrt_5_over_5.is_valid());
        assert_eq!(sqrt_5_over_5.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1 | 2, Ceiling | Up | Nearest) | (3, Ceiling | Up) => 0,
            _ => -1,
        };
        assert_eq!(sqrt_5_over_5.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_5_over_5_alt, o_alt) = Float::sqrt_5_over_5_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_5_over_5.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(sqrt_5_over_5_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_5_over_5.is_power_of_2() {
            let (sqrt_5_over_5_alt, o_alt) = Float::sqrt_5_over_5_prec_round(prec, Floor);
            let mut next_lower = sqrt_5_over_5.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(sqrt_5_over_5_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sqrt_5_over_5, rug_o) = rug_sqrt_5_over_5_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sqrt_5_over_5)),
                ComparableFloatRef(&sqrt_5_over_5)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::sqrt_5_over_5_prec_round(prec, Exact));
    });

    test_constant(Float::sqrt_5_over_5_prec_round, 10000);
}
