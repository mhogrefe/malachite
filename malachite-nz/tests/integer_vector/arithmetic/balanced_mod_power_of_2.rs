// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    BalancedMod, BalancedModPowerOf2, BalancedModPowerOf2Assign, PowerOf2,
};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_unsigned_pair_gen_var_2;

#[test]
fn test_balanced_mod_power_of_2() {
    let test = |s, pow, out| {
        let x = IntegerVector::from_str(s).unwrap();
        let r = (&x).balanced_mod_power_of_2(pow);
        assert_eq!(r.to_string(), out);
        assert_eq!(x.clone().balanced_mod_power_of_2(pow), r);
        let mut y = x;
        y.balanced_mod_power_of_2_assign(pow);
        assert_eq!(y, r);
    };
    test("()", 3, "()");
    test("(-19, 23, -20)", 3, "(-3, -1, 4)");
    test("(-8, 7)", 3, "(0, -1)");
    test("(1, -3)", 0, "(0, 0)");
    test("(-9223372036854775808)", 64, "(9223372036854775808)");
    test("(-12345)", 100, "(-12345)");
}

#[test]
fn balanced_mod_power_of_2_properties() {
    integer_vector_unsigned_pair_gen_var_2().test_properties(|(v, pow)| {
        let r = (&v).balanced_mod_power_of_2(pow);
        assert_eq!(v.clone().balanced_mod_power_of_2(pow), r);
        let mut y = v.clone();
        y.balanced_mod_power_of_2_assign(pow);
        assert_eq!(y, r);
        // It is balanced_mod with modulus 2^k.
        assert_eq!((&v).balanced_mod(Integer::power_of_2(pow)), r);
        // The dimension is unchanged, and element by element this is the scalar operation.
        assert_eq!(r.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&r.elements) {
            assert_eq!(*y, x.balanced_mod_power_of_2(pow));
        }
        // Reducing again changes nothing.
        assert_eq!((&r).balanced_mod_power_of_2(pow), r);
    });
}
