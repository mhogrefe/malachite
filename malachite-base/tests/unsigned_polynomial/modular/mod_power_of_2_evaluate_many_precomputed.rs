// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::modular::traits::{
    ModPowerOf2EvaluateMany, ModPowerOf2EvaluateManyPrecomputed,
};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_evaluate_many::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Evaluates `p` at `xs` with precomputed data, both ways of calling it, checks the values against
// `mod_power_of_2_evaluate_many`, and returns them.
fn verify<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, xs: &[T], pow: u64) -> Vec<T> {
    let data = UnsignedPolynomial::precompute_mod_power_of_2_evaluate_many_data(xs, pow);
    let ys = p.mod_power_of_2_evaluate_many_precomputed(pow, &data);
    assert_eq!(
        p.clone()
            .mod_power_of_2_evaluate_many_precomputed(pow, &data),
        ys
    );
    assert_eq!(p.mod_power_of_2_evaluate_many(xs, pow), ys);
    ys
}

#[test]
fn test_mod_power_of_2_evaluate_many_precomputed() {
    fn test<T: PrimitiveUnsigned>(s: &str, xs: &[T], pow: u64, out: &[T]) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        assert_eq!(verify(&p, xs, pow), out);
        assert_eq!(mod_power_of_2_evaluate_many_naive(&p, xs, pow), out);
    }
    // - modulo 1
    test::<u8>("0", &[0, 0], 0, &[0, 0]);
    // - the zero polynomial
    test::<u8>("0", &[5, 1, 3], 3, &[0, 0, 0]);
    // - a constant polynomial
    test::<u8>("6", &[1, 5, 2], 3, &[6, 6, 6]);
    // - one point
    test::<u8>("4*x+2", &[5], 3, &[6]);
    // - three points, so that the last node of a level is shorter
    test::<u8>("20*x^2+11*x+15", &[29, 23, 1], 5, &[2, 0, 14]);
    // - seven points, so that left > pow both while building the tree and while descending it
    test::<u8>(
        "43*x^5+110*x^4+48*x^3+193*x^2+87*x+221",
        &[72, 4, 226, 28, 57, 168, 4],
        8,
        &[149, 73, 239, 113, 190, 53, 73],
    );
    // - a polynomial much longer than the number of points, reduced by the top of the tree
    test::<u16>(
        "25801*x^39+14777*x^38+24869*x^37+38569*x^36+15915*x^35+13222*x^34+7879*x^33+38807*x^32+28\
        408*x^31+18565*x^30+24056*x^29+36003*x^28+12563*x^27+5485*x^26+54454*x^25+34536*x^24+38405\
        *x^23+58111*x^22+7294*x^21+14768*x^20+43780*x^19+1654*x^18+61831*x^17+18996*x^16+64301*x^1\
        5+35613*x^14+11519*x^13+17290*x^12+10318*x^11+14403*x^10+39172*x^9+2216*x^8+51769*x^7+1666\
        8*x^6+26082*x^5+57604*x^4+21576*x^3+16939*x^2+4718*x+53040",
        &[53807, 55915, 59333],
        16,
        &[54092, 56076, 4328],
    );
    // - a polynomial much shorter than the number of points, reduced low in the tree
    test::<u32>(
        "762948207*x^2+1273810737*x+3315791689",
        &[
            1770977793, 3001590205, 2542684343, 2617005822, 2048255763, 519996, 3377052106,
            2225131142, 334893194, 2178248799, 1238314412, 192352192, 1758382576, 198663624,
            1808697124, 3991426102, 1824111708, 1649315048, 2366404085, 492839538, 3281795534,
            4077582542, 556387588, 4070275194, 1780709958, 3472696067, 814847669, 1029198859,
            4169586084, 375056839, 3015064584, 3331673697, 3170646926, 1865701352, 3384191777,
            1440408392, 1990617597, 1762621072, 291533402, 1472990391,
        ],
        32,
        &[
            892212969, 2181750749, 1874685423, 74827171, 3989109619, 3779098293, 1283272015,
            1916076683, 2304844047, 2952043943, 3054472357, 2346199817, 806640953, 2930741841,
            184896285, 3961628667, 4130356949, 3276501361, 1079679141, 689512471, 3531417011,
            3628732595, 1317679101, 2275174975, 2397828427, 2878940099, 1289692645, 72796379,
            4216274333, 2468027935, 3129591953, 2707399817, 3493775987, 750017649, 3501716425,
            3803138001, 2571636125, 1504671705, 3169425567, 1725315567,
        ],
    );
    // - a modulus of 2^64
    test::<u64>(
        "4153779640656190790*x^16+10752331797402058873*x^15+3565298965181803271*x^14+1677882441629\
        4144665*x^13+3815812731494880105*x^12+14109463761634632400*x^11+5259143550729649357*x^10+2\
        821619943931465711*x^9+8497769705454490422*x^8+8183273106499878149*x^7+5207045694297483106\
        *x^6+17290673938605400774*x^5+3163358602770528136*x^4+7544196415011786514*x^3+182437668594\
        68323587*x^2+17980743635356023634*x+2077558100173857224",
        &[
            12313082405921463958,
            10530760392857831471,
            11386293030714504543,
            13011709684541881860,
            5990169711268327552,
            9910979641553589583,
            16567419763550513101,
            9594116105595963274,
            9724267384654585691,
        ],
        64,
        &[
            10548607965175468272,
            8412490143923809870,
            17926855692759225646,
            12082444510753517504,
            6441003938309892808,
            16708836592790519182,
            18260667275064023046,
            247261847608858808,
            11841130042887157686,
        ],
    );
    // - a modulus of 2^100 in a u128
    test::<u128>(
        "1026728195891912930419385649976*x^5+239495194867650161103848654147*x^4+126508815484207243\
        6704991092465*x^3+1131772187317638253344312868327*x^2+975698962753174290022209787777*x+105\
        8401238105566421094213511413",
        &[
            105164257206376797525715615822,
            543101110150186409399941325561,
            549799124200131902782019917304,
            620161581804597830479927809667,
            1179713799736074542333534428498,
        ],
        100,
        &[
            80168852105203085966419865991,
            55178473250781576531510954649,
            986456220992137236137497995949,
            589510425885160533716694324573,
            35394018075097765832922422043,
        ],
    );
}

#[test]
#[should_panic]
fn precompute_mod_power_of_2_evaluate_many_data_fail_1() {
    // pow is too large.
    UnsignedPolynomial::<u8>::precompute_mod_power_of_2_evaluate_many_data(&[], 9);
}

#[test]
#[should_panic]
fn precompute_mod_power_of_2_evaluate_many_data_fail_2() {
    // A point is not reduced.
    UnsignedPolynomial::<u8>::precompute_mod_power_of_2_evaluate_many_data(&[1, 8], 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_many_precomputed_fail_1() {
    // A coefficient is not reduced.
    let data = UnsignedPolynomial::<u8>::precompute_mod_power_of_2_evaluate_many_data(&[1, 2], 3);
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_evaluate_many_precomputed(3, &data);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_many_precomputed_fail_2() {
    // The data was precomputed for a different power of 2.
    let data = UnsignedPolynomial::<u8>::precompute_mod_power_of_2_evaluate_many_data(&[1, 2], 3);
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_evaluate_many_precomputed(4, &data);
}

fn mod_power_of_2_evaluate_many_precomputed_properties_helper<T: PrimitiveUnsigned>() {
    let test = |(p, xs, pow): (UnsignedPolynomial<T>, Vec<T>, u64)| {
        let ys = verify(&p, &xs, pow);
        assert_eq!(ys.len(), xs.len());
        assert_eq!(mod_power_of_2_evaluate_many_naive(&p, &xs, pow), ys);
    };
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_2::<T>().test_properties(test);

    // Long polynomials and many points, so that the tree has several levels.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 40);
    config.insert("mean_length_d", 1);
    config.insert("mean_points_n", 40);
    config.insert("mean_points_d", 1);
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_2::<T>()
        .test_properties_with_config(&config, test);
}

#[test]
fn mod_power_of_2_evaluate_many_precomputed_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_evaluate_many_precomputed_properties_helper);
}
