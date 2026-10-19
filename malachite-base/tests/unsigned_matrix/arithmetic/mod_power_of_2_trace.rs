// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Trace};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_7;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_trace() {
    let test = |s, pow, out: u8| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let trace = a.to_mod_power_of_2_trace(pow);
        assert_eq!(trace, out);
        assert_eq!(a.into_mod_power_of_2_trace(pow), trace);
    };
    test("[]_{0,0}", 0, 0);
    test("[]_{0,0}", 5, 0);
    test("[0]", 0, 0);
    test("[5]", 3, 5);
    test("[5, 1; 2, 7]", 3, 4);
    test("[5, 1; 2, 7]", 4, 12);
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", 4, 15);
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", 8, 15);
    test("[255, 0; 0, 255]", 8, 254);
}

#[test]
#[should_panic]
fn to_mod_power_of_2_trace_fail_1() {
    // Not square.
    let _ = UnsignedMatrix::<u8>::from_str("[1, 2]")
        .unwrap()
        .to_mod_power_of_2_trace(3);
}

#[test]
#[should_panic]
fn to_mod_power_of_2_trace_fail_2() {
    // An off-diagonal element that is not reduced.
    let _ = UnsignedMatrix::<u8>::from_str("[1, 8; 0, 1]")
        .unwrap()
        .to_mod_power_of_2_trace(3);
}

#[test]
#[should_panic]
fn to_mod_power_of_2_trace_fail_3() {
    // `pow` is too large.
    let _ = UnsignedMatrix::<u8>::from_str("[]_{0,0}")
        .unwrap()
        .to_mod_power_of_2_trace(9);
}

#[test]
#[should_panic]
fn into_mod_power_of_2_trace_fail_1() {
    let _ = UnsignedMatrix::<u8>::from_str("[]_{0,2}")
        .unwrap()
        .into_mod_power_of_2_trace(3);
}

#[test]
#[should_panic]
fn into_mod_power_of_2_trace_fail_2() {
    let _ = UnsignedMatrix::<u8>::from_str("[8]")
        .unwrap()
        .into_mod_power_of_2_trace(3);
}

#[test]
#[should_panic]
fn into_mod_power_of_2_trace_fail_3() {
    let _ = UnsignedMatrix::<u8>::from_str("[1]")
        .unwrap()
        .into_mod_power_of_2_trace(9);
}

#[test]
fn mod_power_of_2_trace_properties() {
    unsigned_matrix_unsigned_pair_gen_var_7().test_properties(|(a, pow)| {
        let trace = a.to_mod_power_of_2_trace(pow);
        assert_eq!(a.clone().into_mod_power_of_2_trace(pow), trace);
        assert!(trace < 1u64.wrapping_shl(u32::try_from(pow).unwrap()) || pow == 64);
        // Summing the diagonal with wrapping arithmetic, which is exact modulo $2^{64}$, and then
        // reducing gives the same result.
        let n = a.height();
        assert_eq!(
            (0..n)
                .map(|i| a.get(i, i).unwrap())
                .fold(0u64, |s, &x| s.wrapping_add(x))
                .mod_power_of_2(pow),
            trace
        );
        // The trace is unchanged by transposition.
        assert_eq!(a.transpose().into_mod_power_of_2_trace(pow), trace);
    });
}
