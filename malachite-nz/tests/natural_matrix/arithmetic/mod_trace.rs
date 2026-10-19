// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Mod, ModPowerOf2Trace, ModTrace, PowerOf2, Trace};
use malachite_base::num::basic::traits::Zero;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_pair_gen_var_3, natural_matrix_unsigned_pair_gen_var_6,
};

#[test]
fn test_mod_trace() {
    let test = |s, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let trace = a.to_mod_trace(&m);
        assert!(trace.is_valid());
        assert_eq!(trace.to_string(), out);
        assert_eq!(a.to_mod_trace(m.clone()), trace);
        assert_eq!(a.clone().into_mod_trace(&m), trace);
        assert_eq!(a.into_mod_trace(m), trace);
    };
    test("[]_{0,0}", "1", "0");
    test("[]_{0,0}", "5", "0");
    test("[0]", "1", "0");
    test("[5]", "6", "5");
    test("[5, 1; 2, 7]", "9", "3");
    test("[5, 1; 2, 7]", "8", "4");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "10", "5");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "16", "15");
    test("[254, 0; 0, 254]", "255", "253");
    test(
        "[18446744073709551615, 0; 0, 18446744073709551615]",
        "18446744073709551616",
        "18446744073709551614",
    );
    test(
        "[18446744073709551628, 0; 0, 18446744073709551627]",
        "18446744073709551629",
        "18446744073709551626",
    );
    test(
        "[18446744073709551628, 5; 5, 1]",
        "18446744073709551629",
        "0",
    );
}

#[test]
#[should_panic]
fn to_mod_trace_fail_1() {
    // Not square.
    let _ = NaturalMatrix::from_str("[1, 2]")
        .unwrap()
        .to_mod_trace(Natural::from(3u32));
}

#[test]
#[should_panic]
fn to_mod_trace_fail_2() {
    // An off-diagonal element that is not reduced.
    let _ = NaturalMatrix::from_str("[1, 9; 0, 1]")
        .unwrap()
        .to_mod_trace(&Natural::from(9u32));
}

#[test]
#[should_panic]
fn to_mod_trace_fail_3() {
    // The modulus is zero.
    let _ = NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .to_mod_trace(Natural::ZERO);
}

#[test]
#[should_panic]
fn into_mod_trace_fail_1() {
    let _ = NaturalMatrix::from_str("[]_{0,2}")
        .unwrap()
        .into_mod_trace(&Natural::from(3u32));
}

#[test]
#[should_panic]
fn into_mod_trace_fail_2() {
    let _ = NaturalMatrix::from_str("[9]")
        .unwrap()
        .into_mod_trace(Natural::from(9u32));
}

#[test]
#[should_panic]
fn into_mod_trace_fail_3() {
    let _ = NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .into_mod_trace(&Natural::ZERO);
}

#[test]
fn mod_trace_properties() {
    natural_matrix_natural_pair_gen_var_3().test_properties(|(a, m)| {
        let trace = a.to_mod_trace(&m);
        assert!(trace.is_valid());
        assert_eq!(a.to_mod_trace(m.clone()), trace);
        assert_eq!(a.clone().into_mod_trace(&m), trace);
        assert_eq!(a.clone().into_mod_trace(m.clone()), trace);
        assert!(trace < m);
        // Reducing the exact trace gives the same result.
        assert_eq!(a.to_trace().mod_op(&m), trace);
        // The trace is unchanged by transposition.
        assert_eq!(a.transpose().into_mod_trace(&m), trace);
        // When the modulus fits in a `u64`, the `UnsignedMatrix` trace agrees.
        if let Ok(m) = u64::try_from(&m) {
            let b = UnsignedMatrix::<u64>::try_from(&a).unwrap();
            assert_eq!(Natural::from(b.into_mod_trace(m)), trace);
        }
    });

    natural_matrix_unsigned_pair_gen_var_6().test_properties(|(a, pow)| {
        // Modulo a power of 2, the trace agrees with the power-of-2 trace.
        assert_eq!(
            a.to_mod_trace(Natural::power_of_2(pow)),
            a.to_mod_power_of_2_trace(pow)
        );
    });
}
