// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::natural::arithmetic::mul::schonhage_strassen::adjust::limbs_fft_adjust;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_34;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_fft_adjust() {
    let test = |i1: &[Limb], i: usize, limbs: usize, w: u64, out: &[Limb]| {
        let mut r = vec![0; limbs + 1];
        limbs_fft_adjust(&mut r, i1, i, limbs, w);
        assert_eq!(r, out);
    };
    // - x == 0
    test(&[5, 0], 0, 1, 1, &[5, 0]);
    test(&[5, 0], 3, 1, 1, &[40, 0]);
    test(
        &[5, 1],
        1,
        1,
        63,
        &[18446744073709551614, 18446744073709551615],
    );
    // - x != 0
    test(&[1, 2, 0], 1, 2, 64, &[18446744073709551614, 0, 0]);
    test(
        &[1, 2, 3],
        1,
        2,
        65,
        &[18446744073709551613, 18446744073709551611, 0],
    );
    test(
        &[18446744073709551615, 18446744073709551615, 18446744073709551615],
        3,
        2,
        40,
        &[1, 18374686479671623680, 0],
    );
}

#[test]
fn limbs_fft_adjust_properties() {
    large_type_gen_var_34().test_properties(|(i1, i, limbs, w)| {
        let mut r = vec![0; limbs + 1];
        limbs_fft_adjust(&mut r, &i1, i, limbs, w);
        assert_eq!(
            residue_mod(&r, limbs),
            fermat_mul_power_of_2(
                &residue_mod(&i1, limbs),
                u64::try_from(i).unwrap() * w,
                limbs
            )
        );
    });
}
