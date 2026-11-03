// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::modular::traits::{ModEvaluate, ModEvaluateMany};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_many::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::modular::mod_evaluate::*;
use malachite_base::unsigned_polynomial::modular::mod_evaluate_many::*;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{
    Modulus, evaluate_many_fast, mod_evaluate_many_tree_preferred,
};

#[test]
fn test_mod_evaluate_many() {
    fn test<T: PrimitiveUnsigned>(s: &str, xs: &[T], m: T, out: &[T]) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let ys = (&p).mod_evaluate_many(xs, m);
        assert_eq!(ys, out);
        let ys_alt: Vec<T> = xs.iter().map(|&x| (&p).mod_evaluate(x, m)).collect();
        assert_eq!(ys_alt, out);
    }
    // - no points
    test::<u8>("5*x^2+3*x+7", &[], 13, &[]);
    // - the zero polynomial
    test::<u8>("0", &[0, 5, 6], 13, &[0, 0, 0]);
    // - a constant polynomial
    test::<u8>("9", &[0, 5, 6], 13, &[9, 9, 9]);
    // - lazy Shoup blocks, since a u8 polynomial of length 2 or more uses Shoup
    test::<u8>("5*x^2+3*x+7", &[0, 1, 2, 3, 4, 5], 13, &[7, 2, 7, 9, 8, 4]);
    // - Shoup blocks, not lazy
    test::<u8>(
        "5*x^2+3*x+7",
        &[0, 1, 2, 3, 4, 5, 6, 7, 8],
        120,
        &[7, 15, 33, 61, 99, 27, 85, 33, 111],
    );
    // - Horner blocks, since the top bit of m is set
    test::<u8>(
        "5*x^2+3*x+7",
        &[0, 1, 2, 3, 4, 5, 6, 7, 8],
        251,
        &[7, 15, 33, 61, 99, 147, 205, 22, 100],
    );
    // - Horner blocks, since the polynomial is too short for Shoup in a u64
    test::<u64>(
        "3*x+1",
        &[0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3],
        7,
        &[1, 4, 0, 3, 6, 2, 5, 1, 4, 0, 3],
    );
    // - lazy Shoup blocks of 8, then of 4, and a point left over
    test::<u64>(
        "12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        &[2, 3, 5, 7, 11, 2, 3, 5, 7, 2, 3, 5, 7],
        1000003,
        &[
            45057, 55777, 160935, 12308, 564144, 45057, 55777, 160935, 12308, 45057, 55777, 160935,
            12308,
        ],
    );
    // - Shoup blocks, not lazy, and points left over
    test::<u64>(
        "x^3+6789*x^2+12345",
        &[
            4611686018427387904,
            4611686018427387905,
            4611686018427387906,
            4611686018427387907,
            4611686018427387908,
            4611686018427387909,
            4611686018427387904,
            4611686018427387905,
            4611686018427387906,
            4611686018427387907,
        ],
        9223372036854775783,
        &[
            3458764513821615998,
            5764607523035486965,
            8070450532249371591,
            1152921504608494099,
            3458764513822406061,
            5764607523036331700,
            3458764513821615998,
            5764607523035486965,
            8070450532249371591,
            1152921504608494099,
        ],
    );
    // - Horner blocks, since the top bit of m is set, and points left over
    test::<u64>(
        "x^3+x^2+x+1",
        &[
            18446744073709551556,
            18446744073709551555,
            18446744073709551554,
            18446744073709551553,
            18446744073709551552,
            18446744073709551551,
            18446744073709551550,
            18446744073709551556,
            18446744073709551555,
            18446744073709551554,
            18446744073709551553,
        ],
        18446744073709551557,
        &[
            0,
            18446744073709551552,
            18446744073709551537,
            18446744073709551506,
            18446744073709551453,
            18446744073709551372,
            18446744073709551257,
            0,
            18446744073709551552,
            18446744073709551537,
            18446744073709551506,
        ],
    );
    // - lazy Shoup blocks in a u128
    test::<u128>(
        "4*x^3+3*x^2+2*x+1",
        &[1267650600228229401496703205376, 3, 5, 7, 9],
        170141183460469231731687303715884105727,
        &[2535301228790657981686254403585, 142, 586, 1534, 3178],
    );
    // - a polynomial of length 32 with the top bit of m set, still evaluated with Horner's rule
    test::<u8>(
        "117*x^31+95*x^30+23*x^29+240*x^28+205*x^27+78*x^26+120*x^25+31*x^24+9*x^23+135*x^22+122*x\
        ^21+197*x^20+214*x^19+49*x^18+134*x^17+19*x^16+11*x^15+121*x^14+28*x^13+17*x^12+152*x^11+9\
        5*x^10+232*x^9+19*x^8+17*x^7+156*x^6+174*x^5+3*x^4+107*x^3+44*x^2+162*x+137",
        &[208, 40, 235, 174, 93],
        251,
        &[197, 117, 127, 204, 184],
    );
    // - Horner's rule with more points than half the coefficients
    test::<u16>(
        "50435*x^39+61741*x^38+62411*x^37+31224*x^36+25519*x^35+61432*x^34+38569*x^33+29804*x^32+5\
        8824*x^31+9966*x^30+15370*x^29+33172*x^28+49842*x^27+18883*x^26+41034*x^25+6240*x^24+65011\
        *x^23+51833*x^22+29728*x^21+21515*x^20+60719*x^19+37461*x^18+10026*x^17+33243*x^16+25525*x\
        ^15+46721*x^14+29589*x^13+38314*x^12+8584*x^11+22290*x^10+64385*x^9+4680*x^8+23573*x^7+401\
        26*x^6+57610*x^5+40530*x^4+13869*x^3+38453*x^2+45758*x+62706",
        &[
            17180, 39143, 1113, 6675, 2472, 44233, 16364, 65056, 28323, 42190, 25745, 6035, 4278,
            4559, 18697, 10049, 49223,
        ],
        65521,
        &[
            41385, 46431, 6567, 16789, 38399, 26424, 25162, 17167, 18343, 35751, 26327, 33394,
            50617, 55656, 25638, 55523, 18520,
        ],
    );
    // - Horner's rule with two points and the top bit of m set
    test::<u64>(
        "6482650153777782682*x^32+12740029904658747632*x^31+5460219442746050545*x^30+1645666468711\
        9886398*x^29+5259652275333616242*x^28+985961420334782878*x^27+6832447381518982292*x^26+914\
        6267180590937766*x^25+2461304796509447490*x^24+16608356244069650985*x^23+11571341271344897\
        928*x^22+17826103307595184281*x^21+11633552638008414789*x^20+6192258379339567526*x^19+1753\
        260066510177725*x^18+13776069099990034173*x^17+13099927866413319710*x^16+89358682776084002\
        40*x^15+3273058043949045006*x^14+11774293614435308243*x^13+4562424452206688964*x^12+104862\
        50461407982340*x^11+9231806679431865576*x^10+695758685649194552*x^9+7584840985487071427*x^\
        8+17478038129184143027*x^7+8438095610251256534*x^6+3663735609123219978*x^5+602995749102407\
        5461*x^4+15006129226884422730*x^3+15950697477026522282*x^2+17897437800188381622*x+58817760\
        03539689676",
        &[9725658475078258066, 14512791114012867654],
        18446744073709551557,
        &[10955379955128360079, 1468204019872219866],
    );
    // - lazy Shoup blocks for a polynomial of length 64
    test::<u64>(
        "114240*x^63+947951*x^62+799362*x^61+392543*x^60+672673*x^59+161137*x^58+378697*x^57+33683\
        4*x^56+798315*x^55+322066*x^54+680521*x^53+641161*x^52+380001*x^51+493534*x^50+969662*x^49\
        +786079*x^48+472620*x^47+706366*x^46+910960*x^45+432691*x^44+65892*x^43+768531*x^42+464521\
        *x^41+242506*x^40+142351*x^39+627492*x^38+682182*x^37+502716*x^36+289042*x^35+76289*x^34+6\
        81200*x^33+913941*x^32+143266*x^31+773246*x^30+249431*x^29+350696*x^28+645618*x^27+935188*\
        x^26+829169*x^25+99859*x^24+695397*x^23+678069*x^22+689766*x^21+161394*x^20+863424*x^19+59\
        1242*x^18+8009*x^17+35061*x^16+706010*x^15+132676*x^14+721267*x^13+468820*x^12+783029*x^11\
        +432774*x^10+223205*x^9+215532*x^8+396168*x^7+666728*x^6+978740*x^5+584741*x^4+118908*x^3+\
        994086*x^2+697494*x+196148",
        &[6100, 856856, 195994, 69579, 519889, 502365, 644643, 959352, 322050],
        1000003,
        &[457153, 991383, 947724, 19092, 69461, 189395, 15650, 956064, 105421],
    );
    // - Horner's rule in a u128
    test::<u128>(
        "118485368605050236750772757874572310171*x^31+48681899364910815114683410234279074406*x^30+\
        137719920298020100300411403165574515111*x^29+93852148782274525077829526859896346409*x^28+6\
        8080675386350565850798138095023570797*x^27+2040221938184725291820879980327223403*x^26+5137\
        6414714985597473072607980917004502*x^25+156436789371601015878767401014005187065*x^24+56330\
        675547702308104728273270784220034*x^23+82918185762879941056592676450885085294*x^22+1686608\
        65185243493214238068240633170033*x^21+17070649091979431807852685525382359352*x^20+15193412\
        4174409142864030353355239636208*x^19+83388809422305747582169740475620298022*x^18+142871581\
        411402672785753360863582459697*x^17+154831690464708261930641251051724792610*x^16+169452546\
        37958939405667273640186749066*x^15+151944684994724960418104914393114612943*x^14+1022914073\
        16243580474321983757093654824*x^13+4075794456558150750757808397880136346*x^12+147257646275\
        38404628143600807656545841*x^11+167597905809350046973869200900246474977*x^10+1370268071818\
        68756294808585151504773297*x^9+14743234150865115770340286034240121628*x^8+9727229693929639\
        6433889010428080749162*x^7+58077365806674365227144190493872054853*x^6+74721438432792750027\
        8753108600415820*x^5+15170630537147644209591714553187146351*x^4+22125575240456776133644861\
        331810806447*x^3+88245636065957986389480845131715175760*x^2+166705345439164371618661732306\
        488444438*x+25183990484041428160894089053313925461",
        &[
            16042908942315453093475919419675589351,
            134106613347405950140475327814411527117,
            40638068493172316454046094631827475893,
        ],
        170141183460469231731687303715884105727,
        &[
            78837551799242312651779330635574731971,
            98699202616797862342466638343143243977,
            151930408405177392491323555333331025352,
        ],
    );
    // - the subproduct tree: the top bit of m is set, the polynomial has length 256, and there are
    //   32 points
    test::<u8>(
        "x^255+3*x^7+1",
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ],
        251,
        &[
            1, 5, 166, 28, 228, 55, 209, 27, 29, 55, 81, 9, 248, 134, 22, 82, 126, 24, 16, 227,
            198, 250, 223, 168, 40, 209, 88, 209, 26, 39, 16, 150,
        ],
    );
    // - the subproduct tree in a u64, with the top bit of m set
    test::<u64>(
        "x^300+x^2+5",
        &[
            17,
            1234567890123456806,
            2469135780246913595,
            3703703670370370384,
            4938271560493827173,
            6172839450617283962,
            7407407340740740751,
            8641975230864197540,
            9876543120987654329,
            11111111011111111118,
            12345678901234567907,
            13580246791358024696,
            14814814681481481485,
            16049382571604938274,
            17283950461728395063,
            71774278142300295,
            1306342168265757084,
            2540910058389213873,
            3775477948512670662,
            5010045838636127451,
            6244613728759584240,
            7479181618883041029,
            8713749509006497818,
            9948317399129954607,
            11182885289253411396,
            12417453179376868185,
            13652021069500324974,
            14886588959623781763,
            16121156849747238552,
            17355724739870695341,
            143548556284600573,
            1378116446408057362,
            2612684336531514151,
            3847252226654970940,
            5081820116778427729,
            6316388006901884518,
            7550955897025341307,
            8785523787148798096,
            10020091677272254885,
            11254659567395711674,
        ],
        18446744073709551557,
        &[
            17661800662790880935,
            3139877267961527089,
            11237881010445138372,
            11856798168944244014,
            6352574002738336594,
            9478506646504023085,
            4585806600541522976,
            15466723247044077493,
            17299742833173508556,
            9242154132408684614,
            2932704220077645264,
            14925476065029537174,
            2742605625101974990,
            8636120982958059250,
            9534148988684162767,
            4237857704462281440,
            2076505328125498778,
            17739920615007759849,
            17128033580336547436,
            9229119548398665775,
            5317514051832683571,
            13725773148106882451,
            7442748229236484965,
            17944349710229838150,
            11610823747768971925,
            17062058967658320505,
            9955639808548773785,
            14218080642842633156,
            16421225102175613491,
            15111963429783606566,
            3760346378691646901,
            4080799375663186222,
            14467959883014535546,
            3103995927588785306,
            5915339799174976312,
            125593004957608957,
            4693211097214965426,
            10075296388742595908,
            12933438671558063952,
            16985302613705027843,
        ],
    );
}

#[test]
#[should_panic]
fn mod_evaluate_many_fail_1() {
    // m is 0.
    (&UnsignedPolynomial::<u8>::ZERO).mod_evaluate_many(&[], 0);
}

#[test]
#[should_panic]
fn mod_evaluate_many_fail_2() {
    // A coefficient is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap()).mod_evaluate_many(&[1], 5);
}

#[test]
#[should_panic]
fn mod_evaluate_many_fail_3() {
    // A point is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap()).mod_evaluate_many(&[1, 2, 5, 3], 5);
}

// Each block kernel that applies agrees with evaluating one point at a time, for a block of `N`
// points. The points are taken from the front of `xs`, repeated if there are fewer than `N`.
fn mod_evaluate_blocks_agree<T: PrimitiveUnsigned, const N: usize>(
    coefficients: &[T],
    xs: &[T],
    m: T,
) {
    if coefficients.is_empty() || xs.is_empty() {
        return;
    }
    let block: [T; N] = core::array::from_fn(|i| xs[i % xs.len()]);
    let expected = block.map(|x| mod_evaluate_horner(coefficients, x, m));
    let mut values = block;
    mod_evaluate_horner_block(coefficients, &mut values, m);
    assert_eq!(values, expected);
    if !m.get_highest_bit() {
        let mut values = block;
        mod_evaluate_shoup_block(coefficients, &mut values, m);
        assert_eq!(values, expected);
        if m <= T::MAX / T::from(3u8) {
            let mut values = block;
            mod_evaluate_shoup_lazy_block(coefficients, &mut values, m);
            assert_eq!(values, expected);
        }
    }
}

fn mod_evaluate_many_properties_helper<T: PrimitiveUnsigned>() {
    let test = |(p, xs, m): (UnsignedPolynomial<T>, Vec<T>, T)| {
        let ys = (&p).mod_evaluate_many(&xs, m);
        assert_eq!(ys.len(), xs.len());
        assert!(ys.iter().all(|&y| y < m));
        let ys_alt: Vec<T> = xs.iter().map(|&x| (&p).mod_evaluate(x, m)).collect();
        assert_eq!(ys_alt, ys);
        assert_eq!(mod_evaluate_many_naive(&p, &xs, m), ys);

        let coefficients = p.coefficients_asc();
        mod_evaluate_blocks_agree::<T, 1>(coefficients, &xs, m);
        mod_evaluate_blocks_agree::<T, 2>(coefficients, &xs, m);
        mod_evaluate_blocks_agree::<T, 3>(coefficients, &xs, m);
        mod_evaluate_blocks_agree::<T, 4>(coefficients, &xs, m);
        mod_evaluate_blocks_agree::<T, 8>(coefficients, &xs, m);

        // Evaluating at the concatenation of two lists concatenates the values.
        let (xs_1, xs_2) = xs.split_at(xs.len() >> 1);
        let mut ys_alt = (&p).mod_evaluate_many(xs_1, m);
        ys_alt.extend((&p).mod_evaluate_many(xs_2, m));
        assert_eq!(ys_alt, ys);
    };
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>().test_properties(test);

    // Long enough for the dispatch to reach Shoup's method.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>()
        .test_properties_with_config(&config, test);

    // Long enough for the dispatch to reach the subproduct tree; both methods agree.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 40);
    config.insert("mean_length_d", 1);
    config.insert("mean_points_n", 40);
    config.insert("mean_points_d", 1);
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>().test_properties_with_config(
        &config,
        |(p, xs, m)| {
            let mut ys = xs.clone();
            mod_evaluate_many_in_place(p.coefficients_asc(), &mut ys, m);
            if m != T::ONE {
                assert_eq!(
                    evaluate_many_fast(p.coefficients_asc(), &xs, &Modulus(m)),
                    ys
                );
            }
            assert_eq!((&p).mod_evaluate_many(&xs, m), ys);
        },
    );
}

#[test]
fn mod_evaluate_many_properties() {
    apply_fn_to_unsigneds!(mod_evaluate_many_properties_helper);
}

#[test]
fn test_mod_evaluate_many_tree_preferred() {
    // - the top bit of m set
    assert!(mod_evaluate_many_tree_preferred(256, 32, 255u8));
    assert!(mod_evaluate_many_tree_preferred(256, 1024, 255u8));
    assert!(!mod_evaluate_many_tree_preferred(255, 32, 255u8));
    assert!(!mod_evaluate_many_tree_preferred(256, 31, 255u8));
    assert!(!mod_evaluate_many_tree_preferred(256, 1025, 255u8));
    assert!(!mod_evaluate_many_tree_preferred(1023, 4093, 255u8));
    assert!(mod_evaluate_many_tree_preferred(1024, 1 << 20, 255u8));
    // - m greater than T::MAX / 3, so that Shoup's multiplication is not lazy
    assert!(mod_evaluate_many_tree_preferred(
        4096,
        4096,
        (1u64 << 63) - 25
    ));
    assert!(!mod_evaluate_many_tree_preferred(
        4095,
        4096,
        (1u64 << 63) - 25
    ));
    assert!(!mod_evaluate_many_tree_preferred(
        4096,
        4095,
        (1u64 << 63) - 25
    ));
    // - lazy Shoup multiplication, with m wider than half a word
    assert!(mod_evaluate_many_tree_preferred(
        8192,
        4096,
        (1u64 << 62) + 135
    ));
    assert!(!mod_evaluate_many_tree_preferred(
        8191,
        4096,
        (1u64 << 62) + 135
    ));
    assert!(!mod_evaluate_many_tree_preferred(
        8192,
        4095,
        (1u64 << 62) + 135
    ));
    // - lazy Shoup multiplication, with m no wider than half a word
    assert!(mod_evaluate_many_tree_preferred(
        4096,
        1024,
        (1u64 << 29) + 11
    ));
    assert!(!mod_evaluate_many_tree_preferred(
        4095,
        1024,
        (1u64 << 29) + 11
    ));
    assert!(!mod_evaluate_many_tree_preferred(
        4096,
        1023,
        (1u64 << 29) + 11
    ));
}
