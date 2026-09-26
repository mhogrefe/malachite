// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::mod_mul::mod_mul_precompute_shoup;
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    ModEvaluate, ModEvaluateGeometric, ModEvaluateMany, ModPowerOf2Evaluate, Polynomial,
};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1,
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2,
    unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1,
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::evaluate::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::evaluate::{
    mod_evaluate_horner, mod_evaluate_horner_block, mod_evaluate_shoup, mod_evaluate_shoup_block,
    mod_evaluate_shoup_lazy, mod_evaluate_shoup_lazy_block,
};

#[test]
fn test_mod_power_of_2_evaluate() {
    fn test<T: PrimitiveUnsigned>(s: &str, x: T, pow: u64, out: T) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        assert_eq!((&p).mod_power_of_2_evaluate(x, pow), out);
        assert_eq!(p.mod_power_of_2_evaluate(x, pow), out);
    }
    test::<u8>("0", 0, 0, 0);
    test::<u8>("0", 5, 3, 0);
    test::<u8>("5*x^2+3*x+7", 5, 3, 3);
    test::<u8>("5*x^2+3*x+7", 6, 4, 13);
    test::<u8>("5*x^2+3*x+7", 6, 8, 205);
    // p(0) is the constant term.
    test::<u8>("5*x^2+3*x+7", 0, 4, 7);
    // The intermediate values wrap around the width of `T`, which is harmless modulo 2^pow.
    test::<u8>("255*x+255", 255, 8, 0);
    test::<u8>("x^7", 2, 8, 128);
    test::<u8>("x^8", 2, 8, 0);
    test::<u64>("3*x^3+2*x+1", 12345, 20, 232110);
    test::<u64>("3*x^3+2*x+1", 12345, 64, 5644097915566);
    test::<u64>("18446744073709551615*x^2+1", u64::MAX, 64, 0);
    test::<u128>("x^2+1", 1 << 127, 128, 1);
    test::<u128>("340282366920938463463374607431768211455*x+5", 3, 128, 2);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_fail_1() {
    // pow is wider than `T`.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_evaluate(1, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_fail_2() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("4*x+1")
        .unwrap()
        .mod_power_of_2_evaluate(1, 2);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_fail_3() {
    // x is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_evaluate(4, 2);
}

#[test]
#[should_panic]
fn mod_power_of_2_evaluate_fail_4() {
    // By reference, x is not reduced.
    (&UnsignedPolynomial::<u64>::from_str("x+1").unwrap()).mod_power_of_2_evaluate(1, 0);
}

fn mod_power_of_2_evaluate_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, x, pow)| {
        let y = (&p).mod_power_of_2_evaluate(x, pow);
        assert!(y.mod_power_of_2_is_reduced(pow));
        assert_eq!(p.clone().mod_power_of_2_evaluate(x, pow), y);
        assert_eq!(mod_power_of_2_evaluate_naive(&p, x, pow), y);

        // Reducing further agrees with evaluating the reduced polynomial at the reduced value.
        for smaller in [0, pow >> 1, pow.saturating_sub(1)] {
            assert_eq!(
                y.mod_power_of_2(smaller),
                (&p).mod_power_of_2(smaller)
                    .mod_power_of_2_evaluate(x.mod_power_of_2(smaller), smaller)
            );
        }

        // p(0) is the constant term, and p(1) the sum of the coefficients, mod 2^pow.
        assert_eq!((&p).mod_power_of_2_evaluate(T::ZERO, pow), p.coefficient(0));
        if pow != 0 {
            assert_eq!(
                (&p).mod_power_of_2_evaluate(T::ONE, pow),
                p.coefficients_asc()
                    .iter()
                    .fold(T::ZERO, |sum, &c| sum.mod_power_of_2_add(c, pow))
            );
        }
        // Everything is 0 mod 2^0.
        if pow == 0 {
            assert_eq!(y, T::ZERO);
        }
    });
}

#[test]
fn mod_power_of_2_evaluate_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_evaluate_properties_helper);
}

#[test]
fn test_mod_evaluate() {
    fn test<T: PrimitiveUnsigned>(s: &str, x: T, m: T, out: T) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        assert_eq!((&p).mod_evaluate(x, m), out);
        assert_eq!(p.mod_evaluate(x, m), out);
    }
    // - the zero polynomial
    // Modulo 1 the only reduced polynomial is 0.
    test::<u8>("0", 0, 1, 0);
    // - Shoup, which a type no wider than 32 bits uses from length 2
    test::<u8>("5*x^2+3*x+7", 6, 13, 10);
    // - x == 0
    // p(0) is the constant term.
    test::<u8>("5*x^2+3*x+7", 0, 13, 7);
    test::<u8>("5*x^2+3*x+7", 12, 13, 9);
    test::<u8>("5*x^2+3*x+7", 6, 211, 205);
    test::<u8>("x^2+x+1", 1, 2, 1);
    // The products overflow a `u8`, but are reduced exactly.
    test::<u8>("200*x^2+100", 150, 251, 172);
    test::<u8>("254*x^3+254*x+254", 254, 255, 1);
    // - Horner, since the polynomial is too short for Shoup in a type wider than 32 bits
    test::<u64>("3*x+1", 5, 7, 2);
    test::<u64>("3*x^3+2*x+1", 12345, 1000003, 983326);
    test::<u64>("x^100+1", 3, 18446744073709551557, 11554422485578774283);
    test::<u64>(
        "18446744073709551614*x^2+1",
        18446744073709551614,
        u64::MAX,
        0,
    );
    test::<u128>(
        "340282366920938463463374607431768211454*x+1",
        340282366920938463463374607431768211454,
        u128::MAX,
        2,
    );
    test::<u128>(
        "x^3+x",
        1 << 100,
        170141183460469231731687303715884105727,
        1267650600228229471865447383040,
    );
}

#[test]
fn test_mod_evaluate_long() {
    // Polynomials long enough for Shoup's method, which needs the top bit of the modulus clear, in
    // its lazy form when the modulus is at most a third of the type's range.
    fn test<T: PrimitiveUnsigned>(s: &str, x: T, m: T, out: T) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        assert_eq!(p.len(), 12);
        assert_eq!((&p).mod_evaluate(x, m), out);
        assert_eq!(mod_evaluate_horner(p.coefficients_asc(), x, m), out);
        assert_eq!(p.mod_evaluate(x, m), out);
    }
    // - lazy Shoup, with the largest lazy modulus
    // - lazy result >= 2 * m
    test::<u8>(
        "61*x^11+39*x^10+11*x^9+62*x^8+22*x^7+61*x^6+9*x^5+36*x^4+57*x^3+72*x^2+81*x+84",
        84,
        85,
        28,
    );
    // - Shoup, not lazy, with the smallest such modulus
    test::<u8>(
        "66*x^11+43*x^10+14*x^9+65*x^8+24*x^7+63*x^6+10*x^5+37*x^4+58*x^3+73*x^2+82*x+85",
        85,
        86,
        26,
    );
    // - Shoup, not lazy, with the largest modulus whose top bit is clear
    test::<u8>(
        "17*x^11+80*x^10+10*x^9+61*x^8+106*x^7+18*x^6+51*x^5+78*x^4+99*x^3+114*x^2+123*x+126",
        126,
        127,
        71,
    );
    // - Horner, since the top bit of m is set
    test::<u8>(
        "20*x^11+83*x^10+12*x^9+63*x^8+108*x^7+19*x^6+52*x^5+79*x^4+100*x^3+115*x^2+124*x+127",
        127,
        128,
        70,
    );
    // - lazy Shoup
    // - m <= lazy result < 2 * m
    test::<u64>(
        "999639*x^11+999702*x^10+999759*x^9+999810*x^8+999855*x^7+999894*x^6+999927*x^5+999954*x^4+\
        999975*x^3+999990*x^2+999999*x+1000002",
        12345,
        1000003,
        861790,
    );
    // - lazy Shoup, with the largest lazy modulus
    test::<u64>(
        "6148914691236516841*x^11+6148914691236516904*x^10+6148914691236516961*x^9+6148914691236517\
        012*x^8+6148914691236517057*x^7+6148914691236517096*x^6+6148914691236517129*x^5+61489146912\
        36517156*x^4+6148914691236517177*x^3+6148914691236517192*x^2+6148914691236517201*x+61489146\
        91236517204",
        6148914691236517204,
        6148914691236517205,
        198,
    );
    // - Shoup, not lazy, with the smallest such modulus
    test::<u64>(
        "6148914691236516842*x^11+6148914691236516905*x^10+6148914691236516962*x^9+6148914691236517\
        013*x^8+6148914691236517058*x^7+6148914691236517097*x^6+6148914691236517130*x^5+61489146912\
        36517157*x^4+6148914691236517178*x^3+6148914691236517193*x^2+6148914691236517202*x+61489146\
        91236517205",
        6148914691236517205,
        6148914691236517206,
        198,
    );
    // - Shoup, not lazy
    test::<u64>(
        "9223372036854775419*x^11+9223372036854775482*x^10+9223372036854775539*x^9+9223372036854775\
        590*x^8+9223372036854775635*x^7+9223372036854775674*x^6+9223372036854775707*x^5+92233720368\
        54775734*x^4+9223372036854775755*x^3+9223372036854775770*x^2+9223372036854775779*x+92233720\
        36854775782",
        9223372036854775782,
        9223372036854775783,
        198,
    );
    // - Horner, since the top bit of m is set
    test::<u64>(
        "18446744073709551193*x^11+18446744073709551256*x^10+18446744073709551313*x^9+1844674407370\
        9551364*x^8+18446744073709551409*x^7+18446744073709551448*x^6+18446744073709551481*x^5+1844\
        6744073709551508*x^4+18446744073709551529*x^3+18446744073709551544*x^2+18446744073709551553\
        *x+18446744073709551556",
        18446744073709551556,
        18446744073709551557,
        198,
    );
    // - Shoup, not lazy
    test::<u128>(
        "170141183460469231731687303715884105363*x^11+170141183460469231731687303715884105426*x^10+\
        170141183460469231731687303715884105483*x^9+170141183460469231731687303715884105534*x^8+170\
        141183460469231731687303715884105579*x^7+170141183460469231731687303715884105618*x^6+170141\
        183460469231731687303715884105651*x^5+170141183460469231731687303715884105678*x^4+170141183\
        460469231731687303715884105699*x^3+170141183460469231731687303715884105714*x^2+170141183460\
        469231731687303715884105723*x+170141183460469231731687303715884105726",
        170141183460469231731687303715884105726,
        170141183460469231731687303715884105727,
        198,
    );
    // - lazy Shoup
    test::<u128>(
        "85070591730234615865843651857942052515*x^11+85070591730234615865843651857942052578*x^10+85\
        070591730234615865843651857942052635*x^9+85070591730234615865843651857942052686*x^8+8507059\
        1730234615865843651857942052731*x^7+85070591730234615865843651857942052770*x^6+850705917302\
        34615865843651857942052803*x^5+85070591730234615865843651857942052830*x^4+85070591730234615\
        865843651857942052851*x^3+85070591730234615865843651857942052866*x^2+8507059173023461586584\
        3651857942052875*x+85070591730234615865843651857942052878",
        85070591730234615865843651857942052864,
        85070591730234615865843651857942052879,
        2983849007964134,
    );
    // - lazy Shoup
    // - lazy result < m
    test::<u64>(
        "11*x^11+10*x^10+9*x^9+8*x^8+7*x^7+6*x^6+5*x^5+4*x^4+3*x^3+2*x^2+x",
        2,
        1000003,
        40962,
    );
}

#[test]
#[should_panic]
fn mod_evaluate_fail_1() {
    // m is 0.
    UnsignedPolynomial::<u8>::ZERO.mod_evaluate(0, 0);
}

#[test]
#[should_panic]
fn mod_evaluate_fail_2() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("5*x+1")
        .unwrap()
        .mod_evaluate(1, 5);
}

#[test]
#[should_panic]
fn mod_evaluate_fail_3() {
    // x is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_evaluate(5, 5);
}

#[test]
#[should_panic]
fn mod_evaluate_fail_4() {
    // By reference, a coefficient is not reduced.
    (&UnsignedPolynomial::<u64>::from_str("x+7").unwrap()).mod_evaluate(1, 7);
}

fn mod_evaluate_algorithms_agree<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    x: T,
    m: T,
    y: T,
) {
    let coefficients = p.coefficients_asc();
    // The algorithms need at least one coefficient.
    if coefficients.is_empty() {
        return;
    }
    assert_eq!(mod_evaluate_horner(coefficients, x, m), y);
    // Shoup's method needs the top bit of the modulus clear.
    if !m.get_highest_bit() {
        let x_precomp = mod_mul_precompute_shoup(x, m);
        assert_eq!(mod_evaluate_shoup(coefficients, x, x_precomp, m), y);
        // The lazy form is congruent, and less than 3m - 1, when m is at most a third of the range.
        if m <= T::MAX / T::from(3u8) {
            let lazy = mod_evaluate_shoup_lazy(coefficients, x, x_precomp, m);
            assert!(lazy < m + m + m - T::ONE);
            assert_eq!(lazy % m, y);
        }
    }
}

fn mod_evaluate_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(p, x, m)| {
        let y = (&p).mod_evaluate(x, m);
        assert!(y < m);
        assert_eq!(p.clone().mod_evaluate(x, m), y);
        assert_eq!(mod_evaluate_naive(&p, x, m), y);
        mod_evaluate_algorithms_agree(&p, x, m, y);

        // Modulo a power of 2, this agrees with mod_power_of_2_evaluate.
        if m.is_power_of_2() {
            assert_eq!((&p).mod_power_of_2_evaluate(x, m.trailing_zeros()), y);
        }

        // p(0) is the constant term, and p(1) the sum of the coefficients, mod m.
        assert_eq!((&p).mod_evaluate(T::ZERO, m), p.coefficient(0));
        if m != T::ONE {
            assert_eq!(
                (&p).mod_evaluate(T::ONE, m),
                p.coefficients_asc()
                    .iter()
                    .fold(T::ZERO, |sum, &c| sum.mod_add(c, m))
            );
        }
        // Everything is 0 mod 1.
        if m == T::ONE {
            assert_eq!(y, T::ZERO);
        }
    });

    // Long enough for the dispatch to reach Shoup's method.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<T>().test_properties_with_config(
        &config,
        |(p, x, m)| {
            let y = (&p).mod_evaluate(x, m);
            assert!(y < m);
            assert_eq!(mod_evaluate_naive(&p, x, m), y);
            mod_evaluate_algorithms_agree(&p, x, m, y);
        },
    );

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, x, pow)| {
        // Below the width of `T`, evaluation mod 2^pow is evaluation mod the value 2^pow.
        if pow < T::WIDTH {
            assert_eq!(
                (&p).mod_evaluate(x, T::power_of_2(pow)),
                p.mod_power_of_2_evaluate(x, pow)
            );
        }
    });
}

#[test]
fn mod_evaluate_properties() {
    apply_fn_to_unsigneds!(mod_evaluate_properties_helper);
}

#[test]
fn test_mod_evaluate_many() {
    fn test<T: PrimitiveUnsigned>(s: &str, xs: &[T], m: T, out: &[T]) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let ys = (&p).mod_evaluate_many(xs, m);
        assert_eq!(ys, out);
        let ys_alt: Vec<T> = xs.iter().map(|&x| (&p).mod_evaluate(x, m)).collect();
        assert_eq!(ys_alt, out);
    }
    // - no points
    test::<u8>("5*x^2+3*x+7", &[], 13, &[]);
    // - the zero polynomial
    test::<u8>("0", &[0, 5, 6], 13, &[0, 0, 0]);
    // - a constant polynomial
    test::<u8>("9", &[0, 5, 6], 13, &[9, 9, 9]);
    // - lazy Shoup blocks, since a u8 polynomial of length 2 or more uses Shoup
    test::<u8>("5*x^2+3*x+7", &[0, 1, 2, 3, 4, 5], 13, &[7, 2, 7, 9, 8, 4]);
    // - Shoup blocks, not lazy
    test::<u8>(
        "5*x^2+3*x+7",
        &[0, 1, 2, 3, 4, 5, 6, 7, 8],
        120,
        &[7, 15, 33, 61, 99, 27, 85, 33, 111],
    );
    // - Horner blocks, since the top bit of m is set
    test::<u8>(
        "5*x^2+3*x+7",
        &[0, 1, 2, 3, 4, 5, 6, 7, 8],
        251,
        &[7, 15, 33, 61, 99, 147, 205, 22, 100],
    );
    // - Horner blocks, since the polynomial is too short for Shoup in a u64
    test::<u64>(
        "3*x+1",
        &[0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3],
        7,
        &[1, 4, 0, 3, 6, 2, 5, 1, 4, 0, 3],
    );
    // - lazy Shoup blocks of 8, then of 4, and a point left over
    test::<u64>(
        "12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        &[2, 3, 5, 7, 11, 2, 3, 5, 7, 2, 3, 5, 7],
        1000003,
        &[
            45057, 55777, 160935, 12308, 564144, 45057, 55777, 160935, 12308, 45057, 55777, 160935,
            12308,
        ],
    );
    // - Shoup blocks, not lazy, and points left over
    test::<u64>(
        "x^3+6789*x^2+12345",
        &[
            4611686018427387904,
            4611686018427387905,
            4611686018427387906,
            4611686018427387907,
            4611686018427387908,
            4611686018427387909,
            4611686018427387904,
            4611686018427387905,
            4611686018427387906,
            4611686018427387907,
        ],
        9223372036854775783,
        &[
            3458764513821615998,
            5764607523035486965,
            8070450532249371591,
            1152921504608494099,
            3458764513822406061,
            5764607523036331700,
            3458764513821615998,
            5764607523035486965,
            8070450532249371591,
            1152921504608494099,
        ],
    );
    // - Horner blocks, since the top bit of m is set, and points left over
    test::<u64>(
        "x^3+x^2+x+1",
        &[
            18446744073709551556,
            18446744073709551555,
            18446744073709551554,
            18446744073709551553,
            18446744073709551552,
            18446744073709551551,
            18446744073709551550,
            18446744073709551556,
            18446744073709551555,
            18446744073709551554,
            18446744073709551553,
        ],
        18446744073709551557,
        &[
            0,
            18446744073709551552,
            18446744073709551537,
            18446744073709551506,
            18446744073709551453,
            18446744073709551372,
            18446744073709551257,
            0,
            18446744073709551552,
            18446744073709551537,
            18446744073709551506,
        ],
    );
    // - lazy Shoup blocks in a u128
    test::<u128>(
        "4*x^3+3*x^2+2*x+1",
        &[1267650600228229401496703205376, 3, 5, 7, 9],
        170141183460469231731687303715884105727,
        &[2535301228790657981686254403585, 142, 586, 1534, 3178],
    );
}

#[test]
#[should_panic]
fn mod_evaluate_many_fail_1() {
    // m is 0.
    (&UnsignedPolynomial::<u8>::ZERO).mod_evaluate_many(&[], 0);
}

#[test]
#[should_panic]
fn mod_evaluate_many_fail_2() {
    // A coefficient is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap()).mod_evaluate_many(&[1], 5);
}

#[test]
#[should_panic]
fn mod_evaluate_many_fail_3() {
    // A point is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap()).mod_evaluate_many(&[1, 2, 5, 3], 5);
}

// Each block kernel that applies agrees with evaluating one point at a time, for a block of `N`
// points. The points are taken from the front of `xs`, repeated if there are fewer than `N`.
fn mod_evaluate_blocks_agree<T: PrimitiveUnsigned, const N: usize>(
    coefficients: &[T],
    xs: &[T],
    m: T,
) {
    if coefficients.is_empty() || xs.is_empty() {
        return;
    }
    let block: [T; N] = core::array::from_fn(|i| xs[i % xs.len()]);
    let expected = block.map(|x| mod_evaluate_horner(coefficients, x, m));
    let mut values = block;
    mod_evaluate_horner_block(coefficients, &mut values, m);
    assert_eq!(values, expected);
    if !m.get_highest_bit() {
        let mut values = block;
        mod_evaluate_shoup_block(coefficients, &mut values, m);
        assert_eq!(values, expected);
        if m <= T::MAX / T::from(3u8) {
            let mut values = block;
            mod_evaluate_shoup_lazy_block(coefficients, &mut values, m);
            assert_eq!(values, expected);
        }
    }
}

fn mod_evaluate_many_properties_helper<T: PrimitiveUnsigned>() {
    let test = |(p, xs, m): (UnsignedPolynomial<T>, Vec<T>, T)| {
        let ys = (&p).mod_evaluate_many(&xs, m);
        assert_eq!(ys.len(), xs.len());
        assert!(ys.iter().all(|&y| y < m));
        let ys_alt: Vec<T> = xs.iter().map(|&x| (&p).mod_evaluate(x, m)).collect();
        assert_eq!(ys_alt, ys);
        assert_eq!(mod_evaluate_many_naive(&p, &xs, m), ys);

        let coefficients = p.coefficients_asc();
        mod_evaluate_blocks_agree::<T, 1>(coefficients, &xs, m);
        mod_evaluate_blocks_agree::<T, 2>(coefficients, &xs, m);
        mod_evaluate_blocks_agree::<T, 3>(coefficients, &xs, m);
        mod_evaluate_blocks_agree::<T, 4>(coefficients, &xs, m);
        mod_evaluate_blocks_agree::<T, 8>(coefficients, &xs, m);

        // Evaluating at the concatenation of two lists concatenates the values.
        let (xs_1, xs_2) = xs.split_at(xs.len() >> 1);
        let mut ys_alt = (&p).mod_evaluate_many(xs_1, m);
        ys_alt.extend((&p).mod_evaluate_many(xs_2, m));
        assert_eq!(ys_alt, ys);
    };
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>().test_properties(test);

    // Long enough for the dispatch to reach Shoup's method.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<T>()
        .test_properties_with_config(&config, test);
}

#[test]
fn mod_evaluate_many_properties() {
    apply_fn_to_unsigneds!(mod_evaluate_many_properties_helper);
}

#[test]
fn test_mod_evaluate_geometric() {
    fn test<T: PrimitiveUnsigned>(s: &str, q: T, k: u64, m: T, out: &[T]) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        assert_eq!((&p).mod_evaluate_geometric(q, k, m), out);
        assert_eq!(mod_evaluate_geometric_naive(&p, q, k, m), out);
    }
    // - no points
    test::<u8>("5*x^2+3*x+7", 2, 0, 13, &[]);
    // - modulo 1, where the first power, 1, is 0
    test::<u8>("0", 0, 3, 1, &[0, 0, 0]);
    // - powers by Shoup multiplication
    test::<u8>("5*x^2+3*x+7", 2, 4, 13, &[2, 7, 8, 0]);
    // - powers by Horner multiplication, since the top bit of m is set
    test::<u8>("5*x^2+3*x+7", 2, 5, 251, &[15, 33, 99, 100, 80]);
    // - powers by Shoup multiplication in a u64
    test::<u64>(
        "12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        3,
        6,
        1000003,
        &[78, 55777, 85524, 367025, 953372, 980990],
    );
    // - powers by Horner multiplication in a u64
    test::<u64>(
        "3*x^2+2*x+1",
        18446744073709551556,
        6,
        18446744073709551557,
        &[6, 2, 6, 2, 6, 2],
    );
    // - q == 0, so every value after the first is the constant term
    test::<u64>("x+5", 0, 4, 7, &[6, 5, 5, 5]);
    // - powers in a u128
    test::<u128>(
        "4*x^3+3*x^2+2*x+1",
        1267650600228229401496703205376,
        5,
        170141183460469231731687303715884105727,
        &[
            10,
            2535301228790657981686254403585,
            19807059518032015876968415233,
            14855280471424704036277854209,
            576461576938192897,
        ],
    );
}

#[test]
#[should_panic]
fn mod_evaluate_geometric_fail_1() {
    // m is 0.
    (&UnsignedPolynomial::<u8>::ZERO).mod_evaluate_geometric(0, 1, 0);
}

#[test]
#[should_panic]
fn mod_evaluate_geometric_fail_2() {
    // A coefficient is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("5*x+1").unwrap()).mod_evaluate_geometric(1, 3, 5);
}

#[test]
#[should_panic]
fn mod_evaluate_geometric_fail_3() {
    // q is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap()).mod_evaluate_geometric(5, 3, 5);
}

fn mod_evaluate_geometric_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(p, q, k, m)| {
            let ys = (&p).mod_evaluate_geometric(q, k, m);
            assert_eq!(u64::exact_from(ys.len()), k);
            assert!(ys.iter().all(|&y| y < m));
            assert_eq!(mod_evaluate_geometric_naive(&p, q, k, m), ys);

            // It is evaluation at the powers of q.
            let powers: Vec<T> = (0..k).map(|j| q.mod_pow(j, m)).collect();
            assert_eq!((&p).mod_evaluate_many(&powers, m), ys);

            // The first value is p(1), and, when q is 0, every later one is p(0).
            if k != 0 {
                assert_eq!(ys[0], (&p).mod_evaluate(T::ONE % m, m));
            }
            if q == T::ZERO && k > 1 {
                assert!(ys[1..].iter().all(|&y| y == p.coefficient(0)));
            }
            // Fewer points give a prefix.
            if k != 0 {
                assert_eq!((&p).mod_evaluate_geometric(q, k - 1, m), ys[..ys.len() - 1]);
            }
        },
    );
}

#[test]
fn mod_evaluate_geometric_properties() {
    apply_fn_to_unsigneds!(mod_evaluate_geometric_properties_helper);
}
