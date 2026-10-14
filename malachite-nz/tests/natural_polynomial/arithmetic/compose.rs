// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::polynomial::{Compose, ComposeAssign, Evaluate, Polynomial, TaylorShift};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_natural_pair_gen, natural_polynomial_pair_gen,
    natural_polynomial_pair_gen_var_1, natural_polynomial_pair_gen_var_2,
    natural_polynomial_triple_gen,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::compose::*;

fn check(p: &NaturalPolynomial, q: &NaturalPolynomial) -> NaturalPolynomial {
    let r = p.compose(q);
    assert!(r.is_valid());
    assert_eq!(p.compose(q.clone()), r);
    assert_eq!(p.clone().compose(q), r);
    assert_eq!(p.clone().compose(q.clone()), r);
    let mut s = p.clone();
    s.compose_assign(q);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.compose_assign(q.clone());
    assert_eq!(s, r);
    // It agrees with the IntegerPolynomial implementation.
    assert_eq!(
        IntegerPolynomial::from(p.clone()).compose(IntegerPolynomial::from(q.clone())),
        IntegerPolynomial::from(r.clone())
    );
    r
}

#[test]
fn test_compose() {
    let test = |s, t, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let r = check(&p, &q);
        assert_eq!(r.to_string(), out);
        assert_eq!(compose_naive(&p, &q), r);
    };
    test("0", "0", "0");
    test("0", "2*x+1", "0");
    test("5", "0", "5");
    test("5", "3*x^2+2*x+1", "5");
    test("3*x^2+2*x+1", "0", "1");
    test("3*x^2+2*x+1", "0", "1");
    test("3*x^2+2*x+1", "4", "57");
    test("x", "5*x^2+4*x+3", "5*x^2+4*x+3");
    test("3*x^2+2*x+1", "x", "3*x^2+2*x+1");
    test("3*x^2+2*x+1", "x+1", "3*x^2+8*x+6");
    test("x^2", "x+1", "x^2+2*x+1");
    test("4*x^3+3*x^2+2*x+1", "x^2+2", "4*x^6+27*x^4+62*x^2+49");
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x+3",
        "5*x^4+64*x^3+309*x^2+668*x+547",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "2*x+3",
        "80*x^4+512*x^3+1236*x^2+1336*x+547",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "2*x",
        "80*x^4+32*x^3+12*x^2+4*x+1",
    );
    test("x^5+x^4+x^3+x^2+x+1", "x", "x^5+x^4+x^3+x^2+x+1");
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2",
        "5*x^8+4*x^6+3*x^4+2*x^2+1",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+x+1",
        "5*x^8+20*x^7+54*x^6+92*x^5+122*x^4+114*x^3+85*x^2+40*x+15",
    );
    test(
        "x^4+1",
        "3*x^2+2*x+1",
        "81*x^8+216*x^7+324*x^6+312*x^5+214*x^4+104*x^3+36*x^2+8*x+2",
    );
    test(
        "x^5+x^3+x",
        "x^2+x+1",
        "x^10+5*x^9+15*x^8+30*x^7+46*x^6+54*x^5+51*x^4+37*x^3+22*x^2+9*x+3",
    );
    test(
        "7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+2",
        "7*x^12+90*x^10+485*x^8+1404*x^6+2307*x^4+2046*x^2+769",
    );
    test(
        "x^8",
        "x^2+x+1",
        "x^16+8*x^15+36*x^14+112*x^13+266*x^12+504*x^11+784*x^10+1016*x^9+1107*x^8+1016*x^7+784*x^\
        6+504*x^5+266*x^4+112*x^3+36*x^2+8*x+1",
    );
    test(
        "x^9+1",
        "x^2+x",
        "x^18+9*x^17+36*x^16+84*x^15+126*x^14+126*x^13+84*x^12+36*x^11+9*x^10+x^9+1",
    );
    test(
        "9*x^10+2*x^9+4*x^8+7*x^7+x^6+5*x^3+2*x+3",
        "x^3+2*x^2+3*x+1",
        "9*x^30+180*x^29+1890*x^28+13592*x^27+74421*x^26+328068*x^25+1204675*x^24+3768400*x^23+101\
        99290*x^22+24149119*x^21+50417986*x^20+93336895*x^19+153799106*x^18+226096567*x^17+296843\
        152*x^16+348019760*x^15+363913475*x^14+338617592*x^13+279414598*x^12+203519647*x^11+13007\
        5813*x^10+72407285*x^9+34781853*x^8+14253648*x^7+4911332*x^6+1396093*x^5+318970*x^4+56438\
        *x^3+7318*x^2+636*x+33",
    );
    test(
        "x^16+x^15+x^14+x^13+x^12+x^11+x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1",
        "x^2+2*x+1",
        "x^32+32*x^31+497*x^30+4990*x^29+36396*x^28+205464*x^27+933976*x^26+3511664*x^25+11132876*\
        x^24+30185504*x^23+70757132*x^22+144583496*x^21+259187048*x^20+409609664*x^19+572756312*x\
        ^18+710454896*x^17+783051638*x^16+767502944*x^15+668955542*x^14+518056436*x^13+355862768*\
        x^12+216258224*x^11+115837592*x^10+54422576*x^9+22282988*x^8+7885280*x^7+2385644*x^6+6083\
        28*x^5+128248*x^4+21760*x^3+2856*x^2+272*x+17",
    );
    test(
        "5*x^4+x^3+1180591620717411303424*x^2+3*x+18446744073709551616",
        "x^2+7*x+36893488147419103232",
        "5*x^8+140*x^7+737869762948382066111*x^6+15495265021916023364321*x^5+408338840305126157253\
        63080130384017502072*x^4+571674376427176618889636478368901004001623*x^3+10043362776618689\
        24305782299543156300012476289777580248137731*x^2+7030353943633082456134453874336266943491\
        928861341715819134997*x+92633671389852956355428336588371163499142471809780690570077576851\
        80071449460736",
    );
    test(
        "x^4+4149515568880992958512407863691161151012446232242436899995657329690652811412908146399\
        70704894710379428819788661130078918239515107541177530788687483411396368706118180340150952\
        3685376*x+1",
        "x^2+3",
        "x^8+12*x^6+54*x^4+41495155688809929585124078636911611510124462322424368999956573296906528\
        11412908146399707048947103794288197886611300789182395151075411775307886874834113963687061\
        181803401509523685484*x^2+124485467066429788755372235910734834530373386967273106999869719\
        89071958434238724439199121146841311382864593659833902367547185453226235325923660624502341\
        891061183545410204528571056210",
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301376*x^3+x+5",
        "x^2+x+1427247692705959881058285969449495136382746624",
        "1606938044258990275541962092341162602522202993782792835301376*x^6+48208141327769708266258\
        86277023487807566608981348378505904128*x^5+6880495847970214534832462685906260822389694972\
        325657503989737344786654905288942231776664640678755565568000*x^4+137609916959404290696649\
        25371812521644779389936616624786684523311863499348872071450942314312443546954629120*x^3+9\
        82017182368842561003956909048279745664992614500969021642008863912086407512028448703950524\
        4482555842665426570543448841293106193847711838490362642432001*x^2+98201718236884256100395\
        69090482797456649926138129194368449874104288401389214023664649810276977712471452660052382\
        680213027651769637656179159985582768129*x+46719391924451279038885597242370983166906659914\
        22525368913093138328857771221685643782054718788988313373329725003360673086145233634688448\
        510210725191396974576462748506425969594854312667996905013253",
    );
}

fn compose_properties_helper(p: &NaturalPolynomial, q: &NaturalPolynomial) {
    let r = check(p, q);
    assert_eq!(compose_naive(p, q), r);
    // The degree of p(q) is the product of the degrees, unless q is constant.
    if p.len() > 1 && q.len() > 1 {
        assert_eq!(r.len() - 1, (p.len() - 1) * (q.len() - 1));
    }
    // Evaluating p(q) at a point is evaluating p at q's value there.
    for x in [Natural::ZERO, Natural::ONE, Natural::TWO] {
        assert_eq!((&r).evaluate(&x), p.evaluate(&q.evaluate(&x)));
    }
}

#[test]
fn compose_properties() {
    natural_polynomial_pair_gen().test_properties(|(p, q)| {
        compose_properties_helper(&p, &q);
    });

    natural_polynomial_pair_gen_var_1().test_properties(|(p, q)| {
        compose_properties_helper(&p, &q);
    });

    natural_polynomial_pair_gen_var_2().test_properties(|(p, q)| {
        compose_properties_helper(&p, &q);
        // Composing with a linear polynomial b x + a is a Taylor shift by a followed by composing
        // with b x.
        let b_x =
            NaturalPolynomial::from_coefficients_asc(vec![Natural::ZERO, q.coefficient(1).clone()]);
        assert_eq!(
            (&p).compose(&q),
            p.taylor_shift(q.coefficient(0)).compose(b_x)
        );
    });

    natural_polynomial_gen().test_properties(|p| {
        let x = NaturalPolynomial::from_coefficients_asc(vec![Natural::ZERO, Natural::ONE]);
        // x is an identity on both sides.
        assert_eq!((&p).compose(&x), p);
        assert_eq!((&x).compose(&p), p);
        // Composing with 0 gives the constant term.
        assert_eq!(
            (&p).compose(NaturalPolynomial::ZERO),
            NaturalPolynomial::from(p.coefficient(0).clone())
        );
        assert_eq!(NaturalPolynomial::ZERO.compose(&p), NaturalPolynomial::ZERO);
    });

    natural_polynomial_natural_pair_gen().test_properties(|(p, c)| {
        // Composing with a constant evaluates.
        assert_eq!(
            (&p).compose(NaturalPolynomial::from(c.clone())),
            NaturalPolynomial::from((&p).evaluate(&c))
        );
        // Composing a constant with anything gives the constant.
        assert_eq!(
            NaturalPolynomial::from(c.clone()).compose(&p),
            NaturalPolynomial::from(c)
        );
    });

    natural_polynomial_triple_gen().test_properties(|(p, q, s)| {
        // Composition is associative.
        assert_eq!((&p).compose(&q).compose(&s), (&p).compose((&q).compose(&s)));
        // It distributes over addition and multiplication on the left.
        assert_eq!((&p + &q).compose(&s), (&p).compose(&s) + (&q).compose(&s));
        assert_eq!((&p * &q).compose(&s), (&p).compose(&s) * (&q).compose(&s));
    });
}
