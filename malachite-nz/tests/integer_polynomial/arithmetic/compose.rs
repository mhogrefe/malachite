// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::{Compose, ComposeAssign, Evaluate, Polynomial, TaylorShift};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::compose::{
    compose_divide_and_conquer, compose_horner,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_integer_pair_gen, integer_polynomial_pair_gen,
    integer_polynomial_pair_gen_var_1, integer_polynomial_pair_gen_var_2,
    integer_polynomial_triple_gen,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::compose::*;

fn check(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
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
    if p != &IntegerPolynomial::ZERO && q != &IntegerPolynomial::ZERO {
        for kernel in [compose_horner, compose_divide_and_conquer] {
            assert_eq!(
                IntegerPolynomial::from_coefficients_asc(kernel(
                    p.coefficients_asc(),
                    q.coefficients_asc()
                )),
                r
            );
        }
    }
    r
}

#[test]
fn test_compose() {
    let test = |s, t, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from_str(t).unwrap();
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
    test("3*x^2+2*x+1", "-1", "2");
    test("x", "5*x^2+4*x+3", "5*x^2+4*x+3");
    test("3*x^2+2*x+1", "x", "3*x^2+2*x+1");
    test("3*x^2+2*x+1", "x+1", "3*x^2+8*x+6");
    test("x^2", "x+1", "x^2+2*x+1");
    test("4*x^3+3*x^2+2*x+1", "x^2-2", "4*x^6-21*x^4+38*x^2-23");
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x+3",
        "5*x^4+64*x^3+309*x^2+668*x+547",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "-x+3",
        "5*x^4-64*x^3+309*x^2-668*x+547",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "2*x+3",
        "80*x^4+512*x^3+1236*x^2+1336*x+547",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "-2*x+3",
        "80*x^4-512*x^3+1236*x^2-1336*x+547",
    );
    test("-x^5+x^4-x^3+x^2-x+1", "-x", "x^5+x^4+x^3+x^2+x+1");
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
        "x^2-x+1",
        "x^10-5*x^9+15*x^8-30*x^7+46*x^6-54*x^5+51*x^4-37*x^3+22*x^2-9*x+3",
    );
    test(
        "7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "-x^2+2",
        "7*x^12-90*x^10+485*x^8-1404*x^6+2307*x^4-2046*x^2+769",
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
        "9*x^10+2*x^9-4*x^8+7*x^7+x^6+5*x^3-2*x+3",
        "x^3+2*x^2-3*x+1",
        "9*x^30+180*x^29+1350*x^28+3872*x^27-3339*x^26-39780*x^25-21781*x^24+218096*x^23+150506*x^\
        22-938177*x^21-215430*x^20+3127817*x^19-1662294*x^18-6346497*x^17+9910496*x^16+1625314*x^\
        15-19084669*x^14+21450730*x^13-2450294*x^12-22380443*x^11+34885373*x^10-31335543*x^9+2001\
        0709*x^8-9679638*x^7+3632260*x^6-1062937*x^5+240674*x^4-41298*x^3+5166*x^2-432*x+21",
    );
    test(
        "x^16+x^15+x^14+x^13+x^12+x^11+x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1",
        "-x^2+2*x+1",
        "x^32-32*x^31+463*x^30-3970*x^29+22116*x^28-81704*x^27+190056*x^26-209040*x^25-182324*x^24\
        +931296*x^23-823884*x^22-1173944*x^21+2551240*x^20+495616*x^19-4074904*x^18+326256*x^17+4\
        866166*x^16-236640*x^15-4686934*x^14-897804*x^13+3270528*x^12+1934192*x^11-1030744*x^10-1\
        587664*x^9-488852*x^8+318240*x^7+400724*x^6+212296*x^5+71128*x^4+16320*x^3+2584*x^2+272*x\
        +17",
    );
    test(
        "5*x^4+x^3-1180591620717411303424*x^2+3*x+18446744073709551616",
        "x^2+7*x-36893488147419103232",
        "5*x^8+140*x^7-737869762948382063169*x^6-15495265021916023350559*x^5+408338840305126155058\
        46825653240353271672*x^4+571674376427176618347302202601840186491223*x^3-10043362776618689\
        20121670315883296953266822116796558320074749*x^2-7030353943633082454857714433648905828577\
        347334257721489752043*x+92633671389852956322285239425529489066089503655244211893054926495\
        74176712032256",
    );
    // - more than 512 bits: divide and conquer from 3 coefficients
    // - more than 128 bits: Horner's rule up to 4 coefficients
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
        "1606938044258990275541962092341162602522202993782792835301376*x^3-x+5",
        "x^2-x+1427247692705959881058285969449495136382746624",
        "1606938044258990275541962092341162602522202993782792835301376*x^6-48208141327769708266258\
        86277023487807566608981348378505904128*x^5+6880495847970214534832462685906260822389694972\
        325657503989737344786654905288942231776664640678755565568000*x^4-137609916959404290696649\
        25371812521644779389936616624786684523311863499348872071450942314312443546954629120*x^3+9\
        82017182368842561003956909048279745664992614500969021642008863912086407512028448703950524\
        4482555842665426570543448841293106193847711838490362642431999*x^2-98201718236884256100395\
        69090482797456649926138129194368449874104288401389214023664649810276977712471452660052382\
        680213027651769637656179159985582768127*x+46719391924451279038885597242370983166906659914\
        22525368913093138328857771221685643782054718788988313373329725003360673086145233634688448\
        510210725191394120081077336586663853022915413677724139520005",
    );
}

fn compose_properties_helper(p: &IntegerPolynomial, q: &IntegerPolynomial) {
    let r = check(p, q);
    assert_eq!(compose_naive(p, q), r);
    // The degree of p(q) is the product of the degrees, unless q is constant.
    if p.len() > 1 && q.len() > 1 {
        assert_eq!(r.len() - 1, (p.len() - 1) * (q.len() - 1));
    }
    // Evaluating p(q) at a point is evaluating p at q's value there.
    for x in [Integer::ZERO, Integer::ONE, Integer::from(-2)] {
        assert_eq!((&r).evaluate(&x), p.evaluate(&q.evaluate(&x)));
    }
}

#[test]
fn compose_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        compose_properties_helper(&p, &q);
    });

    integer_polynomial_pair_gen_var_1().test_properties(|(p, q)| {
        compose_properties_helper(&p, &q);
    });

    integer_polynomial_pair_gen_var_2().test_properties(|(p, q)| {
        compose_properties_helper(&p, &q);
        // Composing with a linear polynomial b x + a is a Taylor shift by a followed by scaling the
        // coefficient of x^i by b^i.
        let r = (&p).compose(&q);
        assert_eq!(
            r,
            p.taylor_shift(q.coefficient(0))
                .compose(IntegerPolynomial::from_coefficients_asc(vec![
                    Integer::ZERO,
                    q.coefficient(1).clone()
                ]))
        );
    });

    integer_polynomial_gen().test_properties(|p| {
        let x = IntegerPolynomial::from_coefficients_asc(vec![Integer::ZERO, Integer::ONE]);
        // x is an identity on both sides.
        assert_eq!((&p).compose(&x), p);
        assert_eq!((&x).compose(&p), p);
        // Composing with 0 gives the constant term.
        assert_eq!(
            (&p).compose(IntegerPolynomial::ZERO),
            IntegerPolynomial::from(p.coefficient(0).clone())
        );
        assert_eq!(IntegerPolynomial::ZERO.compose(&p), IntegerPolynomial::ZERO);
    });

    integer_polynomial_integer_pair_gen().test_properties(|(p, c)| {
        // Composing with a constant evaluates.
        assert_eq!(
            (&p).compose(IntegerPolynomial::from(c.clone())),
            IntegerPolynomial::from((&p).evaluate(&c))
        );
        // Composing a constant with anything gives the constant.
        assert_eq!(
            IntegerPolynomial::from(c.clone()).compose(&p),
            IntegerPolynomial::from(c)
        );
    });

    integer_polynomial_triple_gen().test_properties(|(p, q, s)| {
        // Composition is associative.
        assert_eq!((&p).compose(&q).compose(&s), (&p).compose((&q).compose(&s)));
        // It distributes over addition and multiplication on the left.
        assert_eq!((&p + &q).compose(&s), (&p).compose(&s) + (&q).compose(&s));
        assert_eq!((&p * &q).compose(&s), (&p).compose(&s) * (&q).compose(&s));
    });
}
