// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Mod;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{Evaluate, EvaluateMany, EvaluateMod, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::evaluate::{
    evaluate_divide_and_conquer, evaluate_horner,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_gen, integer_polynomial_integer_pair_gen, integer_polynomial_integer_pair_gen_var_2,
    integer_polynomial_integer_vec_pair_gen, integer_polynomial_unsigned_unsigned_triple_gen_var_1,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::evaluate::{
    evaluate_many_naive, evaluate_mod_u64_naive, evaluate_naive,
};

#[test]
fn test_evaluate() {
    let test = |s, x, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let x = Integer::from_str(x).unwrap();
        let y = (&p).evaluate(&x);
        assert_eq!(y.to_string(), out);
        assert_eq!((&p).evaluate(x.clone()), y);
        assert_eq!(evaluate_horner(p.coefficients_asc(), &x), y);
        assert_eq!(evaluate_divide_and_conquer(p.coefficients_asc(), &x), y);
        assert_eq!(evaluate_naive(&p, &x), y);
    };
    test("0", "0", "0");
    test("0", "5", "0");
    test("5", "0", "5");
    test("5", "-3", "5");
    test("x", "7", "7");
    test("x", "-7", "-7");
    test("x^2+3*x+2", "0", "2");
    test("x^2+3*x+2", "1", "6");
    test("x^2+3*x+2", "-1", "0");
    test("x^2+3*x+2", "-2", "0");
    test("x^2+3*x+2", "10", "132");
    test("-x^3+x", "2", "-6");
    test("2*x^5-x", "-3", "-483");
    test(
        "1000000000000000000000*x+1",
        "1000000000000000000000",
        "1000000000000000000000000000000000000000001",
    );
    test("x^100", "2", "1267650600228229401496703205376");
    test("x^100-1", "-1", "0");
    test("x^100-1", "1", "0");
    test("-x^51+x^50", "3", "-1435795975383705177540498");
    test(
        "x^64+x^63+x^62+x^61+x^60+x^59+x^58+x^57+x^56+x^55+x^54+x^53+x^52+x^51+x^50+x^49+x^48+x^47\
        +x^46+x^45+x^44+x^43+x^42+x^41+x^40+x^39+x^38+x^37+x^36+x^35+x^34+x^33+x^32+x^31+x^30+x^29\
        +x^28+x^27+x^26+x^25+x^24+x^23+x^22+x^21+x^20+x^19+x^18+x^17+x^16+x^15+x^14+x^13+x^12+x^11\
        +x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1",
        "2",
        "36893488147419103231",
    );
    test(
        "-60*x^59+59*x^58-58*x^57+57*x^56-56*x^55+55*x^54-54*x^53+53*x^52-52*x^51+51*x^50-50*x^49+\
        49*x^48-48*x^47+47*x^46-46*x^45+45*x^44-44*x^43+43*x^42-42*x^41+41*x^40-40*x^39+39*x^38-38\
        *x^37+37*x^36-36*x^35+35*x^34-34*x^33+33*x^32-32*x^31+31*x^30-30*x^29+29*x^28-28*x^27+27*x\
        ^26-26*x^25+25*x^24-24*x^23+23*x^22-22*x^21+21*x^20-20*x^19+19*x^18-18*x^17+17*x^16-16*x^1\
        5+15*x^14-14*x^13+13*x^12-12*x^11+11*x^10-10*x^9+9*x^8-8*x^7+7*x^6-6*x^5+5*x^4-4*x^3+3*x^2\
        -2*x+1",
        "-7",
        "5066106889042355226407376748809603448230666511559010",
    );
    test(
        "9784*x^99+9587*x^98+9392*x^97+9199*x^96+9008*x^95+8819*x^94+8632*x^93+8447*x^92+8264*x^91\
        +8083*x^90+7904*x^89+7727*x^88+7552*x^87+7379*x^86+7208*x^85+7039*x^84+6872*x^83+6707*x^82\
        +6544*x^81+6383*x^80+6224*x^79+6067*x^78+5912*x^77+5759*x^76+5608*x^75+5459*x^74+5312*x^73\
        +5167*x^72+5024*x^71+4883*x^70+4744*x^69+4607*x^68+4472*x^67+4339*x^66+4208*x^65+4079*x^64\
        +3952*x^63+3827*x^62+3704*x^61+3583*x^60+3464*x^59+3347*x^58+3232*x^57+3119*x^56+3008*x^55\
        +2899*x^54+2792*x^53+2687*x^52+2584*x^51+2483*x^50+2384*x^49+2287*x^48+2192*x^47+2099*x^46\
        +2008*x^45+1919*x^44+1832*x^43+1747*x^42+1664*x^41+1583*x^40+1504*x^39+1427*x^38+1352*x^37\
        +1279*x^36+1208*x^35+1139*x^34+1072*x^33+1007*x^32+944*x^31+883*x^30+824*x^29+767*x^28+712\
        *x^27+659*x^26+608*x^25+559*x^24+512*x^23+467*x^22+424*x^21+383*x^20+344*x^19+307*x^18+272\
        *x^17+239*x^16+208*x^15+179*x^14+152*x^13+127*x^12+104*x^11+83*x^10+64*x^9+47*x^8+32*x^7+1\
        9*x^6+8*x^5-x^4-8*x^3-13*x^2-16*x-17",
        "123456789",
        "11233088674388196659638106735131591839166548083223584752609337477840565263920759467205201\
        792569605062323689106967242747447142719735787704935988060074488242556418098342821416865689\
        089919782161504815853514764869873044444515035189993474293983892651357072996407659905992643\
        370741697506205191413279313299517054957672257373531426095292237506291594818469678780098989\
        926572878993093480071655931849110899831120462935877745542469464218352312688993920936452050\
        494412396196484060948698541230081486603196996630328122818305356600433091601255228498382535\
        348704462211149100175714658354611477366969883180664273267520510824913404473291092274169854\
        120387534200186218496993372669172389310944542158471295824345459646532687811319382642589462\
        110839631441644713386357192784175490905703838543133970901163980134681850567601726986050",
    );
    test(
        "100000000000000000050*x^50+100000000000000000049*x^49+100000000000000000048*x^48+10000000\
        0000000000047*x^47+100000000000000000046*x^46+100000000000000000045*x^45+10000000000000000\
        0044*x^44+100000000000000000043*x^43+100000000000000000042*x^42+100000000000000000041*x^41\
        +100000000000000000040*x^40+100000000000000000039*x^39+100000000000000000038*x^38+10000000\
        0000000000037*x^37+100000000000000000036*x^36+100000000000000000035*x^35+10000000000000000\
        0034*x^34+100000000000000000033*x^33+100000000000000000032*x^32+100000000000000000031*x^31\
        +100000000000000000030*x^30+100000000000000000029*x^29+100000000000000000028*x^28+10000000\
        0000000000027*x^27+100000000000000000026*x^26+100000000000000000025*x^25+10000000000000000\
        0024*x^24+100000000000000000023*x^23+100000000000000000022*x^22+100000000000000000021*x^21\
        +100000000000000000020*x^20+100000000000000000019*x^19+100000000000000000018*x^18+10000000\
        0000000000017*x^17+100000000000000000016*x^16+100000000000000000015*x^15+10000000000000000\
        0014*x^14+100000000000000000013*x^13+100000000000000000012*x^12+100000000000000000011*x^11\
        +100000000000000000010*x^10+100000000000000000009*x^9+100000000000000000008*x^8+1000000000\
        00000000007*x^7+100000000000000000006*x^6+100000000000000000005*x^5+100000000000000000004*\
        x^4+100000000000000000003*x^3+100000000000000000002*x^2+100000000000000000001*x+1000000000\
        00000000000",
        "-10000000000",
        "99999999990000000050999999995000000000489999999952000000004699999999540000000044999999995\
        600000000429999999958000000004099999999600000000038999999996200000000369999999964000000003\
        499999999660000000032999999996800000000309999999970000000002899999999720000000026999999997\
        400000000249999999976000000002299999999780000000020999999998000000000189999999982000000001\
        699999999840000000014999999998600000000129999999988000000001099999999900000000008999999999\
        20000000006999999999400000000049999999996000000000299999999990000000000",
    );
}

// Every length from 0 through 130, which covers every residue of the length modulo small powers of
// 2 and so every path through the divide-and-conquer loops, at points of both signs. The point with
// more than one limb sends the longer polynomials down the divide-and-conquer path.
#[test]
fn test_evaluate_all_lengths() {
    for len in 0..=130 {
        let p = IntegerPolynomial::from_coefficients_asc(
            (1..=len)
                .map(|i| Integer::from(i) * Integer::from(if i % 3 == 0 { -1 } else { 1 }))
                .collect(),
        );
        for x in [
            "-3",
            "-1",
            "0",
            "1",
            "2",
            "1000000007",
            "-1000000000000000000000000000000000000000000000000000000000000",
        ] {
            let x = Integer::from_str(x).unwrap();
            let y = evaluate_naive(&p, &x);
            assert_eq!(evaluate_horner(p.coefficients_asc(), &x), y);
            assert_eq!(evaluate_divide_and_conquer(p.coefficients_asc(), &x), y);
            assert_eq!((&p).evaluate(&x), y);
        }
    }
}

fn evaluate_properties_helper(p: &IntegerPolynomial, x: &Integer) {
    let y = p.evaluate(x);
    assert_eq!(p.evaluate(x.clone()), y);
    assert_eq!(evaluate_horner(p.coefficients_asc(), x), y);
    assert_eq!(evaluate_divide_and_conquer(p.coefficients_asc(), x), y);
    assert_eq!(evaluate_naive(p, x), y);

    // Evaluation respects congruence: p(x) and p(x mod m) agree modulo m.
    let m = Integer::from(1000003);
    assert_eq!((&y).mod_op(&m), p.evaluate(x.mod_op(&m)).mod_op(&m));
}

#[test]
fn evaluate_properties() {
    integer_polynomial_integer_pair_gen().test_properties(|(p, x)| {
        evaluate_properties_helper(&p, &x);
    });

    integer_polynomial_integer_pair_gen_var_2().test_properties(|(p, x)| {
        evaluate_properties_helper(&p, &x);
    });

    integer_polynomial_integer_pair_gen().test_properties(|(p, _)| {
        let cs = p.coefficients_asc();
        // p(0) is the constant term, p(1) the sum of the coefficients, and p(-1) their alternating
        // sum.
        assert_eq!(p.evaluate(Integer::ZERO), *p.coefficient(0));
        assert_eq!(p.evaluate(Integer::ONE), cs.iter().sum::<Integer>());
        assert_eq!(
            p.evaluate(Integer::NEGATIVE_ONE),
            cs.iter()
                .enumerate()
                .map(|(i, c)| if i % 2 == 0 { c.clone() } else { -c })
                .sum::<Integer>()
        );
    });

    integer_gen().test_properties(|x| {
        assert_eq!(IntegerPolynomial::ZERO.evaluate(&x), 0u32);
        assert_eq!(IntegerPolynomial::one().evaluate(&x), 1u32);
        assert_eq!(IntegerPolynomial::x().evaluate(&x), x);
        // A constant polynomial evaluates to itself everywhere.
        assert_eq!(
            IntegerPolynomial::from(x.clone()).evaluate(Integer::from(7)),
            x
        );
    });
}

#[test]
fn test_evaluate_mod_u64() {
    let test = |s, x: u64, m: u64, out: u64| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        assert_eq!((&p).evaluate_mod(x, m), out);
        assert_eq!(evaluate_mod_u64_naive(&p, x, m), out);
    };
    // - the zero polynomial
    test("0", 0, 1, 0);
    test("0", 3, 7, 0);
    // - x == 0, so only the constant term is reduced
    // A negative coefficient is reduced into [0, m).
    test("-1", 0, 7, 6);
    test("-7", 0, 7, 0);
    test("-5*x^2+3*x-7", 0, 11, 4);
    // Everything is 0 mod 1.
    test("x^3-1", 0, 1, 0);
    // - Horner, since the polynomial is short
    test("-5*x^2+3*x-7", 6, 11, 7);
    // The coefficients need not be reduced.
    test("100*x+1", 3, 10, 1);
    // - lazy Shoup
    test(
        "-1000000000000000000000000*x^2+999999999999999999999*x-1",
        123456789,
        1000000007,
        204882185,
    );
    // - Shoup, not lazy
    test(
        "-340282366920938463463374607431768211456*x^3+12345",
        9223372036854775782,
        9223372036854775783,
        14845,
    );
    // - Horner, since the top bit of m is set
    test(
        "-x^4+x^3-x^2+x-1",
        18446744073709551556,
        18446744073709551557,
        18446744073709551552,
    );
}

#[test]
#[should_panic]
fn evaluate_mod_u64_fail_1() {
    // m is 0.
    (&IntegerPolynomial::from_str("x+1").unwrap()).evaluate_mod(0, 0);
}

#[test]
#[should_panic]
fn evaluate_mod_u64_fail_2() {
    // x is not reduced.
    (&IntegerPolynomial::from_str("x+1").unwrap()).evaluate_mod(7, 7);
}

#[test]
fn evaluate_mod_u64_properties() {
    integer_polynomial_unsigned_unsigned_triple_gen_var_1().test_properties(|(p, x, m)| {
        let y = (&p).evaluate_mod(x, m);
        assert!(y < m);
        assert_eq!(evaluate_mod_u64_naive(&p, x, m), y);

        // Reducing the coefficients first, into a `NaturalPolynomial`, gives the same value.
        let m_natural = Natural::from(m);
        assert_eq!(
            (&p).mod_op(m_natural.clone())
                .evaluate_mod(Natural::from(x), m_natural),
            y
        );

        // p(0) is the constant term mod m.
        assert_eq!(
            (&p).evaluate_mod(0, m),
            u64::exact_from(&p.coefficient(0).mod_op(Integer::from(m)))
        );
        // Everything is 0 mod 1.
        if m == 1 {
            assert_eq!(y, 0);
        }
    });
}

#[test]
fn test_evaluate_many() {
    let test = |s, xs: &[i32], out: &[i32]| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let xs: Vec<Integer> = xs.iter().map(|&x| Integer::from(x)).collect();
        let out: Vec<Integer> = out.iter().map(|&y| Integer::from(y)).collect();
        assert_eq!((&p).evaluate_many(&xs), out);
        assert_eq!(evaluate_many_naive(&p, &xs), out);
    };
    test("x^2-3*x+2", &[], &[]);
    test("0", &[-1, 5], &[0, 0]);
    test("x^2-3*x+2", &[-1, 0, 1, 2, 3], &[6, 2, 0, 0, 2]);
}

#[test]
fn evaluate_many_properties() {
    integer_polynomial_integer_vec_pair_gen().test_properties(|(p, xs)| {
        let ys = (&p).evaluate_many(&xs);
        assert_eq!(ys.len(), xs.len());
        for (x, y) in xs.iter().zip(&ys) {
            assert_eq!((&p).evaluate(x), *y);
        }
        assert_eq!(evaluate_many_naive(&p, &xs), ys);
    });
}
