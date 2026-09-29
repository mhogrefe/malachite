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
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::mul_high::classical::mul_high_to_out_classical;
use malachite_nz::test_util::generators::integer_vec_integer_vec_unsigned_triple_gen_var_4;
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::integers_mul_naive;

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_mul_high_to_out_classical() {
    let test = |xs: &[&str], ys: &[&str], start: usize, out: &[&str]| {
        let xs = parse(xs);
        let ys = parse(ys);
        let mut result = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_high_to_out_classical(&mut result, &xs, &ys, start);
        assert!(result.iter().all(Integer::is_valid));
        assert_eq!(result, parse(out));
        assert_eq!(&integers_mul_naive(&xs, &ys)[start..], &result[start..]);
    };
    // - len1 == 1 && len2 == 1 && start == 0
    test(&["3"], &["4"], 0, &["12"]);
    // - len1 == 1 && len2 == 1 && start != 0
    test(&["3"], &["4"], 1, &["0"]);
    // - start < len1
    // - the accumulation loop runs
    test(&["1", "2", "3"], &["4", "5"], 1, &["0", "13", "22", "15"]);
    // - start >= len1
    test(&["1", "2"], &["3", "4", "5"], 3, &["0", "0", "0", "10"]);
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &["-18446744073709551616", "5"],
        0,
        &[
            "-21778071482940061661655974875633165533184",
            "5958298335808185171968",
            "-680564733841876926945195958937245974543",
            "184467440737095516165",
        ],
    );
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &["-18446744073709551616", "5"],
        2,
        &["0", "0", "-680564733841876926945195958937245974543", "184467440737095516165"],
    );
    test(
        &["1180591620717411303424", "-3", "36893488147419103233"],
        &["-18446744073709551616", "5"],
        4,
        &["0", "0", "0", "0"],
    );
    test(
        &["100000000000000000000", "-10000000000000000000000000", "7", "0", "3"],
        &["-1", "1267650600228229401496703205376", "11"],
        0,
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
        &["100000000000000000000", "-10000000000000000000000000", "7", "0", "3"],
        &["-1", "1267650600228229401496703205376", "11"],
        2,
        &[
            "0",
            "0",
            "-12676506002282294014967032053759998900000000000000000007",
            "8873444201597605810476922437632",
            "74",
            "3802951800684688204490109616128",
            "33",
        ],
    );
    test(
        &["100000000000000000000", "-10000000000000000000000000", "7", "0", "3"],
        &["-1", "1267650600228229401496703205376", "11"],
        4,
        &["0", "0", "0", "0", "74", "3802951800684688204490109616128", "33"],
    );
}

#[test]
fn mul_high_to_out_classical_properties() {
    integer_vec_integer_vec_unsigned_triple_gen_var_4().test_properties(|(xs, ys, start)| {
        let start = usize::exact_from(start);
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_high_to_out_classical(&mut out, &xs, &ys, start);
        assert!(out.iter().all(Integer::is_valid));
        // The low coefficients are zero, and the high ones are those of the product.
        assert!(out[..start].iter().all(|x| *x == 0u32));
        assert_eq!(&integers_mul_naive(&xs, &ys)[start..], &out[start..]);
        // Multiplication is commutative.
        let mut out_alt = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_high_to_out_classical(&mut out_alt, &ys, &xs, start);
        assert_eq!(out_alt, out);
    });
}
