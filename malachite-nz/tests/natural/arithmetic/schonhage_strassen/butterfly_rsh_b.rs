// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAdd, ModSub};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::butterfly_rsh_b::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_36;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_butterfly_rsh_b() {
    let test = |i1: &[Limb],
                i2: &[Limb],
                limbs: usize,
                x: usize,
                y: usize,
                t_out: &[Limb],
                u_out: &[Limb]| {
        let mut t = vec![0; limbs + 1];
        let mut u = vec![0; limbs + 1];
        let mut i1 = i1.to_vec();
        let mut i2 = i2.to_vec();
        limbs_butterfly_rsh_b(&mut t, &mut u, &mut i1, &mut i2, limbs, x, y);
        assert_eq!(t, t_out);
        assert_eq!(u, u_out);
    };
    // - x == 0
    // - y == 0
    test(
        &[1, 0],
        &[2, 0],
        1,
        0,
        0,
        &[3, 0],
        &[18446744073709551615, 18446744073709551615],
    );
    // - y != 0
    test(
        &[1, 2, 0],
        &[3, 4, 0],
        2,
        0,
        1,
        &[5, 18446744073709551615, 18446744073709551615],
        &[18446744073709551613, 4, 0],
    );
    // - x != 0
    // - y == 0
    test(
        &[1, 2, 0],
        &[3, 4, 0],
        2,
        1,
        0,
        &[5, 3, 0],
        &[18446744073709551615, 18446744073709551610, 18446744073709551615],
    );
    // - y != 0
    // - x != y
    // - x > y
    test(
        &[1, 2, 3, 0],
        &[4, 5, 6, 1],
        3,
        2,
        1,
        &[8, 5, 18446744073709551611, 18446744073709551615],
        &[18446744073709551614, 18446744073709551608, 0, 0],
    );
    // - x <= y
    test(
        &[1, 2, 3, 0],
        &[4, 5, 6, 1],
        3,
        1,
        2,
        &[8, 0, 18446744073709551610, 18446744073709551615],
        &[18446744073709551612, 5, 4, 0],
    );
    // - x == y
    test(
        &[18446744073709551615, 18446744073709551615, 2],
        &[1, 0, 18446744073709551615],
        2,
        1,
        1,
        &[18446744073709551615, 1, 18446744073709551615],
        &[18446744073709551615, 5, 18446744073709551615],
    );
}

#[test]
fn limbs_butterfly_rsh_b_properties() {
    large_type_gen_var_36().test_properties(|(mut i1, mut i2, limbs, x, y)| {
        let p = fermat_modulus(limbs);
        let a = residue_mod(&i1, limbs);
        let b = residue_mod(&i2, limbs);
        let mut t = vec![0; limbs + 1];
        let mut u = vec![0; limbs + 1];
        limbs_butterfly_rsh_b(&mut t, &mut u, &mut i1, &mut i2, limbs, x, y);
        let x = u64::try_from(x).unwrap() << Limb::LOG_WIDTH;
        let y = u64::try_from(y).unwrap() << Limb::LOG_WIDTH;
        let a = fermat_div_power_of_2(&a, x, limbs);
        let b = fermat_div_power_of_2(&b, y, limbs);
        assert_eq!(residue_mod(&t, limbs), (&a).mod_add(&b, &p));
        assert_eq!(residue_mod(&u, limbs), a.mod_sub(b, p));
    });
}
