// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::fft_negacyclic::fft_negacyclic;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::ifft_negacyclic::ifft_negacyclic;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::large_type_gen_var_43;
use malachite_nz::test_util::natural::arithmetic::schonhage_strassen::*;

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_fft_negacyclic() {
    let test = |ii: &[&[Limb]], n: usize, w: u64, out: &[Limb]| {
        let mut ii: Vec<Vec<Limb>> = ii.iter().map(|x| x.to_vec()).collect();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_negacyclic(&mut ii, n, w, &mut t1, &mut t2, &mut temp);
        assert_eq!(ii.concat(), out);
    };
    // - !w.odd()
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 0]],
        2,
        32,
        &[
            1125912791875585,
            0,
            18445618186687479809,
            18446744073709551615,
            562937068781569,
            0,
            18446181110870966273,
            18446744073709551615,
        ],
    );
    test(
        &[&[1, 0], &[2, 0], &[3, 0], &[4, 1]],
        2,
        32,
        &[
            844437815164929,
            0,
            18445899661664190465,
            18446744073709551615,
            562937068716033,
            0,
            18446181110871031809,
            18446744073709551615,
        ],
    );
    test(
        &[&[1, 2, 0], &[3, 4, 1], &[5, 6, 0], &[7, 8, 18446744073709551615]],
        2,
        64,
        &[
            18446744047939747835,
            51539607558,
            0,
            25769803771,
            18446744022169944071,
            18446744073709551615,
            17179869190,
            42949672957,
            18446744073709551615,
            18446744056529682440,
            18446744030759878652,
            0,
        ],
    );
}

#[test]
fn fft_negacyclic_properties() {
    large_type_gen_var_43().test_properties_with_limit(2000, |(mut ii, n, w)| {
        let limbs = ii[0].len() - 1;
        let twisted: Vec<Natural> = residues_mod(&ii, limbs)
            .iter()
            .enumerate()
            .map(|(i, x)| fermat_mul_sqrt_2_power(x, u64::try_from(i).unwrap() * w, limbs))
            .collect();
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_negacyclic(&mut ii, n, w, &mut t1, &mut t2, &mut temp);
        let ys = fermat_dft(&twisted, w, false, limbs);
        let bits = u64::from((n << 1).trailing_zeros());
        for (k, y) in ys.iter().enumerate() {
            assert_eq!(&residue_mod(&ii[revbin(k, bits)], limbs), y);
        }
    });
}

// With `n >= Limb::WIDTH`, `w` may be odd, which takes the square-root-of-2 paths. The inputs are
// too large to write out, so the transform is checked against its inverse.
#[test]
fn test_fft_negacyclic_odd_w() {
    let test = |n: usize, w: u64| {
        let limbs = usize::try_from((u64::try_from(n).unwrap() * w) >> Limb::LOG_WIDTH).unwrap();
        let mut ii = pseudorandom_residues(n << 1, limbs, 2);
        let xs = residues_mod(&ii, limbs);
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        fft_negacyclic(&mut ii, n, w, &mut t1, &mut t2, &mut temp);
        ifft_negacyclic(&mut ii, n, w, &mut t1, &mut t2, &mut temp);
        for (x, y) in xs.iter().zip(ii.iter()) {
            assert_eq!(
                residue_mod(y, limbs),
                fermat_mul_power_of_2(x, u64::from((n << 1).trailing_zeros()), limbs)
            );
        }
    };
    // - w.odd()
    test(Limb::WIDTH as usize, 1);
}
