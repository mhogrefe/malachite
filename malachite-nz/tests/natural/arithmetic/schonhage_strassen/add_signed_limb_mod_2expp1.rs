// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Mod;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::limbs_add_signed_limb_mod_2expp1;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::large_type_gen_var_32;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_add_signed_limb_mod_2expp1() {
    let test = |r: &[Limb], limbs: usize, c: Limb, out: &[Limb]| {
        let mut r = r.to_vec();
        limbs_add_signed_limb_mod_2expp1(&mut r, limbs, c);
        assert_eq!(r, out);
    };
    // - SignedLimb::wrapping_from(sum ^ r[0]) >= 0
    test(&[1, 0], 1, 1, &[2, 0]);
    // - SignedLimb::wrapping_from(sum ^ r[0]) < 0
    // - SignedLimb::wrapping_from(c) >= 0
    test(&[18446744073709551615, 0], 1, 1, &[0, 1]);
    // - SignedLimb::wrapping_from(c) < 0
    test(
        &[0, 0],
        1,
        18446744073709551615,
        &[18446744073709551615, 18446744073709551615],
    );
    test(&[5, 7, 0], 2, 3, &[8, 7, 0]);
    test(
        &[0, 0, 1],
        2,
        18446744073709551615,
        &[18446744073709551615, 18446744073709551615, 0],
    );
    test(
        &[18446744073709551615, 18446744073709551615, 2],
        2,
        1,
        &[0, 0, 3],
    );
}

#[test]
fn limbs_add_signed_limb_mod_2expp1_properties() {
    large_type_gen_var_32().test_properties(|(mut r, limbs, c)| {
        let old = residue_mod(&r, limbs);
        limbs_add_signed_limb_mod_2expp1(&mut r, limbs, c);
        let p = Integer::from(fermat_modulus(limbs));
        assert_eq!(
            Integer::from(residue_mod(&r, limbs)),
            (Integer::from(old) + Integer::from(SignedLimb::wrapping_from(c))).mod_op(p)
        );
    });
}
