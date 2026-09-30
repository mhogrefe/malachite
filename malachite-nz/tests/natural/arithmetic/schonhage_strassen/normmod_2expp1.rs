// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::natural::arithmetic::mul::schonhage_strassen::normmod_2expp1::*;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_29;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_norm_mod_2expp1() {
    let test = |t: &[Limb], limbs: usize, out: &[Limb]| {
        let mut t = t.to_vec();
        limbs_norm_mod_2expp1(&mut t, limbs);
        assert_eq!(t, out);
    };
    // - hi == 0
    test(&[5, 0], 1, &[5, 0]);
    // - hi != 0
    // - hi != 0
    // - t[limbs] != Limb::MAX
    test(&[0, 1], 1, &[0, 1]);
    // - hi == 0
    test(&[5, 1], 1, &[4, 0]);
    test(&[5, 2], 1, &[3, 0]);
    test(&[5, 18446744073709551615], 1, &[6, 0]);
    test(&[0, 18446744073709551615], 1, &[1, 0]);
    test(&[1, 2, 3], 2, &[18446744073709551614, 1, 0]);
    test(&[0, 0, 18446744073709551613], 2, &[3, 0, 0]);
    // - t[limbs] == Limb::MAX
    test(&[18446744073709551615, 18446744073709551615], 1, &[0, 1]);
    test(
        &[18446744073709551615, 18446744073709551615, 18446744073709551615],
        2,
        &[0, 0, 1],
    );
}

#[test]
fn limbs_norm_mod_2expp1_properties() {
    large_type_gen_var_29().test_properties(|(mut t, limbs)| {
        let old = residue_mod(&t, limbs);
        limbs_norm_mod_2expp1(&mut t, limbs);
        assert!(residue_is_normalized(&t, limbs));
        assert_eq!(residue_mod(&t, limbs), old);
        assert_eq!(t, normalized_residue(&old, limbs));
    });
}
