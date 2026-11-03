// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::modular::traits::{ModPowerOf2Evaluate, ModPowerOf2EvaluateMany};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_evaluate_many::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{
    PowerOf2Modulus, evaluate_many_fast, mod_power_of_2_evaluate_many_tree_preferred,
};

#[test]
fn test_mod_power_of_2_evaluate_many() {
    fn test<T: PrimitiveUnsigned>(s: &str, xs: &[T], pow: u64, out: &[T]) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        assert_eq!((&p).mod_power_of_2_evaluate_many(xs, pow), out);
        assert_eq!(mod_power_of_2_evaluate_many_naive(&p, xs, pow), out);
    }
    // - modulo 1
    test::<u8>("0", &[0, 0], 0, &[0, 0]);
    // - no points
    test::<u8>("5*x^2+3*x+7", &[], 3, &[]);
    // - a short polynomial, evaluated at each point
    test::<u8>("3*x^2+x+5", &[6, 1, 5, 2], 3, &[7, 1, 5, 3]);
    // - a polynomial of length 32, still evaluated at each point
    test::<u8>(
        "255*x^31+126*x^30+176*x^29+4*x^28+118*x^27+135*x^26+52*x^25+45*x^24+29*x^23+255*x^22+138*\
        x^21+78*x^20+67*x^19+4*x^18+168*x^17+57*x^16+28*x^15+226*x^14+4*x^13+72*x^12+43*x^11+110*x\
        ^10+48*x^9+193*x^8+87*x^7+221*x^6+52*x^5+43*x^4+79*x^3+221*x^2+212*x+10",
        &[5, 232, 182, 109, 19],
        8,
        &[12, 106, 110, 236, 164],
    );
    // - a modulus of 2^31 in a u32
    test::<u32>(
        "1930098894*x^39+1134311886*x^38+492839538*x^37+218920437*x^36+1649315048*x^35+1824111708*\
        x^34+1843942454*x^33+1808697124*x^32+198663624*x^31+1758382576*x^30+192352192*x^29+1238314\
        412*x^28+30765151*x^27+334893194*x^26+77647494*x^25+1229568458*x^24+519996*x^23+2048255763\
        *x^22+469522174*x^21+395200695*x^20+854106557*x^19+1770977793*x^18+762948207*x^17+12738107\
        37*x^16+1168308041*x^15+995944389*x^14+1463409259*x^13+344707631*x^12+635593929*x^11+19388\
        31801*x^10+1801478437*x^9+610703017*x^8+2006335019*x^7+1378628518*x^6+1441865415*x^5+13273\
        39415*x^4+384724728*x^3+13715589*x^2+420961784*x+524127395",
        &[
            556387588, 1922791546, 1780709958, 1325212419, 814847669, 1029198859, 2022102436,
            375056839, 867580936,
        ],
        31,
        &[
            1319058899, 1714493463, 1930992503, 1516024153, 762882851, 1223436961, 1189418451,
            1043268797, 2121162147,
        ],
    );
    // - a modulus of 2^64
    test::<u64>(
        "14567273582237603816*x+2632578571148828784",
        &[8302403854198970355, 5052968737278683645],
        64,
        &[6866646809838545320, 1125285621290150072],
    );
    // - a modulus of 2^64 and a polynomial of length 33
    test::<u64>(
        "14153758981580440899*x^32+5029653676597812977*x^31+7798050612493237735*x^30+8157922972520\
        781697*x^29+14206550669911288053*x^28+9724267384654585691*x^27+9594116105595963274*x^26+16\
        567419763550513101*x^25+9910979641553589583*x^24+5990169711268327552*x^23+1301170968454188\
        1860*x^22+11386293030714504543*x^21+10530760392857831471*x^20+12313082405921463958*x^19+41\
        53779640656190790*x^18+10752331797402058873*x^17+3565298965181803271*x^16+1677882441629414\
        4665*x^15+3815812731494880105*x^14+14109463761634632400*x^13+5259143550729649357*x^12+2821\
        619943931465711*x^11+8497769705454490422*x^10+8183273106499878149*x^9+5207045694297483106*\
        x^8+17290673938605400774*x^7+3163358602770528136*x^6+7544196415011786514*x^5+1824376685946\
        8323587*x^4+17980743635356023634*x^3+2077558100173857224*x^2+36806316421549239*x+179624391\
        08259354267",
        &[
            16726186759067355960,
            10125500187307573326,
            3801272929692672761,
            7757487034042603000,
            4235229624661939843,
            4594462462797700434,
        ],
        64,
        &[
            10327178154271914403,
            10094830991391199581,
            9947595373511539658,
            15665627591315007715,
            10585075525493385382,
            10662008414277264089,
        ],
    );
    // - a modulus of 2^128
    test::<u128>(
        "270354392579776777760350415694449522962*x^31+325411908550502865400693094796294104718*x^30\
        +313598403874511566030697243010903033084*x^29+44133287845789961352681037602858003432*x^28+\
        270390181589095846698848852966668217439*x^27+34356084329116621657494650469280591840*x^26+9\
        5891394620134416175533101436153003334*x^25+126983138988401451476719772075191079724*x^24+21\
        7548921371626343163820041224451847111*x^23+238800396795396336659299721708489749841*x^22+13\
        9098726049212857600671555423935607877*x^21+105498502271744169592953357055303853733*x^20+16\
        9066070689361288903684439828527713221*x^19+143971806786801499501477255685944348321*x^18+82\
        933254536864317875693810313482219546*x^17+124214686995312509610840585540720707149*x^16+101\
        997031371252054681016983273650045868*x^15+186869790812318745111620534236402910041*x^14+717\
        09612240863861891988088685725936479*x^13+211538827941986575179320090443671962824*x^12+2061\
        59973233625563039207238790320583695*x^11+320410483339915116542316348267488996662*x^10+2219\
        65311260644278001071456894507616321*x^9+315347528236315458954346847191974274682*x^8+915993\
        70283365403507008506747646623859*x^7+141265964326459101618840108995070350959*x^6+207900473\
        93800650609458312691687263278*x^5+869165727294565478825335778792633456*x^4+108769364121383\
        605383661093274760855159*x^3+145525346377417670076086293288299197624*x^2+20822645052438065\
        3581185018221512179588*x+155105041866481121665837465365837541252",
        &[
            319935116594400711718841377221000211886,
            22125575240456776122397017828360719381,
            228682915958001519502732128664762530522,
        ],
        128,
        &[
            163202460818171912554277213020512000420,
            232762417169503120564909393560872530251,
            249504438481933856563911039917369166820,
        ],
    );
    // - Horner's rule in blocks of points, for a polynomial of length 48 or more
    test::<u16>(
        "x^49+7*x+3",
        &[11, 40514, 15481, 55984, 30951, 5918, 46421, 21388, 61891, 36858],
        16,
        &[795, 21457, 61515, 64211, 3259, 41429, 25067, 18647, 4955, 61401],
    );
    // - the subproduct tree: a polynomial of length 1024 and 256 points
    test::<u8>(
        "x^1023+x+1",
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
            109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
            126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
            143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
            160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
            177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193,
            194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
            211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227,
            228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
            245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
        ],
        8,
        &[
            1, 3, 3, 175, 5, 211, 7, 191, 9, 67, 11, 175, 13, 211, 15, 255, 17, 3, 19, 47, 21, 83,
            23, 191, 25, 67, 27, 47, 29, 83, 31, 255, 33, 3, 35, 175, 37, 211, 39, 191, 41, 67, 43,
            175, 45, 211, 47, 255, 49, 3, 51, 47, 53, 83, 55, 191, 57, 67, 59, 47, 61, 83, 63, 255,
            65, 3, 67, 175, 69, 211, 71, 191, 73, 67, 75, 175, 77, 211, 79, 255, 81, 3, 83, 47, 85,
            83, 87, 191, 89, 67, 91, 47, 93, 83, 95, 255, 97, 3, 99, 175, 101, 211, 103, 191, 105,
            67, 107, 175, 109, 211, 111, 255, 113, 3, 115, 47, 117, 83, 119, 191, 121, 67, 123, 47,
            125, 83, 127, 255, 129, 3, 131, 175, 133, 211, 135, 191, 137, 67, 139, 175, 141, 211,
            143, 255, 145, 3, 147, 47, 149, 83, 151, 191, 153, 67, 155, 47, 157, 83, 159, 255, 161,
            3, 163, 175, 165, 211, 167, 191, 169, 67, 171, 175, 173, 211, 175, 255, 177, 3, 179,
            47, 181, 83, 183, 191, 185, 67, 187, 47, 189, 83, 191, 255, 193, 3, 195, 175, 197, 211,
            199, 191, 201, 67, 203, 175, 205, 211, 207, 255, 209, 3, 211, 47, 213, 83, 215, 191,
            217, 67, 219, 47, 221, 83, 223, 255, 225, 3, 227, 175, 229, 211, 231, 191, 233, 67,
            235, 175, 237, 211, 239, 255, 241, 3, 243, 47, 245, 83, 247, 191, 249, 67, 251, 47,
            253, 83, 255, 255,
        ],
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_many_fail_1() {
    // pow is too large.
    (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap()).mod_power_of_2_evaluate_many(&[1], 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_many_fail_2() {
    // A coefficient is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap()).mod_power_of_2_evaluate_many(&[1], 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_many_fail_3() {
    // A point is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap())
        .mod_power_of_2_evaluate_many(&[1, 2, 8], 3);
}

fn mod_power_of_2_evaluate_many_properties_helper<T: PrimitiveUnsigned>() {
    let test = |(p, xs, pow): (UnsignedPolynomial<T>, Vec<T>, u64)| {
        let ys = (&p).mod_power_of_2_evaluate_many(&xs, pow);
        assert_eq!(ys.len(), xs.len());
        assert!(ys.iter().all(|y| y.significant_bits() <= pow));
        let ys_alt: Vec<T> = xs
            .iter()
            .map(|&x| (&p).mod_power_of_2_evaluate(x, pow))
            .collect();
        assert_eq!(ys_alt, ys);
        assert_eq!(mod_power_of_2_evaluate_many_naive(&p, &xs, pow), ys);
        if pow != 0 {
            assert_eq!(
                evaluate_many_fast(p.coefficients_asc(), &xs, &PowerOf2Modulus(pow)),
                ys
            );
        }

        // Evaluating at the concatenation of two lists concatenates the values.
        let (xs_1, xs_2) = xs.split_at(xs.len() >> 1);
        let mut ys_alt = (&p).mod_power_of_2_evaluate_many(xs_1, pow);
        ys_alt.extend((&p).mod_power_of_2_evaluate_many(xs_2, pow));
        assert_eq!(ys_alt, ys);
    };
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_2::<T>().test_properties(test);

    // Long enough for the dispatch to reach the subproduct tree.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 40);
    config.insert("mean_length_d", 1);
    config.insert("mean_points_n", 40);
    config.insert("mean_points_d", 1);
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_2::<T>()
        .test_properties_with_config(&config, test);
}

#[test]
fn mod_power_of_2_evaluate_many_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_evaluate_many_properties_helper);
}

#[test]
fn test_mod_power_of_2_evaluate_many_tree_preferred() {
    assert!(mod_power_of_2_evaluate_many_tree_preferred(1024, 256));
    assert!(mod_power_of_2_evaluate_many_tree_preferred(1024, 8192));
    assert!(!mod_power_of_2_evaluate_many_tree_preferred(1023, 256));
    assert!(!mod_power_of_2_evaluate_many_tree_preferred(1024, 255));
    assert!(!mod_power_of_2_evaluate_many_tree_preferred(1024, 8193));
    assert!(!mod_power_of_2_evaluate_many_tree_preferred(2047, 16377));
    assert!(mod_power_of_2_evaluate_many_tree_preferred(2048, 1 << 20));
}
