// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::modular::traits::{ModEvaluateMany, ModEvaluateManyPrecomputed};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_natural_vec_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::modular::mod_evaluate_many::*;

fn naturals(xs: &[&str]) -> Vec<Natural> {
    xs.iter().map(|x| Natural::from_str(x).unwrap()).collect()
}

// Evaluates `p` at `xs` with precomputed data, every way of calling it, checks the values against
// `mod_evaluate_many`, and returns them.
fn verify(p: &NaturalPolynomial, xs: &[Natural], m: &Natural) -> Vec<Natural> {
    let data = NaturalPolynomial::precompute_mod_evaluate_many_data(xs, m);
    assert_eq!(
        <NaturalPolynomial as ModEvaluateManyPrecomputed<Natural, Natural>>::
            precompute_mod_evaluate_many_data(xs, m),
        data
    );
    let ys = p.mod_evaluate_many_precomputed(m, &data);
    assert_eq!(p.mod_evaluate_many_precomputed(m.clone(), &data), ys);
    assert_eq!(p.clone().mod_evaluate_many_precomputed(m, &data), ys);
    assert_eq!(
        p.clone().mod_evaluate_many_precomputed(m.clone(), &data),
        ys
    );
    assert_eq!(p.mod_evaluate_many(xs, m), ys);
    ys
}

#[test]
fn test_mod_evaluate_many_precomputed() {
    let test = |s, xs: &[&str], m, out: &[&str]| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let xs = naturals(xs);
        let m = Natural::from_str(m).unwrap();
        let out = naturals(out);
        assert_eq!(verify(&p, &xs, &m), out);
        assert_eq!(mod_evaluate_many_naive(&p, &xs, &m), out);
    };
    // - modulo 1
    test("0", &["0", "0"], "1", &["0", "0"]);
    // - a modulus that fits in a limb
    test("4*x^2+7*x+8", &["10", "12", "12"], "13", &["10", "5", "5"]);
    // - a modulus of a full limb
    test(
        "3034071297813313764*x^4+37523478889237405*x^3+16520892875486206790*x^2+115022767640256457\
        24*x+16549788049713249703",
        &[
            "8249933567053396428",
            "14901333013904053884",
            "9039887607555180449",
            "13628502321698848544",
        ],
        "18446744073709551557",
        &[
            "2449715459747527325",
            "14917822894315887124",
            "7317205410380506644",
            "4996865266679385690",
        ],
    );
    // - a modulus that fits in two limbs
    test(
        "46341325947644698437862952463171517400*x^3+178911424186763537666538124964073253258*x^2+54\
        986113079526207266363925804352821303*x+89962488054104055783058609956844936702",
        &[
            "78941368373618912363984083089063192313",
            "63313440092975010444453639013117169747",
            "84348771799070031569625402901384671055",
            "1507876604505255403747086323005027713",
            "52031416518387438794238498405817895920",
        ],
        "340282366920938463463374607431768211297",
        &[
            "261240706363764192106991039711461774659",
            "190339313671122192031878265497652255032",
            "63605029019581385338777092364973167205",
            "340066974603026640661863962922641158756",
            "308429297852892606473720785675321984474",
        ],
    );
    // - a modulus of more than two limbs, and the zero polynomial
    test(
        "0",
        &[
            "542164227003374998829091362288414846334888757566545444535600",
            "339158588342831198478515923084592301774086610462170715107921",
        ],
        "1000000000000000000000000000000000000000000000000000000000007",
        &["0", "0"],
    );
    // - a modulus of more than two limbs, and a constant polynomial
    test(
        "371468525543552575254870511998128154582210113640704516671298",
        &[
            "773363022999340367968853984871582516315192855474234686419859",
            "427710607271827356821494006433479393873489433052908357000020",
        ],
        "1000000000000000000000000000000000000000000000000000000000007",
        &[
            "371468525543552575254870511998128154582210113640704516671298",
            "371468525543552575254870511998128154582210113640704516671298",
        ],
    );
    // - a modulus of more than two limbs, the zero polynomial, and one point
    test(
        "0",
        &["179737380497617911163461244194076917222829237139679011060949"],
        "1000000000000000000000000000000000000000000000000000000000007",
        &["0"],
    );
    // - a modulus of more than two limbs, a constant polynomial, and one point
    test(
        "482309093102922228482058373755365812031355682043648670592758",
        &["11504148996696298917719590302127195623549047738328152159359"],
        "1000000000000000000000000000000000000000000000000000000000007",
        &["482309093102922228482058373755365812031355682043648670592758"],
    );
    // - a modulus of more than two limbs, and one point
    test(
        "755953401059092489840721199694312128258738833231857122813662*x^5+408273767809121027409969\
        974014530415484264363078353529735366*x^4+1679128875382252060952195481594773119072064837061\
        82652156093*x^3+335146519865508329962097705341997924771381789560217723401462*x^2+661855066\
        434273140595057191928555679900864017820308598995353*x+993154181803987074453178516472387825\
        521788944838510967046316",
        &["970910730245633417737217376583707465566057817700207092310528"],
        "1000000000000000000000000000000000000000000000000000000000007",
        &["737317802874136014538844416448617080020697589624497297033679"],
    );
    // - a modulus of more than two limbs, and one point, x = 0
    test(
        "3*x^2+2*x+1",
        &["0"],
        "1000000000000000000000000000000000000000000000000000000000007",
        &["1"],
    );
    // - a modulus of more than two limbs
    test(
        "899390498481995883317099619860251796182139050808863422882068*x^5+183063080175886285065464\
        004737671017694999517727470548809679*x^4+4957123299506293582324284899572500926842420804401\
        07427546557*x^3+179406821353330445009751163723782079859265955266895528332552*x^2+524024306\
        102163272666580326196430596926913700182325200048177*x+126413145048969025735302605019187333\
        708318710005029092239052",
        &[
            "611239589552969451648819825465154755533652719361918370720039",
            "323952980864712171055794225658611431501328409154710278128862",
            "508082918526249740196086619810233440123587954512457139978926",
            "865600552551294856736545481359115438174523209791909104389229",
            "608587570910532941720660645997534414095034152334547387182054",
            "345844461752376714836379179150136693948543813439272982439933",
            "943655480242721468488680792900572631316376713417120205442042",
        ],
        "1000000000000000000000000000000000000000000000000000000000007",
        &[
            "274479919182293994061047183305636408960053294741777532107035",
            "47660175393116094662767742776428951071256531749362774784965",
            "584151575926588854881365624264607724825934746340505640077504",
            "377916532860884714326217271464262708526109776832039821853315",
            "71399659745129402171423112324297920312861874502843039461259",
            "414312482943227398350643116054385851145189090530947089631954",
            "695111389639252021809534326596171541721825496192490905179435",
        ],
    );
}

#[test]
#[should_panic]
fn precompute_mod_evaluate_many_data_fail_1() {
    // m is 0.
    NaturalPolynomial::precompute_mod_evaluate_many_data(&[], &Natural::ZERO);
}

#[test]
#[should_panic]
fn precompute_mod_evaluate_many_data_fail_2() {
    // A point is not reduced.
    NaturalPolynomial::precompute_mod_evaluate_many_data(
        &[Natural::ONE, Natural::from(7u32)],
        &Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_evaluate_many_precomputed_fail_1() {
    // A coefficient is not reduced.
    let m = Natural::from(7u32);
    let data = NaturalPolynomial::precompute_mod_evaluate_many_data(&[Natural::ONE], &m);
    NaturalPolynomial::from_str("7*x+1")
        .unwrap()
        .mod_evaluate_many_precomputed(&m, &data);
}

#[test]
#[should_panic]
fn mod_evaluate_many_precomputed_fail_2() {
    // The data was precomputed for a different modulus.
    let data =
        NaturalPolynomial::precompute_mod_evaluate_many_data(&[Natural::ONE], &Natural::from(7u32));
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_evaluate_many_precomputed(&Natural::from(5u32), &data);
}

#[test]
#[should_panic]
fn mod_evaluate_many_precomputed_fail_3() {
    // The data was precomputed for a modulus of a different size.
    let data =
        NaturalPolynomial::precompute_mod_evaluate_many_data(&[Natural::ONE], &Natural::from(7u32));
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_evaluate_many_precomputed(
            &Natural::from_str("1000000000000000000000000").unwrap(),
            &data,
        );
}

#[test]
fn mod_evaluate_many_precomputed_properties() {
    let test = |(p, xs, m): (NaturalPolynomial, Vec<Natural>, Natural)| {
        let ys = verify(&p, &xs, &m);
        assert_eq!(ys.len(), xs.len());
        assert_eq!(mod_evaluate_many_naive(&p, &xs, &m), ys);
    };
    natural_polynomial_natural_vec_natural_triple_gen_var_1().test_properties(test);

    // Long polynomials and many points, so that the tree has several levels.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 40);
    config.insert("mean_length_d", 1);
    config.insert("mean_points_n", 40);
    config.insert("mean_points_d", 1);
    natural_polynomial_natural_vec_natural_triple_gen_var_1()
        .test_properties_with_config(&config, test);
}
