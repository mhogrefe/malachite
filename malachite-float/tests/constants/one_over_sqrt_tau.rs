// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::OneOverSqrtTau;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::constants::one_over_sqrt_tau::one_over_sqrt_tau_prec_round_simple;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_one_over_sqrt_tau_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::one_over_sqrt_tau_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = one_over_sqrt_tau_prec_round_simple(prec, Nearest);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_one_over_sqrt_tau_prec() {
    test_one_over_sqrt_tau_prec_helper(1, "0.5", "0x0.8#1", Greater);
    test_one_over_sqrt_tau_prec_helper(2, "0.4", "0x0.6#2", Less);
    test_one_over_sqrt_tau_prec_helper(3, "0.38", "0x0.6#3", Less);
    test_one_over_sqrt_tau_prec_helper(4, "0.41", "0x0.68#4", Greater);
    test_one_over_sqrt_tau_prec_helper(5, "0.41", "0x0.68#5", Greater);
    test_one_over_sqrt_tau_prec_helper(6, "0.4", "0x0.66#6", Less);
    test_one_over_sqrt_tau_prec_helper(7, "0.398", "0x0.66#7", Less);
    test_one_over_sqrt_tau_prec_helper(8, "0.398", "0x0.660#8", Less);
    test_one_over_sqrt_tau_prec_helper(9, "0.399", "0x0.664#9", Greater);
    test_one_over_sqrt_tau_prec_helper(10, "0.3989", "0x0.662#10", Less);
    test_one_over_sqrt_tau_prec_helper(
        100,
        "0.3989422804014326779399460599344",
        "0x0.662114cf50d942343f2cf14030#100",
        Greater,
    );
    test_one_over_sqrt_tau_prec_helper(
        1000,
        "0.398942280401432677939946059934381868475858631164934657665925829670657925899301838501252\
        333907306936430302558862635182685510991954555837242996212730625507706345270582720499317564\
        516345807530597253642732083669593478271702999186419063456032808933388606704653652796716869\
        34195477117721206532537536913347877",
        "0x0.662114cf50d942343f2cf1402eae38bfd3829f30512706d8c0471b4802639d2e9df8ac55447d3db723f5d\
        3abb374e88deb14277970be1513278e993445f08cff98282e1fd947ce4b4ecdbd6e65c23395cacb544de69eea3\
        2c276937b1bc567691c1cec4882c77aacefe1f6708329ab3fc8d2ab2435ee45c9150c426bbb8#1000",
        Greater,
    );
    test_one_over_sqrt_tau_prec_helper(
        10000,
        "0.398942280401432677939946059934381868475858631164934657665925829670657925899301838501252\
        333907306936430302558862635182685510991954555837242996212730625507706345270582720499317564\
        516345807530597253642732083669593478271702999186419063456032808933388606704653652796716869\
        341954771177212065325375369133478750560424055704884258180482317903772804997176338575363992\
        839140318693283694771754858239775054447927761155070412703969672485047337603814813923901300\
        564676023356305570085700726641100015721563953577823123410952609069269089244567245554672105\
        743928915256735109303850680783183519806551964687438189980165959781887721458861617459900501\
        712960940366313293846201865045309966814316491432421060417455294539282219688799792718106125\
        413701644536367652874648406122597740302757632013709422194511725465470758442141422502838061\
        868594135257554774549801530578349147613022007422892027821093302633276582742943413612643384\
        980057963587894437275171155013545859889393745518894340738320491519829619307071761750803329\
        086547364282269194590675379988171293765563498747919820220506439472756588960162518241653865\
        682958549834199174484028901472961826557349795720893467152719051181680313262827229623899368\
        921510110599838914400198674472347351963204746387291915197608912260762509601476407511767145\
        433601016214971648007948262922613392149108721308143700798328558749768734347897980278300510\
        262826071460132831874309251391448950500864351371232094749134907629411222727979591421075299\
        673747197316387505002135776055200631744235271327518724616860849157649748825471682030207497\
        062098265106007658380606023541897775864990099263845899892122682397288552012228221816314178\
        308158121229466255828963270625452661691131034742094301703313846663457879729080007652468485\
        334179957678287388085633885615200206213603246796710415477475796066423508536474271747792912\
        434828025073907614687889180477807434263307378988209019313548164988087811605658554801862326\
        665697388456830642702724077905143312033897044455936044032662126236766581278672374404901166\
        564541339777091280689982201428151967506588958639140595771858450705515561437105861118338079\
        716914044380378761129991061804988776438561675358245271277544532102159485888359954032827725\
        246069745445363049745582651877816164759265756297881307521577254233707200857137834531266312\
        782357208128645404040264033722898206313179116571251682514874032991214269126845955003622262\
        315257794830645021004783385421309666323873108778765242160746712375568508761910339254792109\
        211573641128195708927088639062119252851547796326120921969626873693129996429236958310270565\
        733834386378238052071363813152698866481153604716886972531676903101376207010580993776843055\
        041892765565102116589831129651509557052479656507203268313538993083521906924099338259623407\
        406961415103750340216958051666093814503142437034516561361464323353515610162799624281647911\
        127783108359666320819610363671588870086956238204933545154454220560124064024272600360105966\
        644223437056162524970715123219160940261473588761105521231222473362842356480453287247436537\
        04133401719034564232591496298172511220088055",
        "0x0.662114cf50d942343f2cf1402eae38bfd3829f30512706d8c0471b4802639d2e9df8ac55447d3db723f5d\
        3abb374e88deb14277970be1513278e993445f08cff98282e1fd947ce4b4ecdbd6e65c23395cacb544de69eea3\
        2c276937b1bc567691c1cec4882c77aacefe1f6708329ab3fc8d2ab2435ee45c9150c426bbb49487e480d8928e\
        623122d2369599582a3983769e7329865c92193ed298857cb81ab87659565e91e51487afe60871d04a18e432e6\
        bff594c697ed53f9d0aaf01aa890d8b742ca9e5ec8b5ea95546cad1961020feb8ed1ea218282c79079f714f4a0\
        bfda67f0e5ab81ec651ef4167fcf0e311b0d6b17578cbb5c65b311eb9071956748defa26992503a86e8300c9f5\
        082467be471e1dec4e7e1e9180296a90caadaed32e1972f7d51f05d9bcb5a7fe9f37a2d620cd24a3f28575328d\
        854f7a09661928fa57a78ad7d45d31fdb6377ddd63a6b7ce9fcd58c0ea0d7d36a37179fa5dd9215dde6682b2d8\
        6a8158f30670945f1eede2fd6ca404d7795a848aa442b3106116c7e9a212cdf14506bbc1ac9ab8d9b98bfd767b\
        d58e4a529fcb015285a0779f53c3be9533137bac5c5f90226a7534c7a819995ffae2499426d04682ef71cbbb0b\
        0e3261ce51b1494f21861a1750090850f2ae343d0a6e302b563eac32d234a07be481f2e2f16df62fb78e9e342d\
        6d3029d2c37a03cbaae2fdc23728ea3136432daed576883b9fe332b136f979cb9de719361019e6e62aa7893d43\
        6a8e51246e9378a70b92afcc9323b414f4de095326a8ef23d71049f3e05b326ad299d22fc0bc8b809450180fef\
        478554821b4ee6e0357f88b14350f4e77386b505ad7c8fa32a5b42163de45cf7b41ad63c4aed86175ef1028a0d\
        932b739351783a42c771c6967ea9212f2c8e86616639d9705a45299031b722cafe8f6042a968f1e5aee9c73c6b\
        a50c6af3a2598b0e128dbf85be2f3a60df9fb6f48a8ee7162d7013b3c1ca232deae1b39f0d797180f7a6a752a6\
        83690e26925c1b164a07f4ac2cb0c0ad0145ef80be80744063f6bc1ef94fc58e85283f61fa89f784b8d0679fc2\
        3f82e80dc04e91054d8cb75f64dbcee3db4759684165e5c21e0a98b9e88456e5c5444a353a73991f4fef88ee23\
        9e8cb2fc7152d3bdb119b8edb56206b31171a7eb12fac6cfba71c9341ef50a6863b38ab43747cfd41e062104bc\
        48549100e7b85f76b1989f0f72ef87e9cb989045f8500dfc329819b44464fa01750b5c8911a7b4776be2e27016\
        1f2ac11cea7136bce9f185370fc22fb213c3cbc9eca4d85875674085728a8d475b0cb557c00062f22e0de84560\
        33321dd367fc84751878bb1ebf079740abebbc8adce82e6ed570f9bb90f741f47f1ce9619b5f354c15a51894ea\
        e8f37251e6f6c2f24f191e380b3cf8f9550968d0b3628248974046f1ed6013aee5bb2817edf5276d43f1c894dd\
        4f87a770f11619efb6f33d62f037de3612014ab96d3ba744922006bd15b4c0e248bfc854e1f4666a12ee7d5bf3\
        765d35191b53037aa8e989ca2868ff144c1bc7b6867eea49a234b963f36812f0119d5b379e2db3e5f1cbf7cbe9\
        8f92866e1120d3ba1ae76dfaa31dd7cfc7b651b490b8a434b8def7a99cb6c880eac6e66bf16ad72ef5492adcb4\
        3ca05e0bab3fae213ca98d72f11460f81ef2366721c30053b2f769cb43c96a42504e21e504c86f85c4916dba52\
        de2edd93889cfd32b8ac6a59cb54912ec0c01f3dd79eaaea0699c78d5e8b7f1b5234617b1d10#10000",
        Less,
    );

    let one_over_sqrt_tau_f32 = Float::one_over_sqrt_tau_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(one_over_sqrt_tau_f32.to_string(), "0.39894229");
    assert_eq!(to_hex_string(&one_over_sqrt_tau_f32), "0x0.6621150#24");
    assert_eq!(one_over_sqrt_tau_f32, f32::ONE_OVER_SQRT_TAU);

    let one_over_sqrt_tau_f64 = Float::one_over_sqrt_tau_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(one_over_sqrt_tau_f64.to_string(), "0.3989422804014327");
    assert_eq!(
        to_hex_string(&one_over_sqrt_tau_f64),
        "0x0.662114cf50d944#53"
    );
    assert_eq!(one_over_sqrt_tau_f64, f64::ONE_OVER_SQRT_TAU);
}

#[test]
#[should_panic]
fn one_over_sqrt_tau_prec_fail_1() {
    Float::one_over_sqrt_tau_prec(0);
}

fn test_one_over_sqrt_tau_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::one_over_sqrt_tau_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = one_over_sqrt_tau_prec_round_simple(prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_one_over_sqrt_tau_prec_round() {
    test_one_over_sqrt_tau_prec_round_helper(1, Floor, "0.2", "0x0.4#1", Less);
    test_one_over_sqrt_tau_prec_round_helper(1, Ceiling, "0.5", "0x0.8#1", Greater);
    test_one_over_sqrt_tau_prec_round_helper(1, Down, "0.2", "0x0.4#1", Less);
    test_one_over_sqrt_tau_prec_round_helper(1, Up, "0.5", "0x0.8#1", Greater);
    test_one_over_sqrt_tau_prec_round_helper(1, Nearest, "0.5", "0x0.8#1", Greater);

    test_one_over_sqrt_tau_prec_round_helper(2, Floor, "0.4", "0x0.6#2", Less);
    test_one_over_sqrt_tau_prec_round_helper(2, Ceiling, "0.5", "0x0.8#2", Greater);
    test_one_over_sqrt_tau_prec_round_helper(2, Down, "0.4", "0x0.6#2", Less);
    test_one_over_sqrt_tau_prec_round_helper(2, Up, "0.5", "0x0.8#2", Greater);
    test_one_over_sqrt_tau_prec_round_helper(2, Nearest, "0.4", "0x0.6#2", Less);

    test_one_over_sqrt_tau_prec_round_helper(3, Floor, "0.38", "0x0.6#3", Less);
    test_one_over_sqrt_tau_prec_round_helper(3, Ceiling, "0.44", "0x0.7#3", Greater);
    test_one_over_sqrt_tau_prec_round_helper(3, Down, "0.38", "0x0.6#3", Less);
    test_one_over_sqrt_tau_prec_round_helper(3, Up, "0.44", "0x0.7#3", Greater);
    test_one_over_sqrt_tau_prec_round_helper(3, Nearest, "0.38", "0x0.6#3", Less);

    test_one_over_sqrt_tau_prec_round_helper(4, Floor, "0.38", "0x0.60#4", Less);
    test_one_over_sqrt_tau_prec_round_helper(4, Ceiling, "0.41", "0x0.68#4", Greater);
    test_one_over_sqrt_tau_prec_round_helper(4, Down, "0.38", "0x0.60#4", Less);
    test_one_over_sqrt_tau_prec_round_helper(4, Up, "0.41", "0x0.68#4", Greater);
    test_one_over_sqrt_tau_prec_round_helper(4, Nearest, "0.41", "0x0.68#4", Greater);

    test_one_over_sqrt_tau_prec_round_helper(5, Floor, "0.39", "0x0.64#5", Less);
    test_one_over_sqrt_tau_prec_round_helper(5, Ceiling, "0.41", "0x0.68#5", Greater);
    test_one_over_sqrt_tau_prec_round_helper(5, Down, "0.39", "0x0.64#5", Less);
    test_one_over_sqrt_tau_prec_round_helper(5, Up, "0.41", "0x0.68#5", Greater);
    test_one_over_sqrt_tau_prec_round_helper(5, Nearest, "0.41", "0x0.68#5", Greater);

    test_one_over_sqrt_tau_prec_round_helper(6, Floor, "0.4", "0x0.66#6", Less);
    test_one_over_sqrt_tau_prec_round_helper(6, Ceiling, "0.406", "0x0.68#6", Greater);
    test_one_over_sqrt_tau_prec_round_helper(6, Down, "0.4", "0x0.66#6", Less);
    test_one_over_sqrt_tau_prec_round_helper(6, Up, "0.406", "0x0.68#6", Greater);
    test_one_over_sqrt_tau_prec_round_helper(6, Nearest, "0.4", "0x0.66#6", Less);

    test_one_over_sqrt_tau_prec_round_helper(7, Floor, "0.398", "0x0.66#7", Less);
    test_one_over_sqrt_tau_prec_round_helper(7, Ceiling, "0.402", "0x0.67#7", Greater);
    test_one_over_sqrt_tau_prec_round_helper(7, Down, "0.398", "0x0.66#7", Less);
    test_one_over_sqrt_tau_prec_round_helper(7, Up, "0.402", "0x0.67#7", Greater);
    test_one_over_sqrt_tau_prec_round_helper(7, Nearest, "0.398", "0x0.66#7", Less);

    test_one_over_sqrt_tau_prec_round_helper(8, Floor, "0.398", "0x0.660#8", Less);
    test_one_over_sqrt_tau_prec_round_helper(8, Ceiling, "0.4", "0x0.668#8", Greater);
    test_one_over_sqrt_tau_prec_round_helper(8, Down, "0.398", "0x0.660#8", Less);
    test_one_over_sqrt_tau_prec_round_helper(8, Up, "0.4", "0x0.668#8", Greater);
    test_one_over_sqrt_tau_prec_round_helper(8, Nearest, "0.398", "0x0.660#8", Less);

    test_one_over_sqrt_tau_prec_round_helper(9, Floor, "0.398", "0x0.660#9", Less);
    test_one_over_sqrt_tau_prec_round_helper(9, Ceiling, "0.399", "0x0.664#9", Greater);
    test_one_over_sqrt_tau_prec_round_helper(9, Down, "0.398", "0x0.660#9", Less);
    test_one_over_sqrt_tau_prec_round_helper(9, Up, "0.399", "0x0.664#9", Greater);
    test_one_over_sqrt_tau_prec_round_helper(9, Nearest, "0.399", "0x0.664#9", Greater);

    test_one_over_sqrt_tau_prec_round_helper(10, Floor, "0.3989", "0x0.662#10", Less);
    test_one_over_sqrt_tau_prec_round_helper(10, Ceiling, "0.3994", "0x0.664#10", Greater);
    test_one_over_sqrt_tau_prec_round_helper(10, Down, "0.3989", "0x0.662#10", Less);
    test_one_over_sqrt_tau_prec_round_helper(10, Up, "0.3994", "0x0.664#10", Greater);
    test_one_over_sqrt_tau_prec_round_helper(10, Nearest, "0.3989", "0x0.662#10", Less);

    test_one_over_sqrt_tau_prec_round_helper(
        100,
        Floor,
        "0.3989422804014326779399460599341",
        "0x0.662114cf50d942343f2cf14028#100",
        Less,
    );
    test_one_over_sqrt_tau_prec_round_helper(
        100,
        Ceiling,
        "0.3989422804014326779399460599344",
        "0x0.662114cf50d942343f2cf14030#100",
        Greater,
    );
    test_one_over_sqrt_tau_prec_round_helper(
        100,
        Down,
        "0.3989422804014326779399460599341",
        "0x0.662114cf50d942343f2cf14028#100",
        Less,
    );
    test_one_over_sqrt_tau_prec_round_helper(
        100,
        Up,
        "0.3989422804014326779399460599344",
        "0x0.662114cf50d942343f2cf14030#100",
        Greater,
    );
    test_one_over_sqrt_tau_prec_round_helper(
        100,
        Nearest,
        "0.3989422804014326779399460599344",
        "0x0.662114cf50d942343f2cf14030#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn one_over_sqrt_tau_prec_round_fail_1() {
    Float::one_over_sqrt_tau_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn one_over_sqrt_tau_prec_round_fail_2() {
    Float::one_over_sqrt_tau_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn one_over_sqrt_tau_prec_round_fail_3() {
    Float::one_over_sqrt_tau_prec_round(1000, Exact);
}

#[test]
fn one_over_sqrt_tau_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (one_over_sqrt_tau, o) = Float::one_over_sqrt_tau_prec(prec);
        assert!(one_over_sqrt_tau.is_valid());
        assert_eq!(one_over_sqrt_tau.get_prec(), Some(prec));
        assert_eq!(
            one_over_sqrt_tau.get_exponent(),
            Some(if prec == 1 { 0 } else { -1 })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (one_over_sqrt_tau_alt, o_alt) = Float::one_over_sqrt_tau_prec_round(prec, Ceiling);
            let mut next_upper = one_over_sqrt_tau.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(one_over_sqrt_tau_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !one_over_sqrt_tau.is_power_of_2() {
            let (one_over_sqrt_tau_alt, o_alt) = Float::one_over_sqrt_tau_prec_round(prec, Floor);
            let mut next_lower = one_over_sqrt_tau.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(one_over_sqrt_tau_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (one_over_sqrt_tau_alt, o_alt) = Float::one_over_sqrt_tau_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&one_over_sqrt_tau_alt),
            ComparableFloatRef(&one_over_sqrt_tau)
        );
        assert_eq!(o_alt, o);

        let (one_over_sqrt_tau_alt, o_alt) = one_over_sqrt_tau_prec_round_simple(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&one_over_sqrt_tau_alt),
            ComparableFloatRef(&one_over_sqrt_tau)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn one_over_sqrt_tau_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (one_over_sqrt_tau, o) = Float::one_over_sqrt_tau_prec_round(prec, rm);
        assert!(one_over_sqrt_tau.is_valid());
        assert_eq!(one_over_sqrt_tau.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 0,
            _ => -1,
        };
        assert_eq!(one_over_sqrt_tau.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (one_over_sqrt_tau_alt, o_alt) = Float::one_over_sqrt_tau_prec_round(prec, Ceiling);
            let mut next_upper = one_over_sqrt_tau.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(one_over_sqrt_tau_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !one_over_sqrt_tau.is_power_of_2() {
            let (one_over_sqrt_tau_alt, o_alt) = Float::one_over_sqrt_tau_prec_round(prec, Floor);
            let mut next_lower = one_over_sqrt_tau.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(one_over_sqrt_tau_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }

        let (one_over_sqrt_tau_alt, o_alt) = one_over_sqrt_tau_prec_round_simple(prec, rm);
        assert_eq!(
            ComparableFloatRef(&one_over_sqrt_tau_alt),
            ComparableFloatRef(&one_over_sqrt_tau)
        );
        assert_eq!(o_alt, o);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::one_over_sqrt_tau_prec_round(prec, Exact));
    });

    test_constant(Float::one_over_sqrt_tau_prec_round, 10000);
}
