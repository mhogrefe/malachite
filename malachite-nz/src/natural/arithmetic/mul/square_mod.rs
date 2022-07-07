use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{Parity, WrappingAddAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::slices::slice_test_zero;
use natural::arithmetic::add::{
    limbs_add_greater_to_out, limbs_add_same_length_to_out, limbs_add_to_out,
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::mul::fft::{limbs_mul_fft, limbs_mul_fft_best_k, SQR_FFT_MODF_THRESHOLD};
use natural::arithmetic::mul::mul_mod::{
    limbs_mul_mod_base_pow_n_minus_1_next_size_helper,
    limbs_mul_mod_base_pow_n_plus_1_basecase_helper, FFT_FIRST_K, MUL_FFT_MODF_THRESHOLD,
};
use natural::arithmetic::shr::limbs_slice_shr_in_place;
use natural::arithmetic::square::limbs_square_to_out;
use natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_greater_to_out, limbs_sub_limb_in_place,
    limbs_sub_same_length_to_out, limbs_sub_same_length_with_borrow_in_in_place_right,
};
use platform::Limb;
use std::cmp::min;

//TODO tune
const SQRMOD_BNM1_THRESHOLD: usize = 16;

// This is equivalent to `mpn_sqrmod_bnm1_next_size` from `mpn/generic/sqrmod_bnm1.c`, GMP 6.2.1.
#[inline]
pub(crate) fn limbs_square_mod_base_pow_n_minus_1_next_size(n: usize) -> usize {
    limbs_mul_mod_base_pow_n_minus_1_next_size_helper(
        n,
        SQRMOD_BNM1_THRESHOLD,
        SQR_FFT_MODF_THRESHOLD,
        true,
    )
}

// This is equivalent to `mpn_sqrmod_bnm1_itch` from `gmp-impl.h`, GMP 6.2.1.
pub(crate) const fn limbs_square_mod_base_pow_n_minus_1_scratch_len(
    n: usize,
    xs_len: usize,
) -> usize {
    if xs_len > n >> 1 {
        n + xs_len + 3
    } else {
        n + 3
    }
}

// Input is {ap,rn}; output is {rp,rn}, computation is
// mod B^rn - 1, and values are semi-normalised; zero is represented
// as either 0 or B^n - 1.  Needs a scratch of 2rn limbs at tp.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_bc_sqrmod_bnm1` from `mpn/generic/sqrmod_bnm1.c`, GMP 6.2.1, where
// `rp != tp`.
fn limbs_square_mod_base_pow_n_minus_1_basecase(
    out: &mut [Limb],
    xs: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_ne!(n, 0);
    assert!(scratch.len() >= xs.len() << 1);
    limbs_square_to_out(scratch, xs);
    split_into_chunks_mut!(scratch, n, [scratch_lo, scratch_hi], _unused);
    // If carry == 1, then the value of out is at most B ^ n - 2, so there can be no overflow when
    // adding in the carry.
    if limbs_add_same_length_to_out(out, scratch_lo, scratch_hi) {
        assert!(!limbs_slice_add_limb_in_place(&mut out[..n], 1));
    }
}

// Input is {xs, n+1}; output is {xs, n + 1}, in
// semi-normalised representation, computation is mod B ^ n + 1.
// Output is normalised.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_bc_sqrmod_bnp1` from `mpn/generic/sqrmod_bnm1.c`, GMP 6.2.1, where
// `rp == tp`.
fn limbs_square_mod_base_pow_n_plus_1_basecase(out: &mut [Limb], xs: &[Limb], n: usize) {
    assert_ne!(n, 0);
    limbs_square_to_out(out, &xs[..n + 1]);
    limbs_mul_mod_base_pow_n_plus_1_basecase_helper(out, n);
}

// Computes {out, min(n, 2 * xs.len())} <- xs ^ 2 mod (B ^ n - 1)
//
// The result is expected to be zero if and only if the operand already is. Otherwise the class
// \[0\] mod (B ^ n - 1) is represented by B ^ n - 1.
//
// It should not be a problem if `limbs_square_mod_base_pow_n_minus_1` is used to compute the full
// square with xs.len() <= 2 * n, because this condition implies (B ^ xs.len() - 1) ^ 2 <
// (B ^ n - 1) .
//
// Requires n / 4 < xs.len() <= n
// Scratch need: n / 2 + (need for recursive call OR n + 3). This gives
//
// S(n) <= n / 2 + MAX (n + 4, S(half_n / 2)) <= 3 / 2 * n + 4
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_sqrmod_bnm1` from `mpn/generic/sqrmod_bnm1.c`, GMP 6.2.1.
pub_crate_test! {limbs_square_mod_base_pow_n_minus_1(
    out: &mut [Limb],
    n: usize,
    xs: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    assert!(xs_len <= n);
    let two_xs_len = xs_len << 1;
    if n < SQRMOD_BNM1_THRESHOLD || n.odd() {
        if xs_len < n {
            if two_xs_len <= n {
                limbs_square_to_out(out, xs);
            } else {
                fail_on_untested_path("(n < SQRMOD_BNM1_THRESHOLD || n.odd()) && two_xs_len > n");
                limbs_square_to_out(scratch, xs);
                let (scratch_lo, scratch_hi) = scratch.split_at(n);
                if limbs_add_to_out(out, scratch_lo, &scratch_hi[..two_xs_len - n]) {
                    assert!(!limbs_slice_add_limb_in_place(&mut out[..n], 1));
                }
            }
        } else {
            limbs_square_mod_base_pow_n_minus_1_basecase(out, &xs[..n], scratch);
        }
    } else {
        let half_n = n >> 1;
        assert!(two_xs_len > half_n);
        // Compute xm = a ^ 2 mod (B ^ half_n - 1), scratch = a ^ 2 mod (B ^ half_n + 1)
        // and CRT together as
        // x = -scratch * B ^ half_n + (B ^ half_n + 1) * [(scratch + xm) / 2 mod (B ^ half_n - 1)]
        let k = if half_n < MUL_FFT_MODF_THRESHOLD {
            0
        } else {
            min(
                limbs_mul_fft_best_k(half_n, true),
                usize::wrapping_from(half_n.trailing_zeros()),
            )
        };
        let m = half_n + 1;
        if xs_len > half_n {
            let (xs_lo, xs_hi) = xs.split_at(half_n);
            let (xp_lo, xp_hi) = scratch.split_at_mut(half_n);
            if limbs_add_greater_to_out(xp_lo, xs_lo, xs_hi) {
                limbs_slice_add_limb_in_place(xp_lo, 1);
            }
            limbs_square_mod_base_pow_n_minus_1(out, half_n, xp_lo, xp_hi);
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(m << 1);
            scratch_hi[half_n] = 0;
            if limbs_sub_greater_to_out(scratch_hi, xs_lo, xs_hi) {
                assert!(!limbs_slice_add_limb_in_place(&mut scratch_hi[..m], 1));
            }
            if k >= FFT_FIRST_K {
                let xs = &scratch_hi[..half_n + usize::exact_from(scratch_hi[half_n])];
                scratch_lo[half_n] = Limb::iverson(limbs_mul_fft(scratch_lo, half_n, xs, xs, k));
            } else {
                limbs_square_mod_base_pow_n_plus_1_basecase(scratch_lo, scratch_hi, half_n);
            }
        } else {
            limbs_square_mod_base_pow_n_minus_1(out, half_n, xs, scratch);
            if k >= FFT_FIRST_K {
                scratch[half_n] = Limb::iverson(limbs_mul_fft(scratch, half_n, xs, xs, k));
            } else {
                assert!(xs_len <= half_n);
                limbs_square_to_out(scratch, xs);
                let (scratch_lo, scratch_hi) = scratch[..two_xs_len].split_at_mut(half_n);
                let carry = limbs_sub_greater_in_place_left(scratch_lo, scratch_hi);
                scratch_hi[0] = 0;
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(&mut scratch[..m], 1));
                }
            }
        }
        // Here the CRT recomposition begins.
        //
        // xm <- (scratch + xm) / 2 = (scratch + xm) B ^ half_n / 2 mod (B ^ half_n - 1)
        // Division by 2 is a bitwise rotation.
        //
        // Assumes scratch normalised mod (B ^ half_n + 1).
        //
        // The residue class [0] is represented by [B ^ half_n - 1], except when both inputs are
        // zero. scratch[half_n] == 1 implies {scratch, half_n} == 0
        let mut carry = scratch[half_n];
        let (out_lo, out_hi) = out.split_at_mut(half_n);
        if limbs_slice_add_same_length_in_place_left(out_lo, &scratch[..half_n]) {
            carry.wrapping_add_assign(1);
        }
        if out_lo[0].odd() {
            carry.wrapping_add_assign(1);
        }
        limbs_slice_shr_in_place(out_lo, 1);
        assert!(carry <= 2);
        let hi = carry << (Limb::WIDTH - 1); // (carry & 1) << ...
        carry >>= 1;
        // We can have carry != 0 only if hi = 0...
        let out_last = out_lo.last_mut().unwrap();
        assert!(!out_last.get_highest_bit());
        *out_last |= hi;
        // out[half_n - 1] + carry can't overflow, so the following increment is correct.
        assert!(carry <= 1);
        // Next increment can not overflow: read the previous comments about carry.
        assert!(carry == 0 || !out_last.get_highest_bit());
        assert!(!limbs_slice_add_limb_in_place(out_lo, carry));
        // Compute the highest half: ([(scratch + xm) / 2 mod (B ^ half_n - 1)] - scratch ) *
        // B ^ half_n
        if two_xs_len < n {
            // Note that in this case, the only way the result can equal zero mod B ^ n - 1 is if
            // the input is zero, and then the output of both the recursive calls and this CRT
            // reconstruction is zero, not B ^ n - 1.
            let k = two_xs_len - half_n;
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(k);
            let borrow = limbs_sub_same_length_to_out(out_hi, &out_lo[..k], scratch_lo);
            let mut carry = scratch_hi[(half_n - xs_len) << 1];
            if limbs_sub_same_length_with_borrow_in_in_place_right(
                &out[k..n - half_n],
                &mut scratch_hi[..n - two_xs_len],
                borrow,
            ) {
                carry.wrapping_add_assign(1);
            }
            assert!(slice_test_zero(&scratch_hi[1..n - two_xs_len]));
            if carry != 0 {
                carry = Limb::iverson(limbs_sub_limb_in_place(&mut out[..two_xs_len], 1));
            }
            assert_eq!(carry, scratch_hi[0]);
        } else {
            let (scratch_last, scratch_init) = scratch[..m].split_last().unwrap();
            let mut carry = *scratch_last;
            if limbs_sub_same_length_to_out(out_hi, out_lo, scratch_init) {
                carry.wrapping_add_assign(1);
            }
            // carry = 1 only if {scratch, half_n + 1} is not zero, i.e. {out, half_n} is not zero.
            // The decrement will affect _at most_ the lowest half_n limbs.
            assert!(!limbs_sub_limb_in_place(&mut out[..half_n << 1], carry));
        }
    }
}}
