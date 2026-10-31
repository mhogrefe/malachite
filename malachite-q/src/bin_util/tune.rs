// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::bench::tune::interleaved_min_pair;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::rational_polynomial::arithmetic::evaluate::{
    evaluate_integer_polynomial_divide_and_conquer, evaluate_integer_polynomial_horner,
};
use std::hint::black_box;

fn random_natural(seed: &str, bits: u64) -> Natural {
    use malachite_base::num::arithmetic::traits::ModPowerOf2;
    let limbs: Vec<Limb> = random_primitive_ints(EXAMPLE_SEED.fork(seed))
        .take(usize::try_from(bits.div_ceil(Limb::WIDTH)).unwrap())
        .collect();
    let mut n = Natural::from_owned_limbs_asc(limbs).mod_power_of_2(bits);
    n.set_bit(bits - 1);
    n
}

fn random_integer(seed: &str, bits: u64) -> Integer {
    let negative = random_primitive_ints::<u8>(EXAMPLE_SEED.fork(&format!("{seed}s")))
        .next()
        .unwrap()
        .odd();
    Integer::from_sign_and_abs(!negative, random_natural(seed, bits))
}

fn random_rational(seed: &str, numerator_bits: u64, denominator_bits: u64) -> Rational {
    Rational::from_integers(
        random_integer(&format!("{seed}n"), numerator_bits),
        Integer::from(random_natural(&format!("{seed}d"), denominator_bits)),
    )
}

type Inputs = Vec<(Vec<Integer>, Rational)>;

fn inputs(len: usize, coefficient_bits: u64, numerator_bits: u64, denominator_bits: u64) -> Inputs {
    (0..4)
        .map(|k| {
            let cs = (0..len)
                .map(|i| random_integer(&format!("c{k}_{i}"), coefficient_bits))
                .collect();
            (
                cs,
                random_rational(&format!("x{k}"), numerator_bits, denominator_bits),
            )
        })
        .collect()
}

// Evaluation of an IntegerPolynomial at a Rational: Horner's rule against divide and conquer. As
// with the evaluation at an Integer in malachite-nz, the crossover depends on the sizes of the
// point's numerator and denominator as well as on the length, so this prints a grid: for each
// (coefficient bits, numerator bits, denominator bits) triple, the ratio of the divide-and-conquer
// time to the Horner time at a range of lengths, and the first length from which divide and conquer
// wins at every length measured.
#[allow(clippy::print_stdout)]
fn tune_evaluate_rational() {
    let lens =
        [2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256, 384, 512, 768, 1024, 1536, 2048];
    for coefficient_bits in [8u64, 64, 256, 2048] {
        for (numerator_bits, denominator_bits) in [
            (8u64, 8u64),
            (64, 64),
            (8, 64),
            (64, 8),
            (256, 256),
            (2048, 2048),
            (64, 2048),
            (2048, 64),
        ] {
            let mut line =
                format!("c {coefficient_bits:>5} a {numerator_bits:>5} b {denominator_bits:>5}:");
            let mut first_win = None;
            let mut wins = 0;
            for &len in &lens {
                let inputs = inputs(len, coefficient_bits, numerator_bits, denominator_bits);
                let (mut i, mut j) = (0usize, 0usize);
                let (th, td) = interleaved_min_pair(
                    &mut || {
                        let (cs, x) = &inputs[i % 4];
                        i += 1;
                        black_box(evaluate_integer_polynomial_horner(black_box(cs), x));
                    },
                    &mut || {
                        let (cs, x) = &inputs[j % 4];
                        j += 1;
                        black_box(evaluate_integer_polynomial_divide_and_conquer(
                            black_box(cs),
                            x,
                        ));
                    },
                );
                let ratio = td / th;
                line.push_str(&format!(" {len}:{ratio:.2}"));
                if ratio < 1.0 {
                    wins += 1;
                    first_win.get_or_insert(len);
                } else {
                    wins = 0;
                    first_win = None;
                }
                if wins >= 3 && ratio < 0.8 {
                    break;
                }
                if th > 2.0e8 {
                    break;
                }
            }
            println!(
                "{line}  => {}",
                first_win.map_or("none".to_string(), |l| l.to_string())
            );
        }
    }
}

// A dense series of `len` rational coefficients with numerators of `numerator_bits` bits and
// denominators of `denominator_bits` bits, and a nonzero constant coefficient.
fn dense_series(
    seed: &str,
    len: usize,
    numerator_bits: u64,
    denominator_bits: u64,
) -> RationalPolynomial {
    use malachite_base::polynomial::Polynomial;
    RationalPolynomial::from_coefficients_asc(
        (0..len)
            .map(|i| random_rational(&format!("{seed}{i}"), numerator_bits, denominator_bits))
            .collect(),
    )
}

// The crossover of rational power series inversion's basecase and Newton iteration: for each size
// of numerator and denominator, the shortest length at which one Newton step, from the basecase's
// inverse to half the precision, beats the basecase, against FLINT's fixed cutoff of 24. A cutoff
// of `len - 1` makes `inverse_truncated_newton_with_cutoff` take exactly one step. Denominators of
// at least 2 bits keep the series off the integer fast path. Prints the ratio of the basecase's
// time to the Newton step's at each length.
#[allow(clippy::print_stdout)]
fn tune_inverse_truncated_rational() {
    use malachite_q::rational_polynomial::arithmetic::inverse_truncated::*;
    let lens = [4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256];
    for (numerator_bits, denominator_bits) in
        [(1u64, 2u64), (4, 4), (16, 16), (64, 64), (8, 64), (64, 8), (256, 256)]
    {
        let mut line = format!("a {numerator_bits:>4} b {denominator_bits:>4}:");
        let mut first_win = None;
        let mut wins = 0;
        for &len in &lens {
            let p = dense_series("inv", len, numerator_bits, denominator_bits);
            let n = u64::try_from(len).unwrap();
            let (tb, tn) = interleaved_min_pair(
                &mut || {
                    black_box(inverse_truncated_basecase(black_box(&p), n));
                },
                &mut || {
                    black_box(inverse_truncated_newton_with_cutoff(
                        black_box(&p),
                        n,
                        len - 1,
                    ));
                },
            );
            line += &format!(" {len}:{:.2}", tb / tn);
            if tn < tb {
                wins += 1;
                if wins == 2 && first_win.is_none() {
                    first_win = Some(len);
                }
            } else {
                wins = 0;
            }
        }
        println!(
            "{line}  => {}",
            first_win.map_or("none".to_string(), |l| l.to_string())
        );
    }
}

// Rational power series division by inverting the divisor and multiplying, FLINT's method, against
// reversed polynomial division, on exact quotients (the divisor times a dense series of the same
// coefficient sizes) and on dense random dividends. Prints the ratio of the inverse's time to the
// division's at each length; above 1, division is faster.
#[allow(clippy::print_stdout)]
fn tune_div_truncated_rational() {
    use malachite_base::polynomial::MulTruncated;
    use malachite_q::rational_polynomial::arithmetic::div_truncated::*;
    let lens = [2, 4, 8, 12, 16, 24, 32, 48, 64, 96, 128, 192, 256];
    for (numerator_bits, denominator_bits) in [(1u64, 1u64), (4, 4), (16, 16), (64, 64), (256, 256)]
    {
        for exact in [true, false] {
            let mut line = format!(
                "{} a {numerator_bits:>4} b {denominator_bits:>4}:",
                if exact { "exact " } else { "random" }
            );
            for &len in &lens {
                let n = u64::try_from(len).unwrap();
                let q = dense_series("divq", len, numerator_bits, denominator_bits);
                let r = dense_series("divr", len, numerator_bits, denominator_bits);
                let p = if exact { (&q).mul_truncated(&r, n) } else { r };
                let (ti, td) = interleaved_min_pair(
                    &mut || {
                        black_box(div_truncated_by_inverse(black_box(&p), &q, n));
                    },
                    &mut || {
                        black_box(div_truncated_by_division(black_box(&p), &q, n));
                    },
                );
                line += &format!(" {len}:{:.2}", ti / td);
            }
            println!("{line}");
        }
    }
}

pub fn tune(key: &str) {
    match key {
        "evaluate_rational" => tune_evaluate_rational(),
        "inverse_truncated_rational" => tune_inverse_truncated_rational(),
        "div_truncated_rational" => tune_div_truncated_rational(),
        _ => panic!("Invalid tuning key: {key}"),
    }
}
