// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2IsReduced};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::{Evaluate, EvaluateModPowerOf2, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::evaluate::{
    evaluate_divide_and_conquer, evaluate_horner,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_gen, natural_polynomial_natural_pair_gen, natural_polynomial_natural_pair_gen_var_2,
    natural_polynomial_natural_unsigned_triple_gen_var_1,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::evaluate::*;

#[test]
fn test_evaluate() {
    let test = |s, x, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let x = Natural::from_str(x).unwrap();
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
    test("5", "3", "5");
    test("x", "7", "7");
    test("x^2+3*x+2", "0", "2");
    test("x^2+3*x+2", "1", "6");
    test("x^2+3*x+2", "10", "132");
    test("2*x^5+x", "3", "489");
    test(
        "1000000000000000000000*x+1",
        "1000000000000000000000",
        "1000000000000000000000000000000000000000001",
    );
    test("x^100", "2", "1267650600228229401496703205376");
    test("x^100+1", "1", "2");
    test("x^51+x^50", "3", "2871591950767410355080996");
    test(
        "x^64+x^63+x^62+x^61+x^60+x^59+x^58+x^57+x^56+x^55+x^54+x^53+x^52+x^51+x^50+x^49+x^48+x^47\
        +x^46+x^45+x^44+x^43+x^42+x^41+x^40+x^39+x^38+x^37+x^36+x^35+x^34+x^33+x^32+x^31+x^30+x^29\
        +x^28+x^27+x^26+x^25+x^24+x^23+x^22+x^21+x^20+x^19+x^18+x^17+x^16+x^15+x^14+x^13+x^12+x^11\
        +x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1",
        "2",
        "36893488147419103231",
    );
    test(
        "60*x^59+59*x^58+58*x^57+57*x^56+56*x^55+55*x^54+54*x^53+53*x^52+52*x^51+51*x^50+50*x^49+4\
        9*x^48+48*x^47+47*x^46+46*x^45+45*x^44+44*x^43+43*x^42+42*x^41+41*x^40+40*x^39+39*x^38+38*\
        x^37+37*x^36+36*x^35+35*x^34+34*x^33+33*x^32+32*x^31+31*x^30+30*x^29+29*x^28+28*x^27+27*x^\
        26+26*x^25+25*x^24+24*x^23+23*x^22+22*x^21+21*x^20+20*x^19+19*x^18+18*x^17+17*x^16+16*x^15\
        +15*x^14+14*x^13+13*x^12+12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+\
        2*x+1",
        "7",
        "5066106889042355226407376748809603448230666511559010",
    );
    test(
        "9818*x^99+9621*x^98+9426*x^97+9233*x^96+9042*x^95+8853*x^94+8666*x^93+8481*x^92+8298*x^91\
        +8117*x^90+7938*x^89+7761*x^88+7586*x^87+7413*x^86+7242*x^85+7073*x^84+6906*x^83+6741*x^82\
        +6578*x^81+6417*x^80+6258*x^79+6101*x^78+5946*x^77+5793*x^76+5642*x^75+5493*x^74+5346*x^73\
        +5201*x^72+5058*x^71+4917*x^70+4778*x^69+4641*x^68+4506*x^67+4373*x^66+4242*x^65+4113*x^64\
        +3986*x^63+3861*x^62+3738*x^61+3617*x^60+3498*x^59+3381*x^58+3266*x^57+3153*x^56+3042*x^55\
        +2933*x^54+2826*x^53+2721*x^52+2618*x^51+2517*x^50+2418*x^49+2321*x^48+2226*x^47+2133*x^46\
        +2042*x^45+1953*x^44+1866*x^43+1781*x^42+1698*x^41+1617*x^40+1538*x^39+1461*x^38+1386*x^37\
        +1313*x^36+1242*x^35+1173*x^34+1106*x^33+1041*x^32+978*x^31+917*x^30+858*x^29+801*x^28+746\
        *x^27+693*x^26+642*x^25+593*x^24+546*x^23+501*x^22+458*x^21+417*x^20+378*x^19+341*x^18+306\
        *x^17+273*x^16+242*x^15+213*x^14+186*x^13+161*x^12+138*x^11+117*x^10+98*x^9+81*x^8+66*x^7+\
        53*x^6+42*x^5+33*x^4+26*x^3+21*x^2+18*x+17",
        "123456789",
        "11272124346402862226822055002451805006164768168677155253992094690087348941850340407059751\
        269334375094532325065293972983593992817388853167630623870017660297504613703464134701626182\
        477789599315874234410693190467936702209984255685704902683309727393249187628781634029018502\
        588280542713690192124271702206035686109683450156552174931718758061550884762448158819596795\
        912435752893331494397385244133022188554384190613767967182738927800292634953823073228272091\
        229761095587680255404764510387184538170247932370334455025149090296543199231032445306671278\
        334942970561737259906692916121327986141714983850522586611720378997095429222436463227755242\
        634934326731353366490301063485041893597295346660443175924059646556798986928468823905760993\
        444391430839005571311018023001184969111670815598927836492704496065582699776382728749050",
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
        "10000000000",
        "10000000001000000005100000000500000000049000000004800000000470000000046000000004500000000\
        440000000043000000004200000000410000000040000000003900000000380000000037000000003600000000\
        350000000034000000003300000000320000000031000000003000000000290000000028000000002700000000\
        260000000025000000002400000000230000000022000000002100000000200000000019000000001800000000\
        170000000016000000001500000000140000000013000000001200000000110000000010000000000900000000\
        080000000007000000000600000000050000000004000000000300000000010000000000",
    );
}

// Every length from 0 through 130, which covers every path through the divide-and-conquer loops.
// The point with more than one limb sends the longer polynomials down the divide-and-conquer path.
#[test]
fn test_evaluate_all_lengths() {
    for len in 0..=130u32 {
        let p = NaturalPolynomial::from_coefficients_asc((1..=len).map(Natural::from).collect());
        for x in [
            "0",
            "1",
            "2",
            "3",
            "1000000007",
            "1000000000000000000000000000000000000000000000000000000000000",
        ] {
            let x = Natural::from_str(x).unwrap();
            let y = evaluate_naive(&p, &x);
            assert_eq!(evaluate_horner(p.coefficients_asc(), &x), y);
            assert_eq!(evaluate_divide_and_conquer(p.coefficients_asc(), &x), y);
            assert_eq!((&p).evaluate(&x), y);
        }
    }
}

fn evaluate_properties_helper(p: &NaturalPolynomial, x: &Natural) {
    let y = p.evaluate(x);
    assert_eq!(p.evaluate(x.clone()), y);
    assert_eq!(evaluate_horner(p.coefficients_asc(), x), y);
    assert_eq!(evaluate_divide_and_conquer(p.coefficients_asc(), x), y);
    assert_eq!(evaluate_naive(p, x), y);
    // Evaluating as an IntegerPolynomial at an Integer gives the same value.
    assert_eq!(
        IntegerPolynomial::from(p.clone()).evaluate(Integer::from(x)),
        Integer::from(&y)
    );
}

#[test]
fn evaluate_properties() {
    natural_polynomial_natural_pair_gen().test_properties(|(p, x)| {
        evaluate_properties_helper(&p, &x);
    });

    natural_polynomial_natural_pair_gen_var_2().test_properties(|(p, x)| {
        evaluate_properties_helper(&p, &x);
    });

    natural_polynomial_natural_pair_gen().test_properties(|(p, _)| {
        // p(0) is the constant term, and p(1) the sum of the coefficients.
        assert_eq!(p.evaluate(Natural::ZERO), *p.coefficient(0));
        assert_eq!(
            p.evaluate(Natural::ONE),
            p.coefficients_asc().iter().sum::<Natural>()
        );
    });

    natural_gen().test_properties(|x| {
        assert_eq!(NaturalPolynomial::ZERO.evaluate(&x), 0u32);
        assert_eq!(NaturalPolynomial::one().evaluate(&x), 1u32);
        assert_eq!(NaturalPolynomial::x().evaluate(&x), x);
        // A constant polynomial evaluates to itself everywhere.
        assert_eq!(
            NaturalPolynomial::from(x.clone()).evaluate(Natural::from(7u32)),
            x
        );
    });
}

#[test]
fn test_evaluate_mod_power_of_2() {
    let test = |s, x, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let x = Natural::from_str(x).unwrap();
        let y = (&p).evaluate_mod_power_of_2(&x, pow);
        assert!(y.is_valid());
        assert_eq!(y.to_string(), out);
        assert_eq!((&p).evaluate_mod_power_of_2(x.clone(), pow), y);
        assert_eq!(p.clone().evaluate_mod_power_of_2(&x, pow), y);
        assert_eq!(p.clone().evaluate_mod_power_of_2(x.clone(), pow), y);
        assert_eq!(evaluate_mod_power_of_2_naive(&p, &x, pow), y);
    };
    test("0", "0", 0, "0");
    test("0", "5", 3, "0");
    test("7", "0", 3, "7");
    test("7", "5", 3, "7");
    test("x", "5", 3, "5");
    test("5*x^2+3*x+7", "6", 4, "13");
    test("5*x^2+3*x+7", "0", 4, "7");
    test("5*x^2+3*x+7", "1", 4, "15");
    test("5*x^2+3*x+7", "15", 4, "9");
    test("x^2+x+1", "1", 1, "1");
    test("x^3+1", "1", 2, "2");
    test("x^100+1", "3", 8, "210");
    test("255*x^3+255*x+255", "255", 8, "1");
    test(
        "18446744073709551615*x^2+1",
        "18446744073709551615",
        64,
        "0",
    );
    test(
        "18446744073709551615*x^2+1",
        "18446744073709551615",
        100,
        "55340232221128654848",
    );
    test(
        "123456789012345678901234567890*x+1",
        "98765432109876543210",
        128,
        "209858559491276873124708178321624481781",
    );
}

#[test]
#[should_panic]
fn evaluate_mod_power_of_2_fail_1() {
    // A coefficient is not reduced.
    (&NaturalPolynomial::from_str("16*x+1").unwrap()).evaluate_mod_power_of_2(&Natural::ONE, 4);
}

#[test]
#[should_panic]
fn evaluate_mod_power_of_2_fail_2() {
    // The value is not reduced.
    (&NaturalPolynomial::from_str("x+1").unwrap())
        .evaluate_mod_power_of_2(&Natural::from(16u32), 4);
}

#[test]
#[should_panic]
fn evaluate_mod_power_of_2_fail_3() {
    NaturalPolynomial::from_str("16*x+1")
        .unwrap()
        .evaluate_mod_power_of_2(Natural::ONE, 4);
}

#[test]
#[should_panic]
fn evaluate_mod_power_of_2_fail_4() {
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .evaluate_mod_power_of_2(Natural::from(16u32), 4);
}

#[test]
fn evaluate_mod_power_of_2_properties() {
    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, x, pow)| {
        let y = (&p).evaluate_mod_power_of_2(&x, pow);
        assert!(y.is_valid());
        assert!(y.mod_power_of_2_is_reduced(pow));
        assert_eq!((&p).evaluate_mod_power_of_2(x.clone(), pow), y);
        assert_eq!(p.clone().evaluate_mod_power_of_2(&x, pow), y);
        assert_eq!(p.clone().evaluate_mod_power_of_2(x.clone(), pow), y);
        assert_eq!(evaluate_mod_power_of_2_naive(&p, &x, pow), y);
        assert_eq!((&p).evaluate(&x).mod_power_of_2(pow), y);

        // Reducing further agrees with evaluating the reduced polynomial at the reduced value.
        for smaller in [0, pow >> 1, pow.saturating_sub(1)] {
            assert_eq!(
                (&y).mod_power_of_2(smaller),
                (&p).mod_power_of_2(smaller)
                    .evaluate_mod_power_of_2((&x).mod_power_of_2(smaller), smaller)
            );
        }

        // p(0) is the constant term, and p(1) the sum of the coefficients, mod 2^pow.
        assert_eq!(
            (&p).evaluate_mod_power_of_2(Natural::ZERO, pow),
            *p.coefficient(0)
        );
        if pow != 0 {
            assert_eq!(
                (&p).evaluate_mod_power_of_2(Natural::ONE, pow),
                p.coefficients_asc()
                    .iter()
                    .sum::<Natural>()
                    .mod_power_of_2(pow)
            );
        }
        // Everything is 0 mod 2^0.
        if pow == 0 {
            assert_eq!(y, 0u32);
        }
    });
}
