// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::orderings::exhaustive::{exhaustive_orderings, orderings_increasing};
use std::cmp::Ordering::*;

#[test]
fn test_orderings_increasing() {
    assert_eq!(
        orderings_increasing().collect_vec(),
        &[Less, Equal, Greater]
    );
}

#[test]
fn test_exhaustive_orderings() {
    assert_eq!(
        exhaustive_orderings().collect_vec(),
        &[Equal, Less, Greater]
    );
}
