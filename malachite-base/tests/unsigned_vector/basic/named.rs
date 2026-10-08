// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_named() {
    assert_eq!(UnsignedVector::<u8>::NAME, "UnsignedVector<u8>");
    assert_eq!(UnsignedVector::<u16>::NAME, "UnsignedVector<u16>");
    assert_eq!(UnsignedVector::<u32>::NAME, "UnsignedVector<u32>");
    assert_eq!(UnsignedVector::<u64>::NAME, "UnsignedVector<u64>");
    assert_eq!(UnsignedVector::<u128>::NAME, "UnsignedVector<u128>");
    assert_eq!(UnsignedVector::<usize>::NAME, "UnsignedVector<usize>");
}
