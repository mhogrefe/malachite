// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Trace, Trace};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_6;

#[test]
fn test_mod_power_of_2_trace() {
    let test = |s, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let trace = a.to_mod_power_of_2_trace(pow);
        assert!(trace.is_valid());
        assert_eq!(trace.to_string(), out);
        assert_eq!(a.into_mod_power_of_2_trace(pow), trace);
    };
    test("[]_{0,0}", 0, "0");
    test("[]_{0,0}", 5, "0");
    test("[0]", 0, "0");
    test("[5]", 3, "5");
    test("[5, 1; 2, 7]", 3, "4");
    test("[5, 1; 2, 7]", 4, "12");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", 4, "15");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", 8, "15");
    test(
        "[18446744073709551615, 0; 0, 18446744073709551615]",
        64,
        "18446744073709551614",
    );
    test(
        "[18446744073709551615, 0; 0, 18446744073709551615]",
        65,
        "36893488147419103230",
    );
    test(
        "[340282366920938463463374607431768211455, 1; 1, 1]",
        128,
        "0",
    );
}

#[test]
#[should_panic]
fn to_mod_power_of_2_trace_fail_1() {
    // Not square.
    let _ = NaturalMatrix::from_str("[1, 2]")
        .unwrap()
        .to_mod_power_of_2_trace(3);
}

#[test]
#[should_panic]
fn to_mod_power_of_2_trace_fail_2() {
    // An off-diagonal element that is not reduced.
    let _ = NaturalMatrix::from_str("[1, 8; 0, 1]")
        .unwrap()
        .to_mod_power_of_2_trace(3);
}

#[test]
#[should_panic]
fn into_mod_power_of_2_trace_fail_1() {
    let _ = NaturalMatrix::from_str("[]_{0,2}")
        .unwrap()
        .into_mod_power_of_2_trace(3);
}

#[test]
#[should_panic]
fn into_mod_power_of_2_trace_fail_2() {
    let _ = NaturalMatrix::from_str("[8]")
        .unwrap()
        .into_mod_power_of_2_trace(3);
}

#[test]
fn mod_power_of_2_trace_properties() {
    natural_matrix_unsigned_pair_gen_var_6().test_properties(|(a, pow)| {
        let trace = a.to_mod_power_of_2_trace(pow);
        assert!(trace.is_valid());
        assert_eq!(a.clone().into_mod_power_of_2_trace(pow), trace);
        assert!(trace.significant_bits() <= pow);
        // Reducing the exact trace gives the same result.
        assert_eq!(a.to_trace().mod_power_of_2(pow), trace);
        // The trace is unchanged by transposition.
        assert_eq!(a.transpose().into_mod_power_of_2_trace(pow), trace);
        // When the elements fit in a `u64`, the `UnsignedMatrix` trace agrees.
        if pow <= 64 {
            let b = UnsignedMatrix::<u64>::try_from(&a).unwrap();
            assert_eq!(Natural::from(b.into_mod_power_of_2_trace(pow)), trace);
        }
    });
}
