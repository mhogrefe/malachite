// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Pow, ModPowerOf2PowAssign, Pow,
};
use malachite_base::polynomial::Polynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_power_of_2_pow::*;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_unsigned_triple_gen_var_4;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_pow::*;
use std::panic::catch_unwind;

// Checks both kernels against `out` when they apply: a polynomial of length at least 2 with a
// nonzero constant term, and an exponent of at least 3.
fn verify_kernels(xs: &[Natural], e: u64, pow: u64, out: &[Natural]) {
    if xs.len() >= 2 && xs[0] != 0u32 && e >= 3 && pow != 0 {
        assert_eq!(mod_power_of_2_pow_binexp(xs, e, pow), out);
        assert_eq!(mod_power_of_2_pow_exact(xs, e, pow), out);
    }
}

#[test]
fn test_mod_power_of_2_pow() {
    let test = |s, e: u64, pow: u64, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let power = p.clone().mod_power_of_2_pow(e, pow);
        assert!(power.is_valid());
        assert_eq!(power.to_string(), out);
        assert_eq!((&p).mod_power_of_2_pow(e, pow).to_string(), out);
        let mut q = p.clone();
        q.mod_power_of_2_pow_assign(e, pow);
        assert_eq!(q.to_string(), out);
        assert_eq!(mod_power_of_2_pow_naive(&p, e, pow).to_string(), out);
        verify_kernels(p.coefficients_asc(), e, pow, power.coefficients_asc());
    };
    // - pow == 0
    test("0", 3, 0, "0");
    test("0", 0, 0, "0");
    // - e == 0
    test("x+1", 0, 4, "1");
    test("0", 0, 4, "1");
    // - the polynomial is zero
    test("0", 3, 4, "0");
    // - the polynomial is a constant
    test("3", 5, 4, "3");
    test("2", 4, 3, "0");
    // - the polynomial is a monomial
    test("3*x^2", 3, 4, "11*x^6");
    test("2*x^2", 3, 3, "0");
    // - e == 1
    test("x^2+x", 1, 2, "x^2+x");
    // - e == 2
    test("2*x+1", 2, 2, "1");
    // - the power needs no reduction
    test("x+1", 3, 10, "x^3+3*x^2+3*x+1");
    test(
        "5*x^2+3*x+7",
        13,
        100,
        "1220703125*x^26+9521484375*x^25+56494140625*x^24+235371093750*x^23+827607421875*x^22\
        +2409605859375*x^21+6176069453125*x^20+13825986281250*x^19+27837936912500*x^18\
        +50142782868750*x^17+82185868908000*x^16+121939345119150*x^15+165607556473515*x^14\
        +204622367798943*x^13+231850579062921*x^12+239001116433534*x^11+225518024283552*x^10\
        +192628514668590*x^9+149719105820284*x^8+104103261440178*x^7+65104112200127*x^6\
        +35560699565391*x^5+17099235662145*x^4+6808218451398*x^3+2287767041651*x^2\
        +539810200839*x+96889010407",
    );
    // - binary exponentiation modulo 2^k
    test("x+1", 5, 3, "x^5+5*x^4+2*x^3+2*x^2+5*x+1");
    test(
        "3*x^2+7*x+5",
        9,
        4,
        "3*x^18+15*x^17+9*x^16+4*x^15+2*x^14+14*x^13+10*x^12+x^10+5*x^9+7*x^8+6*x^6+14*x^5+14*x^4\
        +4*x^3+15*x^2+15*x+5",
    );
    // - leading coefficients vanish modulo 2^k
    test(
        "2*x^2+x+1",
        7,
        3,
        "4*x^9+2*x^8+5*x^7+x^6+x^5+x^4+7*x^3+3*x^2+7*x+1",
    );
    test("2*x^2+2*x+1", 4, 2, "1");
    // - the power vanishes after a square
    test("2*x+2", 3, 2, "0");
    // - the power vanishes after a multiplication
    test("2*x+2", 3, 3, "0");
    // - a factor of x^2 removed first
    test("x^3+x^2", 5, 2, "x^15+x^14+2*x^13+2*x^12+x^11+x^10");
    test(
        "6*x^7+2*x^6+9*x^5+5*x^4+x^3+4*x^2+x+3",
        11,
        5,
        "24*x^61+24*x^60+4*x^59+28*x^58+2*x^57+30*x^56+3*x^55+9*x^54+12*x^53+21*x^52+27*x^51\
        +16*x^50+20*x^49+3*x^48+29*x^47+31*x^46+4*x^45+2*x^44+29*x^43+13*x^42+7*x^41+2*x^40\
        +21*x^39+21*x^38+26*x^37+30*x^36+11*x^35+25*x^34+23*x^33+8*x^32+14*x^31+14*x^30+22*x^29\
        +9*x^28+26*x^27+x^26+31*x^25+13*x^24+25*x^23+13*x^22+8*x^21+19*x^20+27*x^19+24*x^18\
        +12*x^17+13*x^16+15*x^15+7*x^14+2*x^13+30*x^12+29*x^11+3*x^10+29*x^9+16*x^8+6*x^7+10*x^6\
        +18*x^5+19*x^4+16*x^3+17*x^2+3*x+27",
    );
    // Generated coefficients, compared with the naive power.
    let test_generated = |len: usize, pow: u64, e: u64| {
        let p = NaturalPolynomial::from_coefficients_asc(
            natural_mod_power_of_2_generated_coefficients(len, pow),
        );
        let power = (&p).mod_power_of_2_pow(e, pow);
        assert_eq!(power, mod_power_of_2_pow_naive(&p, e, pow));
        verify_kernels(p.coefficients_asc(), e, pow, power.coefficients_asc());
    };
    test_generated(8, 100, 13);
    test_generated(30, 64, 7);
    test_generated(3, 1000, 5);
    test_generated(40, 10, 9);
}

#[test]
fn mod_power_of_2_pow_fail() {
    let p = NaturalPolynomial::from_str("x+4").unwrap();
    assert_panic!(p.clone().mod_power_of_2_pow(3, 2));
    assert_panic!((&p).mod_power_of_2_pow(3, 2));
    assert_panic!({
        let mut q = p.clone();
        q.mod_power_of_2_pow_assign(3, 2);
    });
}

#[test]
fn mod_power_of_2_pow_properties() {
    natural_polynomial_unsigned_unsigned_triple_gen_var_4().test_properties(|(p, e, pow)| {
        let power = p.clone().mod_power_of_2_pow(e, pow);
        assert!(power.is_valid());
        assert!(power.mod_power_of_2_is_reduced(pow));
        assert_eq!((&p).mod_power_of_2_pow(e, pow), power);
        let mut q = p.clone();
        q.mod_power_of_2_pow_assign(e, pow);
        assert_eq!(q, power);
        assert_eq!(mod_power_of_2_pow_naive(&p, e, pow), power);
        assert_eq!((&p).pow(e).mod_power_of_2(pow), power);
        verify_kernels(p.coefficients_asc(), e, pow, power.coefficients_asc());
        let half = e >> 1;
        assert_eq!(
            (&p).mod_power_of_2_pow(half, pow)
                .mod_power_of_2_mul((&p).mod_power_of_2_pow(e - half, pow), pow),
            power
        );
        // reducing to a smaller power of 2 commutes with powering
        let smaller = pow >> 1;
        assert_eq!(
            (&p).mod_power_of_2(smaller).mod_power_of_2_pow(e, smaller),
            (&power).mod_power_of_2(smaller)
        );
    });
}
