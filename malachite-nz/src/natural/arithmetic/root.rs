// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Contributed by Paul Zimmermann (algorithm) and Paul Zimmermann and Torbjörn Granlund
//      (implementation). Marco Bodrato wrote `logbased_root` to seed the loop.
//
//      Copyright © 2002, 2005, 2009-2012, 2015 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::div::{limbs_div_limb_to_out, limbs_div_to_out};
use crate::natural::arithmetic::mul::limb::limbs_slice_mul_limb_in_place;
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use crate::natural::arithmetic::pow::limbs_pow;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::shr::limbs_shr_to_out;
use crate::natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_greater_to_out, limbs_sub_limb_in_place,
    limbs_sub_limb_to_out,
};
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CeilingSqrt, CheckedRoot, CheckedSqrt, DivMod, DivRound,
    FloorRoot, FloorRootAssign, FloorSqrt, ModPowerOf2Assign, PowerOf2, RootAssignRem, RootRem,
    SqrtRem,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{LeadingZeros, LowMask, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::{slice_set_zero, slice_trailing_zeros};

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
fn limbs_shl_helper(xs: &mut [Limb], len: usize, out_start_index: usize, bits: u64) -> Limb {
    assert!(bits < Limb::WIDTH);
    if len == 0 {
        0
    } else {
        xs.copy_within(0..len, out_start_index);
        if bits == 0 {
            0
        } else {
            limbs_slice_shl_in_place(&mut xs[out_start_index..out_start_index + len], bits)
        }
    }
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
fn shr_helper(out: &mut [Limb], xs: &[Limb], shift: u64) {
    if shift == 0 {
        out[..xs.len()].copy_from_slice(xs);
    } else {
        limbs_shr_to_out(out, xs, shift);
    }
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
fn div_helper(qs: &mut [Limb], ns: &mut [Limb], ds: &mut [Limb]) {
    assert!(*ns.last().unwrap() != 0);
    assert!(*ds.last().unwrap() != 0);
    if ns.len() == 1 {
        if ds.len() == 1 {
            qs[0] = ns[0] / ds[0];
        } else {
            qs[0] = 0;
        }
    } else if ds.len() == 1 {
        limbs_div_limb_to_out(qs, ns, ds[0]);
    } else {
        limbs_div_to_out(qs, ns, ds);
    }
}

// vlog=vector(256,i,floor((log(256+i)/log(2)-8)*256)-(i>255))
const V_LOG: [u8; 256] = [
    1, 2, 4, 5, 7, 8, 9, 11, 12, 14, 15, 16, 18, 19, 21, 22, 23, 25, 26, 27, 29, 30, 31, 33, 34,
    35, 37, 38, 39, 40, 42, 43, 44, 46, 47, 48, 49, 51, 52, 53, 54, 56, 57, 58, 59, 61, 62, 63, 64,
    65, 67, 68, 69, 70, 71, 73, 74, 75, 76, 77, 78, 80, 81, 82, 83, 84, 85, 87, 88, 89, 90, 91, 92,
    93, 94, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 108, 109, 110, 111, 112, 113, 114,
    115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 131, 132, 133, 134,
    135, 136, 137, 138, 139, 140, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152,
    153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 162, 163, 164, 165, 166, 167, 168, 169, 170,
    171, 172, 173, 173, 174, 175, 176, 177, 178, 179, 180, 181, 181, 182, 183, 184, 185, 186, 187,
    188, 188, 189, 190, 191, 192, 193, 194, 194, 195, 196, 197, 198, 199, 200, 200, 201, 202, 203,
    204, 205, 205, 206, 207, 208, 209, 209, 210, 211, 212, 213, 214, 214, 215, 216, 217, 218, 218,
    219, 220, 221, 222, 222, 223, 224, 225, 225, 226, 227, 228, 229, 229, 230, 231, 232, 232, 233,
    234, 235, 235, 236, 237, 238, 239, 239, 240, 241, 242, 242, 243, 244, 245, 245, 246, 247, 247,
    248, 249, 250, 250, 251, 252, 253, 253, 254, 255, 255,
];

// vexp=vector(256,i,floor(2^(8+i/256)-256)-(i>255))
const V_EXP: [u8; 256] = [
    0, 1, 2, 2, 3, 4, 4, 5, 6, 7, 7, 8, 9, 9, 10, 11, 12, 12, 13, 14, 14, 15, 16, 17, 17, 18, 19,
    20, 20, 21, 22, 23, 23, 24, 25, 26, 26, 27, 28, 29, 30, 30, 31, 32, 33, 33, 34, 35, 36, 37, 37,
    38, 39, 40, 41, 41, 42, 43, 44, 45, 45, 46, 47, 48, 49, 50, 50, 51, 52, 53, 54, 55, 55, 56, 57,
    58, 59, 60, 61, 61, 62, 63, 64, 65, 66, 67, 67, 68, 69, 70, 71, 72, 73, 74, 75, 75, 76, 77, 78,
    79, 80, 81, 82, 83, 84, 85, 86, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100,
    101, 102, 103, 104, 105, 106, 107, 108, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 119,
    120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138,
    139, 140, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 154, 155, 156, 157, 158, 159,
    160, 161, 163, 164, 165, 166, 167, 168, 169, 171, 172, 173, 174, 175, 176, 178, 179, 180, 181,
    182, 183, 185, 186, 187, 188, 189, 191, 192, 193, 194, 196, 197, 198, 199, 200, 202, 203, 204,
    205, 207, 208, 209, 210, 212, 213, 214, 216, 217, 218, 219, 221, 222, 223, 225, 226, 227, 229,
    230, 231, 232, 234, 235, 236, 238, 239, 240, 242, 243, 245, 246, 247, 249, 250, 251, 253, 254,
    255,
];

const LOGROOT_USED_BITS: u64 = 8;
const LOGROOT_NEEDS_TWO_CORRECTIONS: bool = true;
const NEEDED_CORRECTIONS: u64 = if LOGROOT_NEEDS_TWO_CORRECTIONS { 2 } else { 1 };
const LOGROOT_RETURNED_BITS: u64 = if LOGROOT_NEEDS_TWO_CORRECTIONS {
    LOGROOT_USED_BITS + 1
} else {
    LOGROOT_USED_BITS
};

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `logbased_root` from `mpn/generic/rootrem.c`, GMP 6.2.1.
fn log_based_root(out: &mut Limb, x: Limb, mut bit_count: u64, exp: u64) -> u64 {
    const LOGROOT_USED_BITS_COMP: u64 = Limb::WIDTH - LOGROOT_USED_BITS;
    let len;
    let b = u64::from(V_LOG[usize::exact_from(x >> LOGROOT_USED_BITS_COMP)]);
    if bit_count.significant_bits() > LOGROOT_USED_BITS_COMP {
        // In this branch, the input is unreasonably large. In the unlikely case, we use two
        // divisions and a modulo.
        fail_on_untested_path("bit_count.significant_bits() > LOGROOT_USED_BITS_COMP");
        let r;
        (len, r) = bit_count.div_mod(exp);
        bit_count = ((r << LOGROOT_USED_BITS) | b) / exp;
    } else {
        bit_count = ((bit_count << LOGROOT_USED_BITS) | b) / exp;
        len = bit_count >> LOGROOT_USED_BITS;
        bit_count.mod_power_of_2_assign(LOGROOT_USED_BITS);
    }
    assert!(bit_count.significant_bits() <= LOGROOT_USED_BITS);
    *out = Limb::power_of_2(LOGROOT_USED_BITS) | Limb::from(V_EXP[usize::exact_from(bit_count)]);
    if !LOGROOT_NEEDS_TWO_CORRECTIONS {
        *out >>= 1;
    }
    len
}

// If approx is non-zero, does not compute the final remainder.
//
/// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_rootrem_internal` from `mpn/generic/rootrem.c`, GMP 6.2.1.
fn limbs_root_to_out_internal(
    out_root: &mut [Limb],
    out_rem: Option<&mut [Limb]>,
    xs: &[Limb],
    exp: u64,
    approx: bool,
) -> usize {
    let mut xs_len = xs.len();
    let mut xs_hi = xs[xs_len - 1];
    let leading_zeros = LeadingZeros::leading_zeros(xs_hi) + 1;
    let bit_count = (u64::exact_from(xs_len) << Limb::LOG_WIDTH) - leading_zeros;
    let out_rem_is_some = out_rem.is_some();
    if bit_count < exp {
        // root is 1
        out_root[0] = 1;
        if out_rem_is_some {
            let out_rem = out_rem.unwrap();
            limbs_sub_limb_to_out(out_rem, xs, 1);
            // There should be at most one zero limb, if we demand x to be normalized
            if out_rem[xs_len - 1] == 0 {
                xs_len -= 1;
            }
        } else if xs[0] == 1 {
            xs_len -= 1;
        }
        return xs_len;
    }
    xs_hi = if leading_zeros == Limb::WIDTH {
        xs[xs_len - 2]
    } else {
        let mut i = xs_len - 1;
        if xs_len != 1 {
            i -= 1;
        }
        (xs_hi << leading_zeros) | xs[i] >> (Limb::WIDTH - leading_zeros)
    };
    assert!(xs_len != 1 || xs[xs_len - 1] >> (Limb::WIDTH - leading_zeros) == 1);
    // - root_bits + 1 is the number of bits of the root R
    // - APPROX_BITS + 1 is the number of bits of the current approximation S
    let mut root_bits = log_based_root(&mut out_root[0], xs_hi, bit_count, exp);
    const APPROX_BITS: u64 = LOGROOT_RETURNED_BITS - 1;
    let mut input_bits = exp * root_bits; // number of truncated bits in the input
    xs_hi = (Limb::exact_from(exp) - 1) >> 1;
    let mut log_exp = 3;
    loop {
        xs_hi >>= 1;
        if xs_hi == 0 {
            break;
        }
        log_exp += 1;
    }
    // log_exp = ceil(log_2(exp)) + 1
    let mut i = 0;
    let mut sizes = [0u64; (Limb::WIDTH + 1) as usize];
    loop {
        // Invariant: here we want root_bits + 1 total bits for the kth root. If c is the new value
        // of root_bits, this means that we'll go from a root of c + 1 bits (say s') to a root of
        // root_bits + 1 bits. It is proved in the book "Modern Computer Arithmetic" by Brent and
        // Zimmermann, Chapter 1, that if s' >= exp * beta, then at most one correction is
        // necessary. Here beta = 2 ^ (root_bits - c), and s' >= 2 ^ c, thus it suffices that c >=
        // ceil((root_bits + log_2(exp)) / 2).
        sizes[i] = root_bits;
        if root_bits <= APPROX_BITS {
            break;
        }
        if root_bits > log_exp {
            root_bits = (root_bits + log_exp) >> 1;
        } else {
            // add just one bit at a time
            root_bits -= 1;
        }
        i += 1;
    }
    out_root[0] >>= APPROX_BITS - root_bits;
    input_bits -= root_bits;
    assert!(i < usize::wrapping_from(Limb::WIDTH + 1));
    // We have sizes[0] = next_bits > sizes[1] > ... > sizes[ni] = 0, with sizes[i] <= 2 * sizes[i +
    // 1]. Newton iteration will first compute sizes[i - 1] extra bits, then sizes[i - 2], ..., then
    // sizes[0] = next_bits. qs and ws need enough space to store S' ^ exp, where S' is an
    // approximate root. Since S' can be as large as S + 2, the worst case is when S = 2 and S' = 4.
    // But then since we know the number of bits of S in advance, S' can only be 3 at most.
    // Similarly for S = 4, then S' can be 6 at most. So the worst case is S' / S = 3 / 2, thus S' ^
    // exp <= (3 / 2) ^ exp * S ^ exp. Since S ^ exp fits in xs_len limbs, the number of extra limbs
    // needed is bounded by ceil(exp * log_2(3 / 2) / B), where B is `Limb::WIDTH`.
    let extra = (((0.585 * (exp as f64)) / (Limb::WIDTH as f64)) as usize) + 2;
    let mut big_scratch = vec![0; 3 * xs_len + 2 * extra + 1];
    let (scratch, remainder) = big_scratch.split_at_mut(xs_len + 1);
    // - qs will contain quotient and remainder of R / (exp * S ^ (exp - 1)).
    // - ws will contain S ^ (k-1) and exp *S^(k-1).
    let (qs, ws) = remainder.split_at_mut(xs_len + extra);
    let rs = if out_rem_is_some {
        out_rem.unwrap()
    } else {
        scratch
    };
    let ss = out_root;
    // Initial approximation has one limb
    let mut ss_len = 1;
    let mut next_bits = root_bits;
    let mut rs_len = 0;
    let mut save_1;
    let mut save_2 = 0;
    let mut qs_len;
    let mut pow_cmp;
    while i != 0 {
        // Loop invariant:
        // - &ss[..ss_len] is the current approximation of the root, which has exactly 1 + sizes[i]
        //   bits.
        // - &rs[..rs_len] is the current remainder.
        // - &ws[..ws_len] = ss[..ss_len] ^ (exp - 1)
        // - input_bits = number of truncated bits of the input
        //
        // Since each iteration treats next_bits bits from the root and thus exp * next_bits bits
        // from the input, and we already considered next_bits bits from the input, we now have to
        // take another (exp - 1) * next_bits bits from the input.
        input_bits -= (exp - 1) * next_bits;
        // &rs[..rs_len] = floor(&xs[..xs_len] / 2 ^ input_bits)
        let input_len = usize::exact_from(input_bits >> Limb::LOG_WIDTH);
        let input_bits_rem = input_bits & Limb::WIDTH_MASK;
        shr_helper(rs, &xs[input_len..xs_len], input_bits_rem);
        rs_len = xs_len - input_len;
        if rs[rs_len - 1] == 0 {
            rs_len -= 1;
        }
        // Current buffers: &ss[..ss_len], &ss[..ss_len]
        let mut correction = 0;
        let mut ws_len;
        loop {
            // - Compute S ^ exp in &qs[..qs_len]
            // - W <- S ^ (exp - 1) for the next iteration, and S ^ k = W * S.
            let ss_trimmed = &mut ss[..ss_len];
            let pow_xs = limbs_pow(ss_trimmed, exp - 1);
            ws[..pow_xs.len()].copy_from_slice(&pow_xs);
            ws_len = pow_xs.len();
            let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ws_len, ss_len)];
            limbs_mul_greater_to_out(qs, &ws[..ws_len], ss_trimmed, &mut mul_scratch);
            qs_len = ws_len + ss_len;
            if qs[qs_len - 1] == 0 {
                qs_len -= 1;
            }
            pow_cmp = Greater;
            // if S^k > floor(U/2^input_bits), the root approximation was too large
            let mut need_adjust = qs_len > rs_len;
            if !need_adjust && qs_len == rs_len {
                pow_cmp = limbs_cmp_same_length(&qs[..rs_len], &rs[..rs_len]);
                need_adjust = pow_cmp == Greater;
            }
            if need_adjust {
                assert!(!limbs_sub_limb_in_place(ss_trimmed, 1));
            } else {
                break;
            }
            correction += 1;
        }
        // - Current buffers: &ss[..ss_len], &rs[..rs_len], &qs[..qs_len], &ws[..ws_len]
        // - Sometimes two corrections are needed with logbased_root.
        assert!(correction <= NEEDED_CORRECTIONS);
        assert!(rs_len >= qs_len);
        // next_bits is the number of bits to compute in the next iteration.
        next_bits = sizes[i - 1] - sizes[i];
        // next_len is the lowest limb from the high part of rs, after shift.
        let next_len = usize::exact_from(next_bits >> Limb::LOG_WIDTH);
        let next_bits_rem = next_bits & Limb::WIDTH_MASK;
        input_bits -= next_bits;
        let input_len = usize::exact_from(input_bits >> Limb::LOG_WIDTH);
        let input_bits_rem = input_bits & Limb::WIDTH_MASK;
        // - n_len is the number of limbs in x which contain bits
        // - [input_bits, input_bits + next_bits - 1]
        //
        // n_len = 1 + floor((input_bits + next_bits - 1) / B) - floor(input_bits / B) <= 1 +
        // (input_bits + next_bits - 1) / B - (input_bits - B + 1) / B = 2 + (next_bits - 2) / B,
        // where B is `Limb::WIDTH`.
        //
        // Thus, since n_len is an integer: n_len <= 2 + floor(next_bits / B) <= 2 + next_len.
        let n_len =
            usize::exact_from((input_bits + next_bits - 1) >> Limb::LOG_WIDTH) + 1 - input_len;
        // - Current buffers: &ss[..ss_len], &rs[..rs_len], &ws[..ws_len]
        // - R = R - Q = floor(X / 2 ^ input_bits) - S ^ exp
        if pow_cmp == Equal {
            rs_len = next_len;
            save_2 = 0;
            save_1 = 0;
        } else {
            let rs_trimmed = &mut rs[..rs_len];
            limbs_sub_greater_in_place_left(rs_trimmed, &qs[..qs_len]);
            rs_len -= slice_trailing_zeros(rs_trimmed);
            // first multiply the remainder by 2^next_bits
            let carry = limbs_shl_helper(rs, rs_len, next_len, next_bits_rem);
            rs_len += next_len;
            if carry != 0 {
                rs[rs_len] = carry;
                rs_len += 1;
            }
            save_1 = rs[next_len];
            // we have to save rs[next_len] up to rs[n_len - 1], i.e. 1 or 2 limbs
            if n_len - 1 > next_len {
                save_2 = rs[next_len + 1];
            }
        }
        // - Current buffers: &ss[..ss_len], &rs[..rs_len], &ws[..ws_len]
        // - Now insert bits [input_bits, input_bits + next_bits - 1] from the input X
        shr_helper(rs, &xs[input_len..input_len + n_len], input_bits_rem);
        // Set to zero high bits of rs[next_len]
        rs[next_len].mod_power_of_2_assign(next_bits_rem);
        // Restore corresponding bits
        rs[next_len] |= save_1;
        if n_len - 1 > next_len {
            // The low next_bits bits go in rs[0..next_len] only, since they start by bit 0 in
            // rs[0], so they use at most ceil(next_bits / B) limbs
            rs[next_len + 1] = save_2;
        }
        // - Current buffers: &ss[..ss_len], &rs[..rs_len], &ws[..ws_len]
        // - Compute &ws[..ws_len] = exp * &ss[..ss_len] ^ (exp-1).
        let carry = limbs_slice_mul_limb_in_place(&mut ws[..ws_len], Limb::exact_from(exp));
        ws[ws_len] = carry;
        if carry != 0 {
            ws_len += 1;
        }
        // - Current buffers: &ss[..ss_len], &qs[..qs_len]
        // - Multiply the root approximation by 2 ^ next_bits
        let carry = limbs_shl_helper(ss, ss_len, next_len, next_bits_rem);
        ss_len += next_len;
        if carry != 0 {
            ss[ss_len] = carry;
            ss_len += 1;
        }
        save_1 = ss[next_len];
        // Number of limbs used by next_bits bits, when least significant bit is aligned to least
        // limb
        let b_rem = usize::exact_from((next_bits - 1) >> Limb::LOG_WIDTH) + 1;
        // - Current buffers: &ss[..ss_len], &rs[..rs_len], &ws[..ws_len]
        // - Now divide &rs[..rs_len] by &ws[..ws_len] to get the low part of the root
        if rs_len < ws_len {
            slice_set_zero(&mut ss[..b_rem]);
        } else {
            let mut qs_len = rs_len - ws_len; // Expected quotient size
            if qs_len <= b_rem {
                // Divide only if result is not too big.
                div_helper(qs, &mut rs[..rs_len], &mut ws[..ws_len]);
                if qs[qs_len] != 0 {
                    qs_len += 1;
                }
            } else {
                fail_on_untested_path("limbs_root_to_out_internal, qs_len > b_rem");
            }
            // - Current buffers: &ss[..ss_len], &qs[..qs_len]
            // - Note: &rs[..rs_len]is not needed any more since we'll compute it from scratch at
            //   the end of the loop.
            //
            // The quotient should be smaller than 2 ^ next_bits, since the previous approximation
            // was correctly rounded toward zero.
            if qs_len > b_rem
                || (qs_len == b_rem
                    && (next_bits_rem != 0)
                    && qs[qs_len - 1].significant_bits() > next_bits_rem)
            {
                qs_len = 1;
                while qs_len < b_rem {
                    ss[qs_len - 1] = Limb::MAX;
                    qs_len += 1;
                }
                ss[qs_len - 1] = Limb::low_mask(((next_bits - 1) & Limb::WIDTH_MASK) + 1);
            } else {
                // - Current buffers: &ss[..ss_len], &qs[..qs_len]
                // - Combine sB and q to form sB + q.
                let (ss_lo, ss_hi) = ss.split_at_mut(qs_len);
                ss_lo.copy_from_slice(&qs[..qs_len]);
                slice_set_zero(&mut ss_hi[..b_rem - qs_len]);
            }
        }
        ss[next_len] |= save_1;
        // 8: current buffer: &ss[..ss_len]
        i -= 1;
    }
    // otherwise we have rn > 0, thus the return value is ok
    if !approx || ss[0] <= 1 {
        let mut c = 0;
        loop {
            // - Compute S ^ exp in &qs[..qs_len].
            // - Last iteration: we don't need W anymore.
            let pow_xs = limbs_pow(&ss[..ss_len], exp);
            qs[..pow_xs.len()].copy_from_slice(&pow_xs);
            qs_len = pow_xs.len();
            pow_cmp = Greater;
            let mut need_adjust = qs_len > xs_len;
            if !need_adjust && qs_len == xs_len {
                pow_cmp = limbs_cmp_same_length(&qs[..xs_len], &xs[..xs_len]);
                need_adjust = pow_cmp == Greater;
            }
            if need_adjust {
                assert!(!limbs_sub_limb_in_place(&mut ss[..ss_len], 1));
            } else {
                break;
            }
            c += 1;
        }
        // Sometimes two corrections are needed with log_based_root.
        assert!(c <= NEEDED_CORRECTIONS);
        rs_len = usize::from(pow_cmp != Equal);
        if rs_len != 0 && out_rem_is_some {
            limbs_sub_greater_to_out(rs, &xs[..xs_len], &qs[..qs_len]);
            rs_len = xs_len;
            rs_len -= slice_trailing_zeros(rs);
        }
    }
    rs_len
}

// Returns the size (in limbs) of the remainder.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_rootrem` from `mpn/generic/rootrem.c`, GMP 6.2.1, where `k != 2` and
// `remp` is not `NULL`.
pub_test! {limbs_root_rem_to_out(
    out_root: &mut [Limb],
    out_rem: &mut [Limb],
    xs: &[Limb],
    exp: u64,
) -> usize {
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    assert_ne!(xs[xs_len - 1], 0);
    assert!(exp > 2);
    // (xs_len - 1) / exp > 2 <=> xs_len > 3 * exp <=> (xs_len + 2) / 3 > exp
    limbs_root_to_out_internal(out_root, Some(out_rem), xs, exp, false)
}}

// Returns a non-zero value iff the remainder is non-zero.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_rootrem` from `mpn/generic/rootrem.c`, GMP 6.2.1, where `remp` is
// `NULL`.
pub_test! {limbs_floor_root_to_out(out_root: &mut [Limb], xs: &[Limb], exp: u64) -> bool {
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    assert_ne!(xs[xs_len - 1], 0);
    assert!(exp > 2);
    // (xs_len - 1) / exp > 2 <=> xs_len > 3 * exp <=> (xs_len + 2) / 3 > exp
    let u_exp = usize::exact_from(exp);
    if (xs_len + 2) / 3 > u_exp {
        // Pad xs with exp zero limbs. This will produce an approximate root with one more limb,
        // allowing us to compute the exact integral result.
        let ws_len = xs_len + u_exp;
        let ss_len = (xs_len - 1) / u_exp + 2; // ceil(xs_len / exp) + 1
        let mut scratch = vec![0; ws_len + ss_len];
        // - ws will contain the padded input.
        // - ss is the approximate root of padded input.
        let (ws, ss) = scratch.split_at_mut(ws_len);
        ws[u_exp..].copy_from_slice(xs);
        let rs_len = limbs_root_to_out_internal(ss, None, ws, exp, true);
        // The approximate root S = ss is either the correct root of ss, or 1 too large. Thus,
        // unless the least significant limb of S is 0 or 1, we can deduce the root of xs is S
        // truncated by one limb. (In case xs[0] = 1, we can deduce the root, but not decide whether
        // it is exact or not.)
        out_root[..ss_len - 1].copy_from_slice(&ss[1..]);
        rs_len != 0
    } else {
        limbs_root_to_out_internal(out_root, None, xs, exp, false) != 0
    }
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_floor_root(xs: &[Limb], exp: u64) -> (Vec<Limb>, bool) {
    let mut out = vec![
        0;
        xs.len()
            .div_round(usize::exact_from(exp), Ceiling).0
    ];
    let inexact = limbs_floor_root_to_out(&mut out, xs, exp);
    (out, inexact)
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_root_rem(xs: &[Limb], exp: u64) -> (Vec<Limb>, Vec<Limb>) {
    let mut root_out = vec![
        0;
        xs.len()
            .div_round(usize::exact_from(exp), Ceiling).0
    ];
    let mut rem_out = vec![0; xs.len()];
    let rem_len = limbs_root_rem_to_out(&mut root_out, &mut rem_out, xs, exp);
    rem_out.truncate(rem_len);
    (root_out, rem_out)
}}

impl FloorRoot<u64> for Natural {
    type Output = Natural;

    /// Returns the floor of the $n$th root of a [`Natural`], taking the [`Natural`] by value.
    ///
    /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorRoot;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(999u16).floor_root(3), 9);
    /// assert_eq!(Natural::from(1000u16).floor_root(3), 10);
    /// assert_eq!(Natural::from(1001u16).floor_root(3), 10);
    /// assert_eq!(Natural::from(100000000000u64).floor_root(5), 158);
    /// ```
    fn floor_root(self, exp: u64) -> Natural {
        match exp {
            0 => panic!("Cannot take 0th root"),
            1 => self,
            2 => self.floor_sqrt(),
            exp => match self {
                Natural(Small(x)) => Natural(Small(x.floor_root(exp))),
                Natural(Large(ref xs)) => {
                    Natural::from_owned_limbs_asc(limbs_floor_root(xs, exp).0)
                }
            },
        }
    }
}

impl FloorRoot<u64> for &Natural {
    type Output = Natural;

    /// Returns the floor of the $n$th root of a [`Natural`], taking the [`Natural`] by reference.
    ///
    /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorRoot;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(999u16)).floor_root(3), 9);
    /// assert_eq!((&Natural::from(1000u16)).floor_root(3), 10);
    /// assert_eq!((&Natural::from(1001u16)).floor_root(3), 10);
    /// assert_eq!((&Natural::from(100000000000u64)).floor_root(5), 158);
    /// ```
    fn floor_root(self, exp: u64) -> Natural {
        match exp {
            0 => panic!("Cannot take 0th root"),
            1 => self.clone(),
            2 => self.floor_sqrt(),
            exp => match self {
                Natural(Small(x)) => Natural(Small(x.floor_root(exp))),
                Natural(Large(ref xs)) => {
                    Natural::from_owned_limbs_asc(limbs_floor_root(xs, exp).0)
                }
            },
        }
    }
}

impl FloorRootAssign<u64> for Natural {
    /// Replaces a [`Natural`] with the floor of its $n$th root.
    ///
    /// $x \gets \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorRootAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(999u16);
    /// x.floor_root_assign(3);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(1000u16);
    /// x.floor_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1001u16);
    /// x.floor_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(100000000000u64);
    /// x.floor_root_assign(5);
    /// assert_eq!(x, 158);
    /// ```
    #[inline]
    fn floor_root_assign(&mut self, exp: u64) {
        *self = (&*self).floor_root(exp);
    }
}

impl CeilingRoot<u64> for Natural {
    type Output = Natural;

    /// Returns the ceiling of the $n$th root of a [`Natural`], taking the [`Natural`] by value.
    ///
    /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingRoot;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(999u16).ceiling_root(3), 10);
    /// assert_eq!(Natural::from(1000u16).ceiling_root(3), 10);
    /// assert_eq!(Natural::from(1001u16).ceiling_root(3), 11);
    /// assert_eq!(Natural::from(100000000000u64).ceiling_root(5), 159);
    /// ```
    fn ceiling_root(self, exp: u64) -> Natural {
        match exp {
            0 => panic!("Cannot take 0th root"),
            1 => self,
            2 => self.ceiling_sqrt(),
            exp => match self {
                Natural(Small(x)) => Natural(Small(x.ceiling_root(exp))),
                Natural(Large(ref xs)) => {
                    let (floor_root_limbs, inexact) = limbs_floor_root(xs, exp);
                    let floor_root = Natural::from_owned_limbs_asc(floor_root_limbs);
                    if inexact {
                        floor_root + Natural::ONE
                    } else {
                        floor_root
                    }
                }
            },
        }
    }
}

impl CeilingRoot<u64> for &Natural {
    type Output = Natural;

    /// Returns the ceiling of the $n$th root of a [`Natural`], taking the [`Natural`] by reference.
    ///
    /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingRoot;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(999u16).ceiling_root(3), 10);
    /// assert_eq!(Natural::from(1000u16).ceiling_root(3), 10);
    /// assert_eq!(Natural::from(1001u16).ceiling_root(3), 11);
    /// assert_eq!(Natural::from(100000000000u64).ceiling_root(5), 159);
    /// ```
    fn ceiling_root(self, exp: u64) -> Natural {
        match exp {
            0 => panic!("Cannot take 0th root"),
            1 => self.clone(),
            2 => self.ceiling_sqrt(),
            exp => match self {
                Natural(Small(x)) => Natural(Small(x.ceiling_root(exp))),
                Natural(Large(ref xs)) => {
                    let (floor_root_limbs, inexact) = limbs_floor_root(xs, exp);
                    let floor_root = Natural::from_owned_limbs_asc(floor_root_limbs);
                    if inexact {
                        floor_root + Natural::ONE
                    } else {
                        floor_root
                    }
                }
            },
        }
    }
}

impl CeilingRootAssign<u64> for Natural {
    /// Replaces a [`Natural`] with the ceiling of its $n$th root.
    ///
    /// $x \gets \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingRootAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(999u16);
    /// x.ceiling_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1000u16);
    /// x.ceiling_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1001u16);
    /// x.ceiling_root_assign(3);
    /// assert_eq!(x, 11);
    ///
    /// let mut x = Natural::from(100000000000u64);
    /// x.ceiling_root_assign(5);
    /// assert_eq!(x, 159);
    /// ```
    #[inline]
    fn ceiling_root_assign(&mut self, exp: u64) {
        *self = (&*self).ceiling_root(exp);
    }
}

impl CheckedRoot<u64> for Natural {
    type Output = Natural;

    /// Returns the the $n$th root of a [`Natural`], or `None` if the [`Natural`] is not a perfect
    /// $n$th power. The [`Natural`] is taken by value.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(999u16).checked_root(3).to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(1000u16).checked_root(3).to_debug_string(),
    ///     "Some(10)"
    /// );
    /// assert_eq!(
    ///     Natural::from(1001u16).checked_root(3).to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(100000000000u64)
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(10000000000u64)
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "Some(100)"
    /// );
    /// ```
    fn checked_root(self, exp: u64) -> Option<Natural> {
        match exp {
            0 => panic!("Cannot take 0th root"),
            1 => Some(self),
            2 => self.checked_sqrt(),
            exp => match self {
                Natural(Small(x)) => x.checked_root(exp).map(|x| Natural(Small(x))),
                Natural(Large(ref xs)) => {
                    let (floor_root_limbs, inexact) = limbs_floor_root(xs, exp);
                    let floor_root = Natural::from_owned_limbs_asc(floor_root_limbs);
                    if inexact {
                        None
                    } else {
                        Some(floor_root)
                    }
                }
            },
        }
    }
}

impl CheckedRoot<u64> for &Natural {
    type Output = Natural;

    /// Returns the the $n$th root of a [`Natural`], or `None` if the [`Natural`] is not a perfect
    /// $n$th power. The [`Natural`] is taken by reference.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(999u16)).checked_root(3).to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1000u16)).checked_root(3).to_debug_string(),
    ///     "Some(10)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1001u16)).checked_root(3).to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(100000000000u64))
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10000000000u64))
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "Some(100)"
    /// );
    /// ```
    fn checked_root(self, exp: u64) -> Option<Natural> {
        match exp {
            0 => panic!("Cannot take 0th root"),
            1 => Some(self.clone()),
            2 => self.checked_sqrt(),
            exp => match self {
                Natural(Small(x)) => x.checked_root(exp).map(|x| Natural(Small(x))),
                Natural(Large(ref xs)) => {
                    let (floor_root_limbs, inexact) = limbs_floor_root(xs, exp);
                    let floor_root = Natural::from_owned_limbs_asc(floor_root_limbs);
                    if inexact {
                        None
                    } else {
                        Some(floor_root)
                    }
                }
            },
        }
    }
}

impl RootRem<u64> for Natural {
    type RootOutput = Natural;
    type RemOutput = Natural;

    /// Returns the floor of the $n$th root of a [`Natural`], and the remainder (the difference
    /// between the [`Natural`] and the $n$th power of the floor). The [`Natural`] is taken by
    /// value.
    ///
    /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^n)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RootRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(999u16).root_rem(3).to_debug_string(),
    ///     "(9, 270)"
    /// );
    /// assert_eq!(
    ///     Natural::from(1000u16).root_rem(3).to_debug_string(),
    ///     "(10, 0)"
    /// );
    /// assert_eq!(
    ///     Natural::from(1001u16).root_rem(3).to_debug_string(),
    ///     "(10, 1)"
    /// );
    /// assert_eq!(
    ///     Natural::from(100000000000u64).root_rem(5).to_debug_string(),
    ///     "(158, 1534195232)"
    /// );
    /// ```
    fn root_rem(self, exp: u64) -> (Natural, Natural) {
        match exp {
            0 => panic!("Cannot take 0th root"),
            1 => (self, Natural::ZERO),
            2 => self.sqrt_rem(),
            exp => match self {
                Natural(Small(x)) => {
                    let (root, rem) = x.root_rem(exp);
                    (Natural(Small(root)), Natural(Small(rem)))
                }
                Natural(Large(ref xs)) => {
                    let (root_limbs, rem_limbs) = limbs_root_rem(xs, exp);
                    (
                        Natural::from_owned_limbs_asc(root_limbs),
                        Natural::from_owned_limbs_asc(rem_limbs),
                    )
                }
            },
        }
    }
}

impl RootRem<u64> for &Natural {
    type RootOutput = Natural;
    type RemOutput = Natural;

    /// Returns the floor of the $n$th root of a [`Natural`], and the remainder (the difference
    /// between the [`Natural`] and the $n$th power of the floor). The [`Natural`] is taken by
    /// reference.
    ///
    /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^n)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RootRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(999u16)).root_rem(3).to_debug_string(),
    ///     "(9, 270)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1000u16)).root_rem(3).to_debug_string(),
    ///     "(10, 0)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1001u16)).root_rem(3).to_debug_string(),
    ///     "(10, 1)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(100000000000u64))
    ///         .root_rem(5)
    ///         .to_debug_string(),
    ///     "(158, 1534195232)"
    /// );
    /// ```
    fn root_rem(self, exp: u64) -> (Natural, Natural) {
        match exp {
            0 => panic!("Cannot take 0th root"),
            1 => (self.clone(), Natural::ZERO),
            2 => self.sqrt_rem(),
            exp => match self {
                Natural(Small(x)) => {
                    let (root, rem) = x.root_rem(exp);
                    (Natural(Small(root)), Natural(Small(rem)))
                }
                Natural(Large(ref xs)) => {
                    let (root_limbs, rem_limbs) = limbs_root_rem(xs, exp);
                    (
                        Natural::from_owned_limbs_asc(root_limbs),
                        Natural::from_owned_limbs_asc(rem_limbs),
                    )
                }
            },
        }
    }
}

impl RootAssignRem<u64> for Natural {
    type RemOutput = Natural;

    /// Replaces a [`Natural`] with the floor of its $n$th root, and returns the remainder (the
    /// difference between the original [`Natural`] and the $n$th power of the floor).
    ///
    /// $f(x, n) = x - \lfloor\sqrt\[n\]{x}\rfloor^n$,
    ///
    /// $x \gets \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RootAssignRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(999u16);
    /// assert_eq!(x.root_assign_rem(3), 270);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(1000u16);
    /// assert_eq!(x.root_assign_rem(3), 0);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1001u16);
    /// assert_eq!(x.root_assign_rem(3), 1);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(100000000000u64);
    /// assert_eq!(x.root_assign_rem(5), 1534195232);
    /// assert_eq!(x, 158);
    /// ```
    #[inline]
    fn root_assign_rem(&mut self, exp: u64) -> Natural {
        let rem;
        (*self, rem) = (&*self).root_rem(exp);
        rem
    }
}
