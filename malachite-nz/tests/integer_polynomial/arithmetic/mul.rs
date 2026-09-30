// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{BitPack, Evaluate, Polynomial};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul::classical::mul_to_out_classical;
use malachite_nz::integer_polynomial::arithmetic::mul::karatsuba::mul_to_out_karatsuba;
use malachite_nz::integer_polynomial::arithmetic::mul::mul_greater_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul::tiny::{
    mul_to_out_tiny_1, mul_to_out_tiny_2,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_integer_pair_gen, integer_polynomial_pair_gen,
    integer_polynomial_triple_gen, integer_vec_pair_gen_var_1, integer_vec_pair_gen_var_2,
    integer_vec_pair_gen_var_3,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::generated_coefficients;
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::scalar_mul::integers_mul_scalar_naive;

fn coefficients(p: &str) -> Vec<Integer> {
    IntegerPolynomial::from_str(p)
        .unwrap()
        .into_coefficients_asc()
}

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_mul_to_out_classical() {
    let test = |xs: &[&str], ys: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_classical(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &ys), result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_to_out_classical(&mut result, &xs, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    test(&["3"], &["4"], &["12"]);
    test(&["-5"], &["7", "0", "-2"], &["-35", "0", "10"]);
    test(&["1", "2", "3"], &["-6"], &["-6", "-12", "-18"]);
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        &["4", "13", "28", "27", "18"],
    );
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &["-18446744073709551616", "5"],
        &[
            "-21778071482940061661655974875633165533184",
            "5958298335808185171968",
            "-680564733841876926945195958937245974543",
            "184467440737095516165",
        ],
    );
    test(&["0", "0", "1"], &["0", "1"], &["0", "0", "0", "1"]);
    test(&["1", "-1"], &["1", "1"], &["1", "0", "-1"]);
    test(
        &["100000000000000000000", "-10000000000000000000000000", "7", "0", "3"],
        &["-1", "1267650600228229401496703205376", "11"],
        &[
            "-100000000000000000000",
            "126765060022822940149670330537600000000000000000000",
            "-12676506002282294014967032053759998900000000000000000007",
            "8873444201597605810476922437632",
            "74",
            "3802951800684688204490109616128",
            "33",
        ],
    );
    test(
        &["9223372036854775808", "-9223372036854775807", "17"],
        &["-9223372036854775808", "9223372036854775807"],
        &[
            "-85070591730234615865843651857942052864",
            "170141183460469231713240559642174554112",
            "-85070591730234616004194232410763689985",
            "156797324626531188719",
        ],
    );
    test_square(
        &["9223372036854775808", "-9223372036854775807", "17"],
        &[
            "85070591730234615865843651857942052864",
            "-170141183460469231713240559642174554112",
            "85070591730234616160991557037294878721",
            "-313594649253062377438",
            "289",
        ],
    );
}

#[test]
fn test_mul_to_out_tiny_1() {
    let test = |xs: &[&str], ys: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_tiny_1(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &ys), result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_to_out_tiny_1(&mut result, &xs, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        &["4", "13", "28", "27", "18"],
    );
    test(&["0", "5", "0"], &["3", "-1"], &["0", "15", "-5", "0"]);
    test(
        &["-7", "0", "11", "-13"],
        &["5", "-1"],
        &["-35", "7", "55", "-76", "13"],
    );
    test(
        &["536870912", "-536870912"],
        &["536870912", "3", "-268435456"],
        &["288230376151711744", "-288230374541099008", "-144115189686468608", "144115188075855872"],
    );
    test(
        &["1", "1", "1", "1", "1", "1", "1", "1"],
        &["-1", "-1", "-1", "-1", "-1", "-1", "-1", "-1"],
        &["-1", "-2", "-3", "-4", "-5", "-6", "-7", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1"],
    );
    test_square(
        &["-7", "0", "11", "-13"],
        &["49", "0", "-154", "182", "121", "-286", "169"],
    );
}

#[test]
fn test_mul_to_out_tiny_2() {
    let test = |xs: &[&str], ys: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_tiny_2(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &ys), result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_to_out_tiny_2(&mut result, &xs, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    test(
        &["2305843009213693952", "-2305843009213693951"],
        &["2305843009213693951", "1152921504606846976"],
        &[
            "5316911983139663489309385231907684352",
            "-2658455991569831741195928102133301249",
            "-2658455991569831744654692615953842176",
        ],
    );
    test(
        &["0", "1099511627776"],
        &["3", "0", "1048576"],
        &["0", "3298534883328", "0", "1152921504606846976"],
    );
    test(
        &["4611686018427387903", "-4611686018427387903", "7"],
        &["4611686018427387903", "1"],
        &[
            "21267647932558653957237540927630737409",
            "-21267647932558653952625854909203349506",
            "27670116110564327418",
            "7",
        ],
    );
    test(
        &["-1099511627776", "5", "1125899906842624"],
        &["3", "-35184372088832", "1048576", "1"],
        &[
            "-3298534883328",
            "38685626227668133590597647",
            "-1149719726746763264",
            "-39614081257132169896278360064",
            "1180591620717411303429",
            "1125899906842624",
        ],
    );
    test_square(
        &["4611686018427387903", "-4611686018427387903", "7"],
        &[
            "21267647932558653957237540927630737409",
            "-42535295865117307914475081855261474818",
            "21267647932558654021801145185614168051",
            "-64563604257983430642",
            "49",
        ],
    );
}

#[test]
fn test_mul_greater_to_out() {
    let test = |xs: &[&str], ys: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_greater_to_out(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &ys), result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_greater_to_out(&mut result, &xs, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    // Long inputs, written as polynomials.
    let test_poly = |p: &str, q: &str, out: &str| {
        let xs = coefficients(p);
        let ys = coefficients(q);
        let mut result = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_greater_to_out(&mut result, &xs, &ys);
        assert_eq!(integers_mul_naive(&xs, &ys), result);
        assert_eq!(
            IntegerPolynomial::from_coefficients_asc(result).to_string(),
            out
        );
    };
    // - len2 == 1
    test(&["1", "2", "3"], &["-6"], &["-6", "-12", "-18"]);
    // - xs and ys are the same slice
    test_square(&["1", "2", "3"], &["1", "4", "10", "12", "9"]);
    // - bits1 > SMALL_FMPZ_BITCOUNT_MAX || bits2 > SMALL_FMPZ_BITCOUNT_MAX
    // - classical
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &["-18446744073709551616", "5"],
        &[
            "-21778071482940061661655974875633165533184",
            "5958298335808185171968",
            "-680564733841876926945195958937245974543",
            "184467440737095516165",
        ],
    );
    // - len2 < 40 + (bits1 + bits2) / 2
    // - rbits <= SMALL_FMPZ_BITCOUNT_MAX
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        &["4", "13", "28", "27", "18"],
    );
    // - rbits > SMALL_FMPZ_BITCOUNT_MAX && rbits < 2 * Limb::WIDTH
    test(
        &["2305843009213693952", "-2305843009213693951"],
        &["2305843009213693951", "1152921504606846976"],
        &[
            "5316911983139663489309385231907684352",
            "-2658455991569831741195928102133301249",
            "-2658455991569831744654692615953842176",
        ],
    );
    // - rbits >= 2 * Limb::WIDTH
    test(
        &[
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
        ],
        &[
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
            "4611686018427387903",
        ],
        &[
            "21267647932558653957237540927630737409",
            "42535295865117307914475081855261474818",
            "63802943797675961871712622782892212227",
            "85070591730234615828950163710522949636",
            "106338239662793269786187704638153687045",
            "127605887595351923743425245565784424454",
            "148873535527910577700662786493415161863",
            "170141183460469231657900327421045899272",
            "148873535527910577700662786493415161863",
            "127605887595351923743425245565784424454",
            "106338239662793269786187704638153687045",
            "85070591730234615828950163710522949636",
            "63802943797675961871712622782892212227",
            "42535295865117307914475081855261474818",
            "21267647932558653957237540927630737409",
        ],
    );
    // - len2 >= 40 + (bits1 + bits2) / 2 && len1 < 70 + (bits1 + bits2) / 2
    test_poly("x^59+1", "-x^45+1", "-x^104+x^59-x^45+1");
    // - len2 >= 40 + (bits1 + bits2) / 2 && len1 >= 70 + (bits1 + bits2) / 2
    test_poly("x^70+1", "-x^40+1", "-x^110+x^70-x^40+1");
    test(
        &["100000000000000000000", "-10000000000000000000000000", "7", "0", "3"],
        &["-1", "1267650600228229401496703205376", "11"],
        &[
            "-100000000000000000000",
            "126765060022822940149670330537600000000000000000000",
            "-12676506002282294014967032053759998900000000000000000007",
            "8873444201597605810476922437632",
            "74",
            "3802951800684688204490109616128",
            "33",
        ],
    );
    test(
        &["9223372036854775808", "-9223372036854775807", "17"],
        &["-9223372036854775808", "9223372036854775807"],
        &[
            "-85070591730234615865843651857942052864",
            "170141183460469231713240559642174554112",
            "-85070591730234616004194232410763689985",
            "156797324626531188719",
        ],
    );
    // Generated coefficients of `bits1` and `bits2` bits.
    let test_generated = |len1: usize, len2: usize, bits1: u64, bits2: u64| {
        let xs = generated_coefficients(len1, bits1);
        let ys = generated_coefficients(len2, bits2);
        let mut result = vec![Integer::ZERO; len1 + len2 - 1];
        mul_greater_to_out(&mut result, &xs, &ys);
        assert_eq!(integers_mul_naive(&xs, &ys), result);
    };
    // - karatsuba_preferred(len2, bits1, bits2), with len2 <= 4
    test_generated(4, 3, 6000, 5500);
    // - karatsuba_preferred(len2, bits1, bits2), with 1500 <= bits1 + bits2 <= 10000
    test_generated(8, 7, 1000, 900);
    // - !karatsuba_preferred(len2, bits1, bits2)
    test_generated(8, 7, 100, 90);
}

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from_str(t).unwrap();
        let r = &p * &q;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(&p * q.clone(), r);
        assert_eq!(p.clone() * &q, r);
        assert_eq!(p.clone() * q.clone(), r);
        let mut s = p.clone();
        s *= &q;
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s *= q.clone();
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mul_naive(&p, &q), r);
    };
    test("0", "x+1", "0");
    test("1", "x^2-3*x+5", "x^2-3*x+5");
    // A constant on the right, which the forms taking the left by value multiply in place.
    test("x^2-3*x+5", "-2", "-2*x^2+6*x-10");
    test("x+1", "x-1", "x^2-1");
    test("2*x^2+3", "-x+4", "-2*x^3+8*x^2-3*x+12");
    test("x^3-x", "x^3-x", "x^6-2*x^4+x^2");
    test(
        "18446744073709551616*x+1",
        "18446744073709551616*x-1",
        "340282366920938463463374607431768211456*x^2-1",
    );
}

#[test]
fn mul_to_out_classical_properties() {
    integer_vec_pair_gen_var_1().test_properties(|(xs, ys)| {
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_classical(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &ys), out);
        // Multiplication is commutative.
        let mut out_alt = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_classical(&mut out_alt, &ys, &xs);
        assert_eq!(out_alt, out);
        // The dispatcher agrees.
        if xs.len() >= ys.len() {
            mul_greater_to_out(&mut out_alt, &xs, &ys);
        } else {
            mul_greater_to_out(&mut out_alt, &ys, &xs);
        }
        assert_eq!(out_alt, out);
        // The square path agrees with the naive square.
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_to_out_classical(&mut out, &xs, &xs);
        assert_eq!(integers_mul_naive(&xs, &xs), out);
    });
}

#[test]
fn mul_to_out_tiny_1_properties() {
    integer_vec_pair_gen_var_2().test_properties(|(xs, ys)| {
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_tiny_1(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &ys), out);
        // The double-word kernel and classical multiplication agree.
        let mut out_alt = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_tiny_2(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
        mul_to_out_classical(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_to_out_tiny_2_properties() {
    integer_vec_pair_gen_var_3().test_properties(|(xs, ys)| {
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_tiny_2(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &ys), out);
        let mut out_alt = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_classical(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_greater_to_out_properties() {
    integer_vec_pair_gen_var_1().test_properties(|(xs, ys)| {
        let (xs, ys) = if xs.len() >= ys.len() {
            (xs, ys)
        } else {
            (ys, xs)
        };
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_greater_to_out(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &ys), out);
    });
}

#[test]
fn mul_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        let r = &p * &q;
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(&p * q.clone(), r);
        assert_eq!(p.clone() * &q, r);
        assert_eq!(p.clone() * q.clone(), r);
        let mut s = p.clone();
        s *= &q;
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s *= q.clone();
        assert!(s.is_valid());
        assert_eq!(s, r);

        assert_eq!(mul_naive(&p, &q), r);
        // Multiplication is commutative.
        assert_eq!(&q * &p, r);
        // Degrees add, since the integers have no zero divisors.
        assert_eq!(
            r.degree(),
            p.degree().and_then(|d| q.degree().map(|e| d + e))
        );
        // Negating a factor negates the product.
        assert_eq!(-&p * &q, -&r);
        // Evaluation is a ring homomorphism.
        for x in [Integer::from(3), Integer::from(-2)] {
            assert_eq!((&r).evaluate(&x), (&p).evaluate(&x) * (&q).evaluate(&x));
        }
        assert_eq!((&r).bit_pack(100), (&p).bit_pack(100) * (&q).bit_pack(100));
    });

    integer_polynomial_gen().test_properties(|p| {
        assert_eq!(&p * &p, (&p).square());
        assert_eq!(&p * IntegerPolynomial::ZERO, IntegerPolynomial::ZERO);
        assert_eq!(IntegerPolynomial::ZERO * &p, IntegerPolynomial::ZERO);
        assert_eq!(&p * IntegerPolynomial::one(), p);
        assert_eq!(IntegerPolynomial::one() * &p, p);
        assert_eq!(&p * IntegerPolynomial::negative_one(), -&p);
    });

    integer_polynomial_integer_pair_gen().test_properties(|(p, c)| {
        // Multiplying by a constant polynomial is scalar multiplication.
        assert_eq!(
            &p * IntegerPolynomial::from(c.clone()),
            IntegerPolynomial::from_coefficients_asc(integers_mul_scalar_naive(
                p.coefficients_asc(),
                &c
            ))
        );
    });

    integer_polynomial_triple_gen().test_properties(|(p, q, r)| {
        // Multiplication is associative and distributes over addition.
        assert_eq!(&(&p * &q) * &r, &p * &(&q * &r));
        assert_eq!(&p * &(&q + &r), &p * &q + &p * &r);
    });
}

#[test]
fn test_mul_to_out_karatsuba() {
    let test = |xs: &[&str], ys: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_karatsuba(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &ys), result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_to_out_karatsuba(&mut result, &xs, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(integers_mul_naive(&xs, &xs), result);
    };
    // - len1 == 1
    test(&["3"], &["4"], &["12"]);
    // - len1 != 1
    // - length == 1
    // - length != 1
    test(&["0", "-11"], &["5"], &["0", "-55"]);
    test(&["-17", "-16"], &["14", "-14"], &["-238", "14", "224"]);
    test(
        &["3", "17", "-17"],
        &["12", "-7"],
        &["36", "183", "-323", "119"],
    );
    test(
        &["-18", "-15", "7", "6"],
        &["-16", "-5", "-15", "15"],
        &["288", "330", "233", "-176", "-360", "15", "90"],
    );
    test(
        &["7", "-17", "16", "-13", "-6"],
        &["20", "20", "17"],
        &["140", "-200", "99", "-229", "-108", "-341", "-102"],
    );
    test(
        &["-17", "16", "17", "5", "-17", "-6", "-18", "15"],
        &["-12", "-2", "6", "-11", "14", "-13", "16", "-1"],
        &[
            "204", "-158", "-338", "189", "-118", "394", "-171", "129", "-119", "488", "-616",
            "365", "-477", "258", "-15",
        ],
    );
    test(
        &["15", "-9", "-14", "17", "16", "20", "-8", "3", "-14"],
        &["15", "-16", "16", "-17"],
        &["225", "-375", "174", "80", "-103", "554", "-473", "221", "-726", "408", "-275", "238"],
    );
    test(
        &["19", "-7", "11", "14", "7", "0", "9", "17", "9", "3", "-1", "-5", "-9"],
        &["-5", "-15", "16", "-1", "13", "11", "1", "8", "-2", "18", "-16", "-13", "12"],
        &[
            "-95", "-250", "354", "-366", "185", "226", "138", "221", "6", "648", "-129", "490",
            "795", "-42", "-154", "213", "79", "-345", "-250", "-41", "13", "-33", "197", "57",
            "-108",
        ],
    );
    test(
        &[
            "6", "-10", "1", "-11", "11", "6", "-18", "-16", "15", "16", "0", "1", "2", "18", "11",
            "17", "9",
        ],
        &["-16", "-15", "-3", "10", "-16"],
        &[
            "-96", "70", "116", "251", "-210", "-58", "39", "794", "-62", "-709", "-157", "342",
            "-127", "-577", "-442", "-487", "-284", "-364", "-33", "-182", "-144",
        ],
    );
    test(
        &[
            "-816811248518200125486424474127",
            "1139759822064785633541069947055",
            "432720081885913666037498687239",
            "-39114388956741277085988108600",
            "-881757175632785598351569251009",
        ],
        &[
            "-1102603777473801234131256647504",
            "510957267290233123464183609831",
            "540279011431855596052708421977",
        ],
        &[
            "900619168099259289203421384436147279967673856455806207129008",
            "-1674059128656283204360955756113748270772674809024022800243257",
            "-336056206702588670577199074735911825850554110838960453192830",
            "880017453494224838365999717907126698677540159027016534008744",
            "1186032589442308647552238191783856588823534944868022310736439",
            "-471672920273191530875583221259852279310841536826031461771679",
            "-476394895173826472909078396243987393842648244046320205024793",
        ],
    );
    // - xs and ys are the same slice
    test_square(
        &["15", "-3", "6", "2", "4", "-6", "-11", "-15", "-9"],
        &[
            "225", "-90", "189", "24", "144", "-180", "-242", "-440", "-320", "-218", "-220",
            "-24", "229", "438", "423", "270", "81",
        ],
    );
}

#[test]
fn mul_to_out_karatsuba_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let test = |(xs, ys): (Vec<Integer>, Vec<Integer>)| {
        let (xs, ys) = if xs.len() >= ys.len() {
            (xs, ys)
        } else {
            (ys, xs)
        };
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_karatsuba(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(integers_mul_naive(&xs, &ys), out);
        let mut out_alt = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_to_out_classical(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
        // The same slice as both arguments.
        let mut out = vec![Integer::ZERO; (xs.len() << 1) - 1];
        mul_to_out_karatsuba(&mut out, &xs, &xs);
        assert_eq!(integers_mul_naive(&xs, &xs), out);
    };
    integer_vec_pair_gen_var_1().test_properties(test);
    integer_vec_pair_gen_var_1().test_properties_with_config(&config, test);
}
