// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;

#[test]
fn test_from_unsigned_matrix() {
    fn test<T: PrimitiveUnsigned>(s: &str)
    where
        Integer: From<T>,
    {
        let m = IntegerMatrix::from(UnsignedMatrix::<T>::from_str(s).unwrap());
        assert!(m.is_valid());
        assert_eq!(m.to_string(), s);
    }
    test::<u64>("[]_{0,0}");
    test::<u64>("[]_{0,4}");
    test::<u64>("[]_{4,0}");
    test::<u64>("[0]");
    test::<u64>("[1, 2, 3; 4, 5, 6]");
    test::<u64>("[1; 2; 3]");
    // The largest element an `UnsignedMatrix` can hold survives.
    test::<u64>("[18446744073709551615, 0]");
    test::<u8>("[255, 1; 0, 7]");
    test::<u128>("[340282366920938463463374607431768211455]");
    test::<usize>("[5, 6]");
}

#[test]
fn from_unsigned_matrix_properties() {
    unsigned_matrix_gen().test_properties(|m| {
        let n = IntegerMatrix::from(m.clone());
        assert!(n.is_valid());
        // Nothing is lost: the shape, every element, and the written form all survive.
        assert_eq!((n.height(), n.width()), (m.height(), m.width()));
        assert_eq!(n.to_string(), m.to_string());
        assert_eq!(
            n.elements,
            m.elements
                .iter()
                .map(|&x| Integer::from(x))
                .collect::<Vec<_>>()
        );
        // Reading the string back as an `IntegerMatrix` gives the same thing.
        assert_eq!(IntegerMatrix::from_str(&m.to_string()).unwrap(), n);
        // Converting commutes with transposing.
        assert_eq!(IntegerMatrix::from(m.transpose()), n.transpose());
    });
}
