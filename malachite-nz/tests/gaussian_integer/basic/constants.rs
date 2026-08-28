// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{I, NegativeI, NegativeOne, One, Two, Zero};
use malachite_nz::gaussian_integer::GaussianInteger;

#[test]
fn test_constants() {
    let test = |c: GaussianInteger, real, imaginary, s| {
        assert_eq!(c.real, real);
        assert_eq!(c.imaginary, imaginary);
        assert_eq!(c.to_string(), s);
    };
    test(GaussianInteger::ZERO, 0, 0, "0");
    test(GaussianInteger::ONE, 1, 0, "1");
    test(GaussianInteger::TWO, 2, 0, "2");
    test(GaussianInteger::NEGATIVE_ONE, -1, 0, "-1");
    test(GaussianInteger::I, 0, 1, "i");
    test(GaussianInteger::NEGATIVE_I, 0, -1, "-i");
    assert_eq!(GaussianInteger::default(), GaussianInteger::ZERO);
}
