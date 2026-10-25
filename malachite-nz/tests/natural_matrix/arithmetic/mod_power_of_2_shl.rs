// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shl;
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2ShlAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_4;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_unsigned_triple_gen_var_2;
use std::panic::catch_unwind;

fn test_mod_power_of_2_shl_helper<T: PrimitiveUnsigned>()
where
    NaturalMatrix: ModPowerOf2Shl<T, Output = NaturalMatrix> + ModPowerOf2ShlAssign<T>,
    for<'a> &'a NaturalMatrix: ModPowerOf2Shl<T, Output = NaturalMatrix>,
{
    let test = |s, bits: u8, pow, out| {
        let bits = T::from(bits);
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = (&a).mod_power_of_2_shl(bits, pow);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_power_of_2_shl(bits, pow), b);
        let mut x = a;
        x.mod_power_of_2_shl_assign(bits, pow);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 1, 3, "[]_{0,0}");
    test("[0, 0]", 5, 0, "[0, 0]");
    test("[5, 1, 3]", 0, 3, "[5, 1, 3]");
    test("[5, 1, 3]", 1, 3, "[2, 2, 6]");
    test("[5, 1; 3, 7]", 1, 3, "[2, 2; 6, 6]");
    test("[]_{2,0}", 1, 3, "[]_{2,0}");
    // Shifting by at least the power zeroes every element, keeping the shape.
    test("[5, 1, 3]", 3, 3, "[0, 0, 0]");
    test("[5, 1, 3]", 200, 3, "[0, 0, 0]");
    test("[1, 128, 1]", 1, 8, "[2, 0, 2]");
    test(
        "[1, 1]",
        100,
        101,
        "[1267650600228229401496703205376, 1267650600228229401496703205376]",
    );
}

#[test]
fn test_mod_power_of_2_shl() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_shl_helper);
    // A shift too large for a `u64` zeroes every element too.
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_eq!(
        (&a).mod_power_of_2_shl(u128::MAX, 100).to_string(),
        "[0, 0]"
    );
}

#[test]
fn mod_power_of_2_shl_fail() {
    // An element is not reduced.
    let a = NaturalMatrix::from_str("[8]").unwrap();
    assert_panic!((&a).mod_power_of_2_shl(1u8, 3));
    assert_panic!(a.clone().mod_power_of_2_shl(1u8, 3));
    assert_panic!({
        let mut b = a.clone();
        b.mod_power_of_2_shl_assign(1u8, 3);
    });
}

fn mod_power_of_2_shl_properties_helper<T: PrimitiveUnsigned>()
where
    NaturalMatrix: ModPowerOf2Shl<T, Output = NaturalMatrix> + ModPowerOf2ShlAssign<T>,
    for<'a> &'a NaturalMatrix:
        ModPowerOf2Shl<T, Output = NaturalMatrix> + Shl<T, Output = NaturalMatrix>,
    for<'a> &'a Natural: ModPowerOf2Shl<T, Output = Natural>,
    u64: ExactFrom<T>,
{
    natural_matrix_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(a, bits, pow)| {
        let b = (&a).mod_power_of_2_shl(bits, pow);
        assert_eq!(a.clone().mod_power_of_2_shl(bits, pow), b);
        let mut x = a.clone();
        x.mod_power_of_2_shl_assign(bits, pow);
        assert_eq!(x, b);

        // The result is reduced, it is the ordinary shift reduced, and element by element it is the
        // scalar operation.
        assert!(b.mod_power_of_2_is_reduced(pow));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!((&a << bits).mod_power_of_2(pow), b);
        for (x, y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(*y, x.mod_power_of_2_shl(bits, pow));
        }
        // Shifting by the same amount of another type gives the same result, and shifting by 0
        // changes nothing.
        assert_eq!(
            <&NaturalMatrix as ModPowerOf2Shl<u64>>::mod_power_of_2_shl(
                &a,
                u64::exact_from(bits),
                pow
            ),
            b
        );
        assert_eq!((&a).mod_power_of_2_shl(T::ZERO, pow), a);
        // It commutes with transposition.
        assert_eq!(a.transpose().mod_power_of_2_shl(bits, pow), b.transpose());
    });
}

#[test]
fn mod_power_of_2_shl_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_shl_properties_helper);

    unsigned_matrix_unsigned_unsigned_triple_gen_var_4::<u64, u64>().test_properties(
        |(a, bits, pow)| {
            // The `u64` elements shift as their `Natural` counterparts do.
            assert_eq!(
                NaturalMatrix::from(a.clone()).mod_power_of_2_shl(bits, pow),
                NaturalMatrix::from(a.mod_power_of_2_shl(bits, pow))
            );
        },
    );
}
