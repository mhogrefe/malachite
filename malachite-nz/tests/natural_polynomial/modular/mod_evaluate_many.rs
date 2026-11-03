// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::modular::traits::{ModEvaluate, ModEvaluateMany};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::modular::subproduct_tree::{
    mod_evaluate_many_fast, mod_evaluate_many_tree_preferred,
};
use malachite_nz::test_util::generators::natural_polynomial_natural_vec_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_polynomial::modular::mod_evaluate_many::*;

#[test]
fn test_mod_evaluate_many() {
    let test = |s, xs: &[u32], m: u32, out: &[u32]| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let xs: Vec<Natural> = xs.iter().map(|&x| Natural::from(x)).collect();
        let m = Natural::from(m);
        let out: Vec<Natural> = out.iter().map(|&y| Natural::from(y)).collect();
        assert_eq!((&p).mod_evaluate_many(&xs, &m), out);
        assert_eq!((&p).mod_evaluate_many(&xs, m.clone()), out);
        assert_eq!(mod_evaluate_many_naive(&p, &xs, &m), out);
    };
    // - no points
    test("5*x^2+3*x+7", &[], 13, &[]);
    // - the zero polynomial
    test("0", &[0, 5], 13, &[0, 0]);
    // - a constant polynomial, and x == 0
    test("9", &[0, 5], 13, &[9, 9]);
    test("5*x^2+3*x+7", &[0, 1, 2, 3], 13, &[7, 2, 7, 9]);
}

#[test]
fn test_mod_evaluate_many_subproduct_tree() {
    let test = |s, xs: &[&str], m, out: &[&str]| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let xs: Vec<Natural> = xs.iter().map(|x| Natural::from_str(x).unwrap()).collect();
        let m = Natural::from_str(m).unwrap();
        let out: Vec<Natural> = out.iter().map(|y| Natural::from_str(y).unwrap()).collect();
        assert_eq!((&p).mod_evaluate_many(&xs, &m), out);
        assert_eq!(mod_evaluate_many_naive(&p, &xs, &m), out);
    };
    // - a modulus that fits in a limb, evaluated as an UnsignedPolynomial
    test(
        "117*x^31+95*x^30+23*x^29+240*x^28+205*x^27+78*x^26+120*x^25+31*x^24+9*x^23+135*x^22+122*x\
        ^21+197*x^20+214*x^19+49*x^18+134*x^17+19*x^16+11*x^15+121*x^14+28*x^13+17*x^12+152*x^11+9\
        5*x^10+232*x^9+19*x^8+17*x^7+156*x^6+174*x^5+3*x^4+107*x^3+44*x^2+162*x+137",
        &["208", "40", "235", "174", "93"],
        "251",
        &["197", "117", "127", "204", "184"],
    );
    // - a modulus that fits in two limbs, evaluated as an UnsignedPolynomial
    test(
        "96505097581870669024275514819046294930*x^32+141736831045017776715944085801160885025*x^31+\
        251237406706269348683765255763928924110*x^30+19200701001278627790763610950258479269*x^29+9\
        641658674507286906127385503574009992*x^28+86744760193370544283442547781704447827*x^27+2661\
        44825892521973071281264094731258765*x^26+265822643857585087151642789019057304729*x^25+8906\
        7113429321427408098632829062315964*x^24+250989954084915746521721401100656604164*x^23+96245\
        011427952836537989938454048397566*x^22+219574376041992525481072359347750051347*x^21+146267\
        783025962693629693873376379419383*x^20+251435667958079896575325720467039455441*x^19+724317\
        51750090819490339363988472712815*x^18+166577594847680916991784136841900327899*x^17+1648886\
        09649009108285325737535760403723*x^16+2257513792557471703471570384877743741*x^15+251038439\
        50410393411024747701696721869*x^14+191721860843861660843850165098461730601*x^13+2229305375\
        47464841732478652685643426154*x^12+49494833121223189064162504364089706099*x^11+91409313332\
        550537018801343786606386812*x^10+3116030050414354135939363864983399969*x^9+207254973697253\
        538398331405947307335960*x^8+1707842902894236775327376573201873293*x^7+3079576114370730352\
        90570272939011120866*x^6+216247030859407973482124698692355413562*x^5+307255013172349997003\
        993665702081135764*x^4+129415495604692742239941446296827673182*x^3+40848812602500106079557\
        915516694125747*x^2+134390964211511475286259149751838437601*x+2228594444668664160469349313\
        34441712897",
        &[
            "49232340196020189369105514918337006797",
            "250383074368785772768912495786282654253",
            "146535960915565662826023256610715964587",
        ],
        "340282366920938463463374607431768211297",
        &[
            "29726208679498486265878884786090375500",
            "210653207902934587822093465023627559743",
            "283365376383710264314533199411549287144",
        ],
    );
    // - a modulus of more than two limbs and a polynomial of length 32, by the subproduct tree
    test(
        "664498328453551138898461698295700232632810779619793065124458*x^31+98716467313057733720454\
        7193475134977924355227043336218793500*x^30+68201271653705457498046953812937695790475069476\
        6396378778505*x^29+346076862837485608611910476883577891856165015437431024569762*x^28+98805\
        402349193993387563853727785014830537459716952536602741*x^27+366627344460178747168605017042\
        304390628617741502373250000370*x^26+715950152663101639517336210798448582043448058094852774\
        233752*x^25+111873113162126433557230127057574094374862934914529606946176*x^24+567988668104\
        639977791906228034891778041836938215691702935469*x^23+715372921925522330692344136757327911\
        945878488815810123315080*x^22+864891032704469133241061774615783005689626962076095285063321\
        *x^21+854001593623035805652229473830259487858855103032212294179821*x^20+862319490274708202\
        671872637016909932489144735852991554683770*x^19+440322597066450984871243205241278070042366\
        194193299641973063*x^18+304784543043653696245001399248043884137424200507182977861871*x^17+\
        92367864148506130177744893448630325121210094505383685702585*x^16+4529533946698495630921571\
        16648018249128092674298741224470335*x^15+9291017177541995997553982653361932551683333125596\
        79404922561*x^14+789803849802823443269979244481623053238962918334315935749104*x^13+2687071\
        57797562998284707569508391246865165625899195954903024*x^12+7699555077838158344908231561680\
        95047384466371934773009042863*x^11+1940260861780399042690002641851140098116643445461346169\
        417*x^10+864763827323191879090635241008923153741482998902799183701358*x^9+1286988484173007\
        9902573402490993805219601911313915287328460*x^8+730981696236757714434478166723017325553514\
        95541199424998181*x^7+517934711061254725454066306812006225420952161099763095807273*x^6+730\
        880520214453564139626973309674080331035594168673761118179*x^5+3227057638622358051962611117\
        30227771598848372090925218769226*x^4+69636329579692884494702614289809138812227666114835519\
        9733746*x^3+199346572671082865055065708555537809324186083842496384791445*x^2+9215392332799\
        60613472566015748071216194590390151009423029189*x+7788766962465151570028566921186872208856\
        50168980618001359235",
        &[
            "86906342899328056943074838371960959312667955409265347148186",
            "448629675834930765277392947033957742073917243984531387286681",
            "422361931567512635873724194880456526646107264125135007119181",
            "629572953751859945559899757139576644442619562490588511090721",
        ],
        "1000000000000000000000000000000000000000000000000000000000007",
        &[
            "210036311977811197381559256218458920162927675216333906495",
            "663122083038717585216835707927154165228117922563986294404455",
            "457964883451724239074767391528424442844227740718924329944305",
            "767878358262323421970444123092752793172172455588549265737712",
        ],
    );
    // - a modulus of more than two limbs and a short polynomial, by Horner's rule
    test(
        "3*x^2+2*x+1",
        &[
            "123456789123456789123456789",
            "100000000000000000000000000000000000000000000000000000000003",
        ],
        "1000000000000000000000000000000000000000000000000000000000007",
        &[
            "45724736342021035638317335090077739793781440497485142",
            "790000000000000000000000000000000000000000000000000000000027",
        ],
    );
}

#[test]
#[should_panic]
fn mod_evaluate_many_fail_1() {
    // A coefficient is not reduced.
    (&NaturalPolynomial::from_str("5*x+1").unwrap())
        .mod_evaluate_many(&[Natural::ONE], &Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_evaluate_many_fail_2() {
    // A point is not reduced.
    (&NaturalPolynomial::from_str("x+1").unwrap())
        .mod_evaluate_many(&[Natural::ONE, Natural::from(5u32)], &Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_evaluate_many_fail_3() {
    // m is 0.
    (&NaturalPolynomial::ZERO).mod_evaluate_many(&[], &Natural::ZERO);
}

#[test]
fn mod_evaluate_many_properties() {
    natural_polynomial_natural_vec_natural_triple_gen_var_1().test_properties(|(p, xs, m)| {
        let ys = (&p).mod_evaluate_many(&xs, &m);
        assert_eq!((&p).mod_evaluate_many(&xs, m.clone()), ys);
        assert_eq!(ys.len(), xs.len());
        for (x, y) in xs.iter().zip(&ys) {
            assert!(*y < m);
            assert_eq!((&p).mod_evaluate(x, &m), *y);
        }
        assert_eq!(mod_evaluate_many_naive(&p, &xs, &m), ys);
    });

    // Long enough for the dispatch to reach the subproduct tree.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 40);
    config.insert("mean_length_d", 1);
    config.insert("mean_points_n", 40);
    config.insert("mean_points_d", 1);
    natural_polynomial_natural_vec_natural_triple_gen_var_1().test_properties_with_config(
        &config,
        |(p, xs, m)| {
            let ys = (&p).mod_evaluate_many(&xs, &m);
            if m != 1u32 {
                assert_eq!(mod_evaluate_many_fast(p.coefficients_asc(), &xs, &m), ys);
            }
            assert_eq!(mod_evaluate_many_naive(&p, &xs, &m), ys);
        },
    );
}

#[test]
fn test_mod_evaluate_many_tree_preferred() {
    assert!(mod_evaluate_many_tree_preferred(32, 0));
    assert!(mod_evaluate_many_tree_preferred(32, 256));
    assert!(!mod_evaluate_many_tree_preferred(31, 1));
    assert!(!mod_evaluate_many_tree_preferred(32, 257));
    assert!(mod_evaluate_many_tree_preferred(64, 1 << 20));
}
