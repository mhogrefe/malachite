// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_bool_pair_gen, natural_gen};
use std::str::FromStr;

#[test]
fn test_from_sign_and_abs() {
    let test = |sign, abs, out| {
        let abs = Natural::from_str(abs).unwrap();
        let x = Integer::from_sign_and_abs(sign, abs.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let x = Integer::from_sign_and_abs_ref(sign, &abs);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(true, "0", "0");
    test(false, "0", "0");
    test(true, "123", "123");
    test(false, "123", "-123");
    test(true, "1000000000000", "1000000000000");
    test(false, "1000000000000", "-1000000000000");
}

#[test]
fn from_sign_and_abs_properties() {
    natural_bool_pair_gen().test_properties(|(abs, sign)| {
        let x = Integer::from_sign_and_abs(sign, abs.clone());
        assert!(x.is_valid());

        let x_alt = Integer::from_sign_and_abs_ref(sign, &abs);
        assert!(x_alt.is_valid());
        assert_eq!(x, x_alt);

        if abs != 0 {
            assert_eq!(x >= 0, sign);
        }
        assert_eq!(x.unsigned_abs(), abs);
    });

    natural_gen()
        .test_properties(|abs| assert_eq!(Integer::from_sign_and_abs_ref(true, &abs), abs));
}
