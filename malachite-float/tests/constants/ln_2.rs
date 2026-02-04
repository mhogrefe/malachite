// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::{Ln2, Two};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::ln_2::rug_ln_2_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_ln_2_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::ln_2_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = Float::ln_prec(Float::TWO, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (rug_x, rug_o) =
        rug_ln_2_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_ln_2_prec() {
    // - in ln_2_prec_round
    // - in sum
    // - n2 != n1 + 1 in sum
    // - n2 == n1 + 1 in sum
    // - n1 == 0 in sum
    // - n1 != 0 in sum
    // - !need_p first time in sum
    // - v == 0 first time in sum
    // - can't round in ln_2_prec_round
    // - need_p first time in sum
    // - v != 0 first time in sum
    // - w < v first time in sum
    // - need_p second time in sum
    // - w < v second time in sum
    // - v != 0 second time in sum
    // - need_p third time in sum
    // - w >= v first time in sum
    // - v == 0 second time in sum
    // - w >= v second time in sum
    // - !need_p second time in sum
    // - !need_p third time in sum
    // - can round in ln_2_prec_round
    test_ln_2_prec_helper(1, "0.5", "0x0.8#1", Less);
    test_ln_2_prec_helper(2, "0.8", "0x0.c#2", Greater);
    test_ln_2_prec_helper(3, "0.8", "0x0.c#3", Greater);
    test_ln_2_prec_helper(4, "0.7", "0x0.b#4", Less);
    test_ln_2_prec_helper(5, "0.69", "0x0.b0#5", Less);
    test_ln_2_prec_helper(6, "0.69", "0x0.b0#6", Less);
    test_ln_2_prec_helper(7, "0.695", "0x0.b2#7", Greater);
    test_ln_2_prec_helper(8, "0.691", "0x0.b1#8", Less);
    test_ln_2_prec_helper(9, "0.693", "0x0.b18#9", Greater);
    test_ln_2_prec_helper(10, "0.693", "0x0.b18#10", Greater);
    test_ln_2_prec_helper(
        100,
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test_ln_2_prec_helper(
        1000,
        "0.693147180559945309417232121458176568075500134360255254120680009493393621969694715605863\
        326996418687542001481020570685733685520235758130557032670751635075961930727570828371435190\
        307038623891673471123350115364497955239120475172681574932065155524734139525882950453007095\
        3263666426541042391578149520437404",
        "0x0.b17217f7d1cf79abc9e3b39803f2f6af40f343267298b62d8a0d175b8baafa2be7b876206debac9855955\
        2fb4afa1b10ed2eae35c138214427573b291169b8253e96ca16224ae8c51acbda11317c387eb9ea9bc3b136603\
        b256fa0ec7657f74b72ce87b19d6548caf5dfa6bd38303248655fa1872f20e3a2da2d97c50f#1000",
        Less,
    );
    test_ln_2_prec_helper(
        10000,
        "0.693147180559945309417232121458176568075500134360255254120680009493393621969694715605863\
        326996418687542001481020570685733685520235758130557032670751635075961930727570828371435190\
        307038623891673471123350115364497955239120475172681574932065155524734139525882950453007095\
        326366642654104239157814952043740430385500801944170641671518644712839968171784546957026271\
        631064546150257207402481637773389638550695260668341137273873722928956493547025762652098859\
        693201965058554764703306793654432547632744951250406069438147104689946506220167720424524529\
        612687946546193165174681392672504103802546259656869144192871608293803172714367782654877566\
        485085674077648451464439940461422603193096735402574446070308096085047486638523138181676751\
        438667476647890881437141985494231519973548803751658612753529166100071053558249879414729509\
        293113897155998205654392871700072180857610252368892132449713893203784393530887748259701715\
        591070882368362758984258918535302436342143670611892367891923723146723217205340164925687274\
        778234453534764811494186423867767744060695626573796008670762571991847340226514628379048830\
        620330611446300737194890027436439650025809365194430411911506080948793067865158870900605203\
        468429736193841289652556539686022194122924207574321757489097706752687115817051137009158942\
        665478595964890653058460258668382940022833005382074005677053046787001841624044188332327983\
        863490015631218895606505531512721993983320307514084260914790012651682434438935724727882054\
        862715527418772430024897945401961872339808608316648114909306675193393128904316413706813977\
        764981769748689038877899912965036192707108892641052309247839173735012298424204995689359922\
        066022046549415106139187885744245577510206837030866619480896412186807790208181588580001688\
        115973056186676199187395200766719214592236720602539595436541655311295175989940056000366513\
        567569051245926825743946483168332624901803824240824231452306140963805700702551387702681785\
        163069025513703234053802145019015374029509942262995779647427138157363801729873940704242179\
        972266962979939312706935747240493386530879758721699645129446491883771156701678598804981838\
        896784134938314014073166472765327635919233511233389338709513209059272185471328975470797891\
        384445466676192702885533423429899321803769154973340267546758873236778342916191810430116091\
        695265547859732891763545556742863877463987101912431754255888301206779210280341206879759143\
        081283307230300883494705792496591005860012341561757413272465943068435465211135021544341539\
        955381856522750221424566440006276183303206472725721975152908278568421320795988638967277119\
        552218819046603957009774706512619505278932296088931405625433442552392062030343941777357945\
        592125901992559114844024239012554259003129537051922061506434583787873002035414421785758013\
        236451660709914383145004985896688577222148652882169418127048860758972203216663128378329156\
        763074987298574638928269373509840778049395004933998762647550703162216139034845299424917248\
        373406136622638349368111684167056925214751383930638455371862687797328895558871634429756244\
        75539236636948887782389017498102735655240503",
        "0x0.b17217f7d1cf79abc9e3b39803f2f6af40f343267298b62d8a0d175b8baafa2be7b876206debac9855955\
        2fb4afa1b10ed2eae35c138214427573b291169b8253e96ca16224ae8c51acbda11317c387eb9ea9bc3b136603\
        b256fa0ec7657f74b72ce87b19d6548caf5dfa6bd38303248655fa1872f20e3a2da2d97c50f3fd5c607f4ca11f\
        b5bfb90610d30f88fe551a2ee569d6dfc1efa157d2e23de1400b39617460775db8990e5c943e732b479cd33ccc\
        c4e659393514c4c1a1e0bd1d6095d25669b333564a3376a9c7f8a5e148e82074db6015cfe7aa30c480a5417350\
        d2c955d5179b1e17b9dae313cdb6c606cb1078f735d1b2db31b5f50b5185064c18b4d162db3b365853d7598a19\
        51ae273ee5570b6c68f96983496d4e6d330af889b44a02554731cdc8ea17293d1228a4ef98d6f5177fbcf07552\
        68a5c1f9538b98261affd446b1ca3cf5e9222b88c66d3c5422183edc99421090bbb16faf3d949f236e02b20cee\
        886b905c128d53d0bd2f9621363196af503020060e49908391a0c57339ba2beba7d052ac5b61cc4e9207cef2f0\
        ce2d7373958d7622658901e646a95184460dc4e7487156e0c292413d5e361c1696dd24aaebd473826fda0c238b\
        90ab111bbbd67c724972cd18bfbbd9d426c472096e76115c05f6f7cebac9f45aececb72f19c38339d8f6826250\
        dea891ef07afff3a892374e175eb4afc8daadd885db6ab03a49bd0dc0b1b31d8a0e23fac5e5767df95884e0642\
        5a41526fac51c3ea8449fe8f70edd062b1a63a6c4c60c52ab33161e238438897a39ce78b63c9f364f5b8aef22e\
        c2fee6e0850eca42d06fb0c75df5497e00c554b03d7d2874a000ca8f58d94f0341cbe2ec92156c9f949db4a931\
        6f281501e53daec3f64f1b783154c60320e2ff79333ce3573facc5fdcf11785903155bbd90f023b220224fcd84\
        71bf4f445f0a88a14f0cd976ea354bb20cdb5ccb3db239288d586554e2a0e8a6fe51a8cfaa72ef2ad8a43dc421\
        2b210b779dfe49d7307cc846532e4b9694edad162af053b1751f3a3d091f65665815412b5e8c202461069ac14b\
        958784934b8d6cce1daa50537011aa4fb42b9a3def41bda1f85ef6fdbf2f2d89d2a4b1835278fd9405789f4568\
        12b552879a6168695c12963b0ff01eaab73e5b5c1585318e7624f14a51a4a026b6808292057fd99b66dc085a98\
        ac8d8caf9eeeea98a2400cac95f260fd10036f9f91096ac3195220a1a356b2a73b7eaadaf6d605871ef7afb80b\
        c423433562e94b12dfab414451579df59eae0517070624012a82962c59cab347f8304d889659e5a9139db14efc\
        c30852be3e8fc99f14d1d822dd6e2f76797e30219c8aa9ce8848a886eb3c87b7295988012e8314186edbaf8685\
        6ccd3c3b6ee94e62f110a6783d2aae89ccc3b76fc435a0ce134c2838fd571ec6c1366a992cbb9ac407ddb6c13a\
        4b8d1ecf7567eb0971cc90b5518569f144e67ebe9b42698fea79d89d5c5ed40ac5e3701d7d7725377cf0656907\
        fb9b1b16ea8911afbf1ae5a66203d62fd1e7093435b9c277736a70fa8601cf6868a055b2238677a2bfbbd843bf\
        a1873f0c446b01b2ae0e98e0e1527a900b1af5e75f87c7cd17af804d933d6f7e1b9e1903d71c7bba0281137609\
        0dd617335dfdd424f2b661cd85063034e341b06e211977b075a8b7808df43bd8eef1fd9678cc0b5e9f60a3eb81\
        747f87e5709468d78ebd2da0116e6b65aeb3be77ee236fdc33bc8e7df1ffc2e2288f8ca9aee#10000",
        Less,
    );

    let ln_2_f32 = Float::ln_2_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(ln_2_f32.to_string(), "0.69314718");
    assert_eq!(to_hex_string(&ln_2_f32), "0x0.b17218#24");
    assert_eq!(ln_2_f32, f32::LN_2);

    let ln_2_f64 = Float::ln_2_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(ln_2_f64.to_string(), "0.6931471805599453");
    assert_eq!(to_hex_string(&ln_2_f64), "0x0.b17217f7d1cf78#53");
    assert_eq!(ln_2_f64, f64::LN_2);
}

#[test]
#[should_panic]
fn ln_2_prec_fail_1() {
    Float::ln_2_prec(0);
}

fn test_ln_2_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::ln_2_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = Float::ln_prec_round(Float::TWO, prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_ln_2_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_ln_2_prec_round() {
    test_ln_2_prec_round_helper(1, Floor, "0.5", "0x0.8#1", Less);
    test_ln_2_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_ln_2_prec_round_helper(1, Down, "0.5", "0x0.8#1", Less);
    test_ln_2_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_ln_2_prec_round_helper(1, Nearest, "0.5", "0x0.8#1", Less);

    test_ln_2_prec_round_helper(2, Floor, "0.5", "0x0.8#2", Less);
    test_ln_2_prec_round_helper(2, Ceiling, "0.8", "0x0.c#2", Greater);
    test_ln_2_prec_round_helper(2, Down, "0.5", "0x0.8#2", Less);
    test_ln_2_prec_round_helper(2, Up, "0.8", "0x0.c#2", Greater);
    test_ln_2_prec_round_helper(2, Nearest, "0.8", "0x0.c#2", Greater);

    test_ln_2_prec_round_helper(3, Floor, "0.6", "0x0.a#3", Less);
    test_ln_2_prec_round_helper(3, Ceiling, "0.8", "0x0.c#3", Greater);
    test_ln_2_prec_round_helper(3, Down, "0.6", "0x0.a#3", Less);
    test_ln_2_prec_round_helper(3, Up, "0.8", "0x0.c#3", Greater);
    test_ln_2_prec_round_helper(3, Nearest, "0.8", "0x0.c#3", Greater);

    test_ln_2_prec_round_helper(4, Floor, "0.7", "0x0.b#4", Less);
    test_ln_2_prec_round_helper(4, Ceiling, "0.75", "0x0.c#4", Greater);
    test_ln_2_prec_round_helper(4, Down, "0.7", "0x0.b#4", Less);
    test_ln_2_prec_round_helper(4, Up, "0.75", "0x0.c#4", Greater);
    test_ln_2_prec_round_helper(4, Nearest, "0.7", "0x0.b#4", Less);

    test_ln_2_prec_round_helper(5, Floor, "0.69", "0x0.b0#5", Less);
    test_ln_2_prec_round_helper(5, Ceiling, "0.72", "0x0.b8#5", Greater);
    test_ln_2_prec_round_helper(5, Down, "0.69", "0x0.b0#5", Less);
    test_ln_2_prec_round_helper(5, Up, "0.72", "0x0.b8#5", Greater);
    test_ln_2_prec_round_helper(5, Nearest, "0.69", "0x0.b0#5", Less);

    test_ln_2_prec_round_helper(6, Floor, "0.69", "0x0.b0#6", Less);
    test_ln_2_prec_round_helper(6, Ceiling, "0.7", "0x0.b4#6", Greater);
    test_ln_2_prec_round_helper(6, Down, "0.69", "0x0.b0#6", Less);
    test_ln_2_prec_round_helper(6, Up, "0.7", "0x0.b4#6", Greater);
    test_ln_2_prec_round_helper(6, Nearest, "0.69", "0x0.b0#6", Less);

    test_ln_2_prec_round_helper(7, Floor, "0.69", "0x0.b0#7", Less);
    test_ln_2_prec_round_helper(7, Ceiling, "0.695", "0x0.b2#7", Greater);
    test_ln_2_prec_round_helper(7, Down, "0.69", "0x0.b0#7", Less);
    test_ln_2_prec_round_helper(7, Up, "0.695", "0x0.b2#7", Greater);
    test_ln_2_prec_round_helper(7, Nearest, "0.695", "0x0.b2#7", Greater);

    test_ln_2_prec_round_helper(8, Floor, "0.691", "0x0.b1#8", Less);
    test_ln_2_prec_round_helper(8, Ceiling, "0.695", "0x0.b2#8", Greater);
    test_ln_2_prec_round_helper(8, Down, "0.691", "0x0.b1#8", Less);
    test_ln_2_prec_round_helper(8, Up, "0.695", "0x0.b2#8", Greater);
    test_ln_2_prec_round_helper(8, Nearest, "0.691", "0x0.b1#8", Less);

    test_ln_2_prec_round_helper(9, Floor, "0.691", "0x0.b10#9", Less);
    test_ln_2_prec_round_helper(9, Ceiling, "0.693", "0x0.b18#9", Greater);
    test_ln_2_prec_round_helper(9, Down, "0.691", "0x0.b10#9", Less);
    test_ln_2_prec_round_helper(9, Up, "0.693", "0x0.b18#9", Greater);
    test_ln_2_prec_round_helper(9, Nearest, "0.693", "0x0.b18#9", Greater);

    test_ln_2_prec_round_helper(10, Floor, "0.692", "0x0.b14#10", Less);
    test_ln_2_prec_round_helper(10, Ceiling, "0.693", "0x0.b18#10", Greater);
    test_ln_2_prec_round_helper(10, Down, "0.692", "0x0.b14#10", Less);
    test_ln_2_prec_round_helper(10, Up, "0.693", "0x0.b18#10", Greater);
    test_ln_2_prec_round_helper(10, Nearest, "0.693", "0x0.b18#10", Greater);

    test_ln_2_prec_round_helper(
        100,
        Floor,
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test_ln_2_prec_round_helper(
        100,
        Ceiling,
        "0.693147180559945309417232121459",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Greater,
    );
    test_ln_2_prec_round_helper(
        100,
        Down,
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test_ln_2_prec_round_helper(
        100,
        Up,
        "0.693147180559945309417232121459",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Greater,
    );
    test_ln_2_prec_round_helper(
        100,
        Nearest,
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
}

#[test]
#[should_panic]
fn ln_2_prec_round_fail_1() {
    Float::ln_2_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn ln_2_prec_round_fail_2() {
    Float::ln_2_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn ln_2_prec_round_fail_3() {
    Float::ln_2_prec_round(1000, Exact);
}

#[test]
fn ln_2_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (ln_2, o) = Float::ln_2_prec(prec);
        assert!(ln_2.is_valid());
        assert_eq!(ln_2.get_prec(), Some(prec));
        assert_eq!(ln_2.get_exponent(), Some(0));
        assert_ne!(o, Equal);
        if o == Less {
            let (ln_2_alt, o_alt) = Float::ln_2_prec_round(prec, Ceiling);
            let mut next_upper = ln_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(ln_2_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !ln_2.is_power_of_2() {
            let (ln_2_alt, o_alt) = Float::ln_2_prec_round(prec, Floor);
            let mut next_lower = ln_2.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(ln_2_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (ln_2_alt, o_alt) = Float::ln_2_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&ln_2_alt), ComparableFloatRef(&ln_2));
        assert_eq!(o_alt, o);

        let (ln_2_alt, o_alt) = Float::ln_prec(Float::TWO, prec);
        assert_eq!(ComparableFloatRef(&ln_2_alt), ComparableFloatRef(&ln_2));
        assert_eq!(o_alt, o);

        let (rug_ln_2, rug_o) =
            rug_ln_2_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln_2)),
            ComparableFloatRef(&ln_2)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn ln_2_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (ln_2, o) = Float::ln_2_prec_round(prec, rm);
        assert!(ln_2.is_valid());
        assert_eq!(ln_2.get_prec(), Some(prec));
        assert_eq!(
            ln_2.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                1
            } else {
                0
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (ln_2_alt, o_alt) = Float::ln_2_prec_round(prec, Ceiling);
            let mut next_upper = ln_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(ln_2_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !ln_2.is_power_of_2() {
            let (ln_2_alt, o_alt) = Float::ln_2_prec_round(prec, Floor);
            let mut next_lower = ln_2.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(ln_2_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        let (ln_2_alt, o_alt) = Float::ln_prec_round(Float::TWO, prec, rm);
        assert_eq!(ComparableFloatRef(&ln_2_alt), ComparableFloatRef(&ln_2));
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_ln_2, rug_o) = rug_ln_2_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_ln_2)),
                ComparableFloatRef(&ln_2)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::ln_2_prec_round(prec, Exact));
    });

    test_constant(Float::ln_2_prec_round, 10000);
}
