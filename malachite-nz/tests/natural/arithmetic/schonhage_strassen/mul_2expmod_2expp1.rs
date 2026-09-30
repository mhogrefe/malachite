// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::natural::arithmetic::mul::schonhage_strassen::div_2expmod_2expp1::*;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::mul_2expmod_2expp1::*;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_31;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_2exp_mod_2expp1() {
    let test = |t: &[Limb], limbs: usize, d: u64, out: &[Limb]| {
        let mut t_alt = vec![0; limbs + 1];
        limbs_mul_2exp_mod_2expp1_to_out(&mut t_alt, t, limbs, d);
        assert_eq!(t_alt, out);
        let mut t = t.to_vec();
        limbs_mul_2exp_mod_2expp1_in_place(&mut t, limbs, d);
        assert_eq!(t, out);
    };
    // - d == 0
    // - d == 0
    test(&[5, 0], 1, 0, &[5, 0]);
    // - d != 0
    // - d != 0
    test(&[5, 0], 1, 1, &[10, 0]);
    test(&[18446744073709551615, 0], 1, 63, &[1, 0]);
    test(&[1, 2], 1, 3, &[18446744073709551608, 18446744073709551615]);
    test(&[1, 18446744073709551615], 1, 4, &[32, 0]);
    test(&[3, 5, 18446744073709551614], 2, 17, &[655360, 655360, 0]);
}

#[test]
fn limbs_mul_2exp_mod_2expp1_properties() {
    large_type_gen_var_31().test_properties(|(t, limbs, d)| {
        let mut t_out = vec![0; limbs + 1];
        limbs_mul_2exp_mod_2expp1_to_out(&mut t_out, &t, limbs, d);
        let mut t_alt = t.clone();
        limbs_mul_2exp_mod_2expp1_in_place(&mut t_alt, limbs, d);
        assert_eq!(t_alt, t_out);
        let x = residue_mod(&t, limbs);
        assert_eq!(
            residue_mod(&t_out, limbs),
            fermat_mul_power_of_2(&x, d, limbs)
        );
        limbs_div_2exp_mod_2expp1_in_place(&mut t_alt, limbs, d);
        assert_eq!(residue_mod(&t_alt, limbs), x);
    });
}
