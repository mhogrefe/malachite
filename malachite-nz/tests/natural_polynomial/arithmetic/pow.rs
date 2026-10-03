// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Pow, PowAssign, Square};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::polynomial::{Content, Evaluate, Polynomial};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::pow::binexp::pow_to_out_binexp;
use malachite_nz::integer_polynomial::arithmetic::pow::binomial::pow_to_out_binomial;
use malachite_nz::integer_polynomial::arithmetic::pow::multinomial::*;
use malachite_nz::integer_polynomial::arithmetic::pow::small::pow_to_out_small;
use malachite_nz::integer_polynomial::arithmetic::pow::{
    pow_ref_with_kernel, pow_to_out, pow_to_out_addchains_e,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_gen, natural_polynomial_gen, natural_polynomial_unsigned_pair_gen_var_5,
    natural_polynomial_unsigned_pair_gen_var_6,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mul::natural_generated_coefficients;
use malachite_nz::test_util::natural_polynomial::arithmetic::pow::pow_naive;

// Checks every kernel that applies to the coefficients `xs` and the exponent `e` against `out`.
fn verify_kernels(xs: &[Natural], e: u64, out: &[Natural]) {
    if xs.len() < 2 || e < 3 {
        return;
    }
    assert_eq!(pow_ref_with_kernel(xs, e, pow_to_out), out);
    assert_eq!(pow_ref_with_kernel(xs, e, pow_to_out_binexp), out);
    assert_eq!(pow_ref_with_kernel(xs, e, pow_to_out_multinomial), out);
    assert_eq!(
        pow_ref_with_kernel(xs, e, pow_to_out_multinomial_multiples),
        out
    );
    assert_eq!(
        pow_ref_with_kernel(xs, e, pow_to_out_multinomial_split),
        out
    );
    if e < 5 {
        assert_eq!(pow_ref_with_kernel(xs, e, pow_to_out_small), out);
    }
    if xs.len() == 2 {
        assert_eq!(pow_ref_with_kernel(xs, e, pow_to_out_binomial), out);
    }
    if e <= 148 {
        assert_eq!(pow_ref_with_kernel(xs, e, pow_to_out_addchains_e), out);
    }
}

#[test]
fn test_pow() {
    let test = |s, e: u64, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let power = p.clone().pow(e);
        assert!(power.is_valid());
        assert_eq!(power.to_string(), out);
        assert_eq!((&p).pow(e).to_string(), out);
        let mut q = p.clone();
        q.pow_assign(e);
        assert_eq!(q.to_string(), out);
        assert_eq!(pow_naive(&p, e).to_string(), out);
        verify_kernels(p.coefficients_asc(), e, power.coefficients_asc());
    };
    test("0", 0, "1");
    test("x+1", 0, "1");
    test("0", 5, "0");
    test("3", 5, "243");
    test("3*x^2", 4, "81*x^8");
    test("x^2+x", 1, "x^2+x");
    test("x+1", 2, "x^2+2*x+1");
    test("x+1", 3, "x^3+3*x^2+3*x+1");
    test(
        "5*x^4+x^3+4*x^2+x+3",
        3,
        "125*x^12+75*x^11+315*x^10+196*x^9+507*x^8+261*x^7+472*x^6+213*x^5+309*x^4+100*x^3+117*x^2+\
        27*x+27",
    );
    test(
        "3*x^3+2*x+1",
        4,
        "81*x^12+216*x^10+108*x^9+216*x^8+216*x^7+150*x^6+144*x^5+88*x^4+44*x^3+24*x^2+8*x+1",
    );
    test("x+1", 5, "x^5+5*x^4+10*x^3+10*x^2+5*x+1");
    test(
        "2*x+3",
        6,
        "64*x^6+576*x^5+2160*x^4+4320*x^3+4860*x^2+2916*x+729",
    );
    test(
        "x^2+x+1",
        5,
        "x^10+5*x^9+15*x^8+30*x^7+45*x^6+51*x^5+45*x^4+30*x^3+15*x^2+5*x+1",
    );
    test(
        "x^2+1",
        10,
        "x^20+10*x^18+45*x^16+120*x^14+210*x^12+252*x^10+210*x^8+120*x^6+45*x^4+10*x^2+1",
    );
    test(
        "x^3+x+1",
        7,
        "x^21+7*x^19+7*x^18+21*x^17+42*x^16+56*x^15+105*x^14+140*x^13+175*x^12+231*x^11+245*x^10+\
        252*x^9+252*x^8+211*x^7+168*x^6+126*x^5+77*x^4+42*x^3+21*x^2+7*x+1",
    );
    test(
        "3*x^2+7*x+5",
        9,
        "19683*x^18+413343*x^17+4153113*x^16+26515188*x^15+120490578*x^14+413720622*x^13+\
        1112139882*x^12+2392703712*x^11+4176093537*x^10+5956716997*x^9+6960155895*x^8+\
        6646399200*x^7+5148795750*x^6+3192288750*x^5+1549518750*x^4+568312500*x^3+148359375*x^2+\
        24609375*x+1953125",
    );
    test("x^3+x^2", 5, "x^15+5*x^14+10*x^13+10*x^12+5*x^11+x^10");
    // Generated coefficients of `bits` bits, compared with the naive power. These reach each
    // algorithm, including both forms of the multinomial recurrence, whose sums for `Natural`
    // coefficients must stay non-negative.
    let test_generated = |len: usize, bits: u64, e: u64| {
        let p = NaturalPolynomial::from_coefficients_asc(natural_generated_coefficients(len, bits));
        let power = (&p).pow(e);
        assert_eq!(power, pow_naive(&p, e));
        verify_kernels(p.coefficients_asc(), e, power.coefficients_asc());
    };
    test_generated(3, 64 * 60, 5);
    test_generated(200, 10, 5);
    test_generated(3, 64, 8);
    test_generated(2, 40, 64);
    test_generated(3, 64, 16);
    test_generated(8, 8, 64);
    test_generated(8, 64, 15);
}

#[test]
fn pow_properties() {
    natural_polynomial_unsigned_pair_gen_var_5().test_properties(|(p, e)| {
        let power = p.clone().pow(e);
        assert!(power.is_valid());
        assert_eq!((&p).pow(e), power);
        let mut q = p.clone();
        q.pow_assign(e);
        assert_eq!(q, power);
        assert_eq!(pow_naive(&p, e), power);
        verify_kernels(p.coefficients_asc(), e, power.coefficients_asc());
        assert_eq!(
            IntegerPolynomial::from(p.clone()).pow(e),
            IntegerPolynomial::from(power.clone())
        );
        if p != 0u32 {
            assert_eq!(power.degree(), p.degree().map(|d| d * e));
            assert_eq!(*power.leading_coefficient(), p.leading_coefficient().pow(e));
            assert_eq!((&power).content(), (&p).content().pow(e));
        }
        for x in [Natural::ONE, Natural::TWO, Natural::from(3u32)] {
            assert_eq!((&power).evaluate(&x), (&p).evaluate(&x).pow(e));
        }
        let half = e >> 1;
        assert_eq!((&p).pow(half) * (&p).pow(e - half), power);
    });

    natural_polynomial_unsigned_pair_gen_var_6().test_properties(|(p, e)| {
        let power = (&p).pow(e);
        assert_eq!(pow_naive(&p, e), power);
        verify_kernels(p.coefficients_asc(), e, power.coefficients_asc());
    });

    natural_polynomial_gen().test_properties(|p| {
        assert_eq!((&p).pow(0), NaturalPolynomial::one());
        assert_eq!((&p).pow(1), p);
        assert_eq!((&p).pow(2), (&p).square());
        assert_eq!((&p).pow(2).pow(3), (&p).pow(6));
    });

    natural_gen().test_properties(|x| {
        for e in [0, 1, 2, 3, 7] {
            assert_eq!(
                NaturalPolynomial::from(x.clone()).pow(e),
                NaturalPolynomial::from((&x).pow(e))
            );
        }
    });
}
