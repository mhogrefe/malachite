// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Mod, ModIsReduced};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::{ModCompose, ModComposeAssign, ModEvaluate, Polynomial};
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_compose::{
    mod_compose_divide_and_conquer, mod_compose_horner,
};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_compose::*;

fn check(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    let r = p.mod_compose(q, m);
    assert!(r.is_valid());
    assert!(r.mod_is_reduced(m));
    assert_eq!(p.mod_compose(q.clone(), m), r);
    assert_eq!(p.clone().mod_compose(q, m), r);
    assert_eq!(p.clone().mod_compose(q.clone(), m), r);
    let mut s = p.clone();
    s.mod_compose_assign(q, m);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_compose_assign(q.clone(), m);
    assert_eq!(s, r);
    if *p != NaturalPolynomial::ZERO && *q != NaturalPolynomial::ZERO {
        // Both kernels, whatever the lengths and the modulus.
        assert_eq!(mod_compose_horner(p.coefficients_asc(), q, m), r);
        assert_eq!(
            mod_compose_divide_and_conquer(p.coefficients_asc(), q, m),
            r
        );
    }
    r
}

#[test]
fn test_mod_compose() {
    let test = |s, t, m, out| {
        let m = Natural::from_str(m).unwrap();
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let r = check(&p, &q, &m);
        assert_eq!(r.to_string(), out);
        assert_eq!(mod_compose_naive(&p, &q, &m), r);
    };
    test("0", "0", "1", "0");
    test("0", "2*x+1", "3", "0");
    test("5", "0", "7", "5");
    test("5", "3*x^2+2*x+1", "7", "5");
    test("3*x^2+2*x+1", "0", "7", "1");
    test("3*x^2+2*x+1", "0", "7", "1");
    test("3*x^2+2*x+1", "4", "7", "1");
    test("3*x^2+2*x+1", "5", "6", "2");
    test("x", "5*x^2+4*x+3", "7", "5*x^2+4*x+3");
    test("3*x^2+2*x+1", "x", "7", "3*x^2+2*x+1");
    test("3*x^2+2*x+1", "x+1", "7", "3*x^2+x+6");
    test("x^2+2*x+3", "2*x+5", "7", "4*x^2+3*x+3");
    test("x^2", "x+1", "2", "x^2+1");
    test(
        "4*x^3+3*x^2+2*x+1",
        "x^2+2",
        "251",
        "4*x^6+27*x^4+62*x^2+49",
    );
    test("5*x^4+4*x^3+3*x^2+2*x+1", "x+3", "7", "5*x^4+x^3+x^2+3*x+1");
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "6*x+3",
        "7",
        "5*x^4+6*x^3+x^2+4*x+1",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "2*x+3",
        "12",
        "8*x^4+8*x^3+4*x+7",
    );
    test("x^2+1", "2*x", "4", "1");
    test("x^3+1", "3*x", "9", "1");
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2",
        "251",
        "5*x^8+4*x^6+3*x^4+2*x^2+1",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+x+1",
        "255",
        "5*x^8+20*x^7+54*x^6+92*x^5+122*x^4+114*x^3+85*x^2+40*x+15",
    );
    test("x^4+1", "3*x^2+2*x+1", "6", "3*x^8+4*x^4+2*x^3+2*x+2");
    test(
        "x^5+x^3+x",
        "x^2+250*x+1",
        "251",
        "x^10+246*x^9+15*x^8+221*x^7+46*x^6+197*x^5+51*x^4+214*x^3+22*x^2+242*x+3",
    );
    test(
        "7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "254*x^2+2",
        "255",
        "7*x^12+165*x^10+230*x^8+126*x^6+12*x^4+249*x^2+4",
    );
    test(
        "x^8",
        "x^2+x+1",
        "10",
        "x^16+8*x^15+6*x^14+2*x^13+6*x^12+4*x^11+4*x^10+6*x^9+7*x^8+6*x^7+4*x^6+4*x^5+6*x^4+2*x^3+\
        6*x^2+8*x+1",
    );
    test(
        "x^9+1",
        "x^2+x",
        "12",
        "x^18+9*x^17+6*x^14+6*x^13+9*x^10+x^9+1",
    );
    test(
        "9*x^10+2*x^9+4*x^8+7*x^7+x^6+5*x^3+2*x+3",
        "x^3+2*x^2+3*x+1",
        "30",
        "9*x^30+2*x^27+21*x^26+18*x^25+25*x^24+10*x^23+10*x^22+19*x^21+16*x^20+25*x^19+26*x^18+7*x\
        ^17+22*x^16+20*x^15+5*x^14+2*x^13+28*x^12+7*x^11+13*x^10+5*x^9+3*x^8+18*x^7+2*x^6+13*x^5+\
        10*x^4+8*x^3+28*x^2+6*x+3",
    );
    test(
        "17*x^16+16*x^15+15*x^14+14*x^13+13*x^12+12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*\
        x^4+4*x^3+3*x^2+2*x+1",
        "250*x^2+2*x+1",
        "251",
        "17*x^32+209*x^31+91*x^30+250*x^29+126*x^28+165*x^27+55*x^26+50*x^25+33*x^24+134*x^23+30*x\
        ^22+62*x^21+135*x^20+83*x^19+54*x^18+11*x^17+84*x^16+39*x^15+126*x^14+66*x^13+58*x^12+220\
        *x^11+74*x^10+50*x^9+102*x^8+140*x^7+241*x^6+21*x^5+222*x^4+10*x^3+199*x^2+x+153",
    );
    test(
        "x^11+x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1",
        "2*x^2+2*x",
        "8",
        "4*x^4+6*x^2+2*x+1",
    );
    test("3*x^2+7", "4*x+2", "9", "3*x^2+3*x+1");
    // Moduli larger than a limb.
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+7*x+3",
        "1267650600228229401496703205379",
        "5*x^8+140*x^7+1534*x^6+8204*x^5+21722*x^4+26278*x^3+15809*x^2+4676*x+547",
    );
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "3*x^2+633825300114114700748351602689*x+5",
        "1267650600228229401496703205379",
        "649539*x^20+1267650600228229401496702122814*x^19+950737950171172051122539238438*x^18+1267\
        650600228229401496686310804*x^17+1109194275199700726309710636310*x^16+7922816251426433759\
        35322923960*x^15+594211218856982531952027536793*x^14+1029966112685436388715604444901*x^13\
        +1203277718185389627203309604291*x^12+272346808642783660476611089123*x^11+127507824046394\
        168317405309977*x^10+831895706399775544730178184274*x^9+604114739171265574154705712494*x^\
        8+554597137599850363152514715064*x^7+574404178228416447556935350155*x^6+95073795017117205\
        1120873154541*x^5+950737950171172051124829663259*x^4+1267650600228229401496010390552*x^3+\
        316912650057057350375002265044*x^2+1267650600228229401496574878963*x+131225586",
    );
    test(
        "3*x^2+2*x+1",
        "x+2",
        "18446744073709551617",
        "3*x^2+14*x+17",
    );
    test(
        "1180591620717411303424*x^2+x+633825300114114700748351602688",
        "2*x+633825300114114700748351602693",
        "1267650600228229401496703205379",
        "4722366482869645213696*x^2+16528282690043758247938*x+14462247353788288466946",
    );
    test(
        "9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "18446744073709551616*x^2+2",
        "18446744073709551617",
        "9*x^16+18446744073709551465*x^14+1127*x^12+18446744073709546823*x^10+12805*x^8+1844674407\
        3709529605*x^6+23811*x^4+18446744073709536771*x^2+4097",
    );
    test(
        "1024*x^7+1",
        "1152921504606846976*x+3",
        "10000000000000000000000000",
        "1210961403571828974157824*x^7+3261266066189998729723904*x^6+9230997132767909994561536*x^5\
        +3395115888866842304839680*x^4+2412544421263884461015040*x^3+6214402486897252732239872*x^\
        2+6024559040520949881372672*x+2239489",
    );
    test(
        "3*x^2+2*x+1",
        "1180591620717411303424",
        "1267650600228229401496703205379",
        "2361183231539217956865",
    );
    test("3*x^2+2*x+1", "0", "1267650600228229401496703205379", "1");
    test(
        "633825300114114700748351602688",
        "x^2+1",
        "1267650600228229401496703205379",
        "633825300114114700748351602688",
    );
    test(
        "x^7+x^6+x^5+x^4+x^3+x^2+x+1",
        "2000000000000*x^3+5000000000000*x",
        "10000000000000000000000000",
        "4000000000000000000000000*x^6+2000000000000*x^3+5000000000000000000000000*x^2+50000000000\
        00*x+1",
    );
}

#[test]
#[should_panic]
fn mod_compose_fail_1() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1").unwrap().mod_compose(
        NaturalPolynomial::from_str("x+1").unwrap(),
        Natural::from(8u32),
    );
}

#[test]
#[should_panic]
fn mod_compose_fail_2() {
    // A coefficient of the inner polynomial is not reduced.
    NaturalPolynomial::from_str("x+1").unwrap().mod_compose(
        NaturalPolynomial::from_str("8*x").unwrap(),
        Natural::from(8u32),
    );
}

#[test]
#[should_panic]
fn mod_compose_assign_fail() {
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    p.mod_compose_assign(
        NaturalPolynomial::from_str("x+1").unwrap(),
        Natural::from(8u32),
    );
}

#[test]
fn mod_compose_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            let r = check(&p, &q, &m);
            // It is the exact composition, reduced.
            assert_eq!(mod_compose_naive(&p, &q, &m), r);
            // The degree is at most the product of the degrees.
            if p.len() > 1 && q.len() > 1 {
                assert!(r.len() <= (p.len() - 1) * (q.len() - 1) + 1);
            }
            // Evaluating p(q) at a point is evaluating p at q's value there.
            for x in [Natural::ZERO, Natural::ONE, Natural::from(3u32)] {
                let x = x.mod_op(&m);
                assert_eq!(
                    (&r).mod_evaluate(&x, &m),
                    (&p).mod_evaluate(&(&q).mod_evaluate(&x, &m), &m)
                );
            }
            // Composition is associative.
            assert_eq!(
                (&r).mod_compose(&q, &m),
                (&p).mod_compose((&q).mod_compose(&q, &m), &m)
            );
            if m > 1u32 {
                // x is an identity on both sides.
                let x = NaturalPolynomial::from_coefficients_asc(vec![Natural::ZERO, Natural::ONE]);
                assert_eq!((&p).mod_compose(&x, &m), p);
                assert_eq!((&x).mod_compose(&p, &m), p);
            }
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<Limb>().test_properties(
        |(p, q, m)| {
            // It agrees with the UnsignedPolynomial implementation.
            let r = (&p).mod_compose(&q, m);
            assert_eq!(
                NaturalPolynomial::from(p)
                    .mod_compose(NaturalPolynomial::from(q), Natural::from(m)),
                NaturalPolynomial::from(r)
            );
        },
    );
}
