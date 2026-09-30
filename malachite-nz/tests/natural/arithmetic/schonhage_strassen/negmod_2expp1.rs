// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModNeg;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::negmod_2expp1::*;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_30;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_neg_mod_2expp1_to_out() {
    let test = |a: &[Limb], limbs: usize, out: &[Limb]| {
        let mut z = vec![0; limbs + 1];
        limbs_neg_mod_2expp1_to_out(&mut z, a, limbs);
        assert_eq!(z, out);
    };
    // - a[limbs] == 0
    // - z[limbs] != 0 && z[0] != 0
    test(&[0, 0], 1, &[0, 0]);
    // - !(z[limbs] != 0 && z[0] != 0)
    test(&[1, 0], 1, &[0, 1]);
    test(&[2, 0], 1, &[18446744073709551615, 0]);
    // - a[limbs] != 0
    test(&[0, 1], 1, &[1, 0]);
    test(&[18446744073709551615, 0], 1, &[2, 0]);
    test(
        &[3, 4, 0],
        2,
        &[18446744073709551614, 18446744073709551611, 0],
    );
}

#[test]
fn limbs_neg_mod_2expp1_to_out_properties() {
    large_type_gen_var_30().test_properties(|(a, limbs)| {
        let mut z = vec![0; limbs + 1];
        limbs_neg_mod_2expp1_to_out(&mut z, &a, limbs);
        assert!(residue_is_normalized(&z, limbs));
        assert_eq!(
            residue_mod(&z, limbs),
            residue_mod(&a, limbs).mod_neg(fermat_modulus(limbs))
        );
        let mut a_alt = vec![0; limbs + 1];
        limbs_neg_mod_2expp1_to_out(&mut a_alt, &z, limbs);
        assert_eq!(a_alt, a);
    });
}
