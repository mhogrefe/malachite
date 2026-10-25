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
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModMul, ModShl, ModShlAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_5;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_natural_triple_gen_var_1;
use std::panic::catch_unwind;

fn test_mod_shl_helper<T: PrimitiveUnsigned>()
where
    NaturalMatrix: ModShl<T, Natural, Output = NaturalMatrix>
        + for<'a> ModShl<T, &'a Natural, Output = NaturalMatrix>
        + ModShlAssign<T, Natural>
        + for<'a> ModShlAssign<T, &'a Natural>,
    for<'a> &'a NaturalMatrix: ModShl<T, Natural, Output = NaturalMatrix>,
    for<'a, 'b> &'a NaturalMatrix: ModShl<T, &'b Natural, Output = NaturalMatrix>,
{
    let test = |s, bits: u8, m, out| {
        let bits = T::from(bits);
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let b = (&a).mod_shl(bits, &m);
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_shl(bits, m.clone()), b);
        assert_eq!(a.clone().mod_shl(bits, &m), b);
        assert_eq!(a.clone().mod_shl(bits, m.clone()), b);
        let mut x = a.clone();
        x.mod_shl_assign(bits, &m);
        assert_eq!(x, b);
        let mut x = a;
        x.mod_shl_assign(bits, m);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 1, "7", "[]_{0,0}");
    // Modulo 1 every element is 0.
    test("[0, 0]", 5, "1", "[0, 0]");
    test("[5, 1, 3]", 0, "7", "[5, 1, 3]");
    test("[5, 1, 3]", 1, "7", "[3, 2, 6]");
    test("[5, 1; 3, 7]", 1, "8", "[2, 2; 6, 6]");
    test("[]_{2,0}", 1, "7", "[]_{2,0}");
    // The modulus need not be odd, so elements can become zero, keeping the shape.
    test("[5, 1, 3]", 3, "8", "[0, 0, 0]");
    test("[1, 2]", 8, "255", "[1, 2]");
    test(
        "[1, 1]",
        100,
        "1000000000000000000000000000000",
        "[267650600228229401496703205376, 267650600228229401496703205376]",
    );
}

#[test]
fn test_mod_shl() {
    apply_fn_to_unsigneds!(test_mod_shl_helper);
}

#[test]
fn mod_shl_fail() {
    // The modulus is zero, even with no element to reduce.
    assert_panic!(
        NaturalMatrix::from_str("[]_{0,0}")
            .unwrap()
            .mod_shl(1u8, Natural::ZERO)
    );
    // An element is not reduced.
    let a = NaturalMatrix::from_str("[7]").unwrap();
    assert_panic!((&a).mod_shl(1u8, Natural::from(7u32)));
    assert_panic!({
        let mut b = a.clone();
        b.mod_shl_assign(1u8, &Natural::from(7u32));
    });
}

fn mod_shl_properties_helper<T: PrimitiveUnsigned>()
where
    NaturalMatrix: for<'a> ModShl<T, &'a Natural, Output = NaturalMatrix>
        + ModShl<T, Natural, Output = NaturalMatrix>
        + for<'a> ModShlAssign<T, &'a Natural>,
    for<'a, 'b> &'a NaturalMatrix: ModShl<T, &'b Natural, Output = NaturalMatrix>,
    for<'a> &'a NaturalMatrix: Shl<T, Output = NaturalMatrix>,
    for<'a> &'a Natural: ModShl<T, &'a Natural, Output = Natural>,
    for<'a> Natural: ModShl<T, &'a Natural, Output = Natural>,
    u64: ExactFrom<T>,
{
    natural_matrix_unsigned_natural_triple_gen_var_1::<T>().test_properties(|(a, bits, m)| {
        let b = (&a).mod_shl(bits, &m);
        assert_eq!(a.clone().mod_shl(bits, &m), b);
        assert_eq!(a.clone().mod_shl(bits, m.clone()), b);
        let mut x = a.clone();
        x.mod_shl_assign(bits, &m);
        assert_eq!(x, b);

        // The result is reduced, it is the ordinary shift reduced, and element by element it is the
        // scalar operation.
        assert!(b.mod_is_reduced(&m));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!((&a << bits) % &m, b);
        for (x, y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(*y, x.mod_shl(bits, &m));
        }
        // It is multiplication by 2^bits mod m, when 1 is reduced modulo m.
        if m != 1u32 {
            assert_eq!((&a).mod_mul(Natural::ONE.mod_shl(bits, &m), &m), b);
        }
        // Shifting by the same amount of another type gives the same result, and shifting by 0
        // changes nothing.
        assert_eq!(
            <&NaturalMatrix as ModShl<u64, &Natural>>::mod_shl(&a, u64::exact_from(bits), &m),
            b
        );
        assert_eq!((&a).mod_shl(T::ZERO, &m), a);
        // It commutes with transposition.
        assert_eq!(a.transpose().mod_shl(bits, &m), b.transpose());
    });
}

#[test]
fn mod_shl_properties() {
    apply_fn_to_unsigneds!(mod_shl_properties_helper);

    unsigned_matrix_unsigned_unsigned_triple_gen_var_5::<u64, u64>().test_properties(
        |(a, bits, m)| {
            // The `u64` elements shift as their `Natural` counterparts do.
            assert_eq!(
                NaturalMatrix::from(a.clone()).mod_shl(bits, Natural::from(m)),
                NaturalMatrix::from(a.mod_shl(bits, m))
            );
        },
    );
}
