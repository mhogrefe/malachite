// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, ShlAssign};
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_5;

fn test_shl_helper<T: PrimitiveUnsigned>()
where
    NaturalMatrix: Shl<T, Output = NaturalMatrix> + ShlAssign<T>,
    for<'a> &'a NaturalMatrix: Shl<T, Output = NaturalMatrix>,
{
    let test = |s, bits: u8, out| {
        let bits = T::from(bits);
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = &a << bits;
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone() << bits, b);
        let mut x = a;
        x <<= bits;
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 0, "[]_{0,0}");
    test("[]_{0,0}", 10, "[]_{0,0}");
    test("[]_{2,0}", 3, "[]_{2,0}");
    test("[1, 2; 3, 4]", 0, "[1, 2; 3, 4]");
    test("[1, 2; 3, 4]", 2, "[4, 8; 12, 16]");
    test("[0, 1]", 1, "[0, 2]");
    test(
        "[7; 1]",
        100,
        "[8873554201597605810476922437632; 1267650600228229401496703205376]",
    );
}

#[test]
fn test_shl() {
    apply_fn_to_unsigneds!(test_shl_helper);
}

fn shl_properties_helper<T: PrimitiveUnsigned>()
where
    NaturalMatrix: Shl<T, Output = NaturalMatrix> + ShlAssign<T>,
    for<'a> &'a NaturalMatrix: Shl<T, Output = NaturalMatrix>,
    for<'a> &'a Natural: Shl<T, Output = Natural>,
    u64: ExactFrom<T>,
    IntegerMatrix: Shl<T, Output = IntegerMatrix>,
{
    natural_matrix_unsigned_pair_gen_var_5::<T>().test_properties(|(a, bits)| {
        let w = &a << bits;
        // The forms agree.
        assert_eq!(a.clone() << bits, w);
        let mut x = a.clone();
        x <<= bits;
        assert_eq!(x, w);

        // Element by element, this is the scalar shift, and the shape is unchanged.
        assert_eq!((w.height(), w.width()), (a.height(), a.width()));
        for (x, y) in a.elements_ref().iter().zip(w.elements_ref()) {
            assert_eq!(*y, x << bits);
        }
        // It is multiplication by a power of 2, and shifting by the same amount of another type
        // gives the same result.
        assert_eq!(&a * Natural::power_of_2(u64::exact_from(bits)), w);
        assert_eq!(
            <&NaturalMatrix as Shl<u64>>::shl(&a, u64::exact_from(bits)),
            w
        );
        // Shifting by 0 changes nothing, and shifts compose.
        assert_eq!(&a << T::ZERO, a);
        if let Some(double) = bits.checked_add(bits) {
            assert_eq!(&w << bits, &a << double);
        }
        // It commutes with transposition.
        assert_eq!(a.transpose() << bits, w.transpose());
    });

    natural_matrix_unsigned_pair_gen_var_5::<T>().test_properties(|(a, bits)| {
        // As an IntegerMatrix, the shift is the same.
        assert_eq!(
            IntegerMatrix::from(&a << bits),
            IntegerMatrix::from(a) << bits
        );
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigneds!(shl_properties_helper);
}
