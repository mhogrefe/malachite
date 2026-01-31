// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::LemniscateConstant;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::constants::lemniscate_constant::lemniscate_constant_prec_round_simple;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_lemniscate_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::lemniscate_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = lemniscate_constant_prec_round_simple(prec, Nearest);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_lemniscate_constant_prec() {
    test_lemniscate_constant_prec_helper(1, "2.0", "0x2.0#1", Less);
    test_lemniscate_constant_prec_helper(2, "3.0", "0x3.0#2", Greater);
    test_lemniscate_constant_prec_helper(3, "2.5", "0x2.8#3", Less);
    test_lemniscate_constant_prec_helper(4, "2.5", "0x2.8#4", Less);
    test_lemniscate_constant_prec_helper(5, "2.6", "0x2.a#5", Greater);
    test_lemniscate_constant_prec_helper(6, "2.62", "0x2.a#6", Greater);
    test_lemniscate_constant_prec_helper(7, "2.62", "0x2.a0#7", Greater);
    test_lemniscate_constant_prec_helper(8, "2.62", "0x2.a0#8", Greater);
    test_lemniscate_constant_prec_helper(9, "2.625", "0x2.a0#9", Greater);
    test_lemniscate_constant_prec_helper(10, "2.621", "0x2.9f#10", Less);
    test_lemniscate_constant_prec_helper(
        100,
        "2.62205755429211981046483958989",
        "0x2.9f3f29f3ea160dcf30eed7580#100",
        Less,
    );
    test_lemniscate_constant_prec_helper(
        1000,
        "2.622057554292119810464839589891119413682754951431623162816821703800790587070414250230295\
        532961429093446135752671783218055608956901393935694701119434775235840422641497164906951936\
        899979932146072383121390810206221897429600856554539772305369549710288888325526487021329012\
        0975408331285685117297522292142966",
        "0x2.9f3f29f3ea160dcf30eed75811b3c2f6d7172e48e43735af6b46c7af912d10e70017b44f6b10c20e6755e\
        81d92714d02ce28c9bbc0133ee370554ff5165f91aae132697ee64025bd01da354b8da35c1ca7832784a9965a0\
        c14398dc6fb92c5154e84157fa74aec0c44dfda20393fe782139b91b083fc85b1441ac82ad0#1000",
        Less,
    );
    test_lemniscate_constant_prec_helper(
        10000,
        "2.622057554292119810464839589891119413682754951431623162816821703800790587070414250230295\
        532961429093446135752671783218055608956901393935694701119434775235840422641497164906951936\
        899979932146072383121390810206221897429600856554539772305369549710288888325526487021329012\
        097540833128568511729752229214296692430513968456455539432881415381331735108409226312132476\
        667633414509988603429421479224714487963907872564189521811027252516629964333384660679333635\
        093139808526237739409142626489848034804572541477046175421256342129955863129980224054609012\
        091499139897885645312480971101149665075060542093841723886900040274785389625483030580303946\
        324783219558325522973037191341918983592199914229536672569106861130938134980725552913015093\
        730332611087045814240765781886530766932476940761626721636249549480066760961388122322476925\
        591018705775743614648912879832686662030731373313562107612636379245785801781364105361306093\
        563472025022592312041202668270457723044608378953311357002940577442011806826257962983642671\
        092116198597298460755620828986569905715850585969733482498916979139661512076092673544847646\
        574626457769227556279085403488486299631791644074183032890572244174725314174445875123605657\
        139575363059725889282920861276365948025081705994780553810740323485381958708594579092174018\
        668477776969917734589963845805236673259560797055893212070139745771343010329460928029259114\
        281387388493544876912962400450451515749437725842216728515086072450581997727624380905274299\
        795351792018910957399605991273238903791116648437850911342937302632750071201301892980462180\
        428765649352238455214584305436298513850108631443435366399087818335289725057991586901139218\
        731261235482234682842959686621093089202655982151707327457126997244722591053622446697155585\
        523244205556551378985416014007281696154803764953004010586969769798728528544268595900874210\
        615976672889856196920646383327838417979669834893047455706005883206470466312323973364023277\
        796282492416941692491715034784541889387264509022763917623129944975660994166520078299733003\
        514497680346375538989464128500976201473352363956758954456057294389704660492210734900562422\
        060698049424698227273322272545152596399605261093570054581266506375784426587891009902248739\
        074851475335571668858945648604938903484149255083213830273958314785697922542663980952768924\
        339176421212403458061264319698865734211695958669092740058571756733772940260843870335168806\
        554093029896061847165969615210879733103894090573687904026615017392722182113253614379462748\
        410822722172415291174195706198674247582236389438854755351468899984473586928701255109272056\
        897315524697141375285481897383672357481534008060452330772039824202794480385098376136184356\
        879523471104690616561399028704017447178011844138034863898042145300962033247526574931328554\
        943387329065193422836581434847505032876745487649716644131511122812960672857587623568606457\
        132192794260190549677753958426178724331815736858905292945584979487114184373394133441249801\
        511678382932856516301228572505799118357873322849053225342038152978949939297661715113526220\
        8853282485258038240456468938147672657060189",
        "0x2.9f3f29f3ea160dcf30eed75811b3c2f6d7172e48e43735af6b46c7af912d10e70017b44f6b10c20e6755e\
        81d92714d02ce28c9bbc0133ee370554ff5165f91aae132697ee64025bd01da354b8da35c1ca7832784a9965a0\
        c14398dc6fb92c5154e84157fa74aec0c44dfda20393fe782139b91b083fc85b1441ac82ad11541d32c7d05a72\
        3a95453cb09b15964d9b9e2bc05b3b7d29877c9a4b44216c4a2dda21dc33dd433090e44a06c3c1f481e075fd94\
        21ecc82d065fe31873cf84d0a6f66b94a68ca0edf58ab7118c279921d8f0b918ce81b4b6f957b508ec630ae014\
        066fb52b9825f76ccb6bc78c38c4492c0220c805403b6d28ebc28cff3c6b354eed64d86b282b6c5045ee4a48d2\
        39d222d69aa29ebba7e23d6ca59093bf052e76d6da8e7567ecbf817d2c1225ec79a3ca30176f7278fe02993af2\
        4f6729cf38a835f6546826d4845bb03bd57075dcb0902f9a09225354aa620358678e3510c30ed57970038d983e\
        b399d694188f2bd2a3cdd0195478f25df26f7f6d941552566f3efbb6f49cd4a9ffe631175ae021fa25809ab898\
        26df98a555338ac2029fc9ee45c5099d7284ef412f878425791628a818f7c826883b3ad7198d30c5f83a7738d7\
        1de0a3310c7cc5e889941fc475a5d9145798b2ecc4c57c6ab448267648a64942b2ae1b4a2b8f2735acdc0e72e6\
        826ee5b166c4c05d874f61c2a55ce9d1ebafc1c0375a43b0df0189b326566e625334856a6d43bc6256ac22f546\
        6d302d1d2242310bf0118124c9cb1bcc69be3a951b4f9f86c419494311bfc1fb0f2d6dd5768703d859fcfe1b4d\
        6b959e9ee6767cf7a152f34269e166cf850fc2d0278f0ba25bf64ed7cfe799d7a324cf238084d10ec8dc667d8f\
        92403b94640db67e3c63473f683fe6a3881b3997069fe1ad958a5433e7720102a273512d2b28d4a46699a3cb1f\
        42532f37658965357b95e149d90c83c02453c15517ecbb80c88df5b1a8e0ee86c0d4d082758058cb2664c62dae\
        f9ed89bd69c0d6f66381e3f93d7474e266a2bf1dc97c19464cbbd0793bd74674dc03e28a2ab5e1d50da6fc8b09\
        76730ce30603718f3072e1081edfb2dbe178162099ea48bf81c3341f1421f77008833a0b30ad6cdad21d2beb60\
        e79f8c891d70c88d7fcf6dfca38032e48280e8d3f40c5a0ab27dc76c32113d090c4686e0c5af335ee8e02bf167\
        565f64587b00b6eeb163f782ee0f30a13c6386b94732d3d795261b00f328f20061e7dd29e5ba1649815e8b86f8\
        749cf82d7ae468a30fc75a44213e70e017de72b2f73bd8698855f4d0d0806980ddd6d3a2d5257165fd1f73ea29\
        d2c6b232528aa4f54a711deccfbe0a41aad688b4c077d9d7bce4d8d4bf0a22d61a55d4f97f147c6f45f5d03023\
        4fd8ba98f9dec471598ea5fa3f0f35ef0e006bcbfa54050e6e4ca5d8734c24cda360ea64146d27c81d0e05d61d\
        577cf76b8b4ed519262488699e7a5a0838dbc88c3be2fc7c488d855b0f9ccfe111091777beb081c1ea5ee0853b\
        3ae2d4ea046a292e33e9ab99d3734355a7bb39462571a408ad450e83de0ee52fdb5aa8d38706117b2c87b03ab5\
        868eff0e9c6c3ab4afce9ddca3db9b0c66305ca71fd6a7842fdb2dc86990eb4bc0b7fde91f503e8788a772ab33\
        41473c80c1503eeca8c1534d984be33f86f904239944078141f06fe1dee3e1eaae3cb2f1984df30e54f0dc6c3c\
        ac27892c1ee783c1d26589418270060cdb2077a890b369215342641df45d1210b1cc551b780#10000",
        Greater,
    );

    let lemniscate_constant_f32 =
        Float::lemniscate_constant_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(lemniscate_constant_f32.to_string(), "2.6220574");
    assert_eq!(to_hex_string(&lemniscate_constant_f32), "0x2.9f3f28#24");
    assert_eq!(lemniscate_constant_f32, f32::LEMNISCATE_CONSTANT);

    let lemniscate_constant_f64 =
        Float::lemniscate_constant_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(lemniscate_constant_f64.to_string(), "2.6220575542921196");
    assert_eq!(
        to_hex_string(&lemniscate_constant_f64),
        "0x2.9f3f29f3ea160#53"
    );
    assert_eq!(lemniscate_constant_f64, f64::LEMNISCATE_CONSTANT);
}

#[test]
#[should_panic]
fn lemniscate_constant_prec_fail_1() {
    Float::lemniscate_constant_prec(0);
}

fn test_lemniscate_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::lemniscate_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = lemniscate_constant_prec_round_simple(prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_lemniscate_constant_prec_round() {
    test_lemniscate_constant_prec_round_helper(1, Floor, "2.0", "0x2.0#1", Less);
    test_lemniscate_constant_prec_round_helper(1, Ceiling, "4.0", "0x4.0#1", Greater);
    test_lemniscate_constant_prec_round_helper(1, Down, "2.0", "0x2.0#1", Less);
    test_lemniscate_constant_prec_round_helper(1, Up, "4.0", "0x4.0#1", Greater);
    test_lemniscate_constant_prec_round_helper(1, Nearest, "2.0", "0x2.0#1", Less);

    test_lemniscate_constant_prec_round_helper(2, Floor, "2.0", "0x2.0#2", Less);
    test_lemniscate_constant_prec_round_helper(2, Ceiling, "3.0", "0x3.0#2", Greater);
    test_lemniscate_constant_prec_round_helper(2, Down, "2.0", "0x2.0#2", Less);
    test_lemniscate_constant_prec_round_helper(2, Up, "3.0", "0x3.0#2", Greater);
    test_lemniscate_constant_prec_round_helper(2, Nearest, "3.0", "0x3.0#2", Greater);

    test_lemniscate_constant_prec_round_helper(3, Floor, "2.5", "0x2.8#3", Less);
    test_lemniscate_constant_prec_round_helper(3, Ceiling, "3.0", "0x3.0#3", Greater);
    test_lemniscate_constant_prec_round_helper(3, Down, "2.5", "0x2.8#3", Less);
    test_lemniscate_constant_prec_round_helper(3, Up, "3.0", "0x3.0#3", Greater);
    test_lemniscate_constant_prec_round_helper(3, Nearest, "2.5", "0x2.8#3", Less);

    test_lemniscate_constant_prec_round_helper(4, Floor, "2.5", "0x2.8#4", Less);
    test_lemniscate_constant_prec_round_helper(4, Ceiling, "2.8", "0x2.c#4", Greater);
    test_lemniscate_constant_prec_round_helper(4, Down, "2.5", "0x2.8#4", Less);
    test_lemniscate_constant_prec_round_helper(4, Up, "2.8", "0x2.c#4", Greater);
    test_lemniscate_constant_prec_round_helper(4, Nearest, "2.5", "0x2.8#4", Less);

    test_lemniscate_constant_prec_round_helper(5, Floor, "2.5", "0x2.8#5", Less);
    test_lemniscate_constant_prec_round_helper(5, Ceiling, "2.6", "0x2.a#5", Greater);
    test_lemniscate_constant_prec_round_helper(5, Down, "2.5", "0x2.8#5", Less);
    test_lemniscate_constant_prec_round_helper(5, Up, "2.6", "0x2.a#5", Greater);
    test_lemniscate_constant_prec_round_helper(5, Nearest, "2.6", "0x2.a#5", Greater);

    test_lemniscate_constant_prec_round_helper(6, Floor, "2.56", "0x2.9#6", Less);
    test_lemniscate_constant_prec_round_helper(6, Ceiling, "2.62", "0x2.a#6", Greater);
    test_lemniscate_constant_prec_round_helper(6, Down, "2.56", "0x2.9#6", Less);
    test_lemniscate_constant_prec_round_helper(6, Up, "2.62", "0x2.a#6", Greater);
    test_lemniscate_constant_prec_round_helper(6, Nearest, "2.62", "0x2.a#6", Greater);

    test_lemniscate_constant_prec_round_helper(7, Floor, "2.59", "0x2.98#7", Less);
    test_lemniscate_constant_prec_round_helper(7, Ceiling, "2.62", "0x2.a0#7", Greater);
    test_lemniscate_constant_prec_round_helper(7, Down, "2.59", "0x2.98#7", Less);
    test_lemniscate_constant_prec_round_helper(7, Up, "2.62", "0x2.a0#7", Greater);
    test_lemniscate_constant_prec_round_helper(7, Nearest, "2.62", "0x2.a0#7", Greater);

    test_lemniscate_constant_prec_round_helper(8, Floor, "2.61", "0x2.9c#8", Less);
    test_lemniscate_constant_prec_round_helper(8, Ceiling, "2.62", "0x2.a0#8", Greater);
    test_lemniscate_constant_prec_round_helper(8, Down, "2.61", "0x2.9c#8", Less);
    test_lemniscate_constant_prec_round_helper(8, Up, "2.62", "0x2.a0#8", Greater);
    test_lemniscate_constant_prec_round_helper(8, Nearest, "2.62", "0x2.a0#8", Greater);

    test_lemniscate_constant_prec_round_helper(9, Floor, "2.617", "0x2.9e#9", Less);
    test_lemniscate_constant_prec_round_helper(9, Ceiling, "2.625", "0x2.a0#9", Greater);
    test_lemniscate_constant_prec_round_helper(9, Down, "2.617", "0x2.9e#9", Less);
    test_lemniscate_constant_prec_round_helper(9, Up, "2.625", "0x2.a0#9", Greater);
    test_lemniscate_constant_prec_round_helper(9, Nearest, "2.625", "0x2.a0#9", Greater);

    test_lemniscate_constant_prec_round_helper(10, Floor, "2.621", "0x2.9f#10", Less);
    test_lemniscate_constant_prec_round_helper(10, Ceiling, "2.625", "0x2.a0#10", Greater);
    test_lemniscate_constant_prec_round_helper(10, Down, "2.621", "0x2.9f#10", Less);
    test_lemniscate_constant_prec_round_helper(10, Up, "2.625", "0x2.a0#10", Greater);
    test_lemniscate_constant_prec_round_helper(10, Nearest, "2.621", "0x2.9f#10", Less);

    test_lemniscate_constant_prec_round_helper(
        100,
        Floor,
        "2.62205755429211981046483958989",
        "0x2.9f3f29f3ea160dcf30eed7580#100",
        Less,
    );
    test_lemniscate_constant_prec_round_helper(
        100,
        Ceiling,
        "2.622057554292119810464839589893",
        "0x2.9f3f29f3ea160dcf30eed7584#100",
        Greater,
    );
    test_lemniscate_constant_prec_round_helper(
        100,
        Down,
        "2.62205755429211981046483958989",
        "0x2.9f3f29f3ea160dcf30eed7580#100",
        Less,
    );
    test_lemniscate_constant_prec_round_helper(
        100,
        Up,
        "2.622057554292119810464839589893",
        "0x2.9f3f29f3ea160dcf30eed7584#100",
        Greater,
    );
    test_lemniscate_constant_prec_round_helper(
        100,
        Nearest,
        "2.62205755429211981046483958989",
        "0x2.9f3f29f3ea160dcf30eed7580#100",
        Less,
    );
}

#[test]
#[should_panic]
fn lemniscate_constant_prec_round_fail_1() {
    Float::lemniscate_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn lemniscate_constant_prec_round_fail_2() {
    Float::lemniscate_constant_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn lemniscate_constant_prec_round_fail_3() {
    Float::lemniscate_constant_prec_round(1000, Exact);
}

#[test]
fn lemniscate_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (lemniscate_constant, o) = Float::lemniscate_constant_prec(prec);
        assert!(lemniscate_constant.is_valid());
        assert_eq!(lemniscate_constant.get_prec(), Some(prec));
        assert_eq!(lemniscate_constant.get_exponent(), Some(2));
        assert_ne!(o, Equal);
        if o == Less {
            let (lemniscate_constant_alt, o_alt) =
                Float::lemniscate_constant_prec_round(prec, Ceiling);
            let mut next_upper = lemniscate_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(lemniscate_constant_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !lemniscate_constant.is_power_of_2() {
            let (lemniscate_constant_alt, o_alt) =
                Float::lemniscate_constant_prec_round(prec, Floor);
            let mut next_lower = lemniscate_constant.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(lemniscate_constant_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (lemniscate_constant_alt, o_alt) = Float::lemniscate_constant_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&lemniscate_constant_alt),
            ComparableFloatRef(&lemniscate_constant)
        );
        assert_eq!(o_alt, o);

        let (lemniscate_constant_alt, o_alt) = lemniscate_constant_prec_round_simple(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&lemniscate_constant_alt),
            ComparableFloatRef(&lemniscate_constant)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn lemniscate_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (lemniscate_constant, o) = Float::lemniscate_constant_prec_round(prec, rm);
        assert!(lemniscate_constant.is_valid());
        assert_eq!(lemniscate_constant.get_prec(), Some(prec));
        assert_eq!(
            lemniscate_constant.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                3
            } else {
                2
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (lemniscate_constant_alt, o_alt) =
                Float::lemniscate_constant_prec_round(prec, Ceiling);
            let mut next_upper = lemniscate_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(lemniscate_constant_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !lemniscate_constant.is_power_of_2() {
            let (lemniscate_constant_alt, o_alt) =
                Float::lemniscate_constant_prec_round(prec, Floor);
            let mut next_lower = lemniscate_constant.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(lemniscate_constant_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }

        let (lemniscate_constant_alt, o_alt) = lemniscate_constant_prec_round_simple(prec, rm);
        assert_eq!(
            ComparableFloatRef(&lemniscate_constant_alt),
            ComparableFloatRef(&lemniscate_constant)
        );
        assert_eq!(o_alt, o);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::lemniscate_constant_prec_round(prec, Exact));
    });

    test_constant(Float::lemniscate_constant_prec_round, 10000);
}
