// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::modular::traits::{
    ModPowerOf2EvaluateMany, ModPowerOf2EvaluateManyPrecomputed,
};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_natural_vec_unsigned_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::modular::mod_power_of_2_evaluate_many::*;

fn naturals(xs: &[&str]) -> Vec<Natural> {
    xs.iter().map(|x| Natural::from_str(x).unwrap()).collect()
}

// Evaluates `p` at `xs` with precomputed data, both ways of calling it, checks the values against
// `mod_power_of_2_evaluate_many`, and returns them.
fn verify(p: &NaturalPolynomial, xs: &[Natural], pow: u64) -> Vec<Natural> {
    let data = NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(xs, pow);
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
    let test = |s, xs: &[&str], pow, out: &[&str]| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let xs = naturals(xs);
        let out = naturals(out);
        assert_eq!(verify(&p, &xs, pow), out);
        assert_eq!(mod_power_of_2_evaluate_many_naive(&p, &xs, pow), out);
    };
    // - modulo 1
    test("0", &["0", "0"], 0, &["0", "0"]);
    // - a modulus that fits in a limb
    test("19*x^2+9*x+5", &["14", "1", "29"], 5, &["15", "1", "21"]);
    // - a modulus of 2^64
    test(
        "15758992454174395339*x^4+10006589572760746539*x^3+536629792805628473*x^2+6909801505670644\
        180*x+8816671109623896822",
        &[
            "8359512107684579415",
            "907877893701031397",
            "5233514391084077678",
            "3401026325537279538",
        ],
        64,
        &[
            "9992990136517419107",
            "14192763971847312101",
            "6235500602942694218",
            "11512080766466321866",
        ],
    );
    // - a modulus of 2^128
    test(
        "190575136332785439177157114604620547280*x^3+247413000509551108075000592729716967871*x^2+1\
        10135934753815330514312072783988365129*x+99109429608799416052323050390687995659",
        &[
            "70505146385691601119481846585928566587",
            "42698246593078068167691675334685391469",
            "237431789753096799174042758733945763311",
            "240678002407984907156500158932497323365",
            "97221509447854466915962394360626451316",
        ],
        128,
        &[
            "5801071741034021465139940021409346165",
            "273228409658771183079177673283313246087",
            "78609703078937725789128170040652177921",
            "214906264393458334182951322379210453327",
            "120580358223948083962684293341821264527",
        ],
    );
    // - a modulus of more than two limbs
    test(
        "397221256357322896667530135315617772028*x^5+310590966358708469500197399214044998831*x^4+9\
        15133624966015679010293891308557909616*x^3+1057724983996706507851725708537220818390*x^2+30\
        915050992844895420380974927331802787*x+1299680544597156947216668628935128205796",
        &[
            "1275690265645588149128923073840676891037",
            "233669632664510293041512620300312498017",
            "1199940834048890565682480652448335378596",
            "237937722306607538947380333262370813155",
            "81899281521806674212956677483659601789",
            "47564909282738481506390194138367633523",
            "1176083902447723267681343669867503312217",
        ],
        130,
        &[
            "640506204820919067720046844496774039868",
            "995815276991154697223996130735689412376",
            "1250023191008378133867429455850333466800",
            "1153736490053583881139684870869747018198",
            "22105822908568232448413125543870476892",
            "785517531898179158702682014968597987782",
            "550158916518812044981961430170226982112",
        ],
    );
    // - a modulus of more than two limbs, and a constant polynomial
    test(
        "256086854887811570369954488776217914119",
        &["933579711858041572766391261347831437863", "845404560167420374103233539205883337400"],
        130,
        &["256086854887811570369954488776217914119", "256086854887811570369954488776217914119"],
    );
    // - a modulus of more than two limbs, the zero polynomial, and one point
    test(
        "0",
        &["493434686015926660112045508050485437256"],
        130,
        &["0"],
    );
}

#[test]
#[should_panic]
fn precompute_mod_power_of_2_evaluate_many_data_fail() {
    // A point is not reduced.
    NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(
        &[Natural::ONE, Natural::from(8u32)],
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_many_precomputed_fail_1() {
    // A coefficient is not reduced.
    let data = NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&[Natural::ONE], 3);
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_evaluate_many_precomputed(3, &data);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_many_precomputed_fail_2() {
    // The data was precomputed for a different power of 2.
    let data = NaturalPolynomial::precompute_mod_power_of_2_evaluate_many_data(&[Natural::ONE], 3);
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_evaluate_many_precomputed(4, &data);
}

#[test]
fn mod_power_of_2_evaluate_many_precomputed_properties() {
    let test = |(p, xs, pow): (NaturalPolynomial, Vec<Natural>, u64)| {
        let ys = verify(&p, &xs, pow);
        assert_eq!(ys.len(), xs.len());
        assert_eq!(mod_power_of_2_evaluate_many_naive(&p, &xs, pow), ys);
    };
    natural_polynomial_natural_vec_unsigned_triple_gen_var_1().test_properties(test);

    // Long polynomials and many points, so that the tree has several levels.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 40);
    config.insert("mean_length_d", 1);
    config.insert("mean_points_n", 40);
    config.insert("mean_points_d", 1);
    natural_polynomial_natural_vec_unsigned_triple_gen_var_1()
        .test_properties_with_config(&config, test);
}
