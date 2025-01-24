// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::hash::hash;
use malachite_float::test_util::generators::{float_gen, float_gen_var_12};
use malachite_float::{ComparableFloatRef, Float};

#[allow(clippy::needless_pass_by_value)]
fn hash_properties_helper(x: Float) {
    assert_eq!(
        hash(&ComparableFloatRef(&x)),
        hash(&ComparableFloatRef(&x.clone()))
    );
}

#[test]
fn hash_properties() {
    float_gen().test_properties(|x| {
        hash_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        hash_properties_helper(x);
    });
}
