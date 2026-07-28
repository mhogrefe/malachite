// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 2001-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_slice_add_limb_in_place,
};
use crate::natural::arithmetic::mul::limb::limbs_slice_mul_limb_in_place;
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use crate::natural::arithmetic::shl::{
    limbs_shl_add_same_length_in_place_left, limbs_shl_add_same_length_to_out,
    limbs_shl_sub_same_length_in_place_left, limbs_shl_sub_same_length_in_place_right,
    limbs_shl_to_out,
};
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right,
};
use crate::platform::{FIB_TABLE, FIB_TABLE_LIMIT, FIB_TABLE_LUCNUM_LIMIT, Limb};
use alloc::vec;
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{Fibonacci, LucasNumber, Parity};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;

// The size in limbs required by `limbs_fibonacci_pair` for `fs` and `f1s`.
//
// From Knuth vol 1 section 1.2.8, F(n) = phi ^ n / sqrt(5) rounded to the nearest integer, where
// phi = (1 + sqrt(5)) / 2. So the number of bits in F(n) is approximately n * log_2(phi) = 0.6942 *
// n, and 23 / 32 = 0.71875 is a safe overestimate.
//
// This is equivalent to `MPN_FIB2_SIZE` from `gmp-impl.h`, GMP 6.3.0.
pub(crate) const fn limbs_fibonacci_pair_alloc_len(n: u64) -> usize {
    (((n >> 5) * 23) >> Limb::LOG_WIDTH) as usize + 4
}

// Writes F(n) to `fs` and F(n - 1) to `f1s`, and returns the number of limbs written. Both slices
// must have length at least `limbs_fibonacci_pair_alloc_len(n)`.
//
// The return value is the actual number of limbs stored; it is at least 1. `fs[size - 1]` is
// nonzero, except when n == 0, in which case `fs[0]` is 0 and `f1s[0]` is 1 (since F(-1) = 1).
// `f1s[size - 1]` can be zero, since F(n - 1) < F(n) for n > 0.
//
// The algorithm takes a starting pair from the table and then doubles down the remaining bits of n,
// using
//
// F(2k - 1) = F(k) ^ 2 + F(k - 1) ^ 2 F(2k + 1) = 4 * F(k) ^ 2 - F(k - 1) ^ 2 + 2 * (-1) ^ k F(2k)
// = F(2k + 1) - F(2k - 1)
//
// In F(2k + 1) with k odd, the -2 is applied to F(k - 1) ^ 2, which is 0 or 1 mod 4 like all
// squares, just by setting a bit. In F(2k + 1) with k even, the +2 is added after the subtraction.
//
// This is equivalent to `mpn_fib2_ui` from `mpn/generic/fib2_ui.c`, GMP 6.3.0.
pub_crate_test! {limbs_fibonacci_pair(fs: &mut [Limb], f1s: &mut [Limb], n: u64) -> usize {
    // Take a starting pair from the table.
    let mut mask = 1u64;
    let mut nfirst = n;
    while nfirst > FIB_TABLE_LIMIT {
        mask <<= 1;
        nfirst >>= 1;
    }
    let nfirst = usize::wrapping_from(nfirst);
    f1s[0] = FIB_TABLE[nfirst]; // F(nfirst - 1)
    fs[0] = FIB_TABLE[nfirst + 1]; // F(nfirst)
    let mut size = 1;
    // Skip the doubling if the table lookup gives the final answer.
    if mask != 1 {
        let alloc = limbs_fibonacci_pair_alloc_len(n);
        let mut xs = vec![0; alloc];
        // The squaring scratch is reused across iterations, growing when needed. Its length is not
        // monotonic in the operand size - it is 0 in both the basecase and FFT regimes - so it is
        // sliced to the exact length each time.
        let mut square_scratch = Vec::new();
        loop {
            // Here fs is F(k) and f1s is F(k - 1), with k being the bits of n from n & mask upward.
            // The next bit of n is n & (mask >> 1), and the pair is doubled to (F(2k), F(2k - 1))
            // or (F(2k + 1), F(2k)) according as that bit is 0 or 1.
            //
            // fs is normalized, and f1s has at most one high zero.
            assert_ne!(fs[size - 1], 0);
            assert!(f1s[size - 1] != 0 || f1s[size - 2] != 0);
            assert!(alloc >= size << 1);
            let scratch_len = limbs_square_to_out_scratch_len(size);
            if square_scratch.len() < scratch_len {
                square_scratch.resize(scratch_len, 0);
            }
            limbs_square_to_out(&mut xs, &fs[..size], &mut square_scratch[..scratch_len]);
            limbs_square_to_out(fs, &f1s[..size], &mut square_scratch[..scratch_len]);
            size <<= 1;
            // Shrink if possible. Since fs was normalized, there is at most one high zero on xs
            // (and if there is one, there is one on the new fs too).
            let xs_last = xs[..size].last().unwrap();
            assert!(*xs_last != 0 || fs[size - 1] == 0);
            if *xs_last == 0 {
                size -= 1;
            }
            let xs_lo = &xs[..size];
            let fs_lo = &mut fs[..size];
            assert_ne!(*xs_lo.last().unwrap(), 0);
            // F(2k - 1) = F(k) ^ 2 + F(k - 1) ^ 2
            f1s[size] = Limb::from(limbs_add_same_length_to_out(
                f1s,
                xs_lo,
                fs_lo,
            ));
            // F(2k + 1) = 4 * F(k) ^ 2 - F(k - 1) ^ 2 + 2 * (-1) ^ k, where n & mask is the low bit
            // of the implied k. fs is F(k - 1) ^ 2, which is 0 or 1 mod 4 like all squares, so the
            // possible -2 can be applied by setting a bit.
            assert_eq!(fs_lo[0] & 2, 0);
            if n & mask != 0 {
                fs_lo[0] |= 2;
            }
            fs[size] = limbs_shl_sub_same_length_in_place_right(xs_lo, fs_lo, 2);
            if n & mask == 0 {
                // The possible +2, applied after the subtraction; it cannot carry out of the high
                // limb, since the result F(2k + 1) fits in size + 1 limbs.
                assert!(!limbs_slice_add_limb_in_place(&mut fs[..=size], 2));
            }
            if fs[size] != 0 {
                size += 1;
            }
            // Now n & mask is the new bit of n being considered.
            mask >>= 1;
            // F(2k) = F(2k + 1) - F(2k - 1), replacing whichever of F(2k + 1) and F(2k - 1) is not
            // wanted.
            if n & mask != 0 {
                assert!(!limbs_sub_same_length_in_place_right(
                    &fs[..size],
                    &mut f1s[..size]
                ));
            } else {
                assert!(!limbs_sub_same_length_in_place_left(
                    &mut fs[..size],
                    &f1s[..size]
                ));
                // There can be a high zero after replacing F(2k + 1) with F(2k). f1s has a high
                // zero if fs does.
                assert!(fs[size - 1] != 0 || f1s[size - 1] == 0);
                if fs[size - 1] == 0 {
                    size -= 1;
                }
            }
            if mask == 1 {
                break;
            }
        }
    }
    size
}}

impl Fibonacci for Natural {
    /// Computes the $n$th Fibonacci number.
    ///
    /// $$
    /// f(n) = F(n),
    /// $$
    /// where $F(0) = 0$, $F(1) = 1$, and $F(n) = F(n-1) + F(n-2)$.
    ///
    /// $F(n) = O(\phi^n)$, where $\phi$ is the golden ratio.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Fibonacci;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::fibonacci(0), 0);
    /// assert_eq!(Natural::fibonacci(1), 1);
    /// assert_eq!(Natural::fibonacci(2), 1);
    /// assert_eq!(Natural::fibonacci(10), 55);
    /// assert_eq!(Natural::fibonacci(100).to_string(), "354224848179261915075");
    /// ```
    ///
    /// This is equivalent to `mpz_fib_ui` from `mpz/fib_ui.c`, GMP 6.3.0.
    fn fibonacci(n: u64) -> Natural {
        if n <= FIB_TABLE_LIMIT {
            return Natural::from(FIB_TABLE[usize::wrapping_from(n) + 1]);
        }
        let n2 = n >> 1;
        let xalloc = limbs_fibonacci_pair_alloc_len(n2) + 1;
        // One allocation serves all three buffers. fs must end up owned, so it is the parent's
        // prefix, and the parent is truncated to the result at the end.
        let mut out = vec![0; xalloc << 2];
        let (fs, rest) = out.split_at_mut(xalloc << 1);
        let (xs, ys) = rest.split_at_mut(xalloc);
        let mut size = limbs_fibonacci_pair(xs, ys, n2);
        if n.odd() {
            // F(2k + 1) = (2 * F(k) + F(k - 1)) * (2 * F(k) - F(k - 1)) + 2 * (-1) ^ k
            let c2 = limbs_shl_to_out(fs, &xs[..size], 1);
            let cx = c2 + Limb::from(limbs_add_same_length_to_out(xs, &fs[..size], &ys[..size]));
            xs[size] = cx;
            let xsize = size + usize::from(cx != 0);
            let cy = c2
                - Limb::from(limbs_sub_same_length_in_place_right(
                    &fs[..size],
                    &mut ys[..size],
                ));
            ys[size] = cy;
            assert!(cy <= 1);
            let ysize = size + usize::from(cy != 0);
            size = xsize + ysize;
            let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(xsize, ysize)];
            limbs_mul_greater_to_out(fs, &xs[..xsize], &ys[..ysize], &mut mul_scratch);
            if n & 2 != 0 {
                // k is odd, so subtract 2. The product is F(4m + 3) + 2, and F(4m + 3) is 1, 2, or
                // 5 mod 8, so the low limb is at least 3 and there is no borrow.
                assert!(fs[0] >= 2);
                fs[0] -= 2;
            } else {
                // k is even, so add 2.
                #[cfg(not(feature = "32_bit_limbs"))]
                {
                    // Since n < 2 ^ Limb::WIDTH, the +2 cannot carry out of the low limb: the
                    // product is F(4m + 1) - 2, and F(3 * 2 ^ b + 1) == 1 mod 2 ^ b is the first
                    // F(4m + 1) congruent to 0 or 1 mod 2 ^ b. See the comments in mpz/fib_ui.c,
                    // GMP 6.3.0.
                    assert!(fs[0] <= const { Limb::MAX - 2 });
                    fs[0] += 2;
                }
                #[cfg(feature = "32_bit_limbs")]
                {
                    // n may be 2 ^ Limb::WIDTH or greater, so the +2 can carry out of the low limb;
                    // but not out of the high limb, since the high limb of a product is never
                    // Limb::MAX.
                    assert_ne!(fs[size - 1], Limb::MAX);
                    assert!(!limbs_slice_add_limb_in_place(&mut fs[..size], 2));
                }
            }
        } else {
            // F(2k) = F(k) * (F(k) + 2 * F(k - 1))
            let cy = limbs_shl_add_same_length_in_place_left(&mut ys[..size], &xs[..size], 1);
            ys[size] = cy;
            let xsize = size;
            let ysize = size + usize::from(cy != 0);
            size += ysize;
            let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ysize, xsize)];
            limbs_mul_greater_to_out(fs, &ys[..ysize], &xs[..xsize], &mut mul_scratch);
        }
        // one or two high zeros
        if fs[size - 1] == 0 {
            size -= 1;
        }
        if fs[size - 1] == 0 {
            // Both multiplicands in the final step are at least F(k), which `limbs_fibonacci_pair`
            // returns normalized, so both are normalized and their product has at most one high
            // zero; and the final +/-2 adjustment cannot clear the top limb. Not triggered by any n
            // up to 65536, at either limb width.
            fail_on_untested_path("fibonacci, two high zeros in the final product");
            size -= 1;
        }
        out.truncate(size);
        // Give back the scratch capacity, so the returned value does not carry it around.
        out.shrink_to_fit();
        Natural::from_owned_limbs_asc(out)
    }

    /// Computes the $n$th Fibonacci number, paired with its predecessor.
    ///
    /// Since $F(-1) = 1$, the pair is defined for $n = 0$ as well.
    ///
    /// $$
    /// f(n) = (F(n), F(n-1)).
    /// $$
    ///
    /// $F(n) = O(\phi^n)$, where $\phi$ is the golden ratio.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Fibonacci;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::fibonacci_pair(0).to_debug_string(), "(0, 1)");
    /// assert_eq!(Natural::fibonacci_pair(1).to_debug_string(), "(1, 0)");
    /// assert_eq!(Natural::fibonacci_pair(10).to_debug_string(), "(55, 34)");
    /// assert_eq!(
    ///     Natural::fibonacci_pair(100).to_debug_string(),
    ///     "(354224848179261915075, 218922995834555169026)"
    /// );
    /// ```
    ///
    /// This is equivalent to `mpz_fib2_ui` from `mpz/fib2_ui.c`, GMP 6.3.0.
    #[cfg_attr(dylint_lib = "malachite_lints", allow(adjacent_vec_allocations))]
    fn fibonacci_pair(n: u64) -> (Natural, Natural) {
        if n <= FIB_TABLE_LIMIT {
            let i = usize::wrapping_from(n);
            return (Natural::from(FIB_TABLE[i + 1]), Natural::from(FIB_TABLE[i]));
        }
        let alloc = limbs_fibonacci_pair_alloc_len(n);
        // The two buffers stay separate allocations: both end up owned, and merging them would
        // force the second one to be copied out of the parent, costing more than the saved
        // allocation.
        let mut fs = vec![0; alloc];
        let mut f1s = vec![0; alloc];
        let size = limbs_fibonacci_pair(&mut fs, &mut f1s, n);
        fs.truncate(size);
        f1s.truncate(size);
        (
            Natural::from_owned_limbs_asc(fs),
            Natural::from_owned_limbs_asc(f1s),
        )
    }
}

// L(n) = F(n) + 2 * F(n - 1), read from the Fibonacci table. Valid whenever n is at most
// `FIB_TABLE_LUCNUM_LIMIT`.
fn one_limb_lucas_number(n: u64) -> Limb {
    let i = usize::wrapping_from(n);
    FIB_TABLE[i + 1] + (FIB_TABLE[i] << 1)
}

impl LucasNumber for Natural {
    /// Computes the $n$th Lucas number.
    ///
    /// $$
    /// f(n) = L(n),
    /// $$
    /// where $L(0) = 2$, $L(1) = 1$, and $L(n) = L(n-1) + L(n-2)$.
    ///
    /// $L(n) = O(\phi^n)$, where $\phi$ is the golden ratio.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LucasNumber;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::lucas_number(0), 2);
    /// assert_eq!(Natural::lucas_number(1), 1);
    /// assert_eq!(Natural::lucas_number(2), 3);
    /// assert_eq!(Natural::lucas_number(10), 123);
    /// assert_eq!(
    ///     Natural::lucas_number(100).to_string(),
    ///     "792070839848372253127"
    /// );
    /// ```
    ///
    /// This is equivalent to `mpz_lucnum_ui` from `mpz/lucnum_ui.c`, GMP 6.3.0.
    #[cfg_attr(dylint_lib = "malachite_lints", allow(adjacent_vec_allocations))]
    fn lucas_number(mut n: u64) -> Natural {
        if n <= FIB_TABLE_LUCNUM_LIMIT {
            return Natural::from(one_limb_lucas_number(n));
        }
        // +1 since L(n) = F(n) + 2 * F(n - 1) might be 1 limb bigger than F(n), and +1 again since
        // the square or multiply below might need an extra limb over the true size.
        let lalloc = limbs_fibonacci_pair_alloc_len(n) + 2;
        // The two buffers stay separate allocations: the doubling loop swaps them, so the result
        // can end up in either one, and only a `Vec` that owns its storage can be passed to
        // `from_owned_limbs_asc` without a copy.
        let mut ls: Vec<Limb> = vec![0; lalloc];
        let mut xs: Vec<Limb> = vec![0; lalloc];
        let mut lsize;
        // Strip trailing zeros from n, until either an odd number is reached, where the L(2k + 1)
        // formula can be used, or until n fits within the table. The table is preferred, of course.
        let mut zeros = 0;
        loop {
            if n.odd() {
                // L(2k + 1) = 5 * F(k - 1) * (2 * F(k) + F(k - 1)) - 4 * (-1) ^ k
                let yalloc = limbs_fibonacci_pair_alloc_len(n >> 1);
                let mut ys = vec![0; yalloc];
                assert!(lalloc >= yalloc);
                let mut xsize = limbs_fibonacci_pair(&mut xs, &mut ys, n >> 1);
                // possible high zero on F(k - 1)
                let mut ysize = xsize;
                if ys[ysize - 1] == 0 {
                    ysize -= 1;
                }
                assert_ne!(ys[ysize - 1], 0);
                // xs = 2 * F(k) + F(k - 1)
                let c = limbs_shl_add_same_length_in_place_left(&mut xs[..xsize], &ys[..xsize], 1);
                assert!(lalloc >= xsize + 1);
                xs[xsize] = c;
                xsize += usize::from(c != 0);
                assert_ne!(xs[xsize - 1], 0);
                assert!(lalloc >= xsize + ysize);
                let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(xsize, ysize)];
                let c =
                    limbs_mul_greater_to_out(&mut ls, &xs[..xsize], &ys[..ysize], &mut mul_scratch);
                lsize = xsize + ysize - usize::from(c == 0);
                // ls = 5 * ls. GMP computes this as ls += 4 * ls, with a comment asking whether a
                // mul-by-5 would be faster; here the mul is used, since it is a single pass.
                let c = limbs_slice_mul_limb_in_place(&mut ls[..lsize], 5);
                assert!(lalloc >= lsize + 1);
                ls[lsize] = c;
                lsize += usize::from(c != 0);
                // ls -= 4 * (-1) ^ k
                if n & 2 != 0 {
                    // k is odd, so add 4. All L(4m + 3) are 4, 5, or 7 mod 8, so there is no carry
                    // when applying +4 to just the low limb.
                    assert!(ls[0] <= { Limb::MAX - 4 });
                    ls[0] += 4;
                } else {
                    // k is even, so subtract 4; this won't go negative.
                    assert!(!limbs_sub_limb_in_place(&mut ls[..lsize], 4));
                }
                break;
            }
            zeros += 1;
            n >>= 1;
            if n <= FIB_TABLE_LUCNUM_LIMIT {
                ls[0] = one_limb_lucas_number(n);
                lsize = 1;
                break;
            }
        }
        // The squaring scratch is reused across iterations, growing when needed; see
        // `limbs_fibonacci_pair`.
        let mut square_scratch = Vec::new();
        // Only the first doubling can have an odd k; afterwards k is always even.
        let mut add_two = n.odd();
        for _ in 0..zeros {
            // L(2k) = L(k) ^ 2 - 2 * (-1) ^ k
            assert!(xs.len() >= lsize << 1);
            let scratch_len = limbs_square_to_out_scratch_len(lsize);
            if square_scratch.len() < scratch_len {
                square_scratch.resize(scratch_len, 0);
            }
            limbs_square_to_out(&mut xs, &ls[..lsize], &mut square_scratch[..scratch_len]);
            lsize <<= 1;
            if xs[lsize - 1] == 0 {
                lsize -= 1;
            }
            if add_two {
                // k is odd, so add 2. L(k) ^ 2 is 0 or 1 mod 4, like all squares, so the +2 gives
                // no carry.
                assert!(xs[0] <= { Limb::MAX - 2 });
                xs[0] += 2;
                add_two = false;
            } else {
                // k is even, so subtract 2; this won't go negative.
                assert!(!limbs_sub_limb_in_place(&mut xs[..lsize], 2));
            }
            swap(&mut ls, &mut xs);
            assert_ne!(ls[lsize - 1], 0);
        }
        ls.truncate(lsize);
        Natural::from_owned_limbs_asc(ls)
    }

    /// Computes the $n$th Lucas number, paired with its predecessor.
    ///
    /// $$
    /// f(n) = (L(n), L(n-1)).
    /// $$
    ///
    /// $L(n) = O(\phi^n)$, where $\phi$ is the golden ratio.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Panics
    /// Panics if `n` is 0, since $L(-1) = -1$ cannot be represented as a [`Natural`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LucasNumber;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::lucas_number_pair(1).to_debug_string(), "(1, 2)");
    /// assert_eq!(Natural::lucas_number_pair(2).to_debug_string(), "(3, 1)");
    /// assert_eq!(
    ///     Natural::lucas_number_pair(10).to_debug_string(),
    ///     "(123, 76)"
    /// );
    /// assert_eq!(
    ///     Natural::lucas_number_pair(100).to_debug_string(),
    ///     "(792070839848372253127, 489526700523968661124)"
    /// );
    /// ```
    ///
    /// This is equivalent to `mpz_lucnum2_ui` from `mpz/lucnum2_ui.c`, GMP 6.3.0, except that GMP
    /// returns $L(-1) = -1$ for `n == 0`, which is not a [`Natural`], so this function panics
    /// instead.
    fn lucas_number_pair(n: u64) -> (Natural, Natural) {
        assert_ne!(n, 0, "L(-1) = -1 cannot be represented as a Natural");
        if n <= FIB_TABLE_LUCNUM_LIMIT {
            let i = usize::wrapping_from(n);
            let f = FIB_TABLE[i + 1];
            let f1 = FIB_TABLE[i];
            // L(n) = F(n) + 2 * F(n - 1) and L(n - 1) = 2 * F(n) - F(n - 1)
            return (Natural::from(f + (f1 << 1)), Natural::from((f << 1) - f1));
        }
        let alloc = limbs_fibonacci_pair_alloc_len(n);
        // L(n)'s buffer must end up owned, so it is the parent's prefix, with the F(n - 1) scratch
        // as the tail. L(n - 1)'s buffer cannot join the allocation, since it must also end up
        // owned.
        let mut out = vec![0; (alloc << 1) + 1];
        let (ls, f1s) = out.split_at_mut(alloc + 1);
        let mut l1s = vec![0; alloc + 1];
        let size = limbs_fibonacci_pair(&mut l1s, f1s, n);
        // L(n) = F(n) + 2 * F(n - 1)
        let c = limbs_shl_add_same_length_to_out(ls, &f1s[..size], &l1s[..size], 1);
        ls[size] = c;
        let ls_len = size + usize::from(c != 0);
        // L(n - 1) = 2 * F(n) - F(n - 1)
        let c = limbs_shl_sub_same_length_in_place_left(&mut l1s[..size], &f1s[..size], 1);
        l1s[size] = c;
        l1s.truncate(size + usize::from(c != 0));
        out.truncate(ls_len);
        // Give back the scratch capacity, so the returned value does not carry it around.
        out.shrink_to_fit();
        (
            Natural::from_owned_limbs_asc(out),
            Natural::from_owned_limbs_asc(l1s),
        )
    }
}
