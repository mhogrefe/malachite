// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Parity, Pow, PowAssign, Square};
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{Content, Evaluate, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::pow::binexp::pow_to_out_binexp;
use malachite_nz::integer_polynomial::arithmetic::pow::binomial::pow_to_out_binomial;
use malachite_nz::integer_polynomial::arithmetic::pow::multinomial::*;
use malachite_nz::integer_polynomial::arithmetic::pow::small::pow_to_out_small;
use malachite_nz::integer_polynomial::arithmetic::pow::{
    pow_ref_with_kernel, pow_to_out, pow_to_out_addchains_e,
};
use malachite_nz::test_util::generators::{
    integer_gen, integer_polynomial_gen, integer_polynomial_unsigned_pair_gen_var_5,
    integer_polynomial_unsigned_pair_gen_var_6,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::generated_coefficients;
use malachite_nz::test_util::integer_polynomial::arithmetic::pow::{
    pow_naive, pow_to_out_multinomial_flint,
};

// Checks every kernel that applies to the coefficients `xs` and the exponent `e` against `out`.
fn verify_kernels(xs: &[Integer], e: u64, out: &[Integer]) {
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
    assert_eq!(
        pow_ref_with_kernel(xs, e, pow_to_out_multinomial_flint),
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
        let p = IntegerPolynomial::from_str(s).unwrap();
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
    // - e == 0
    test("0", 0, "1");
    test("x+1", 0, "1");
    // - the polynomial is zero
    test("0", 5, "0");
    // - the polynomial is a constant
    test("-3", 5, "-243");
    // - the polynomial is a monomial
    test("3*x^2", 4, "81*x^8");
    // - e == 1
    test("x^2-x", 1, "x^2-x");
    // - e == 2
    test("x+1", 2, "x^2+2*x+1");
    // - e < 5, e == 3
    test("x+1", 3, "x^3+3*x^2+3*x+1");
    test(
        "5*x^4+x^3+4*x^2+x+3",
        3,
        "125*x^12+75*x^11+315*x^10+196*x^9+507*x^8+261*x^7+472*x^6+213*x^5+309*x^4+100*x^3+117*x^2+\
        27*x+27",
    );
    // - e < 5, e == 4
    test(
        "3*x^3+2*x-1",
        4,
        "81*x^12+216*x^10-108*x^9+216*x^8-216*x^7+150*x^6-144*x^5+88*x^4-44*x^3+24*x^2-8*x+1",
    );
    // - len == 2, e odd
    test("x+1", 5, "x^5+5*x^4+10*x^3+10*x^2+5*x+1");
    test(
        "-x+2",
        7,
        "-x^7+14*x^6-84*x^5+280*x^4-560*x^3+672*x^2-448*x+128",
    );
    // - len == 2, e even
    test(
        "2*x-3",
        6,
        "64*x^6-576*x^5+2160*x^4-4320*x^3+4860*x^2-2916*x+729",
    );
    // - len >= 3, e >= 5, binexp: small e relative to the length
    test(
        "x^2+x+1",
        5,
        "x^10+5*x^9+15*x^8+30*x^7+45*x^6+51*x^5+45*x^4+30*x^3+15*x^2+5*x+1",
    );
    test(
        "-x^2+1",
        10,
        "x^20-10*x^18+45*x^16-120*x^14+210*x^12-252*x^10+210*x^8-120*x^6+45*x^4-10*x^2+1",
    );
    test(
        "3*x^2-7*x+5",
        9,
        "19683*x^18-413343*x^17+4153113*x^16-26515188*x^15+120490578*x^14-413720622*x^13+\
        1112139882*x^12-2392703712*x^11+4176093537*x^10-5956716997*x^9+6960155895*x^8-\
        6646399200*x^7+5148795750*x^6-3192288750*x^5+1549518750*x^4-568312500*x^3+148359375*x^2-\
        24609375*x+1953125",
    );
    // - a factor of x^2 removed first, leaving length 2
    test("x^3+x^2", 5, "x^15+5*x^14+10*x^13+10*x^12+5*x^11+x^10");
    // Generated coefficients of `bits` bits, compared with the naive power.
    let test_generated = |len: usize, bits: u64, e: u64| {
        let p = IntegerPolynomial::from_coefficients_asc(generated_coefficients(len, bits));
        let power = (&p).pow(e);
        assert_eq!(power, pow_naive(&p, e));
        verify_kernels(p.coefficients_asc(), e, power.coefficients_asc());
    };
    // - binexp: large coefficients
    test_generated(3, 64 * 60, 5);
    // - binexp: long polynomial
    test_generated(200, 10, 5);
    // - binexp: short polynomial with small coefficients
    test_generated(3, 64, 8);
    // - len == 2, multinomial_preferred_for_binomial
    test_generated(2, 40, 64);
    // - multinomial_preferred, sum split by sign
    test_generated(3, 64, 16);
    // - multinomial_preferred, precomputed multiples
    test_generated(8, 8, 64);
    // - addition_chain_preferred
    test_generated(8, 64, 15);
}

#[test]
fn test_pow_to_out_small() {
    // The exponents below 3, which `pow_ref_with_kernel` handles before calling a kernel.
    let test = |s, e: u64, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let xs = p.coefficients_asc();
        let mut result = vec![Integer::ZERO; usize::exact_from(e) * (xs.len() - 1) + 1];
        pow_to_out_small(&mut result, xs, e);
        assert_eq!(
            IntegerPolynomial::from_coefficients_asc(result).to_string(),
            out
        );
    };
    // - e == 0
    test("x^2-3*x+2", 0, "1");
    // - e == 1
    test("x^2-3*x+2", 1, "x^2-3*x+2");
    // - e == 2
    test("x^2-3*x+2", 2, "x^4-6*x^3+13*x^2-12*x+4");
    // - e == 3
    test("x-1", 3, "x^3-3*x^2+3*x-1");
    // - e == 4
    test("x-1", 4, "x^4-4*x^3+6*x^2-4*x+1");
}

#[test]
#[should_panic]
fn pow_to_out_small_fail() {
    let xs = [Integer::ONE, Integer::ONE];
    let mut out = vec![Integer::ZERO; 6];
    pow_to_out_small(&mut out, &xs, 5);
}

#[test]
fn pow_properties() {
    integer_polynomial_unsigned_pair_gen_var_5().test_properties(|(p, e)| {
        let power = p.clone().pow(e);
        assert!(power.is_valid());
        assert_eq!((&p).pow(e), power);
        let mut q = p.clone();
        q.pow_assign(e);
        assert_eq!(q, power);
        assert_eq!(pow_naive(&p, e), power);
        verify_kernels(p.coefficients_asc(), e, power.coefficients_asc());
        if p != 0u32 {
            assert_eq!(power.degree(), p.degree().map(|d| d * e));
            assert_eq!(*power.leading_coefficient(), p.leading_coefficient().pow(e));
            // Gauss's lemma: the content of a product is the product of the contents
            assert_eq!((&power).content(), (&p).content().pow(e));
        }
        let neg_power = (-&p).pow(e);
        assert_eq!(if e.even() { neg_power } else { -neg_power }, power);
        for x in [Integer::TWO, Integer::NEGATIVE_ONE, Integer::from(-3)] {
            assert_eq!((&power).evaluate(&x), (&p).evaluate(&x).pow(e));
        }
        let half = e >> 1;
        assert_eq!((&p).pow(half) * (&p).pow(e - half), power);
    });

    integer_polynomial_unsigned_pair_gen_var_6().test_properties(|(p, e)| {
        let power = (&p).pow(e);
        assert_eq!(pow_naive(&p, e), power);
        verify_kernels(p.coefficients_asc(), e, power.coefficients_asc());
    });

    integer_polynomial_gen().test_properties(|p| {
        assert_eq!((&p).pow(0), IntegerPolynomial::one());
        assert_eq!((&p).pow(1), p);
        assert_eq!((&p).pow(2), (&p).square());
        assert_eq!((&p).pow(2).pow(3), (&p).pow(6));
    });

    integer_gen().test_properties(|x| {
        for e in [0, 1, 2, 3, 7] {
            assert_eq!(
                IntegerPolynomial::from(x.clone()).pow(e),
                IntegerPolynomial::from((&x).pow(e))
            );
        }
    });
}
