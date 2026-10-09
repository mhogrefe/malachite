// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::{unsigned_pair_gen_var_51, unsigned_vector_gen};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_is_zero() {
    let test = |s, out| {
        assert_eq!(UnsignedVector::<u8>::from_str(s).unwrap().is_zero(), out);
    };
    test("()", true);
    test("(0)", true);
    test("(0, 0, 0)", true);
    test("(0, 1, 0)", false);
    test("(0, 2, 0)", false);
}

#[test]
fn is_zero_properties() {
    unsigned_vector_gen().test_properties(|v| {
        let zero = v.is_zero();
        // It is equality with the zero vector of the same dimension, and means there is no pivot.
        assert_eq!(zero, v == UnsignedVector::<u64>::zero(v.dimension()));
        assert_eq!(zero, v.pivot_index().is_none());
        let mut w = v.clone();
        w.set_zero();
        assert!(w.is_zero());
    });

    unsigned_pair_gen_var_51().test_properties(|(index, dimension)| {
        // A standard basis vector is never zero.
        assert!(!UnsignedVector::<u64>::standard_basis_vector(dimension, index).is_zero());
    });
}
