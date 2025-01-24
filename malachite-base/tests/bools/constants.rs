// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::comparison::traits::{Max, Min};
use malachite_base::named::Named;

#[test]
fn test_min() {
    assert_eq!(bool::MIN, false);
}

#[test]
fn test_max() {
    assert_eq!(bool::MAX, true);
}

#[test]
pub fn test_named() {
    assert_eq!(bool::NAME, "bool");
}
