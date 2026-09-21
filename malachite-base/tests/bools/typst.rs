// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::assert_typst_compiles;
use malachite_base::strings::typst::ToTypst;

#[test]
fn test_to_typst() {
    assert_eq!(true.to_typst_string(), r#""T""#);
    assert_eq!(false.to_typst_string(), r#""F""#);
    assert_typst_compiles(&[true.to_typst_string(), false.to_typst_string()]);
}
