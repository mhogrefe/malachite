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
use malachite_base::polynomial::modular::traits::{ModEvaluateMany, ModEvaluateManyPrecomputed};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_many::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Evaluates `p` at `xs` with precomputed data, both ways of calling it, checks the values against
// `mod_evaluate_many`, and returns them.
fn verify<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, xs: &[T], m: T) -> Vec<T> {
    let data = UnsignedPolynomial::precompute_mod_evaluate_many_data(xs, &m);
    let ys = p.mod_evaluate_many_precomputed(m, &data);
    assert_eq!(p.clone().mod_evaluate_many_precomputed(m, &data), ys);
    assert_eq!(p.mod_evaluate_many(xs, m), ys);
    ys
}

#[test]
fn test_mod_evaluate_many_precomputed() {
    fn test<T: PrimitiveUnsigned>(s: &str, xs: &[T], m: T, out: &[T]) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        assert_eq!(verify(&p, xs, m), out);
        assert_eq!(mod_evaluate_many_naive(&p, xs, m), out);
    }
    // - modulo 1
    test::<u8>("0", &[0, 0], 1, &[0, 0]);
    // - the zero polynomial
    test::<u8>("0", &[8, 7, 4], 13, &[0, 0, 0]);
    // - a constant polynomial
    test::<u8>("10", &[12, 12, 6], 13, &[10, 10, 10]);
    // - one point
    test::<u8>("2*x+11", &[0], 13, &[11]);
    // - no points
    test::<u8>("x^2+7", &[], 13, &[]);
    // - two points, so that the tree has one level
    test::<u8>("12*x+11", &[7, 12], 13, &[4, 12]);
    // - three points, so that the last node of a level is shorter
    test::<u8>("x^2+10*x+4", &[7, 9, 1], 13, &[6, 6, 2]);
    // - a polynomial longer than the number of points
    test::<u8>(
        "205*x^4+78*x^3+120*x^2+31*x+9",
        &[240, 23, 95, 117],
        251,
        &[175, 41, 149, 99],
    );
    // - a linear polynomial and a full tree of 8 points
    test::<u8>(
        "40*x+208",
        &[235, 174, 93, 54, 211, 13, 149, 12],
        251,
        &[70, 140, 163, 109, 114, 226, 144, 186],
    );
    // - seven points, so that left > pow both while building the tree and while descending it
    test::<u8>(
        "174*x^5+3*x^4+107*x^3+44*x^2+162*x+137",
        &[156, 17, 19, 232, 95, 152, 17],
        251,
        &[146, 41, 145, 81, 137, 98, 41],
    );
    // - a polynomial much longer than the number of points, reduced by the top of the tree
    test::<u16>(
        "2472*x^39+6675*x^38+1113*x^37+39143*x^36+17180*x^35+50435*x^34+61741*x^33+62411*x^32+3122\
        4*x^31+25519*x^30+61432*x^29+38569*x^28+29804*x^27+58824*x^26+9966*x^25+15370*x^24+33172*x\
        ^23+49842*x^22+18883*x^21+41034*x^20+6240*x^19+65011*x^18+51833*x^17+29728*x^16+21515*x^15\
        +60719*x^14+37461*x^13+10026*x^12+33243*x^11+25525*x^10+46721*x^9+29589*x^8+38314*x^7+8584\
        *x^6+22290*x^5+64385*x^4+4680*x^3+23573*x^2+40126*x+57610",
        &[44233, 16364, 65056],
        65521,
        &[16968, 62928, 10178],
    );
    // - a polynomial much shorter than the number of points, reduced low in the tree
    test::<u32>(
        "3075360275*x^2+2197857306*x+66422512",
        &[
            2178971906, 2898689303, 240981649, 1130373053, 3286422606, 3924770760, 97103488,
            1112314075, 3670802046, 3353487660, 3881995376, 3286473434, 1685846585, 3520470859,
            3501003461, 337500708, 4068883195, 2430753509, 834375168, 1265063311, 3239644414,
            400977582, 909115173, 1822881001, 4057592448, 885071940, 2005473395, 1495706497,
            3401019052, 3488207546, 3817825065, 1423512241, 1161464909, 718809426, 3972274917,
            1117499366, 2748592266, 1875558334, 2999949766, 3908457443,
        ],
        4294967291,
        &[
            1712371177, 3998339910, 258816894, 3028666631, 3726702601, 1065988152, 3436942458,
            2255461934, 814694780, 1171243745, 3763261693, 3070493380, 640638230, 818999388,
            1253940642, 1060017216, 4085525710, 2876291251, 2008389968, 442075365, 10570744,
            1748557976, 755638843, 829505814, 1249117216, 2507253349, 2092255679, 2792273424,
            696601038, 2494757839, 3763756468, 2713034964, 904773365, 3689379885, 477050308,
            1223917445, 725422814, 3761186417, 4102474767, 2435667024,
        ],
    );
    // - a u64 modulus with the top bit set
    test::<u64>(
        "10624524628020505098*x^16+5338770869030954780*x^15+266391902986759855*x^14+27930359185659\
        18602*x^13+515579805183680803*x^12+9662177003159973573*x^11+6621033480598061276*x^10+14672\
        771375527252026*x^9+5336962263969429303*x^8+2711395405206029961*x^7+445456164590157946*x^6\
        +12826258226845309962*x^5+3548092241046006696*x^4+17076273894023436765*x^3+158884711599165\
        1780*x^2+1098541420799927025*x+12180819611139392589",
        &[
            562835242219410405,
            10968208273407128258,
            14579505652522450916,
            16718765484672572896,
            8903580229004482723,
            14559654988593618682,
            4589302787934055734,
            4336070991289013020,
            5532737433440314499,
        ],
        18446744073709551557,
        &[
            18015397535936635653,
            13278738547760553939,
            9907031902403148880,
            13377886510388811466,
            11702011782087143521,
            9139635846258589695,
            18246497624220840521,
            12655497948797824044,
            2021191300387627040,
        ],
    );
    // - a u128 modulus
    test::<u128>(
        "127627632399887882695425330474169233056*x^5+21734982122967635073194317255583945592*x^4+10\
        1541670700678705940972557838494721303*x^3+141265964326459101633249862592969206359*x^2+1480\
        76091118156395105206444763153792089*x+51224231040158874991882612498150866243",
        &[
            51824127800175046303303617080380261370,
            104795580439806812418350966612591290048,
            97256670643657797870326767223285679698,
            27467719136264093838091463007209095245,
            41397644481517343477685896590818398985,
        ],
        170141183460469231731687303715884105727,
        &[
            27926854057420888049149582514049727471,
            27803710990392027868751016726978256283,
            52881792394090629660245041282963157760,
            25369075890675309714046842283348769653,
            40317542714253463394254581940953226998,
        ],
    );
}

#[test]
#[should_panic]
fn precompute_mod_evaluate_many_data_fail_1() {
    // m is 0.
    UnsignedPolynomial::<u8>::precompute_mod_evaluate_many_data(&[], &0);
}

#[test]
#[should_panic]
fn precompute_mod_evaluate_many_data_fail_2() {
    // A point is not reduced.
    UnsignedPolynomial::<u8>::precompute_mod_evaluate_many_data(&[1, 7], &7);
}

#[test]
#[should_panic]
fn mod_evaluate_many_precomputed_fail_1() {
    // A coefficient is not reduced.
    let data = UnsignedPolynomial::<u8>::precompute_mod_evaluate_many_data(&[1, 2], &7);
    UnsignedPolynomial::<u8>::from_str("7*x+1")
        .unwrap()
        .mod_evaluate_many_precomputed(7, &data);
}

#[test]
#[should_panic]
fn mod_evaluate_many_precomputed_fail_2() {
    // The data was precomputed for a different modulus.
    let data = UnsignedPolynomial::<u8>::precompute_mod_evaluate_many_data(&[1, 2], &7);
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_evaluate_many_precomputed(5, &data);
}

fn mod_evaluate_many_precomputed_properties_helper<T: PrimitiveUnsigned>() {
    let test = |(p, xs, m): (UnsignedPolynomial<T>, Vec<T>, T)| {
        let ys = verify(&p, &xs, m);
        assert_eq!(ys.len(), xs.len());
        assert_eq!(mod_evaluate_many_naive(&p, &xs, m), ys);

        // The same data serves every polynomial.
        let data = UnsignedPolynomial::precompute_mod_evaluate_many_data(&xs, &m);
        let q = UnsignedPolynomial::<T>::ZERO;
        assert!(
            (&q).mod_evaluate_many_precomputed(m, &data)
                .iter()
                .all(|&y| y == T::ZERO)
        );
    };
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>().test_properties(test);

    // Long polynomials and many points, so that the tree has several levels.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 40);
    config.insert("mean_length_d", 1);
    config.insert("mean_points_n", 40);
    config.insert("mean_points_d", 1);
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>()
        .test_properties_with_config(&config, test);
}

#[test]
fn mod_evaluate_many_precomputed_properties() {
    apply_fn_to_unsigneds!(mod_evaluate_many_precomputed_properties_helper);
}
