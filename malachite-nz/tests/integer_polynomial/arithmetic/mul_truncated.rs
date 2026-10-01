// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{EqTruncated, MulTruncated, MulTruncatedAssign, Polynomial};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::classical::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::karatsuba::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::kronecker::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::mul_truncated_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::schonhage_strassen::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::tiny::{
    mul_truncated_to_out_tiny_1, mul_truncated_to_out_tiny_2,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1,
    integer_vec_integer_vec_unsigned_triple_gen_var_1,
    integer_vec_integer_vec_unsigned_triple_gen_var_2,
    integer_vec_integer_vec_unsigned_triple_gen_var_3,
    integer_vec_integer_vec_unsigned_triple_gen_var_5,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::{
    generated_coefficients, integers_mul_naive,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul_truncated::*;

fn coefficients(p: &str) -> Vec<Integer> {
    IntegerPolynomial::from_str(p)
        .unwrap()
        .into_coefficients_asc()
}

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_mul_truncated_to_out_classical() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_classical(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // - n == 1, so xs.len() == 1 after truncation
    test(&["1", "2", "3"], &["4", "5", "6"], 1, &["4"]);
    // - xs.len() == 1
    test(&["3"], &["4", "5", "6"], 2, &["12", "15"]);
    // - ys.len() == 1
    test(&["4", "5", "6"], &["3"], 2, &["12", "15"]);
    // - xs.len() != 1 && ys.len() != 1
    test(&["1", "2", "3"], &["4", "5", "6"], 3, &["4", "13", "28"]);
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &["-18446744073709551616", "5"],
        3,
        &[
            "-21778071482940061661655974875633165533184",
            "5958298335808185171968",
            "-680564733841876926945195958937245974543",
        ],
    );
    test(
        &["100000000000000000000", "-10000000000000000000000000", "7", "0", "3"],
        &["-1", "1267650600228229401496703205376", "11"],
        5,
        &[
            "-100000000000000000000",
            "126765060022822940149670330537600000000000000000000",
            "-12676506002282294014967032053759998900000000000000000007",
            "8873444201597605810476922437632",
            "74",
        ],
    );
}

#[test]
fn test_mul_truncated_to_out_tiny_1() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_tiny_1(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // - x != 0
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        4,
        &["4", "13", "28", "27"],
    );
    // - x == 0
    test(&["0", "1", "2"], &["3", "4"], 3, &["0", "3", "10"]);
    test(
        &["1", "1", "1", "1", "1", "1", "1", "1"],
        &["-1", "-1", "-1", "-1", "-1", "-1", "-1", "-1"],
        10,
        &["-1", "-2", "-3", "-4", "-5", "-6", "-7", "-8", "-7", "-6"],
    );
}

#[test]
fn test_mul_truncated_to_out_tiny_2() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_tiny_2(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // - x != 0
    // - y != 0
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        &["2305843009213693952", "-2305843009213693951"],
        &["2305843009213693951", "1152921504606846976"],
        2,
        &["5316911983139663489309385231907684352", "-2658455991569831741195928102133301249"],
    );
    #[cfg(feature = "32_bit_limbs")]
    test(
        &["536870912", "-536870911"],
        &["536870911", "268435456"],
        2,
        &["288230375614840832", "-144115187002114049"],
    );
    // - x == 0
    // - y == 0
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        &["0", "1099511627776", "5"],
        &["3", "0", "1048576"],
        4,
        &["0", "3298534883328", "15", "1152921504606846976"],
    );
    #[cfg(feature = "32_bit_limbs")]
    test(
        &["0", "256", "0"],
        &["0", "0", "0"],
        4,
        &["0", "0", "0", "0"],
    );
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        &["-1099511627776", "5", "1125899906842624"],
        &["3", "-35184372088832", "1048576", "1"],
        5,
        &[
            "-3298534883328",
            "38685626227668133590597647",
            "-1149719726746763264",
            "-39614081257132169896278360064",
            "1180591620717411303429",
        ],
    );
    #[cfg(feature = "32_bit_limbs")]
    test(
        &["-256", "0", "262144"],
        &["0", "-8192", "0", "0"],
        5,
        &["0", "2097152", "0", "-2147483648", "0"],
    );
}

#[test]
fn test_mul_truncated_to_out() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out(&mut result, &xs, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    // Long inputs, written as polynomials.
    let test_poly = |p: &str, q: &str, n: usize, out: &str| {
        let xs = coefficients(p);
        let ys = coefficients(q);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out(&mut result, &xs, &ys);
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
        assert_eq!(
            IntegerPolynomial::from_coefficients_asc(result).to_string(),
            out
        );
    };
    // - xs.len() < ys.len()
    test(&["3", "4"], &["1", "2", "3"], 3, &["3", "10", "17"]);
    // - ys.len() == 1
    test(&["1", "2", "3"], &["-6"], 2, &["-6", "-12"]);
    // - xs and ys are the same slice
    test_square(&["1", "2", "3"], 4, &["1", "4", "10", "12"]);
    // - len2 < 50
    // - rbits <= SMALL_FMPZ_BITCOUNT_MAX
    test(
        &["1", "2", "3"],
        &["4", "5", "6"],
        4,
        &["4", "13", "28", "27"],
    );
    // - rbits > SMALL_FMPZ_BITCOUNT_MAX && rbits < 2 * Limb::WIDTH
    test(
        &["2305843009213693952", "-2305843009213693951"],
        &["2305843009213693951", "1152921504606846976"],
        2,
        &["5316911983139663489309385231907684352", "-2658455991569831741195928102133301249"],
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
        15,
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
    // - bits1 > SMALL_FMPZ_BITCOUNT_MAX || bits2 > SMALL_FMPZ_BITCOUNT_MAX
    // - classical
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &["-18446744073709551616", "5"],
        3,
        &[
            "-21778071482940061661655974875633165533184",
            "5958298335808185171968",
            "-680564733841876926945195958937245974543",
        ],
    );
    // - len2 >= 50 && 4 * len2 >= 3 * n && n < 150 + bits1 + bits2
    test_poly("x^59+1", "-x^59+2*x^30+1", 60, "2*x^30+1");
    // - len2 >= 50 && !(4 * len2 >= 3 * n && n < 150 + bits1 + bits2)
    test_poly("x^99+1", "-x^99+1", 199, "-x^198+1");
    // Generated coefficients of `bits1` and `bits2` bits.
    let test_generated = |len1: usize, len2: usize, bits1: u64, bits2: u64, n: usize| {
        let xs = generated_coefficients(len1, bits1);
        let ys = generated_coefficients(len2, bits2);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out(&mut result, &xs, &ys);
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // - karatsuba_preferred(len2, bits1, bits2), with len2 <= 4
    test_generated(4, 3, 6000, 5500, 5);
    // - karatsuba_preferred(len2, bits1, bits2), with 1500 <= bits1 + bits2 <= 10000
    test_generated(8, 7, 1000, 900, 12);
    // - !karatsuba_preferred(len2, bits1, bits2)
    test_generated(8, 7, 100, 90, 12);
    // - fft_preferred(len2, bits1, bits2, 100, 200), and the FFT computes the product
    test_generated(120, 110, 20, 20, 150);
    // - fft_preferred(len2, bits1, bits2, 100, 200), but the FFT declines
    test_generated(120, 110, 250, 250, 150);
    // - schonhage_strassen_preferred(len1, len2, bits1, bits2, 4097), with 16 <= len2 <= 100
    test_generated(30, 30, 500, 400, 40);
    // - schonhage_strassen_preferred(len1, len2, bits1, bits2, 4097), with len2 > 100
    test_generated(150, 120, 600, 500, 200);
    // - schonhage_strassen_preferred(len1, len2, bits1, bits2, 4097), with 8 <= len2 < 16
    test_generated(12, 10, 600, 500, 15);
    // - !schonhage_strassen_preferred(len1, len2, bits1, bits2, 4097), with 8 <= len2 < 16 and
    //   bits1 + bits2 < 1000
    test_generated(12, 10, 500, 400, 15);
    // - !schonhage_strassen_preferred(len1, len2, bits1, bits2, 4097), with len1 + len2 > 4097
    test_generated(2100, 2000, 600, 500, 2500);
}

#[test]
fn test_mul_truncated() {
    let test = |s, t, len, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from_str(t).unwrap();
        let r = (&p).mul_truncated(&q, len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mul_truncated(q.clone(), len), r);
        assert_eq!(p.clone().mul_truncated(&q, len), r);
        assert_eq!(p.clone().mul_truncated(q.clone(), len), r);
        let mut s = p.clone();
        s.mul_truncated_assign(&q, len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mul_truncated_assign(q.clone(), len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mul_truncated_naive(&p, &q, len), r);
    };
    test("0", "x+1", 3, "0");
    test("x+1", "x-1", 0, "0");
    test("x+1", "x-1", 1, "-1");
    test("x+1", "x-1", 2, "-1");
    test("x+1", "x-1", 3, "x^2-1");
    test("x+1", "x-1", 100, "x^2-1");
    // Constants on either side, which the forms taking the other by value multiply in place.
    test("3", "x^2+x-1", 2, "3*x-3");
    test("x^2+x-1", "-2", 3, "-2*x^2-2*x+2");
    test("x^2+x-1", "-2", 0, "0");
    test("2*x^2+3", "-x+4", 2, "-3*x+12");
    test("x^3+x^2+x+1", "x-1", 3, "-1");
    test(
        "18446744073709551616*x+1",
        "18446744073709551616*x-1",
        2,
        "-1",
    );
}

#[test]
fn mul_truncated_to_out_classical_properties() {
    integer_vec_integer_vec_unsigned_triple_gen_var_1().test_properties(|(xs, ys, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out_classical(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        // Multiplication is commutative.
        let mut out_alt = vec![Integer::ZERO; n];
        mul_truncated_to_out_classical(&mut out_alt, &ys, &xs);
        assert_eq!(out_alt, out);
        // The dispatcher agrees.
        mul_truncated_to_out(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_truncated_to_out_tiny_1_properties() {
    integer_vec_integer_vec_unsigned_triple_gen_var_2().test_properties(|(xs, ys, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out_tiny_1(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        mul_truncated_to_out_tiny_2(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
        mul_truncated_to_out_classical(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_truncated_to_out_tiny_2_properties() {
    integer_vec_integer_vec_unsigned_triple_gen_var_3().test_properties(|(xs, ys, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out_tiny_2(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        mul_truncated_to_out_classical(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
    });
}

#[test]
fn mul_truncated_to_out_properties() {
    integer_vec_integer_vec_unsigned_triple_gen_var_1().test_properties(|(xs, ys, n)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        // The square path.
        if n < xs.len() << 1 {
            mul_truncated_to_out(&mut out, &xs, &xs);
            assert_eq!(&integers_mul_naive(&xs, &xs)[..n], out);
        }
    });
}

#[test]
fn mul_truncated_properties() {
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, len)| {
            let r = (&p).mul_truncated(&q, len);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mul_truncated(q.clone(), len), r);
            assert_eq!(p.clone().mul_truncated(&q, len), r);
            assert_eq!(p.clone().mul_truncated(q.clone(), len), r);
            let mut s = p.clone();
            s.mul_truncated_assign(&q, len);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mul_truncated_assign(q.clone(), len);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // This is the truncation of the whole product, and of the product of the truncations.
            assert_eq!(mul_truncated_naive(&p, &q, len), r);
            assert_eq!((p.truncate(len) * q.truncate(len)).truncate(len), r);
            assert!(r.eq_truncated(&(&p * &q), len));
            assert!(r.len() <= len);
            // Multiplication is commutative.
            assert_eq!((&q).mul_truncated(&p, len), r);
            // Negating a factor negates the product.
            assert_eq!((-&p).mul_truncated(&q, len), -&r);
            // The square path.
            assert_eq!((&p).mul_truncated(&p, len), (&p * &p).truncate(len));
        },
    );
}

#[test]
fn test_mul_truncated_to_out_karatsuba_n() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_karatsuba_n(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // - n == 1
    test(&["3", "1"], &["4", "5"], 1, &["12"]);
    // - n != 1
    // - len <= 6
    test(&["-11", "-6"], &["-6", "-20"], 2, &["66", "256"]);
    test(
        &["11", "17", "-9", "-4", "-2", "-20"],
        &["-11", "6", "14", "3", "19", "16"],
        6,
        &["-121", "-121", "355", "261", "132", "624"],
    );
    // - len > 6
    // - len odd
    test(
        &["0", "-12", "12", "19", "-17", "9", "15"],
        &["5", "5", "5", "5", "-14", "10", "20"],
        7,
        &["0", "-60", "0", "95", "10", "283", "-158"],
    );
    test(
        &["5", "-17", "-8", "-16", "-7", "8", "-10"],
        &["-13", "1", "18", "-17", "-14", "-20", "16", "-11", "14"],
        7,
        &["-65", "226", "177", "-191", "150", "-125", "816"],
    );
    // - len even
    test(
        &["-14", "3", "19", "-19", "-16", "-7", "19", "4"],
        &["-11", "20", "-4", "2", "18", "3", "10", "-13"],
        8,
        &["154", "-313", "-93", "549", "-526", "-117", "-112", "259"],
    );
    test(
        &["-13", "11", "9", "10", "10", "-1", "-15", "-11", "-14", "1", "-4", "10", "-10"],
        &["13", "-19", "-7", "13", "3", "-11", "14", "-19", "13", "-1", "-15", "-4", "13"],
        13,
        &[
            "-169", "390", "-1", "-287", "-19", "20", "-392", "611", "-213", "173", "101", "-47",
            "-619",
        ],
    );
    test(
        &["3", "-10", "2", "-6", "14", "14", "12", "1", "20", "-6", "19", "-8", "-5", "5"],
        &["-6", "-8", "13", "11", "2", "-19", "-19", "-3", "10", "-4", "-8", "18", "2", "8"],
        14,
        &[
            "-18", "36", "107", "-77", "-114", "-329", "69", "365", "346", "-221", "-249", "-430",
            "0", "-160",
        ],
    );
    test(
        &[
            "2", "3", "-15", "-6", "-14", "-6", "10", "-8", "1", "-7", "10", "19", "19", "-20",
            "10", "2", "-15", "-13", "4", "-8", "10", "-9", "7", "20", "1", "-15", "5", "9", "5",
            "-15",
        ],
        &[
            "-10", "-10", "-12", "-19", "-11", "17", "9", "-11", "19", "18", "10", "2", "-11",
            "15", "15", "-12", "-19", "-20", "-14", "13", "-12", "7", "-8", "-7", "-19", "-4",
            "-7", "-2", "12", "-5",
        ],
        30,
        &[
            "-20", "-50", "96", "136", "301", "558", "476", "134", "-14", "-2", "-373", "-217",
            "-898", "-955", "-451", "-552", "399", "344", "-339", "1427", "2179", "91", "-183",
            "-485", "41", "530", "-918", "-900", "556", "164",
        ],
    );
    test(
        &[
            "1063163706938890324893935975217",
            "-454763477228498539230279462980",
            "1173768953842122485502204965713",
            "703400998902306310627207900417",
            "-675301765116898074686887669334",
            "-219977481268136083068520771607",
            "-170324900689545855763533692245",
            "-136683892998160668393226032438",
            "-674893116231446755376985510117",
        ],
        &[
            "1117601129296510483585569264818",
            "-21939630935524758010898240249",
            "-44504787126949462546721639250",
            "-1236951896353484377947228387850",
            "-669663825816813576819066720685",
            "891131101724247349517719701150",
            "1259153770815232730586330928645",
            "-486233884590374956027962079697",
            "-104262661335290497304978652644",
        ],
        9,
        &[
            "1188192959501968145983716463127948712576543952645180558015506",
            "-531569595067661552967415887097823923742775798269021423346673",
            "1274466976742075155946806596981949477331383448623291434200004",
            "-534473518614456946729250263585481734310009672322238209901881",
            "-971830440961321053000573531439882366623996955381203066480940",
            "-462274759564905448123183742189195685589479692652607486811810",
            "-878146010946456846493205780315961857278145658681640137009657",
            "181461092874684380908486801737040986620391478688951610682327",
            "2195695730962830162107934932281906498410275962586300254920093",
        ],
    );
}

#[test]
fn test_mul_truncated_to_out_karatsuba() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_karatsuba(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_karatsuba(&mut result, &xs, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    // - xs.len() >= n in padded
    test(&["-5"], &["7"], 1, &["-35"]);
    // - xs.len() < n in padded
    test(
        &["-16", "-7", "-1"],
        &["-13", "-11"],
        4,
        &["208", "267", "90", "11"],
    );
    test(
        &["3", "-11"],
        &["-4", "-12", "9", "-6", "-14", "5", "11"],
        5,
        &["-12", "8", "159", "-117", "24"],
    );
    test(
        &["-10", "-6", "-10", "7", "12", "5", "1"],
        &["6", "-8", "2", "0", "-15", "3", "-19"],
        13,
        &["-60", "44", "-32", "110", "146", "8", "312", "-19", "33", "-172", "-228", "-92", "-19"],
    );
    test(
        &["1", "15", "9", "8", "-19", "4", "1", "13", "19", "-2"],
        &["12", "-16", "-13"],
        8,
        &["12", "164", "-145", "-243", "-473", "248", "195", "88"],
    );
    test(
        &["-6", "-14", "-15", "-4", "-3", "-18", "-9", "-3", "-12", "7", "-4", "5", "-11"],
        &[
            "14", "12", "16", "11", "0", "-15", "-3", "-17", "-9", "7", "-16", "-3", "-19", "20",
            "-15", "-4", "-15", "18", "-6", "-16",
        ],
        25,
        &[
            "-84", "-268", "-474", "-526", "-484", "-427", "-206", "-102", "-149", "203", "283",
            "415", "692", "642", "-29", "393", "451", "650", "115", "167", "839", "43", "462",
            "-160", "809",
        ],
    );
    test(
        &[
            "-4", "-13", "9", "-20", "1", "15", "6", "-3", "19", "-12", "-18", "13", "-5", "-13",
            "-10",
        ],
        &["-4", "-17", "-9", "-8", "-1", "20", "-1", "13", "-7", "-2", "8", "12", "-9", "-3", "2"],
        15,
        &[
            "16", "120", "221", "76", "363", "-36", "-393", "-72", "-750", "-55", "93", "301",
            "152", "607", "-432",
        ],
    );
    // - xs and ys are the same slice
    test_square(
        &["-19", "-4", "-18", "-20", "-19", "12", "15", "-8", "12", "10", "-5", "8"],
        12,
        &[
            "361", "152", "700", "904", "1206", "416", "418", "512", "-1051", "-1244", "-428",
            "-440",
        ],
    );
}

#[test]
fn mul_truncated_to_out_karatsuba_n_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let test = |(xs, ys, n): (Vec<Integer>, Vec<Integer>, u64)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out_karatsuba_n(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs[..n], &ys[..n])[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        mul_truncated_to_out_classical(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
    };
    integer_vec_integer_vec_unsigned_triple_gen_var_5().test_properties(test);
    integer_vec_integer_vec_unsigned_triple_gen_var_5().test_properties_with_config(&config, test);
}

#[test]
fn mul_truncated_to_out_karatsuba_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let test = |(xs, ys, n): (Vec<Integer>, Vec<Integer>, u64)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out_karatsuba(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        mul_truncated_to_out_karatsuba(&mut out_alt, &ys, &xs);
        assert_eq!(out_alt, out);
        mul_truncated_to_out(&mut out_alt, &xs, &ys);
        assert_eq!(out_alt, out);
        // The same slice as both arguments.
        if n < xs.len() << 1 {
            mul_truncated_to_out_karatsuba(&mut out, &xs, &xs);
            assert_eq!(&integers_mul_naive(&xs, &xs)[..n], out);
        }
    };
    integer_vec_integer_vec_unsigned_triple_gen_var_1().test_properties(test);
    integer_vec_integer_vec_unsigned_triple_gen_var_1().test_properties_with_config(&config, test);
}

#[test]
fn test_mul_truncated_to_out_kronecker() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_kronecker(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_kronecker(&mut result, &xs, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    test(&["3"], &["4"], 1, &["12"]);
    test(&["1", "2", "3"], &["4", "5", "6"], 3, &["4", "13", "28"]);
    test(
        &["1", "-2", "3"],
        &["4", "5", "-6"],
        5,
        &["4", "-3", "-4", "27", "-18"],
    );
    test(
        &[
            "-249280210867129111655121130485",
            "774272247025418148342010903136",
            "109823698889072694555589859827",
            "-838450632265568977780093348695",
            "-425350271670103455527167616781",
        ],
        &[
            "601125560895236399022862340007",
            "-806766728941798427326353285204",
            "-1048060506937750767971840644108",
        ],
        4,
        &[
            "-149848706577585791160335678754689229414827769322918282813395",
            "666545819089965023668422513634740818970863517732934478405892",
            "-297378511277876598114991372218805575453317633564910767368575",
            "-1404100376641923078592484163909680189289370632896284649963261",
        ],
    );
    test_square(
        &["19", "-5", "-8", "-10", "20", "15", "-8"],
        9,
        &["361", "-190", "-279", "-300", "924", "530", "-674", "-560", "228"],
    );
}

#[test]
fn test_mul_truncated_to_out_schonhage_strassen() {
    let test = |xs: &[&str], ys: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_schonhage_strassen(&mut result, &xs, &ys);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], result);
    };
    // Both arguments are the same slice.
    let test_square = |xs: &[&str], n: usize, out: &[&str]| {
        let xs = parse(xs);
        let mut result = vec![Integer::ZERO; n];
        mul_truncated_to_out_schonhage_strassen(&mut result, &xs, &xs);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &xs)[..n], result);
    };
    test(&["3"], &["4"], 1, &["12"]);
    test(&["1", "2", "3"], &["4", "5", "6"], 3, &["4", "13", "28"]);
    test(
        &["1", "-2", "3"],
        &["4", "5", "-6"],
        5,
        &["4", "-3", "-4", "27", "-18"],
    );
    test(
        &[
            "-249280210867129111655121130485",
            "774272247025418148342010903136",
            "109823698889072694555589859827",
            "-838450632265568977780093348695",
            "-425350271670103455527167616781",
        ],
        &[
            "601125560895236399022862340007",
            "-806766728941798427326353285204",
            "-1048060506937750767971840644108",
        ],
        4,
        &[
            "-149848706577585791160335678754689229414827769322918282813395",
            "666545819089965023668422513634740818970863517732934478405892",
            "-297378511277876598114991372218805575453317633564910767368575",
            "-1404100376641923078592484163909680189289370632896284649963261",
        ],
    );
    test_square(
        &["19", "-5", "-8", "-10", "20", "15", "-8"],
        9,
        &["361", "-190", "-279", "-300", "924", "530", "-674", "-560", "228"],
    );
}

#[test]
fn mul_truncated_to_out_kronecker_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let test = |(xs, ys, n): (Vec<Integer>, Vec<Integer>, u64)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out_kronecker(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        mul_truncated_to_out_kronecker(&mut out_alt, &ys, &xs);
        assert_eq!(out_alt, out);
    };
    integer_vec_integer_vec_unsigned_triple_gen_var_1().test_properties(test);
    integer_vec_integer_vec_unsigned_triple_gen_var_1().test_properties_with_config(&config, test);
}

#[test]
fn mul_truncated_to_out_schonhage_strassen_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_len_n", 32);
    let mut wide_config = GenConfig::new();
    wide_config.insert("mean_len_n", 20);
    wide_config.insert("mean_bits_n", 1000);
    let test = |(xs, ys, n): (Vec<Integer>, Vec<Integer>, u64)| {
        let n = usize::exact_from(n);
        let mut out = vec![Integer::ZERO; n];
        mul_truncated_to_out_schonhage_strassen(&mut out, &xs, &ys);
        assert!(out.iter().all(Integer::is_valid));
        assert_eq!(&integers_mul_naive(&xs, &ys)[..n], out);
        let mut out_alt = vec![Integer::ZERO; n];
        mul_truncated_to_out_schonhage_strassen(&mut out_alt, &ys, &xs);
        assert_eq!(out_alt, out);
    };
    integer_vec_integer_vec_unsigned_triple_gen_var_1().test_properties(test);
    integer_vec_integer_vec_unsigned_triple_gen_var_1().test_properties_with_config(&config, test);
    integer_vec_integer_vec_unsigned_triple_gen_var_1()
        .test_properties_with_config(&wide_config, test);
}
