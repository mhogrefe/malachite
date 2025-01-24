// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::hash::hash;

fn hash_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let x = NiceFloat(x);
        assert_eq!(hash(&x), hash(&x.clone()));
    });
}

#[test]
fn hash_properties() {
    apply_fn_to_primitive_floats!(hash_properties_helper);
}
