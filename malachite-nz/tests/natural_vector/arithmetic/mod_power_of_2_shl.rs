// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shl;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2ShlAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_3;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_unsigned_unsigned_triple_gen_var_1;
use std::panic::catch_unwind;

fn test_mod_power_of_2_shl_helper<T: PrimitiveUnsigned>()
where
    NaturalVector: ModPowerOf2Shl<T, Output = NaturalVector> + ModPowerOf2ShlAssign<T>,
    for<'a> &'a NaturalVector: ModPowerOf2Shl<T, Output = NaturalVector>,
{
    let test = |s, bits: u8, pow, out| {
        let bits = T::from(bits);
        let v = NaturalVector::from_str(s).unwrap();
        let w = (&v).mod_power_of_2_shl(bits, pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_power_of_2_shl(bits, pow), w);
        let mut x = v;
        x.mod_power_of_2_shl_assign(bits, pow);
        assert_eq!(x, w);
    };
    test("()", 1, 3, "()");
    test("(0, 0)", 5, 0, "(0, 0)");
    test("(5, 1, 3)", 0, 3, "(5, 1, 3)");
    test("(5, 1, 3)", 1, 3, "(2, 2, 6)");
    // Shifting by at least the power zeroes every element, keeping the dimension.
    test("(5, 1, 3)", 3, 3, "(0, 0, 0)");
    test("(5, 1, 3)", 200, 3, "(0, 0, 0)");
    test("(1, 128, 1)", 1, 8, "(2, 0, 2)");
    test(
        "(1, 1)",
        100,
        101,
        "(1267650600228229401496703205376, 1267650600228229401496703205376)",
    );
}

#[test]
fn test_mod_power_of_2_shl() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_shl_helper);
    // A shift too large for a `u64` zeroes every element too.
    let v = NaturalVector::from_str("(1, 2)").unwrap();
    assert_eq!(
        (&v).mod_power_of_2_shl(u128::MAX, 100).to_string(),
        "(0, 0)"
    );
}

#[test]
fn mod_power_of_2_shl_fail() {
    // An element is not reduced.
    let v = NaturalVector::from_str("(8)").unwrap();
    assert_panic!((&v).mod_power_of_2_shl(1u8, 3));
    assert_panic!(v.clone().mod_power_of_2_shl(1u8, 3));
    assert_panic!({
        let mut w = v.clone();
        w.mod_power_of_2_shl_assign(1u8, 3);
    });
}

fn mod_power_of_2_shl_properties_helper<T: PrimitiveUnsigned>()
where
    NaturalVector: ModPowerOf2Shl<T, Output = NaturalVector> + ModPowerOf2ShlAssign<T>,
    for<'a> &'a NaturalVector:
        ModPowerOf2Shl<T, Output = NaturalVector> + Shl<T, Output = NaturalVector>,
    for<'a> &'a Natural: ModPowerOf2Shl<T, Output = Natural>,
    u64: ExactFrom<T>,
{
    natural_vector_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(v, bits, pow)| {
        let w = (&v).mod_power_of_2_shl(bits, pow);
        assert_eq!(v.clone().mod_power_of_2_shl(bits, pow), w);
        let mut x = v.clone();
        x.mod_power_of_2_shl_assign(bits, pow);
        assert_eq!(x, w);

        // The result is reduced, it is the ordinary shift reduced, and element by element it is the
        // scalar operation.
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!(w.dimension(), v.dimension());
        assert_eq!((&v << bits).mod_power_of_2(pow), w);
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.mod_power_of_2_shl(bits, pow));
        }
        // Shifting by the same amount of another type gives the same result, and shifting by 0
        // changes nothing.
        assert_eq!(
            <&NaturalVector as ModPowerOf2Shl<u64>>::mod_power_of_2_shl(
                &v,
                u64::exact_from(bits),
                pow
            ),
            w
        );
        assert_eq!((&v).mod_power_of_2_shl(T::ZERO, pow), v);
    });
}

#[test]
fn mod_power_of_2_shl_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_shl_properties_helper);

    unsigned_vector_unsigned_unsigned_triple_gen_var_3::<u64, u64>().test_properties(
        |(v, bits, pow)| {
            // The `u64` elements shift as their `Natural` counterparts do.
            assert_eq!(
                NaturalVector::from(v.clone()).mod_power_of_2_shl(bits, pow),
                NaturalVector::from(v.mod_power_of_2_shl(bits, pow))
            );
        },
    );
}
