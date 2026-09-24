// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Pow, Reciprocal};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::IsInteger;
use malachite_base::polynomial::{Evaluate, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_integer_pair_gen;
use malachite_q::Rational;
use malachite_q::rational_polynomial::arithmetic::evaluate::{
    evaluate_integer_polynomial_divide_and_conquer, evaluate_integer_polynomial_horner,
};
use malachite_q::test_util::generators::{
    integer_polynomial_rational_pair_gen, integer_polynomial_rational_pair_gen_var_1,
};
use malachite_q::test_util::rational_polynomial::arithmetic::evaluate::*;

#[test]
fn test_evaluate_integer_polynomial_rational() {
    let test = |s, x, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let x = Rational::from_str(x).unwrap();
        let y = (&p).evaluate(&x);
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);
        assert_eq!((&p).evaluate(x.clone()), y);
        assert_eq!(
            evaluate_integer_polynomial_horner(p.coefficients_asc(), &x),
            y
        );
        assert_eq!(
            evaluate_integer_polynomial_divide_and_conquer(p.coefficients_asc(), &x),
            y
        );
        assert_eq!(evaluate_integer_polynomial_naive(&p, &x), y);
    };
    test("0", "0", "0");
    test("0", "1/2", "0");
    test("5", "0", "5");
    test("5", "-3/7", "5");
    test("x", "1/2", "1/2");
    test("x", "-7/3", "-7/3");
    test("x^2+3*x+2", "0", "2");
    test("x^2+3*x+2", "1", "6");
    test("x^2+3*x+2", "-1", "0");
    test("x^2+3*x+2", "1/2", "15/4");
    test("x^2+3*x+2", "-2/3", "4/9");
    test("4*x^2-1", "1/2", "0");
    test("4*x^2-1", "-1/2", "0");
    test("3*x^2-2*x+5", "2/3", "5");
    test("2*x^5-x", "-3/2", "-219/16");
    test(
        "1000000000000000000000*x+1",
        "1/1000000000000000000000",
        "2",
    );
    test("x^100", "1/2", "1/1267650600228229401496703205376");
    test("x^100-1", "-1", "0");
    test(
        "-x^51+x^50",
        "3/5",
        "1435795975383705177540498/444089209850062616169452667236328125",
    );
    test(
        "x^64+x^63+x^62+x^61+x^60+x^59+x^58+x^57+x^56+x^55+x^54+x^53+x^52+x^51+x^50+x^49+x^48+x^47\
        +x^46+x^45+x^44+x^43+x^42+x^41+x^40+x^39+x^38+x^37+x^36+x^35+x^34+x^33+x^32+x^31+x^30+x^29\
        +x^28+x^27+x^26+x^25+x^24+x^23+x^22+x^21+x^20+x^19+x^18+x^17+x^16+x^15+x^14+x^13+x^12+x^11\
        +x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1",
        "1/2",
        "36893488147419103231/18446744073709551616",
    );
    test(
        "-60*x^59+59*x^58-58*x^57+57*x^56-56*x^55+55*x^54-54*x^53+53*x^52-52*x^51+51*x^50-50*x^49+\
        49*x^48-48*x^47+47*x^46-46*x^45+45*x^44-44*x^43+43*x^42-42*x^41+41*x^40-40*x^39+39*x^38-38\
        *x^37+37*x^36-36*x^35+35*x^34-34*x^33+33*x^32-32*x^31+31*x^30-30*x^29+29*x^28-28*x^27+27*x\
        ^26-26*x^25+25*x^24-24*x^23+23*x^22-22*x^21+21*x^20-20*x^19+19*x^18-18*x^17+17*x^16-16*x^1\
        5+15*x^14-14*x^13+13*x^12-12*x^11+11*x^10-10*x^9+9*x^8-8*x^7+7*x^6-6*x^5+5*x^4-4*x^3+3*x^2\
        -2*x+1",
        "-7/3",
        "2508357937401890366278306875355211178514115109779830/4710128697246244834921603689",
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
        "2/3",
        "-3607642645124092491947916080347518664151166126775/17179250691067044367882037658854042423\
        4035840667",
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
        "-1/10",
        "113636363636363636363533057851239669421487603305785237603305785123967/1250000000000000000\
        000000000000000000000000000000",
    );
    test(
        "984770902183611232881*x^44+328256967394537077627*x^43+109418989131512359209*x^42+36472996\
        377170786403*x^41+12157665459056928801*x^40+4052555153018976267*x^39+1350851717672992089*x\
        ^38+450283905890997363*x^37+150094635296999121*x^36+50031545098999707*x^35+166771816996665\
        69*x^34+5559060566555523*x^33+1853020188851841*x^32+617673396283947*x^31+205891132094649*x\
        ^30+68630377364883*x^29+22876792454961*x^28+7625597484987*x^27+2541865828329*x^26+84728860\
        9443*x^25+282429536481*x^24+94143178827*x^23+31381059609*x^22+10460353203*x^21+3486784401*\
        x^20+1162261467*x^19+387420489*x^18+129140163*x^17+43046721*x^16+14348907*x^15+4782969*x^1\
        4+1594323*x^13+531441*x^12+177147*x^11+59049*x^10+19683*x^9+6561*x^8+2187*x^7+729*x^6+243*\
        x^5+81*x^4+27*x^3+9*x^2+3*x+1",
        "1/3",
        "45",
    );
}

// Every length from 0 through 130, which covers every residue of the length modulo small powers of
// 2 and so every path through the divide-and-conquer loops, at points of both signs and several
// denominators. The one-limb point with a 64-bit denominator sends lengths 128 through 130 down the
// divide-and-conquer path, and the 200-bit point sends lengths from 32 on; the other points stay on
// Horner's rule, or on the evaluation at an Integer.
#[test]
fn test_evaluate_integer_polynomial_rational_all_lengths() {
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
            "1/2",
            "-2/3",
            "5/7",
            "-1000000007/3",
            "1/1000003",
            "1/18446744073709551557",
            "1/1000000000000000000000000000000000000000000000000000000000000",
        ] {
            let x = Rational::from_str(x).unwrap();
            let y = evaluate_integer_polynomial_naive(&p, &x);
            assert_eq!(
                evaluate_integer_polynomial_horner(p.coefficients_asc(), &x),
                y
            );
            assert_eq!(
                evaluate_integer_polynomial_divide_and_conquer(p.coefficients_asc(), &x),
                y
            );
            assert_eq!((&p).evaluate(&x), y);
        }
    }
}

fn evaluate_integer_polynomial_rational_properties_helper(p: &IntegerPolynomial, x: &Rational) {
    let y = p.evaluate(x);
    assert!(y.is_valid());
    assert_eq!(p.evaluate(x.clone()), y);
    assert_eq!(
        evaluate_integer_polynomial_horner(p.coefficients_asc(), x),
        y
    );
    assert_eq!(
        evaluate_integer_polynomial_divide_and_conquer(p.coefficients_asc(), x),
        y
    );
    assert_eq!(evaluate_integer_polynomial_naive(p, x), y);

    if let Some(d) = p.degree() {
        // Clearing the denominator of x from every term leaves an integer.
        assert!((&y * Rational::from(x.denominator_ref().pow(d))).is_integer());
        // Reversing the coefficients evaluates at the reciprocal: x^d p(1/x) is the reversal.
        if *x != 0u32 {
            assert_eq!(p.reverse(d + 1).evaluate(x.reciprocal()), &y / x.pow(d));
        }
    }
}

#[test]
fn evaluate_integer_polynomial_rational_properties() {
    integer_polynomial_rational_pair_gen().test_properties(|(p, x)| {
        evaluate_integer_polynomial_rational_properties_helper(&p, &x);
    });

    integer_polynomial_rational_pair_gen_var_1().test_properties(|(p, x)| {
        evaluate_integer_polynomial_rational_properties_helper(&p, &x);
    });

    integer_polynomial_rational_pair_gen().test_properties(|(p, _)| {
        let cs = p.coefficients_asc();
        // p(0) is the constant term, p(1) the sum of the coefficients, and p(-1) their alternating
        // sum.
        assert_eq!(p.evaluate(Rational::ZERO), Rational::from(p.coefficient(0)));
        assert_eq!(
            p.evaluate(Rational::ONE),
            Rational::from(cs.iter().sum::<Integer>())
        );
        assert_eq!(
            p.evaluate(Rational::NEGATIVE_ONE),
            Rational::from(
                cs.iter()
                    .enumerate()
                    .map(|(i, c)| if i % 2 == 0 { c.clone() } else { -c })
                    .sum::<Integer>()
            )
        );
    });

    // At an integer, evaluation agrees with evaluation at an Integer.
    integer_polynomial_integer_pair_gen().test_properties(|(p, x)| {
        assert_eq!(
            p.evaluate(Rational::from(&x)),
            Rational::from(p.evaluate(&x))
        );
    });
}
