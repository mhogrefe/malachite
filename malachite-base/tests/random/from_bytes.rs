// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::random::Seed;
use malachite_base::random::EXAMPLE_SEED;

#[test]
fn test_from_bytes() {
    assert_eq!(Seed::from_bytes(EXAMPLE_SEED.bytes), EXAMPLE_SEED);
}
