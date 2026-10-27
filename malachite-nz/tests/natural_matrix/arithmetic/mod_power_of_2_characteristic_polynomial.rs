// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    CharacteristicPolynomial, ModPowerOf2, ModPowerOf2CharacteristicPolynomial,
    ModPowerOf2SimilarityTransform,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_matrix_unsigned_natural_unsigned_quadruple_gen_var_1,
    natural_matrix_unsigned_pair_gen_var_4,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::{
    mod_power_of_2_characteristic_polynomial::*,
};
use std::panic::catch_unwind;

// Checks the characteristic polynomial modulo 2^pow of `a` by value and by reference, against
// Berkowitz's algorithm, against that of the transpose, and against the characteristic polynomial
// of the corresponding `IntegerMatrix`, reduced modulo 2^pow, and returns it.
fn verify(a: &NaturalMatrix, pow: u64) -> NaturalPolynomial {
    let p = a.mod_power_of_2_characteristic_polynomial(pow);
    assert!(p.is_valid());
    assert_eq!(a.clone().mod_power_of_2_characteristic_polynomial(pow), p);
    assert_eq!(
        NaturalPolynomial::from_coefficients_asc(
            mod_power_of_2_characteristic_polynomial_berkowitz(
                a.elements_ref(),
                usize::exact_from(a.height()),
                pow
            )
        ),
        p
    );
    assert_eq!(
        a.transpose().mod_power_of_2_characteristic_polynomial(pow),
        p
    );
    let coefficients = IntegerMatrix::from(a)
        .characteristic_polynomial()
        .into_coefficients_asc()
        .into_iter()
        .map(|c| c.mod_power_of_2(pow))
        .collect();
    assert_eq!(NaturalPolynomial::from_coefficients_asc(coefficients), p);
    p
}

#[test]
fn test_mod_power_of_2_characteristic_polynomial() {
    let test = |s, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        assert_eq!(verify(&a, pow).to_string(), out);
    };
    // - dimension 0
    test("[]_{0,0}", 3, "1");
    // - modulo 1
    test("[0]", 0, "0");
    // - modulo 1, dimension 3
    test("[0, 0, 0; 0, 0, 0; 0, 0, 0]", 0, "0");
    // - dimension 1
    test("[5]", 3, "x+3");
    // - modulo 2
    test("[1, 1; 1, 0]", 1, "x^2+x+1");
    // - dimension 2
    test("[1, 2; 3, 4]", 3, "x^2+3*x+6");
    // - a pivot of smaller valuation lower down: a row and column swap
    test("[1, 2, 3; 4, 5, 6; 7, 0, 1]", 4, "x^3+9*x^2+14*x+8");
    // - an even pivot that divides the elements below it
    test("[0, 1, 1; 2, 0, 1; 4, 1, 0]", 3, "x^3+x+2");
    // - zeros below the subdiagonal entry: nothing to clear
    test("[1, 2, 3; 0, 4, 5; 0, 6, 7]", 3, "x^3+4*x^2+x+2");
    // - a zero column below the diagonal: block triangular
    test(
        "[1, 2, 3, 4; 0, 5, 6, 7; 0, 0, 1, 2; 0, 0, 3, 4]",
        4,
        "x^4+5*x^3+x^2+3*x+6",
    );
    // - modulo 2^64, on single limbs
    test(
        "[7816759979465426170, 6605407279706790158, 3541722407261185182, 259655754482422499; 19535\
        05775214600398, 13097339534544522437, 13543766959282107621, 14218276157327355758; 14445688\
        816656446545, 11455270527606957331, 13562241285837953686, 10569718541743550357; 8744982034\
        414591169, 3017607418922330719, 1518667627101345808, 2011542381109682302]",
        64,
        "x^4+405604966461518637*x^3+9262703837385376924*x^2+1636464734084660022*x+1543984985208685\
        0428",
    );
    // - modulo 2^65, on double limbs
    test(
        "[10127142104026649026, 2758401833107437422, 14458725242473908183, 22837556075298172491; 1\
        0933306642100184666, 7065743250665641678, 17985694867015480289, 30083893164290130598; 4309\
        67736113605800, 17962252319458503058, 6385203097676985184, 32986980663044453065; 356601187\
        2238356041, 23322528937163927046, 3983594920067682244, 4668176573245385408]",
        65,
        "x^4+8647223121804441936*x^3+29624202140340298971*x^2+10523967045934781397*x+6935625700994\
        635070",
    );
    // - modulo 2^128, the full width of a double limb
    test(
        "[4008190921717078259511689021199215063, 303643778824897107906356114893751284418, 25376400\
        7257254899304016204614647877346; 139385242124229092375761676601761820829, 3690235449004502\
        7696457264662177654747, 84509882015544649435283892633934419752; 27206771068125154413834537\
        8415293742994, 1341388034955210434380411030862964148, 325516490105637916164996015037598312\
        814]",
        128,
        "x^3+314137698324476904805784246142561240288*x^2+117760348785041905670108033322736254059*x\
        +177069265307413654309145325860240442522",
    );
    // - modulo 2^129, on Naturals
    test(
        "[563037030050164558018300367084881598841, 368117371373993937572750749209218465541, 189153\
        047127773168401500038186027686207; 298297007852390547303050630513453574404, 54617822869906\
        3547404270807910368540308, 478900739924024168779345646442185755569; 3548437295587581888027\
        15646996393094534, 523625848320801408685062870594956211836, 602189742455022078345210530737\
        088182663]",
        129,
        "x^3+330289200321380597012465938858270946924*x^2+213264058935753323491885186037415561477*x\
        +169359441075418275609877989542455669238",
    );
    // - modulo 2^200
    test(
        "[356858644622741422177459808300759802346099488832326235944489, 13476613707137586488342605\
        6092804993195298973979550668743138, 543628464901108409335932896077131546001186740973986627\
        842203, 509025803995949865234521534289409092441971823988590643746029, 10231177447742264098\
        38375089132743554603128436735235627985891; 12199961605915504752957789603078430394041963311\
        49044802862111, 1575800579237122176648954252442842067347390922614670222372791, 12590952782\
        4787597423669315658466287736459477605898154297399, 108689765576504489562615364011177061165\
        262319245415648123853, 1228033914036068801624260140766942684936246912399113557104326; 1126\
        052647293615953604666453409170613259857635979918702328970, 8619225237893900153689480573571\
        50959566627419491911243900998, 73686866804099429243337608949790708868606447648477962713433\
        , 809555534385455807603544088483778387910954566688995498427207, 26256664042467139825632223\
        2405710863984440459783809294771975; 230226795335213529845416018771988760302013106516138023\
        188142, 1570639711729732067667951201285121564571841546881546977262564, 1491387115305394782\
        51859071206515222149606859490504043629529, 13368702045938310537658533166232306644151940969\
        06900662762685, 918504247612218208083673464774432120274334594717223196358183; 666417653512\
        827057049638994175978753989929176410982202393177, 5030652952703723683429759830550735116542\
        44113023055450749663, 551189481739729550419924282702192209048927638624184876785583, 138415\
        2414653168047995456728793672997268326284489409582059988, 472337657003127265330323617929673\
        738597563744556339439789559]",
        200,
        "x^5+1005260180516049479459957672777190825991754280789663982321171*x^4+3546374943736767733\
        22481888419968106616742352322301358050389*x^3+50493808819844471605120601701700274219897372\
        4920123194005121*x^2+883670861043779394393467011275055820701844572230900678089629*x+155226\
        868656922087190236311922479716618236293066490596178668",
    );
    // - elements divisible by 2^10: the coefficient of x^(n-k) is divisible by 2^(10k)
    test(
        "[697713427147750389161197263784547498594583297781115768105984, 10776725261721778019084536\
        03268558604576386114190920430074880, 30926647184552720862272023479295586427534859962012100\
        0204288, 225931326588681414717226681828927524643879641430267215471616; 1145998740105870093\
        35067886770356549915354807410324811095040, 33558265408979091369076304895746759663573664964\
        2934258783232, 410151414022283364295788404959194813672869252606842609517568, 5716036736002\
        10921072685673630708152271011154721612776728576; 15416520004111328119053140070931340107249\
        33092222842437664768, 1327950541145704260869906295432139650239687230407263690702848, 56940\
        6836057104006138149109185328135821656659813104187614208, 645760076519294093907325615034626\
        431557766879864102912729088; 1032814038773437553009838345856277728747621200932967775815680\
        , 1189631066794691271321572889150979130729773002565409712258048, 4094958168648543567818979\
        25857934993985069047053130508736512, 91295133182770208914583129153371619119632240331277913\
        5320064]",
        200,
        "x^4+698221839395633152947983471221265782796106977015652320779264*x^3+39899018958026032230\
        2205430872162085665034112000059703820288*x^2+216663590408237028250694122166502462538324276\
        701178730905600*x+632368158280443946536475818605147571700277763975118447443968",
    );
}

#[test]
fn mod_power_of_2_characteristic_polynomial_fail() {
    // - not square
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_power_of_2_characteristic_polynomial(3));
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!((&a).mod_power_of_2_characteristic_polynomial(3));
    // - not reduced
    let a = NaturalMatrix::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_power_of_2_characteristic_polynomial(3));
    let a = NaturalMatrix::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!((&a).mod_power_of_2_characteristic_polynomial(3));
}

#[test]
fn mod_power_of_2_characteristic_polynomial_properties() {
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if a.is_square() {
            verify(&a, pow);
        }
    });

    natural_matrix_unsigned_natural_unsigned_quadruple_gen_var_1().test_properties(
        |(a, r, d, pow)| {
            // Similar matrices have the same characteristic polynomial.
            assert_eq!(
                (&a).mod_power_of_2_similarity_transform(r, &d, pow)
                    .mod_power_of_2_characteristic_polynomial(pow),
                verify(&a, pow)
            );
        },
    );

    // The word-sized characteristic polynomials, including modulo 2^64, agree with those of the
    // corresponding NaturalMatrix.
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if !a.is_square() {
            return;
        }
        let coefficients = (&a)
            .mod_power_of_2_characteristic_polynomial(pow)
            .into_coefficients_asc()
            .into_iter()
            .map(Natural::from)
            .collect();
        assert_eq!(
            NaturalPolynomial::from_coefficients_asc(coefficients),
            NaturalMatrix::from(a).mod_power_of_2_characteristic_polynomial(pow)
        );
    });
}
