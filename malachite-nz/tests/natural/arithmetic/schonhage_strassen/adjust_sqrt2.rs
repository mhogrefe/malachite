// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::natural::arithmetic::mul::schonhage_strassen::adjust_sqrt2::*;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_35;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_fft_adjust_sqrt2() {
    let test = |i1: &[Limb], i: usize, limbs: usize, w: u64, out: &[Limb]| {
        let mut r = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        limbs_fft_adjust_sqrt2(&mut r, i1, i, limbs, w, &mut temp);
        assert_eq!(r, out);
    };
    // - y == 0
    // - limbs.odd()
    // - !negate
    // - !negate
    // - y == 0
    test(&[5, 0], 1, 1, 1, &[1407374883225600, 0]);
    // - negate
    // - negate
    test(
        &[5, 0],
        127,
        1,
        1,
        &[18446040386267938816, 18446744073709551615],
    );
    test(&[5, 1], 3, 1, 1, &[2251799813160960, 0]);
    // - y != 0
    test(
        &[1, 2, 3, 0],
        5,
        3,
        3,
        &[18410715276673810432, 18374686479646457855, 18338657682661048319, 18446744073709551615],
    );
    test(
        &[18446744073709551615, 18446744073709551615, 18446744073709551615, 18446744073709551615],
        99,
        3,
        3,
        &[18446744073709551600, 68719476735, 0, 0],
    );
    // - y != 0
    test(
        &[1, 2, 3, 0],
        21,
        3,
        3,
        &[18446462598732939264, 18446321861244452863, 140737488289791, 0],
    );
    // - !limbs.odd()
    test(
        &[1, 2, 0],
        3,
        2,
        1,
        &[18446744047939747840, 18446744065119617023, 18446744073709551615],
    );
}

#[test]
fn limbs_fft_adjust_sqrt2_properties() {
    large_type_gen_var_35().test_properties(|(i1, i, limbs, w)| {
        let mut r = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        limbs_fft_adjust_sqrt2(&mut r, &i1, i, limbs, w, &mut temp);
        assert_eq!(
            residue_mod(&r, limbs),
            fermat_mul_sqrt_2_power(
                &residue_mod(&i1, limbs),
                u64::try_from(i).unwrap() * w,
                limbs
            )
        );
    });
}
