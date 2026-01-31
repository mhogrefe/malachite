// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::OneOverSqrtPi;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::constants::one_over_sqrt_pi::one_over_sqrt_pi_prec_round_simple;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_one_over_sqrt_pi_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::one_over_sqrt_pi_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = one_over_sqrt_pi_prec_round_simple(prec, Nearest);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_one_over_sqrt_pi_prec() {
    test_one_over_sqrt_pi_prec_helper(1, "0.5", "0x0.8#1", Less);
    test_one_over_sqrt_pi_prec_helper(2, "0.5", "0x0.8#2", Less);
    test_one_over_sqrt_pi_prec_helper(3, "0.6", "0x0.a#3", Greater);
    test_one_over_sqrt_pi_prec_helper(4, "0.56", "0x0.9#4", Less);
    test_one_over_sqrt_pi_prec_helper(5, "0.56", "0x0.90#5", Less);
    test_one_over_sqrt_pi_prec_helper(6, "0.56", "0x0.90#6", Less);
    test_one_over_sqrt_pi_prec_helper(7, "0.56", "0x0.90#7", Less);
    test_one_over_sqrt_pi_prec_helper(8, "0.562", "0x0.90#8", Less);
    test_one_over_sqrt_pi_prec_helper(9, "0.564", "0x0.908#9", Greater);
    test_one_over_sqrt_pi_prec_helper(10, "0.564", "0x0.908#10", Greater);
    test_one_over_sqrt_pi_prec_helper(
        100,
        "0.564189583547756286948079451561",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test_one_over_sqrt_pi_prec_helper(
        1000,
        "0.564189583547756286948079451560772585844050629328998856844085721710642468441493414486743\
        660202107363443028347906361707351689931494826162866365489520017768993292837637059598439760\
        352464350217972571211580245772820220554508527173216622208463308111399512763454486023068237\
        6909187451587466158513010663983578",
        "0x0.906eba8214db688d71d48a7f6bfec3441409a0ebac3e751739a15830cce620b0c0759cf859270f1140c03\
        6096cc79aebbd1f4eee48e1ca7874f76f877ffec25152561dcc244dc65e9c22f47f7b7fb57c9522f2f93e16b2a\
        3d27a3282dada7316eb9feb2436f2f272ac2c88bbba81b1c750754b409e94d32d18d3e91add#1000",
        Greater,
    );
    test_one_over_sqrt_pi_prec_helper(
        10000,
        "0.564189583547756286948079451560772585844050629328998856844085721710642468441493414486743\
        660202107363443028347906361707351689931494826162866365489520017768993292837637059598439760\
        352464350217972571211580245772820220554508527173216622208463308111399512763454486023068237\
        690918745158746615851301066398357771999377334160357798876166744076233039380216350601643621\
        696235050458312531946879456656288325815521624434548865703189877430881778182948389475108500\
        918458534221631782589335251833012024622562223724947270033897431264299659426350428304490363\
        315803937695985608159337829220557382923788231579233105761964777468253090171561808059522229\
        617632465359040085344294862502894739216418119274309774225569878795779049874819136937239692\
        072860633424767969948609588763043633726455878751543080934419738483288491379175361895663509\
        241348925308830394965405841057254148274823475174742009396988341677714885589167833739498591\
        581650137685988686204396412907286928963807377326731118428678802115524866218971908896807531\
        495120147455271759129815272122063234346757415260256184982277894986952530166949696856638623\
        092204424861433131061101739704316554435364616300266810889304772054130274832143365864710291\
        854949083838769247696374058754103158473317076216953144890497317398112138525262894966581881\
        072472305017305600073055507380269070239209325129365185654408027303045608558893318652586305\
        258834720981091615225551340163448096077340629067652092477744457915659936286913810650697399\
        221160635018381201634827945255588045897195279903092298856115545628197444322318409489548225\
        698923189346820367003206229210032439562925718525419045578836831335596305728880148673271426\
        404156118265947052921488282109621556115877884575869370769316568975733036330605808042677655\
        428887246839586538128191754446004116925982320701014879775987396655241201231020855972580774\
        163232436519687057937186995686641316370707460164674562326647852763293612556036730458147509\
        675836925881775102516171791588543511319449779511302064465744235464169847898040837501074701\
        352751880478226229284695968227276043713529635662439179198513462262203022255831394816897442\
        156283042895891974583056032928037616331070074684468077543352388043093250182093443276971756\
        604585358146189293483000847521883563156134850300525631317511502402815253573480281037755873\
        096872058049839116785448367821638308739616614732338613368580643691303623270229201209584205\
        184540016221297602224920502091385620779268452045138092065191048228713446511588759776466334\
        955164279752771529006304592626662132102474121626192672652575831748945391640021506027583121\
        434323762213688659265886677223270051307283677840267588527026126262046233048278337730934597\
        251048975207110619022526431266585290657052824558964508055413035731525022451056993474245131\
        940271984820050179010512183653933500150867883341285338203943259891864588601821281032378193\
        761352726418609752619573339217755306901824708807431947047706039526585252022757276732244053\
        528355676567827486661805285681843895340144818064306646936344654007472918717723921069499452\
        62293257821840527730356884864031512332566992",
        "0x0.906eba8214db688d71d48a7f6bfec3441409a0ebac3e751739a15830cce620b0c0759cf859270f1140c03\
        6096cc79aebbd1f4eee48e1ca7874f76f877ffec25152561dcc244dc65e9c22f47f7b7fb57c9522f2f93e16b2a\
        3d27a3282dada7316eb9feb2436f2f272ac2c88bbba81b1c750754b409e94d32d18d3e91adcff6c039ad39cf86\
        6dd09628cd2681847e759247bb68d0db3217d6928d2c52986a1ef82fd6bde19d4863cd81de6bb94f13db14d3ac\
        25186f4f23712538d461ea97b001e73157a8e5fd0894dfd79bea110cd0c886c16e0cc2d71f2331782fe3af6af3\
        8d95ebaf4bdcaf407f127fcee21139e560f41988fba498490a2f046bd5d928060833f81efeffc9b1f476e77a5c\
        560d16d31aff355b18658c49dfffbf2b24ebd85570ea4e3551e6045d0a36dcb1e46fe9121958e5e0085a461d2e\
        69efd1284fb5f445a03e882c983dc91ec8ffd8edb30c57e8ad4952dd036641d7d5223048f5567aa5a9178ca2e8\
        c8f26d66d0899c3cba579d1a1fbe71b4cabf7ee1c212c2987e7c81ba09dd83fd1b82dc9a5d640a2bb4106903a0\
        5532629934b6622adbce3256ae672fffa96cd508a764ff883a76e3184d13014db85491972cf555280d50565251\
        3b94139e1397635d9a38584adbe3a3d177cf1e310732f87b24661689d25a18170cbd38f4766de0088c2c050e08\
        d1a0e3c9a6988eb9f9a25c2e839623d90f4226b5c965b39c56a5777a8a4e07a5f76e5dac5d468d209c0346efe2\
        57ef980d1deaf815ccd5e0661a456ebeaf04e66aad8ed6462a7117e127986462672d4372a902f7db4341aeac82\
        533af16409add2cb38cfdd50cdd3c49336dbd8d5a6d9b58e5a095a6af709feef8bb46de1a3e9536a629b5d6487\
        ffff1ab63baf70322c7216ba26b50b76a8d62074f1019a114b1ebbd2a2fc5ad771751d9be11eda7a826a3dcc36\
        516aa1d5bf8ac7936a37a25d41c716753bba34924d7fb957eb8baad78e7560fd3d6864629b76a7959212a2c2c1\
        9e83efb12a9f58ffffe0e879efbb8a0c0567130c689128e7f54f353052ff8436c6696bd07a1748582a5756e413\
        401db988220bc7b3b589f914a6cbdccf698c70552afd06a795b55b0ce2bb849e6704291c36f27aa171cee7eae5\
        a646b9b49e5a8fe668b1fbe87c2a9c3a09bebd4e81c43620dbd1a3d4418f0d6ea7c30ea4749dbded9a534c38ea\
        7457b0a410276136368d0aa59998f7ef31f091494d36e75bbc31316a0b4a715d23ed230545370985cc1222f507\
        22da00753753538227128b1aa8de13ca2840e9a229209879f8f4687e0510d2dad16e5431258280d050039bf530\
        2194770c6354c1c43f61d1192f579f9440ca53f792c8bbffc33132f201844b7e8dafbab89aeda56c8b23b471de\
        02baaebbae4a33608e0d4e6ce9c02fe99164d5ffaa134ff9246ae00f4cc6d6560cad52a48ba21174a58bbaf3ac\
        19e64f7c8282a70460a445d41d2fc237829e680bb1795b47a64cbc7620d29fc77362dc8769613f41fd347815e5\
        28b411c31003911777199ab2f233e757e919885780c6ec05d414c0c5c164f4b12e388f4383cfe534a6ede9fddf\
        b68089f73397e212e0df99ff496d43a9f35abb5e03f8b7a9c0d6b0afe2ec113c6918476e1f63429f60e3910430\
        7aeaf0e9c06c51f49e31cafa107a56947bed2ac19415640ce6db45372d33b0f0d5c87c6bc4c0e0fd0a81da36fd\
        bc11afa300997175943be0fe51f3d0b221b9d99cb1aa10f1c4761e8be0cf7920bfa3b689d53#10000",
        Less,
    );

    let one_over_sqrt_pi_f32 = Float::one_over_sqrt_pi_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(one_over_sqrt_pi_f32.to_string(), "0.56418961");
    assert_eq!(to_hex_string(&one_over_sqrt_pi_f32), "0x0.906ebb#24");
    assert_eq!(one_over_sqrt_pi_f32, f32::ONE_OVER_SQRT_PI);

    let one_over_sqrt_pi_f64 = Float::one_over_sqrt_pi_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(one_over_sqrt_pi_f64.to_string(), "0.5641895835477563");
    assert_eq!(
        to_hex_string(&one_over_sqrt_pi_f64),
        "0x0.906eba8214db68#53"
    );
    assert_eq!(one_over_sqrt_pi_f64, f64::ONE_OVER_SQRT_PI);
}

#[test]
#[should_panic]
fn one_over_sqrt_pi_prec_fail_1() {
    Float::one_over_sqrt_pi_prec(0);
}

fn test_one_over_sqrt_pi_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::one_over_sqrt_pi_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = one_over_sqrt_pi_prec_round_simple(prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_one_over_sqrt_pi_prec_round() {
    test_one_over_sqrt_pi_prec_round_helper(1, Floor, "0.5", "0x0.8#1", Less);
    test_one_over_sqrt_pi_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_one_over_sqrt_pi_prec_round_helper(1, Down, "0.5", "0x0.8#1", Less);
    test_one_over_sqrt_pi_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_one_over_sqrt_pi_prec_round_helper(1, Nearest, "0.5", "0x0.8#1", Less);

    test_one_over_sqrt_pi_prec_round_helper(2, Floor, "0.5", "0x0.8#2", Less);
    test_one_over_sqrt_pi_prec_round_helper(2, Ceiling, "0.8", "0x0.c#2", Greater);
    test_one_over_sqrt_pi_prec_round_helper(2, Down, "0.5", "0x0.8#2", Less);
    test_one_over_sqrt_pi_prec_round_helper(2, Up, "0.8", "0x0.c#2", Greater);
    test_one_over_sqrt_pi_prec_round_helper(2, Nearest, "0.5", "0x0.8#2", Less);

    test_one_over_sqrt_pi_prec_round_helper(3, Floor, "0.5", "0x0.8#3", Less);
    test_one_over_sqrt_pi_prec_round_helper(3, Ceiling, "0.6", "0x0.a#3", Greater);
    test_one_over_sqrt_pi_prec_round_helper(3, Down, "0.5", "0x0.8#3", Less);
    test_one_over_sqrt_pi_prec_round_helper(3, Up, "0.6", "0x0.a#3", Greater);
    test_one_over_sqrt_pi_prec_round_helper(3, Nearest, "0.6", "0x0.a#3", Greater);

    test_one_over_sqrt_pi_prec_round_helper(4, Floor, "0.56", "0x0.9#4", Less);
    test_one_over_sqrt_pi_prec_round_helper(4, Ceiling, "0.62", "0x0.a#4", Greater);
    test_one_over_sqrt_pi_prec_round_helper(4, Down, "0.56", "0x0.9#4", Less);
    test_one_over_sqrt_pi_prec_round_helper(4, Up, "0.62", "0x0.a#4", Greater);
    test_one_over_sqrt_pi_prec_round_helper(4, Nearest, "0.56", "0x0.9#4", Less);

    test_one_over_sqrt_pi_prec_round_helper(5, Floor, "0.56", "0x0.90#5", Less);
    test_one_over_sqrt_pi_prec_round_helper(5, Ceiling, "0.59", "0x0.98#5", Greater);
    test_one_over_sqrt_pi_prec_round_helper(5, Down, "0.56", "0x0.90#5", Less);
    test_one_over_sqrt_pi_prec_round_helper(5, Up, "0.59", "0x0.98#5", Greater);
    test_one_over_sqrt_pi_prec_round_helper(5, Nearest, "0.56", "0x0.90#5", Less);

    test_one_over_sqrt_pi_prec_round_helper(6, Floor, "0.56", "0x0.90#6", Less);
    test_one_over_sqrt_pi_prec_round_helper(6, Ceiling, "0.58", "0x0.94#6", Greater);
    test_one_over_sqrt_pi_prec_round_helper(6, Down, "0.56", "0x0.90#6", Less);
    test_one_over_sqrt_pi_prec_round_helper(6, Up, "0.58", "0x0.94#6", Greater);
    test_one_over_sqrt_pi_prec_round_helper(6, Nearest, "0.56", "0x0.90#6", Less);

    test_one_over_sqrt_pi_prec_round_helper(7, Floor, "0.56", "0x0.90#7", Less);
    test_one_over_sqrt_pi_prec_round_helper(7, Ceiling, "0.57", "0x0.92#7", Greater);
    test_one_over_sqrt_pi_prec_round_helper(7, Down, "0.56", "0x0.90#7", Less);
    test_one_over_sqrt_pi_prec_round_helper(7, Up, "0.57", "0x0.92#7", Greater);
    test_one_over_sqrt_pi_prec_round_helper(7, Nearest, "0.56", "0x0.90#7", Less);

    test_one_over_sqrt_pi_prec_round_helper(8, Floor, "0.562", "0x0.90#8", Less);
    test_one_over_sqrt_pi_prec_round_helper(8, Ceiling, "0.566", "0x0.91#8", Greater);
    test_one_over_sqrt_pi_prec_round_helper(8, Down, "0.562", "0x0.90#8", Less);
    test_one_over_sqrt_pi_prec_round_helper(8, Up, "0.566", "0x0.91#8", Greater);
    test_one_over_sqrt_pi_prec_round_helper(8, Nearest, "0.562", "0x0.90#8", Less);

    test_one_over_sqrt_pi_prec_round_helper(9, Floor, "0.562", "0x0.900#9", Less);
    test_one_over_sqrt_pi_prec_round_helper(9, Ceiling, "0.564", "0x0.908#9", Greater);
    test_one_over_sqrt_pi_prec_round_helper(9, Down, "0.562", "0x0.900#9", Less);
    test_one_over_sqrt_pi_prec_round_helper(9, Up, "0.564", "0x0.908#9", Greater);
    test_one_over_sqrt_pi_prec_round_helper(9, Nearest, "0.564", "0x0.908#9", Greater);

    test_one_over_sqrt_pi_prec_round_helper(10, Floor, "0.563", "0x0.904#10", Less);
    test_one_over_sqrt_pi_prec_round_helper(10, Ceiling, "0.564", "0x0.908#10", Greater);
    test_one_over_sqrt_pi_prec_round_helper(10, Down, "0.563", "0x0.904#10", Less);
    test_one_over_sqrt_pi_prec_round_helper(10, Up, "0.564", "0x0.908#10", Greater);
    test_one_over_sqrt_pi_prec_round_helper(10, Nearest, "0.564", "0x0.908#10", Greater);

    test_one_over_sqrt_pi_prec_round_helper(
        100,
        Floor,
        "0.56418958354775628694807945156",
        "0x0.906eba8214db688d71d48a7f6#100",
        Less,
    );
    test_one_over_sqrt_pi_prec_round_helper(
        100,
        Ceiling,
        "0.564189583547756286948079451561",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test_one_over_sqrt_pi_prec_round_helper(
        100,
        Down,
        "0.56418958354775628694807945156",
        "0x0.906eba8214db688d71d48a7f6#100",
        Less,
    );
    test_one_over_sqrt_pi_prec_round_helper(
        100,
        Up,
        "0.564189583547756286948079451561",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
    test_one_over_sqrt_pi_prec_round_helper(
        100,
        Nearest,
        "0.564189583547756286948079451561",
        "0x0.906eba8214db688d71d48a7f7#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn one_over_sqrt_pi_prec_round_fail_1() {
    Float::one_over_sqrt_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn one_over_sqrt_pi_prec_round_fail_2() {
    Float::one_over_sqrt_pi_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn one_over_sqrt_pi_prec_round_fail_3() {
    Float::one_over_sqrt_pi_prec_round(1000, Exact);
}

#[test]
fn one_over_sqrt_pi_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (one_over_sqrt_pi, o) = Float::one_over_sqrt_pi_prec(prec);
        assert!(one_over_sqrt_pi.is_valid());
        assert_eq!(one_over_sqrt_pi.get_prec(), Some(prec));
        assert_eq!(one_over_sqrt_pi.get_exponent(), Some(0));
        assert_ne!(o, Equal);
        if o == Less {
            let (one_over_sqrt_pi_alt, o_alt) = Float::one_over_sqrt_pi_prec_round(prec, Ceiling);
            let mut next_upper = one_over_sqrt_pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(one_over_sqrt_pi_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !one_over_sqrt_pi.is_power_of_2() {
            let (one_over_sqrt_pi_alt, o_alt) = Float::one_over_sqrt_pi_prec_round(prec, Floor);
            let mut next_lower = one_over_sqrt_pi.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(one_over_sqrt_pi_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (one_over_sqrt_pi_alt, o_alt) = Float::one_over_sqrt_pi_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&one_over_sqrt_pi_alt),
            ComparableFloatRef(&one_over_sqrt_pi)
        );
        assert_eq!(o_alt, o);

        let (one_over_sqrt_pi_alt, o_alt) = one_over_sqrt_pi_prec_round_simple(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&one_over_sqrt_pi_alt),
            ComparableFloatRef(&one_over_sqrt_pi)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn one_over_sqrt_pi_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (one_over_sqrt_pi, o) = Float::one_over_sqrt_pi_prec_round(prec, rm);
        assert!(one_over_sqrt_pi.is_valid());
        assert_eq!(one_over_sqrt_pi.get_prec(), Some(prec));
        assert_eq!(
            one_over_sqrt_pi.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                1
            } else {
                0
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (one_over_sqrt_pi_alt, o_alt) = Float::one_over_sqrt_pi_prec_round(prec, Ceiling);
            let mut next_upper = one_over_sqrt_pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(one_over_sqrt_pi_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !one_over_sqrt_pi.is_power_of_2() {
            let (one_over_sqrt_pi_alt, o_alt) = Float::one_over_sqrt_pi_prec_round(prec, Floor);
            let mut next_lower = one_over_sqrt_pi.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(one_over_sqrt_pi_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }

        let (one_over_sqrt_pi_alt, o_alt) = one_over_sqrt_pi_prec_round_simple(prec, rm);
        assert_eq!(
            ComparableFloatRef(&one_over_sqrt_pi_alt),
            ComparableFloatRef(&one_over_sqrt_pi)
        );
        assert_eq!(o_alt, o);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::one_over_sqrt_pi_prec_round(prec, Exact));
    });

    test_constant(Float::one_over_sqrt_pi_prec_round, 10000);
}
