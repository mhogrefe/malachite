// Copyright © 2025 William Youmans
//
// Uses code adopted from the FLINT Library.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::div_exact::limbs_modular_div_mod_wrap;
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::Ordering::Equal;
use core::mem::swap;
use malachite_base::slices::slice_test_zero;

#[cfg(feature = "32_bit_limbs")]
const LOG: usize = 32; // For 32-bit limbs

#[cfg(not(feature = "32_bit_limbs"))]
const LOG: usize = 64; // For 64-bit limbs

// This is `mpn_remove` from GMP 6.3.0.
// Remove the largest power of V from U that doesn't exceed the given cap
pub fn limbs_remove(
    wp: &mut Vec<Limb>, // Output: U / V^k
    up: &[Limb],        // Input number U
    vp: &[Limb],        // Divisor V (must be odd)
    cap: usize,         // Maximum power to attempt
) -> usize {
    let un = up.len();
    let vn = vp.len();

    assert!(un > 0);
    assert!(vn > 0);
    assert!(vp[0] % 2 != 0, "V must be odd for 2-adic division");
    assert!(vn > 1 || vp[0] > 1, "V must be > 1 to avoid infinite loop");

    // Temporary work buffers
    let mut qp = vec![0; un + 1];
    let mut qp2 = vec![0; un + 1];
    let mut tp = vec![0; (un + 1 + vn) / 2];

    // Copy input into quotient buffer
    qp[..un].copy_from_slice(up);
    let mut qn = un;

    // Store the powers of V
    let mut pwpsn = Vec::with_capacity(LOG);
    let mut pwpsp_offsets = Vec::with_capacity(LOG);

    // All generated powers of V are stored here
    let mut powers_storage = Vec::new();

    let mut current_power_is_vp = true; // true if current power is vp, false if in powers_storage
    let mut current_power_offset = 0; // offset in powers_storage if current_power_is_vp is false
    let mut pn = vn;
    let mut npowers = 0;

    while qn >= pn {
        qp[qn] = 0;

        if current_power_is_vp {
            // Use original vp directly
            limbs_modular_div_mod_wrap(
                &mut qp2[..qn - pn + 1],
                &mut tp[..pn],
                &qp[..qn],
                &vp[..pn],
            );
            if !slice_test_zero(&tp[..pn]) && limbs_cmp_same_length(&tp[..pn], &vp[..pn]) != Equal {
                break; // cannot divide
            }
        } else {
            // Access the power from storage without creating a conflicting borrow
            let power_slice = &powers_storage[current_power_offset..current_power_offset + pn];
            limbs_modular_div_mod_wrap(
                &mut qp2[..qn - pn + 1],
                &mut tp[..pn],
                &qp[..qn],
                power_slice,
            );
            if !slice_test_zero(&tp[..pn]) && limbs_cmp_same_length(&tp[..pn], power_slice) != Equal
            {
                break; // cannot divide
            }
        }

        swap(&mut qp, &mut qp2);
        qn = qn - pn;
        limbs_neg_in_place(&mut qp[..qn + 1]);
        if qp[qn] != 0 {
            qn += 1;
        }

        // record power
        pwpsp_offsets.push(if current_power_is_vp {
            usize::MAX
        } else {
            current_power_offset
        });
        pwpsn.push(pn);
        npowers += 1;

        if ((2usize << npowers) - 1) > cap {
            break;
        }

        let nn = 2 * pn - 1;
        if nn > qn {
            break;
        }

        // allocate powers_storage on first use
        if npowers == 1 {
            powers_storage = vec![0; qn + LOG];
        }

        // compute square of current power into powers_storage
        let np_offset = if npowers == 1 {
            0
        } else {
            powers_storage.len()
        };
        let np_end = np_offset + 2 * pn;
        powers_storage.resize(np_end, 0);

        let mut scratch = vec![0; limbs_square_to_out_scratch_len(pn)];

        if current_power_is_vp {
            limbs_square_to_out(
                &mut powers_storage[np_offset..np_end],
                &vp[..pn],
                &mut scratch,
            );
        } else {
            // Square the current power from powers_storage into a new location
            // need to be careful about overlapping borrows
            // we can use split_at_mut to get non-overlapping mutable slices

            let src_end = current_power_offset + pn;
            if src_end <= np_offset {
                // Source and destination don't overlap - safe to borrow both
                let (src_part, dst_part) = powers_storage.split_at_mut(np_offset);
                let src = &src_part[current_power_offset..src_end];
                limbs_square_to_out(&mut dst_part[..2 * pn], src, &mut scratch);
            } else {
                // Fallback: copy source data to avoid overlapping borrows
                // This should rarely happen with our offset calculation
                let src_data: Vec<Limb> = powers_storage[current_power_offset..src_end].to_vec();
                limbs_square_to_out(
                    &mut powers_storage[np_offset..np_end],
                    &src_data,
                    &mut scratch,
                );
            }
        }

        pn = nn;
        if powers_storage[np_offset + nn] != 0 {
            pn += 1;
        }

        current_power_is_vp = false;
        current_power_offset = np_offset;
    }

    let mut pwr = (1usize << npowers) - 1;

    for i in (0..npowers).rev() {
        let pn = pwpsn[i];
        if qn < pn {
            continue;
        }
        if pwr + (1usize << i) > cap {
            continue;
        }

        let power_slice = if pwpsp_offsets[i] == usize::MAX {
            &vp[..pn] // Use original vp
        } else {
            let offset = pwpsp_offsets[i];
            &powers_storage[offset..offset + pn]
        };

        qp[qn] = 0;
        limbs_modular_div_mod_wrap(
            &mut qp2[..=(qn - pn)],
            &mut tp[..pn],
            &qp[..qn],
            power_slice,
        );

        if !slice_test_zero(&tp[..pn]) && limbs_cmp_same_length(&tp[..pn], power_slice) != Equal {
            continue;
        }

        swap(&mut qp, &mut qp2);
        qn -= pn;
        limbs_neg_in_place(&mut qp[..=qn]);
        if qp[qn] != 0 {
            qn += 1;
        }

        pwr += 1usize << i;
    }

    wp.clear();
    wp.extend_from_slice(&qp[..qn]);

    pwr
}
