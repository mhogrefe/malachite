// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::IsReal;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, big_exponent_float_to_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1,
};
use malachite_nz::integer::Integer;

#[test]
fn test_is_real() {
    assert!(big_exponent_float_power_of_2(Integer::from(1u64 << 40)).is_real());
    assert_eq!(
        big_exponent_float_power_of_2(-Integer::from(1u64 << 40)).is_real(),
        true
    );
}

#[test]
fn is_real_properties() {
    big_exponent_float_gen().test_properties(|x| {
        assert_eq!(x.is_real(), x.is_finite());
    });

    big_exponent_float_gen_var_1().test_properties(|x| {
        assert_eq!(
            x.is_real(),
            big_exponent_float_to_float(&x).unwrap().is_real()
        );
    });
}
