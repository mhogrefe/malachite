// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;

#[test]
fn test_default() {
    let default = GaussianInteger::default();
    assert!(default.real.is_valid());
    assert!(default.imaginary.is_valid());
    assert_eq!(default, GaussianInteger::ZERO);
    assert_eq!(default.to_string(), "0");
}
