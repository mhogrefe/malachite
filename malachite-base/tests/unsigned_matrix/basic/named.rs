// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_named() {
    assert_eq!(UnsignedMatrix::<u8>::NAME, "UnsignedMatrix<u8>");
    assert_eq!(UnsignedMatrix::<u16>::NAME, "UnsignedMatrix<u16>");
    assert_eq!(UnsignedMatrix::<u32>::NAME, "UnsignedMatrix<u32>");
    assert_eq!(UnsignedMatrix::<u64>::NAME, "UnsignedMatrix<u64>");
    assert_eq!(UnsignedMatrix::<u128>::NAME, "UnsignedMatrix<u128>");
    assert_eq!(UnsignedMatrix::<usize>::NAME, "UnsignedMatrix<usize>");
}
