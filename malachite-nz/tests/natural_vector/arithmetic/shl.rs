// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, ShlAssign};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Height, L1Norm, PowerOf2};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_4;

fn test_shl_helper<T: PrimitiveUnsigned>()
where
    NaturalVector: Shl<T, Output = NaturalVector> + ShlAssign<T>,
    for<'a> &'a NaturalVector: Shl<T, Output = NaturalVector>,
{
    let test = |s, bits: u8, out| {
        let bits = T::from(bits);
        let v = NaturalVector::from_str(s).unwrap();
        let w = &v << bits;
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone() << bits, w);
        let mut x = v;
        x <<= bits;
        assert_eq!(x, w);
    };
    test("()", 0, "()");
    test("()", 10, "()");
    test("(1, 2, 3)", 0, "(1, 2, 3)");
    test("(1, 2, 3)", 2, "(4, 8, 12)");
    test("(0, 1)", 1, "(0, 2)");
    test(
        "(7, 1)",
        100,
        "(8873554201597605810476922437632, 1267650600228229401496703205376)",
    );
}

#[test]
fn test_shl() {
    apply_fn_to_unsigneds!(test_shl_helper);
}

fn shl_properties_helper<T: PrimitiveUnsigned>()
where
    NaturalVector: Shl<T, Output = NaturalVector> + ShlAssign<T>,
    for<'a> &'a NaturalVector: Shl<T, Output = NaturalVector>,
    Natural: Shl<T, Output = Natural>,
    for<'a> &'a Natural: Shl<T, Output = Natural>,
    u64: ExactFrom<T>,
    IntegerVector: Shl<T, Output = IntegerVector>,
{
    natural_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, bits)| {
        let w = &v << bits;
        // The forms agree.
        assert_eq!(v.clone() << bits, w);
        let mut x = v.clone();
        x <<= bits;
        assert_eq!(x, w);

        // Element by element, this is the scalar shift, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x << bits);
        }
        // It is multiplication by a power of 2, and shifting by the same amount of another type
        // gives the same result.
        assert_eq!(&v * Natural::power_of_2(u64::exact_from(bits)), w);
        assert_eq!(
            <&NaturalVector as Shl<u64>>::shl(&v, u64::exact_from(bits)),
            w
        );
        // Shifting by 0 changes nothing, and shifts compose.
        assert_eq!(&v << T::ZERO, v);
        if let Some(double) = bits.checked_add(bits) {
            assert_eq!(&w << bits, &v << double);
        }
        // The height and the l^1 norm are shifted too.
        assert_eq!(w.to_height(), v.to_height() << bits);
        assert_eq!(w.to_l1_norm(), v.to_l1_norm() << bits);
    });

    natural_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, bits)| {
        // As an IntegerVector, the shift is the same.
        assert_eq!(
            IntegerVector::from(&v << bits),
            IntegerVector::from(v) << bits
        );
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigneds!(shl_properties_helper);
}
