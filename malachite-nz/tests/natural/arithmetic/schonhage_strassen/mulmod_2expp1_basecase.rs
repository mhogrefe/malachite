// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Mod, PowerOf2};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1_basecase::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_48;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_mod_2expp1_basecase() {
    let test = |xs: &[Limb], ys: Option<&[Limb]>, c: Limb, b: u64, carry: Limb, out: &[Limb]| {
        let mut xs = xs.to_vec();
        let mut tp = vec![0; limbs_mul_mod_2expp1_basecase_scratch_len(xs.len())];
        assert_eq!(
            limbs_mul_mod_2expp1_basecase(&mut xs, ys, c, b, &mut tp),
            carry
        );
        assert_eq!(xs, out);
    };
    // - if let Some(ys) = ys
    // - k == 0
    // - cy == 0
    // - cz == 0
    test(&[3], Some(&[5]), 0, 64, 0, &[15]);
    // - k != 0
    test(&[3], Some(&[5]), 0, 10, 0, &[15]);
    test(&[1000], Some(&[1000]), 0, 10, 0, &[625]);
    // - cy != 0
    // - cz == 0
    // - let c = if let Some(ys) = ys
    test(&[0], Some(&[5]), 2, 64, 0, &[18446744073709551612]);
    // - cz != 0
    test(&[7], Some(&[0]), 1, 64, 0, &[18446744073709551610]);
    // - cz != 0
    test(&[0], Some(&[0]), 3, 64, 0, &[1]);
    test(
        &[18446744073709551615, 1],
        Some(&[18446744073709551615, 1]),
        0,
        65,
        0,
        &[4, 0],
    );
    test(
        &[18446744073709551615, 18446744073709551615],
        None,
        0,
        128,
        0,
        &[4, 0],
    );
    test(&[0, 0], None, 3, 100, 0, &[1, 0]);
}

#[test]
fn limbs_mul_mod_2expp1_basecase_properties() {
    large_type_gen_var_48().test_properties(|(mut xs, ys, c, b)| {
        let p = Natural::power_of_2(b) + Natural::ONE;
        let value = |limbs: &[Limb], top: bool| {
            Natural::from_limbs_asc(limbs)
                + if top {
                    Natural::power_of_2(b)
                } else {
                    Natural::ZERO
                }
        };
        let x = value(&xs, c & 2 != 0);
        let y = ys
            .as_ref()
            .map_or_else(|| x.clone(), |ys| value(ys, c & 1 != 0));
        let mut tp = vec![0; limbs_mul_mod_2expp1_basecase_scratch_len(xs.len())];
        let carry = limbs_mul_mod_2expp1_basecase(&mut xs, ys.as_deref(), c, b, &mut tp);
        assert!(carry < 2);
        let result = value(&xs, carry == 1);
        assert!(result < p);
        assert_eq!(result, (x * y).mod_op(p));
    });
}
