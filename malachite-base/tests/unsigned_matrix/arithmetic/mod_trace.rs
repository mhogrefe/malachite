// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Trace, ModTrace};
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_7, unsigned_matrix_unsigned_pair_gen_var_8,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_trace() {
    let test = |s, m, out: u8| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let trace = a.to_mod_trace(m);
        assert_eq!(trace, out);
        assert_eq!(a.into_mod_trace(m), trace);
    };
    test("[]_{0,0}", 1, 0);
    test("[]_{0,0}", 5, 0);
    test("[0]", 1, 0);
    test("[5]", 6, 5);
    test("[5, 1; 2, 7]", 9, 3);
    test("[5, 1; 2, 7]", 8, 4);
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", 10, 5);
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", 16, 15);
    test("[254, 0; 0, 254]", 255, 253);
}

#[test]
#[should_panic]
fn to_mod_trace_fail_1() {
    // Not square.
    let _ = UnsignedMatrix::<u8>::from_str("[1, 2]")
        .unwrap()
        .to_mod_trace(3);
}

#[test]
#[should_panic]
fn to_mod_trace_fail_2() {
    // An off-diagonal element that is not reduced.
    let _ = UnsignedMatrix::<u8>::from_str("[1, 9; 0, 1]")
        .unwrap()
        .to_mod_trace(9);
}

#[test]
#[should_panic]
fn to_mod_trace_fail_3() {
    // The modulus is zero.
    let _ = UnsignedMatrix::<u8>::from_str("[]_{0,0}")
        .unwrap()
        .to_mod_trace(0);
}

#[test]
#[should_panic]
fn into_mod_trace_fail_1() {
    let _ = UnsignedMatrix::<u8>::from_str("[]_{0,2}")
        .unwrap()
        .into_mod_trace(3);
}

#[test]
#[should_panic]
fn into_mod_trace_fail_2() {
    let _ = UnsignedMatrix::<u8>::from_str("[9]")
        .unwrap()
        .into_mod_trace(9);
}

#[test]
#[should_panic]
fn into_mod_trace_fail_3() {
    let _ = UnsignedMatrix::<u8>::from_str("[]_{0,0}")
        .unwrap()
        .into_mod_trace(0);
}

#[test]
fn mod_trace_properties() {
    unsigned_matrix_unsigned_pair_gen_var_8().test_properties(|(a, m)| {
        let trace = a.to_mod_trace(m);
        assert_eq!(a.clone().into_mod_trace(m), trace);
        assert!(trace < m);
        // Summing the diagonal exactly, in a `u128`, and then reducing gives the same result.
        assert_eq!(
            u64::try_from(
                (0..a.height())
                    .map(|i| u128::from(*a.get(i, i).unwrap()))
                    .sum::<u128>()
                    % u128::from(m)
            )
            .unwrap(),
            trace
        );
        // The trace is unchanged by transposition.
        assert_eq!(a.transpose().into_mod_trace(m), trace);
    });

    unsigned_matrix_unsigned_pair_gen_var_7().test_properties(|(a, pow)| {
        // Modulo a power of 2 that fits, the trace agrees with the power-of-2 trace.
        if pow < 64 {
            assert_eq!(a.to_mod_trace(1 << pow), a.to_mod_power_of_2_trace(pow));
        }
    });
}
