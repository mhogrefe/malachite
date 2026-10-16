// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_matrix_pair_gen;

#[test]
fn test_partial_eq_unsigned_matrix() {
    fn test<T: PrimitiveUnsigned>(s: &str, t: &str, out: bool)
    where
        Natural: PartialEq<T>,
    {
        let m = NaturalMatrix::from_str(s).unwrap();
        let n = UnsignedMatrix::<T>::from_str(t).unwrap();
        assert_eq!(m == n, out);
        assert_eq!(n == m, out);
    }
    test::<u8>("[]_{0,0}", "[]_{0,0}", true);
    test::<u8>("[]_{0,3}", "[]_{0,3}", true);
    // Matrices with no elements are equal only when their shapes match.
    test::<u8>("[]_{0,3}", "[]_{3,0}", false);
    test::<u64>("[]_{0,0}", "[0]", false);
    test::<u16>("[1]", "[1]", true);
    test::<u8>("[1, 2; 3, 4]", "[1, 2; 3, 4]", true);
    test::<u8>("[1, 2; 3, 4]", "[1, 3; 2, 4]", false);
    // The same elements in a different shape are not equal.
    test::<u32>("[1, 2, 3, 4]", "[1, 2; 3, 4]", false);
    test::<u32>("[1; 2; 3; 4]", "[1, 2; 3, 4]", false);
    test::<u128>("[1, 2, 3]", "[1, 2]", false);
    test::<u8>("[3, 255]", "[3, 255]", true);
    test::<u8>("[3; 254]", "[3; 255]", false);
    // An element too large for the element type matches nothing, including the value it would wrap
    // to.
    test::<u8>("[3, 256]", "[3, 0]", false);
    test::<u8>("[256]", "[0]", false);
    test::<u64>(
        "[18446744073709551615, 1; 0, 18446744073709551615]",
        "[18446744073709551615, 1; 0, 18446744073709551615]",
        true,
    );
    test::<u64>("[18446744073709551616]", "[0]", false);
    test::<usize>("[5, 6]", "[5, 6]", true);
}

// Comparing with a converted matrix is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
fn partial_eq_unsigned_matrix_properties_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T> + PartialEq<T>,
{
    natural_matrix_unsigned_matrix_pair_gen::<T>().test_properties(|(m, n)| {
        let eq = m == n;
        assert_eq!(n == m, eq);
        // Extra refs for type inference: with `NaturalMatrix: PartialEq<UnsignedMatrix<T>>` in
        // scope, `m == n` would look for that impl.
        assert_eq!(&m == &NaturalMatrix::from(n.clone()), eq);
        if eq {
            assert_eq!((m.height(), m.width()), (n.height(), n.width()));
        }

        let n_m = NaturalMatrix::from(n.clone());
        assert!(n_m == n);
        assert!(n == n_m);
        // Transposing is a bijection, so it preserves both equality and inequality.
        assert_eq!(m.transpose() == n.transpose(), eq);
    });
}

#[test]
fn partial_eq_unsigned_matrix_properties() {
    apply_fn_to_unsigneds!(partial_eq_unsigned_matrix_properties_helper);
}
