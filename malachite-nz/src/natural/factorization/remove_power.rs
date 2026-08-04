// Copyright © 2026 William Youmans
//
// Uses code adopted from the FLINT Library.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural::arithmetic::div_exact::limbs_modular_div_mod_wrap;
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::platform::{LIMB_WIDTH_USIZE, Limb};
use alloc::vec::Vec;
use core::cmp::Ordering::Equal;
use core::cmp::min;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{Parity, Pow, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::{RemovePower, RemovePowerAssign};
use malachite_base::num::logic::traits::{LowMask, SignificantBits};
use malachite_base::slices::slice_test_zero;

// Remove the largest power of V from U that doesn't exceed the given cap
//
// This is `mpn_remove` from GMP 6.3.0.
#[doc(hidden)]
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
    assert!(vp[0].odd(), "V must be odd for 2-adic division");
    assert!(vn > 1 || vp[0] > 1, "V must be > 1 to avoid infinite loop");

    // Temporary work buffers, sharing one allocation: the algorithm only ever uses them as slices,
    // so the ping-pong `swap`s exchange the slice references.
    let mut work = vec![0; ((un + 1) << 1) + ((un + 1 + vn) >> 1)];
    let (mut qp, rest) = work.split_at_mut(un + 1);
    let (mut qp2, tp) = rest.split_at_mut(un + 1);

    // Copy input into quotient buffer
    qp[..un].copy_from_slice(up);
    let mut qn = un;

    // Store the powers of V
    let mut pwpsn = Vec::with_capacity(LIMB_WIDTH_USIZE);
    let mut pwpsp_offsets = Vec::with_capacity(LIMB_WIDTH_USIZE);

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
            limbs_modular_div_mod_wrap(&mut qp2[..=qn - pn], &mut tp[..pn], &qp[..=qn], &vp[..pn]);
            if !slice_test_zero(&tp[..pn]) && limbs_cmp_same_length(&tp[..pn], &vp[..pn]) != Equal {
                break; // cannot divide
            }
        } else {
            // Access the power from storage without creating a conflicting borrow
            let power_slice = &powers_storage[current_power_offset..current_power_offset + pn];
            limbs_modular_div_mod_wrap(
                &mut qp2[..=qn - pn],
                &mut tp[..pn],
                &qp[..=qn],
                power_slice,
            );
            if !slice_test_zero(&tp[..pn]) && limbs_cmp_same_length(&tp[..pn], power_slice) != Equal
            {
                break; // cannot divide
            }
        }
        swap(&mut qp, &mut qp2);
        qn -= pn;
        // GMP negates the quotient here, because `mpn_bdiv_qr` returns its negation;
        // `limbs_modular_div_mod` returns the quotient itself, so only the length is adjusted.
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

        let nn = (pn << 1) - 1;
        if nn > qn {
            break;
        }
        // allocate powers_storage on first use
        if npowers == 1 {
            powers_storage = vec![0; qn + LIMB_WIDTH_USIZE];
        }
        // compute square of current power into powers_storage
        let np_offset = if npowers == 1 {
            0
        } else {
            powers_storage.len()
        };
        let np_end = np_offset + (pn << 1);
        powers_storage.resize(np_end, 0);
        let mut scratch = vec![0; limbs_square_to_out_scratch_len(pn)];
        if current_power_is_vp {
            limbs_square_to_out(
                &mut powers_storage[np_offset..np_end],
                &vp[..pn],
                &mut scratch,
            );
        } else {
            // Square the current power from `powers_storage` into a new location. The source always
            // ends at or before the destination: each squaring leaves `powers_storage.len()` at
            // `current_power_offset + 2 * pn`, and the next `pn` is at most that same `2 * pn`, so
            // `split_at_mut` at the destination never splits the source.
            let (src_part, dst_part) = powers_storage.split_at_mut(np_offset);
            let src = &src_part[current_power_offset..current_power_offset + pn];
            limbs_square_to_out(&mut dst_part[..pn << 1], src, &mut scratch);
        }

        pn = nn;
        if powers_storage[np_offset + nn] != 0 {
            pn += 1;
        }

        current_power_is_vp = false;
        current_power_offset = np_offset;
    }

    let mut pwr = usize::low_mask(u64::exact_from(npowers));

    for i in (0..npowers).rev() {
        let pn = pwpsn[i];
        if qn < pn || pwr + usize::power_of_2(u64::exact_from(i)) > cap {
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
            &qp[..=qn],
            power_slice,
        );

        if !slice_test_zero(&tp[..pn]) && limbs_cmp_same_length(&tp[..pn], power_slice) != Equal {
            continue;
        }

        swap(&mut qp, &mut qp2);
        qn -= pn;
        if qp[qn] != 0 {
            qn += 1;
        }

        pwr += usize::power_of_2(u64::exact_from(i));
    }

    wp.clear();
    wp.extend_from_slice(&qp[..qn]);

    pwr
}

// Divides `x` by the largest power of the odd `y` that divides it, at most `cap` times.
fn remove_odd_power(x: &Natural, y: &Natural, cap: usize) -> (Natural, u64) {
    let mut out = Vec::new();
    let k = limbs_remove(&mut out, &x.to_limbs_asc(), &y.to_limbs_asc(), cap);
    (Natural::from_owned_limbs_asc(out), u64::exact_from(k))
}

fn remove_power_helper(x: &Natural, y: &Natural) -> (Natural, u64) {
    assert!(*y > 1u32, "Cannot remove powers of {y}");
    if *x == 0u32 {
        // every power of `y` divides zero, so, as GMP does, leave it alone
        return (Natural::ZERO, 0);
    }
    if x < y {
        // nothing to remove, and the kernel needs a dividend at least as large as its divisor
        return (x.clone(), 0);
    }
    if let (Natural(Small(sx)), Natural(Small(sy))) = (x, y) {
        // use the single-limb implementation for primitive integers
        let (q, k) = sx.remove_power(*sy);
        return (Natural::from(q), k);
    }
    // The exponent cannot exceed the bit length, since the smallest divisor above 1 is 2.
    let cap = usize::exact_from(x.significant_bits());
    let two_pow = y.trailing_zeros().unwrap();
    if two_pow == 0 {
        return remove_odd_power(x, y, cap);
    }
    // `y` is even. A power of `y` divides `x` only when its power of two and its odd part both
    // divide `x`, and those are coprime, so the exponent is the smaller of the two limits. The
    // kernel's own cap cannot express this, since it always performs one division before consulting
    // it.
    let two_limit = x.trailing_zeros().unwrap() / two_pow;
    let odd = y >> two_pow;
    if odd == 1u32 {
        return (x >> (two_pow * two_limit), two_limit);
    }
    let (q, odd_limit) = remove_odd_power(x, &odd, cap);
    let k = min(two_limit, odd_limit);
    // restore the powers of the odd part that the supply of twos cannot support
    let q = if k == odd_limit {
        q
    } else {
        q * (&odd).pow(odd_limit - k)
    };
    (q >> (two_pow * k), k)
}

impl RemovePower<Self> for Natural {
    type Output = Self;

    /// Removes the largest power of a factor from a [`Natural`], returning the reduced [`Natural`]
    /// together with the exponent of that power.
    ///
    /// If $f^k$ is the largest power of `other` that divides `self`, this returns
    /// $(\text{self}/f^k, k)$. The factor need not be prime. Zero is left alone, with an exponent
    /// of 0, since every power of the factor divides it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or 1, since every power of those either divides everything or divides
    /// nothing more than once.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_nz::natural::Natural;
    ///
    /// let (q, k) = Natural::from(12u32).remove_power(Natural::from(2u32));
    /// assert_eq!(q, 3);
    /// assert_eq!(k, 2);
    ///
    /// // the factor need not be prime
    /// let (q, k) = Natural::from(1000u32).remove_power(Natural::from(10u32));
    /// assert_eq!(q, 1);
    /// assert_eq!(k, 3);
    ///
    /// // a factor that does not divide at all is removed zero times
    /// let (q, k) = Natural::from(7u32).remove_power(Natural::from(3u32));
    /// assert_eq!(q, 7);
    /// assert_eq!(k, 0);
    /// ```
    #[inline]
    fn remove_power(self, other: Self) -> (Self, u64) {
        remove_power_helper(&self, &other)
    }
}

impl RemovePower<&Self> for Natural {
    type Output = Self;

    /// Removes the largest power of a factor from a [`Natural`], returning the reduced [`Natural`]
    /// together with the exponent of that power.
    ///
    /// If $f^k$ is the largest power of `other` that divides `self`, this returns
    /// $(\text{self}/f^k, k)$. The factor need not be prime. Zero is left alone, with an exponent
    /// of 0, since every power of the factor divides it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or 1, since every power of those either divides everything or divides
    /// nothing more than once.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_nz::natural::Natural;
    ///
    /// let (q, k) = Natural::from(12u32).remove_power(Natural::from(2u32));
    /// assert_eq!(q, 3);
    /// assert_eq!(k, 2);
    ///
    /// // the factor need not be prime
    /// let (q, k) = Natural::from(1000u32).remove_power(Natural::from(10u32));
    /// assert_eq!(q, 1);
    /// assert_eq!(k, 3);
    ///
    /// // a factor that does not divide at all is removed zero times
    /// let (q, k) = Natural::from(7u32).remove_power(Natural::from(3u32));
    /// assert_eq!(q, 7);
    /// assert_eq!(k, 0);
    /// ```
    #[inline]
    fn remove_power(self, other: &Self) -> (Self, u64) {
        remove_power_helper(&self, other)
    }
}

impl RemovePower<Natural> for &Natural {
    type Output = Natural;

    /// Removes the largest power of a factor from a [`Natural`], returning the reduced [`Natural`]
    /// together with the exponent of that power.
    ///
    /// If $f^k$ is the largest power of `other` that divides `self`, this returns
    /// $(\text{self}/f^k, k)$. The factor need not be prime. Zero is left alone, with an exponent
    /// of 0, since every power of the factor divides it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or 1, since every power of those either divides everything or divides
    /// nothing more than once.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_nz::natural::Natural;
    ///
    /// let (q, k) = Natural::from(12u32).remove_power(Natural::from(2u32));
    /// assert_eq!(q, 3);
    /// assert_eq!(k, 2);
    ///
    /// // the factor need not be prime
    /// let (q, k) = Natural::from(1000u32).remove_power(Natural::from(10u32));
    /// assert_eq!(q, 1);
    /// assert_eq!(k, 3);
    ///
    /// // a factor that does not divide at all is removed zero times
    /// let (q, k) = Natural::from(7u32).remove_power(Natural::from(3u32));
    /// assert_eq!(q, 7);
    /// assert_eq!(k, 0);
    /// ```
    #[inline]
    fn remove_power(self, other: Natural) -> (Natural, u64) {
        remove_power_helper(self, &other)
    }
}

impl RemovePower<&Natural> for &Natural {
    type Output = Natural;

    /// Removes the largest power of a factor from a [`Natural`], returning the reduced [`Natural`]
    /// together with the exponent of that power.
    ///
    /// If $f^k$ is the largest power of `other` that divides `self`, this returns
    /// $(\text{self}/f^k, k)$. The factor need not be prime. Zero is left alone, with an exponent
    /// of 0, since every power of the factor divides it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or 1, since every power of those either divides everything or divides
    /// nothing more than once.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_nz::natural::Natural;
    ///
    /// let (q, k) = Natural::from(12u32).remove_power(Natural::from(2u32));
    /// assert_eq!(q, 3);
    /// assert_eq!(k, 2);
    ///
    /// // the factor need not be prime
    /// let (q, k) = Natural::from(1000u32).remove_power(Natural::from(10u32));
    /// assert_eq!(q, 1);
    /// assert_eq!(k, 3);
    ///
    /// // a factor that does not divide at all is removed zero times
    /// let (q, k) = Natural::from(7u32).remove_power(Natural::from(3u32));
    /// assert_eq!(q, 7);
    /// assert_eq!(k, 0);
    /// ```
    #[inline]
    fn remove_power(self, other: &Natural) -> (Natural, u64) {
        remove_power_helper(self, other)
    }
}

impl RemovePowerAssign<Self> for Natural {
    /// Divides a [`Natural`] by the largest power of a factor that divides it, in place, returning
    /// the exponent of that power. The factor is taken by value.
    ///
    /// The factor need not be prime. Zero is left alone, with an exponent of 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::RemovePowerAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(12u32);
    /// assert_eq!(x.remove_power_assign(Natural::from(2u32)), 2);
    /// assert_eq!(x, 3);
    /// ```
    #[inline]
    fn remove_power_assign(&mut self, other: Self) -> u64 {
        let (q, k) = remove_power_helper(self, &other);
        *self = q;
        k
    }
}

impl RemovePowerAssign<&Self> for Natural {
    /// Divides a [`Natural`] by the largest power of a factor that divides it, in place, returning
    /// the exponent of that power. The factor is taken by reference.
    ///
    /// The factor need not be prime. Zero is left alone, with an exponent of 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or 1.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::RemovePowerAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(12u32);
    /// assert_eq!(x.remove_power_assign(Natural::from(2u32)), 2);
    /// assert_eq!(x, 3);
    /// ```
    #[inline]
    fn remove_power_assign(&mut self, other: &Self) -> u64 {
        let (q, k) = remove_power_helper(self, other);
        *self = q;
        k
    }
}
