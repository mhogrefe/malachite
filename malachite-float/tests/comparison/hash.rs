// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::hash::hash;
use malachite_float::test_util::generators::float_gen;
use malachite_float::ComparableFloatRef;

#[test]
fn hash_properties() {
    float_gen().test_properties(|x| {
        assert_eq!(
            hash(&ComparableFloatRef(&x)),
            hash(&ComparableFloatRef(&x.clone()))
        );
    });
}
