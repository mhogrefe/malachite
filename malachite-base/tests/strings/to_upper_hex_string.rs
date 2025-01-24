// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToUpperHexString;

#[test]
pub fn test_to_upper_hex_string() {
    assert_eq!(50u32.to_upper_hex_string(), "32");
    assert_eq!((-100i32).to_upper_hex_string(), "FFFFFF9C");
}
