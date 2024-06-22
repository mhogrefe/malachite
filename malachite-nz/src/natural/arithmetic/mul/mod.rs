// Copyright © 2024 Mikhail Hogrefe
//
// Some optimizations contributed by florian1345.
//
// Uses code adopted from the GNU MP Library.
//
//      `mpn_mul` and `TOOM44_OK` contributed to the GNU project by Torbjörn Granlund.
//
//      Copyright © 1991—1994, 1996, 1996-2003, 2005-2007, 2008, 2009, 2010, 2012, 2014, 2019
//      Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::limbs_slice_add_greater_in_place_left;
use crate::natural::arithmetic::add_mul::{
    limbs_slice_add_mul_limb_same_length_in_place_left,
    limbs_slice_add_mul_two_limbs_matching_length_in_place_left,
};
use crate::natural::arithmetic::mul::fft::{
    limbs_mul_greater_to_out_fft, limbs_mul_greater_to_out_fft_scratch_len,
};
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::mul::toom::MUL_TOOM33_THRESHOLD_LIMIT;
use crate::natural::arithmetic::mul::toom::{
    limbs_mul_greater_to_out_toom_22, limbs_mul_greater_to_out_toom_22_scratch_len,
    limbs_mul_greater_to_out_toom_32, limbs_mul_greater_to_out_toom_32_scratch_len,
    limbs_mul_greater_to_out_toom_33, limbs_mul_greater_to_out_toom_33_scratch_len,
    limbs_mul_greater_to_out_toom_42, limbs_mul_greater_to_out_toom_42_scratch_len,
    limbs_mul_greater_to_out_toom_43, limbs_mul_greater_to_out_toom_43_scratch_len,
    limbs_mul_greater_to_out_toom_44, limbs_mul_greater_to_out_toom_44_scratch_len,
    limbs_mul_greater_to_out_toom_53, limbs_mul_greater_to_out_toom_53_scratch_len,
    limbs_mul_greater_to_out_toom_63, limbs_mul_greater_to_out_toom_63_scratch_len,
    limbs_mul_greater_to_out_toom_6h, limbs_mul_greater_to_out_toom_6h_scratch_len,
    limbs_mul_greater_to_out_toom_8h, limbs_mul_greater_to_out_toom_8h_scratch_len,
};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{
    Limb, MUL_FFT_THRESHOLD, MUL_TOOM22_THRESHOLD, MUL_TOOM32_TO_TOOM43_THRESHOLD,
    MUL_TOOM32_TO_TOOM53_THRESHOLD, MUL_TOOM33_THRESHOLD, MUL_TOOM42_TO_TOOM53_THRESHOLD,
    MUL_TOOM42_TO_TOOM63_THRESHOLD, MUL_TOOM44_THRESHOLD, MUL_TOOM6H_THRESHOLD,
    MUL_TOOM8H_THRESHOLD,
};
use alloc::vec::Vec;
use core::cmp::max;
use core::iter::Product;
use core::ops::{Mul, MulAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::traits::Zero;

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
// the limbs of the product of the `Natural`s. `xs` must be as least as long as `ys` and `ys` cannot
// be empty.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is shorter than `ys` or `ys` is empty.
//
// This is equivalent to `mpn_mul` from `mpn/generic/mul.c`, GMP 6.2.1, where `prodp` is returned.
pub_test! {limbs_mul_greater(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let out_len = xs_len + ys_len;
    let mut scratch = vec![0; out_len + limbs_mul_greater_to_out_scratch_len(xs_len, ys_len)];
    let (out, mul_scratch) = scratch.split_at_mut(out_len);
    limbs_mul_greater_to_out(out, xs, ys, mul_scratch);
    scratch.truncate(out_len);
    scratch.shrink_to_fit();
    scratch
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
// the limbs of the product of the `Natural`s. Neither slice can be empty. The length of the
// resulting slice is always the sum of the lengths of the input slices, so it may have trailing
// zeros.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if either slice is empty.
//
// This is equivalent to `mpn_mul` from mpn/generic/mul.c, GMP 6.2.1, where `un` may be less than
// `vn` and `prodp` is returned.
pub_crate_test! {limbs_mul(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    if xs.len() >= ys.len() {
        limbs_mul_greater(xs, ys)
    } else {
        limbs_mul_greater(ys, xs)
    }
}}

pub_crate_test! { limbs_mul_same_length_to_out_scratch_len(len: usize) -> usize {
    assert_ne!(len, 0);
    if len < MUL_TOOM22_THRESHOLD {
        0
    } else if len < MUL_TOOM33_THRESHOLD {
        limbs_mul_greater_to_out_toom_22_scratch_len(
            MUL_TOOM33_THRESHOLD_LIMIT - 1,
            MUL_TOOM33_THRESHOLD_LIMIT - 1,
        )
    } else if len < MUL_TOOM44_THRESHOLD {
        limbs_mul_greater_to_out_toom_33_scratch_len(len, len)
    } else if len < MUL_TOOM6H_THRESHOLD {
        limbs_mul_greater_to_out_toom_44_scratch_len(len, len)
    } else if len < MUL_TOOM8H_THRESHOLD {
        limbs_mul_greater_to_out_toom_6h_scratch_len(len, len)
    } else if len < MUL_FFT_THRESHOLD {
        limbs_mul_greater_to_out_toom_8h_scratch_len(len, len)
    } else {
        limbs_mul_greater_to_out_fft_scratch_len(len, len)
    }
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the `2 * xs.len()` least-significant limbs of the product of the `Natural`s to
// an output slice. The output must be at least as long as `2 * xs.len()`, `xs` must be as long as
// `ys`, and neither slice can be empty. Returns the result limb at index `2 * xs.len() - 1` (which
// may be zero).
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `out` is too short, `xs` and `ys` have different lengths, or either slice is empty.
//
// This is equivalent to `mpn_mul_n` from `mpn/generic/mul_n.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_same_length_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb]
) {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    assert_ne!(len, 0);
    if len < MUL_TOOM22_THRESHOLD {
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if len < MUL_TOOM33_THRESHOLD {
        limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else if len < MUL_TOOM44_THRESHOLD {
        limbs_mul_greater_to_out_toom_33(out, xs, ys, scratch);
    } else if len < MUL_TOOM6H_THRESHOLD {
        limbs_mul_greater_to_out_toom_44(out, xs, ys, scratch);
    } else if len < MUL_TOOM8H_THRESHOLD {
        limbs_mul_greater_to_out_toom_6h(out, xs, ys, scratch);
    } else if len < MUL_FFT_THRESHOLD {
        limbs_mul_greater_to_out_toom_8h(out, xs, ys, scratch);
    } else {
        limbs_mul_greater_to_out_fft(out, xs, ys, scratch);
    }
}}

// This is equivalent to `TOOM44_OK` from `mpn/generic/mul.c`, GMP 6.2.1.
const fn toom44_ok(xs_len: usize, ys_len: usize) -> bool {
    12 + 3 * xs_len < ys_len << 2
}

pub_crate_test! { limbs_mul_greater_to_out_scratch_len(xs_len: usize, ys_len: usize) -> usize {
    assert!(xs_len >= ys_len);
    assert_ne!(ys_len, 0);
    if xs_len == ys_len {
        limbs_mul_same_length_to_out_scratch_len(xs_len)
    } else if ys_len < MUL_TOOM22_THRESHOLD {
        0
    } else if ys_len < MUL_TOOM33_THRESHOLD {
        if xs_len >= 3 * ys_len {
            let two_ys_len = ys_len << 1;
            let three_ys_len = two_ys_len + ys_len;
            let four_ys_len = two_ys_len << 1;
            let mut xs_len = xs_len - two_ys_len;
            while xs_len >= three_ys_len {
                xs_len -= two_ys_len;
            }
            let four_xs_len = xs_len << 2;
            let first_mul_scratch_len =
                limbs_mul_greater_to_out_toom_42_scratch_len(two_ys_len, ys_len);
            let second_mul_scratch_len = if four_xs_len < 5 * ys_len {
                limbs_mul_greater_to_out_toom_22_scratch_len(xs_len, ys_len)
            } else if four_xs_len < 7 * ys_len {
                limbs_mul_greater_to_out_toom_32_scratch_len(xs_len, ys_len)
            } else {
                limbs_mul_greater_to_out_toom_42_scratch_len(xs_len, ys_len)
            };
            max(first_mul_scratch_len, second_mul_scratch_len) + four_ys_len
        } else if 4 * xs_len < 5 * ys_len {
            limbs_mul_greater_to_out_toom_22_scratch_len(xs_len, ys_len)
        } else if 4 * xs_len < 7 * ys_len {
            limbs_mul_greater_to_out_toom_32_scratch_len(xs_len, ys_len)
        } else {
            limbs_mul_greater_to_out_toom_42_scratch_len(xs_len, ys_len)
        }
    } else if (xs_len + ys_len) >> 1 < MUL_FFT_THRESHOLD || 3 * ys_len < MUL_FFT_THRESHOLD {
        if ys_len < MUL_TOOM44_THRESHOLD || !toom44_ok(xs_len, ys_len) {
            // Use ToomX3 variants
            if xs_len << 1 >= 5 * ys_len {
                let two_ys_len = ys_len << 1;
                let four_ys_len = two_ys_len << 1;
                let first_mul_scratch_len = if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                    limbs_mul_greater_to_out_toom_42_scratch_len(two_ys_len, ys_len)
                } else {
                    limbs_mul_greater_to_out_toom_63_scratch_len(two_ys_len, ys_len)
                };
                let mut xs_len = xs_len - two_ys_len;
                while xs_len << 1 >= 5 * ys_len {
                    xs_len -= two_ys_len;
                }
                let second_mul_scratch_len = limbs_mul_to_out_scratch_len(xs_len, ys_len);
                max(first_mul_scratch_len, second_mul_scratch_len) + four_ys_len
            } else if 6 * xs_len < 7 * ys_len {
                limbs_mul_greater_to_out_toom_33_scratch_len(xs_len, ys_len)
            } else if xs_len << 1 < 3 * ys_len {
                if ys_len < MUL_TOOM32_TO_TOOM43_THRESHOLD {
                    limbs_mul_greater_to_out_toom_32_scratch_len(xs_len, ys_len)
                } else {
                    limbs_mul_greater_to_out_toom_43_scratch_len(xs_len, ys_len)
                }
            } else if 6 * xs_len < 11 * ys_len {
                if xs_len << 2 < 7 * ys_len {
                    if ys_len < MUL_TOOM32_TO_TOOM53_THRESHOLD {
                        limbs_mul_greater_to_out_toom_32_scratch_len(xs_len, ys_len)
                    } else {
                        limbs_mul_greater_to_out_toom_53_scratch_len(xs_len, ys_len)
                    }
                } else if ys_len < MUL_TOOM42_TO_TOOM53_THRESHOLD {
                    limbs_mul_greater_to_out_toom_42_scratch_len(xs_len, ys_len)
                } else {
                    limbs_mul_greater_to_out_toom_53_scratch_len(xs_len, ys_len)
                }
            } else if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                limbs_mul_greater_to_out_toom_42_scratch_len(xs_len, ys_len)
            } else {
                limbs_mul_greater_to_out_toom_63_scratch_len(xs_len, ys_len)
            }
        } else if ys_len < MUL_TOOM6H_THRESHOLD {
            limbs_mul_greater_to_out_toom_44_scratch_len(xs_len, ys_len)
        } else if ys_len < MUL_TOOM8H_THRESHOLD {
            limbs_mul_greater_to_out_toom_6h_scratch_len(xs_len, ys_len)
        } else {
            limbs_mul_greater_to_out_toom_8h_scratch_len(xs_len, ys_len)
        }
    } else {
        limbs_mul_greater_to_out_fft_scratch_len(xs_len, ys_len)
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. The output must be at least as long as `xs.len() + ys.len()`, `xs` must be as least as
// long as `ys`, and `ys` cannot be empty. Returns the result limb at index `xs.len() + ys.len() -
// 1` (which may be zero).
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `out` is too short, `xs` is shorter than `ys`, or `ys` is empty.
//
// This is equivalent to `mpn_mul` from `mpn/generic/mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb]
) -> Limb {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert_ne!(ys_len, 0);
    assert!(out.len() >= xs_len + ys_len);
    if xs_len == ys_len {
        limbs_mul_same_length_to_out(out, xs, ys, scratch);
    } else if ys_len < MUL_TOOM22_THRESHOLD {
        // Plain schoolbook multiplication. Unless xs_len is very large, or else if
        // `limbs_mul_same_length_to_out` applies, perform basecase multiply directly.
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if ys_len < MUL_TOOM33_THRESHOLD {
        if xs_len >= 3 * ys_len {
            let two_ys_len = ys_len << 1;
            let three_ys_len = two_ys_len + ys_len;
            let four_ys_len = two_ys_len << 1;
            let (scratch, mul_scratch) = scratch.split_at_mut(four_ys_len);
            limbs_mul_greater_to_out_toom_42(out, &xs[..two_ys_len], ys, mul_scratch);
            let mut xs = &xs[two_ys_len..];
            let mut out_offset = two_ys_len;
            while xs.len() >= three_ys_len {
                let out = &mut out[out_offset..];
                let (xs_lo, xs_hi) = xs.split_at(two_ys_len);
                limbs_mul_greater_to_out_toom_42(scratch, xs_lo, ys, mul_scratch);
                let (scratch_lo, scratch_hi) = scratch.split_at(ys_len);
                out[ys_len..three_ys_len].copy_from_slice(&scratch_hi[..two_ys_len]);
                assert!(!limbs_slice_add_greater_in_place_left(out, scratch_lo));
                xs = xs_hi;
                out_offset += two_ys_len;
            }
            let xs_len = xs.len();
            let out = &mut out[out_offset..];
            // ys_len <= xs_len < 3 * ys_len
            let four_xs_len = xs_len << 2;
            if four_xs_len < 5 * ys_len {
                limbs_mul_greater_to_out_toom_22(scratch, xs, ys, mul_scratch);
            } else if four_xs_len < 7 * ys_len {
                limbs_mul_greater_to_out_toom_32(scratch, xs, ys, mul_scratch);
            } else {
                limbs_mul_greater_to_out_toom_42(scratch, xs, ys, mul_scratch);
            }
            let (scratch_lo, scratch_hi) = scratch.split_at(ys_len);
            out[ys_len..ys_len + xs_len].copy_from_slice(&scratch_hi[..xs_len]);
            assert!(!limbs_slice_add_greater_in_place_left(out, scratch_lo));
        } else if 4 * xs_len < 5 * ys_len {
            limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
        } else if 4 * xs_len < 7 * ys_len {
            limbs_mul_greater_to_out_toom_32(out, xs, ys, scratch);
        } else {
            limbs_mul_greater_to_out_toom_42(out, xs, ys, scratch);
        }
    } else if (xs_len + ys_len) >> 1 < MUL_FFT_THRESHOLD || 3 * ys_len < MUL_FFT_THRESHOLD {
        // Handle the largest operands that are not in the FFT range. The 2nd condition makes very
        // unbalanced operands avoid the FFT code (except perhaps as coefficient products of the
        // Toom code).
        if ys_len < MUL_TOOM44_THRESHOLD || !toom44_ok(xs_len, ys_len) {
            // Use ToomX3 variants
            if xs_len << 1 >= 5 * ys_len {
                let two_ys_len = ys_len << 1;
                let four_ys_len = two_ys_len << 1;
                let (scratch, mul_scratch) = scratch.split_at_mut(four_ys_len);
                let (xs_lo, mut xs) = xs.split_at(two_ys_len);
                if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                    limbs_mul_greater_to_out_toom_42(out, xs_lo, ys, mul_scratch);
                } else {
                    limbs_mul_greater_to_out_toom_63(out, xs_lo, ys, mul_scratch);
                }
                let mut out_offset = two_ys_len;
                // xs_len >= 2.5 * ys_len
                while xs.len() << 1 >= 5 * ys_len {
                    let out = &mut out[out_offset..];
                    let (xs_lo, xs_hi) = xs.split_at(two_ys_len);
                    if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                        limbs_mul_greater_to_out_toom_42(scratch, xs_lo, ys, mul_scratch);
                    } else {
                        limbs_mul_greater_to_out_toom_63(scratch, xs_lo, ys, mul_scratch);
                    }
                    let (scratch_lo, scratch_hi) = scratch.split_at(ys_len);
                    out[ys_len..ys_len + two_ys_len].copy_from_slice(&scratch_hi[..two_ys_len]);
                    assert!(!limbs_slice_add_greater_in_place_left(out, scratch_lo));
                    xs = xs_hi;
                    out_offset += two_ys_len;
                }
                let xs_len = xs.len();
                let out = &mut out[out_offset..];
                // ys_len / 2 <= xs_len < 2.5 * ys_len
                limbs_mul_to_out(scratch, xs, ys, mul_scratch);
                let (scratch_lo, scratch_hi) = scratch.split_at(ys_len);
                out[ys_len..xs_len + ys_len].copy_from_slice(&scratch_hi[..xs_len]);
                assert!(!limbs_slice_add_greater_in_place_left(out, scratch_lo));
            } else if 6 * xs_len < 7 * ys_len {
                limbs_mul_greater_to_out_toom_33(out, xs, ys, scratch);
            } else if xs_len << 1 < 3 * ys_len {
                if ys_len < MUL_TOOM32_TO_TOOM43_THRESHOLD {
                    limbs_mul_greater_to_out_toom_32(out, xs, ys, scratch);
                } else {
                    limbs_mul_greater_to_out_toom_43(out, xs, ys, scratch);
                }
            } else if 6 * xs_len < 11 * ys_len {
                if xs_len << 2 < 7 * ys_len {
                    if ys_len < MUL_TOOM32_TO_TOOM53_THRESHOLD {
                        limbs_mul_greater_to_out_toom_32(out, xs, ys, scratch);
                    } else {
                        limbs_mul_greater_to_out_toom_53(out, xs, ys, scratch);
                    }
                } else if ys_len < MUL_TOOM42_TO_TOOM53_THRESHOLD {
                    limbs_mul_greater_to_out_toom_42(out, xs, ys, scratch);
                } else {
                    limbs_mul_greater_to_out_toom_53(out, xs, ys, scratch);
                }
            } else if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                limbs_mul_greater_to_out_toom_42(out, xs, ys, scratch);
            } else {
                limbs_mul_greater_to_out_toom_63(out, xs, ys, scratch);
            }
        } else if ys_len < MUL_TOOM6H_THRESHOLD {
            limbs_mul_greater_to_out_toom_44(out, xs, ys, scratch);
        } else if ys_len < MUL_TOOM8H_THRESHOLD {
            limbs_mul_greater_to_out_toom_6h(out, xs, ys, scratch);
        } else {
            limbs_mul_greater_to_out_toom_8h(out, xs, ys, scratch);
        }
    } else {
        limbs_mul_greater_to_out_fft(out, xs, ys, scratch);
    }
    out[xs_len + ys_len - 1]
}}

pub_crate_test! {limbs_mul_to_out_scratch_len(xs_len: usize, ys_len: usize) -> usize {
    if xs_len >= ys_len {
        limbs_mul_greater_to_out_scratch_len(xs_len, ys_len)
    } else {
        limbs_mul_greater_to_out_scratch_len(ys_len, xs_len)
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. The output must be at least as long as `xs.len() + ys.len()`, and neither slice can be
// empty. Returns the result limb at index `xs.len() + ys.len() - 1` (which may be zero).
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `out` is too short or either slice is empty.
//
// This is equivalent to `mpn_mul` from `mpn/generic/mul.c`, GMP 6.2.1, where `un` may be less than
// `vn`.
pub_crate_test! {limbs_mul_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb]
) -> Limb {
    if xs.len() >= ys.len() {
        limbs_mul_greater_to_out(out, xs, ys, scratch)
    } else {
        limbs_mul_greater_to_out(out, ys, xs, scratch)
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
// slice. The output must be at least as long as `xs.len() + ys.len()`, `xs` must be as least as
// long as `ys`, and `ys` cannot be empty. Returns the result limb at index `xs.len() + ys.len() -
// 1` (which may be zero).
//
// This uses the basecase, quadratic, schoolbook algorithm, and it is most critical code for
// multiplication. All multiplies rely on this, both small and huge. Small ones arrive here
// immediately, and huge ones arrive here as this is the base case for Karatsuba's recursive
// algorithm.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `out` is too short, `xs` is shorter than `ys`, or `ys` is empty.
//
// This is equivalent to `mpn_mul_basecase` from `mpn/generic/mul_basecase.c`, GMP 6.2.1.
pub_crate_test! {limbs_mul_greater_to_out_basecase(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    assert!(out.len() >= xs_len + ys_len);
    let out = &mut out[..(xs_len + ys_len)];
    // We first multiply by the low order limb. This result can be stored, not added, to out.
    out[xs_len] = limbs_mul_limb_to_out(out, xs, ys[0]);
    // Now accumulate the product of xs and the next higher limb from ys.
    let window_size = xs_len + 1;
    let mut i = 1;
    let max = ys_len - 1;
    while i < max {
        let (out_last, out_init) = out[i..=i + window_size].split_last_mut().unwrap();
        *out_last = limbs_slice_add_mul_two_limbs_matching_length_in_place_left(
            out_init,
            xs,
            [ys[i], ys[i + 1]],
        );
        i += 2;
    }
    if i <= max {
        let (out_last, out_init) = out[i..i + window_size].split_last_mut().unwrap();
        *out_last = limbs_slice_add_mul_limb_same_length_in_place_left(out_init, xs, ys[i]);
    }
}}

impl Mul<Natural> for Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ONE * Natural::from(123u32), 123);
    /// assert_eq!(Natural::from(123u32) * Natural::ZERO, 0);
    /// assert_eq!(Natural::from(123u32) * Natural::from(456u32), 56088);
    /// assert_eq!(
    ///     (Natural::from_str("123456789000").unwrap()
    ///         * Natural::from_str("987654321000").unwrap())
    ///     .to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    #[inline]
    fn mul(mut self, other: Natural) -> Natural {
        self *= other;
        self
    }
}

impl<'a> Mul<&'a Natural> for Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ONE * &Natural::from(123u32), 123);
    /// assert_eq!(Natural::from(123u32) * &Natural::ZERO, 0);
    /// assert_eq!(Natural::from(123u32) * &Natural::from(456u32), 56088);
    /// assert_eq!(
    ///     (Natural::from_str("123456789000").unwrap()
    ///         * &Natural::from_str("987654321000").unwrap())
    ///         .to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    #[inline]
    fn mul(mut self, other: &'a Natural) -> Natural {
        self *= other;
        self
    }
}

impl<'a> Mul<Natural> for &'a Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(&Natural::ONE * Natural::from(123u32), 123);
    /// assert_eq!(&Natural::from(123u32) * Natural::ZERO, 0);
    /// assert_eq!(&Natural::from(123u32) * Natural::from(456u32), 56088);
    /// assert_eq!(
    ///     (&Natural::from_str("123456789000").unwrap()
    ///         * Natural::from_str("987654321000").unwrap())
    ///     .to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    #[inline]
    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

impl<'a, 'b> Mul<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(&Natural::ONE * &Natural::from(123u32), 123);
    /// assert_eq!(&Natural::from(123u32) * &Natural::ZERO, 0);
    /// assert_eq!(&Natural::from(123u32) * &Natural::from(456u32), 56088);
    /// assert_eq!(
    ///     (&Natural::from_str("123456789000").unwrap()
    ///         * &Natural::from_str("987654321000").unwrap())
    ///         .to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    fn mul(self, other: &'a Natural) -> Natural {
        match (self, other) {
            (Natural(Small(x)), y) => y.mul_limb_ref(*x),
            (x, Natural(Small(y))) => x.mul_limb_ref(*y),
            (Natural(Large(ref xs)), Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_mul(xs, ys))
            }
        }
    }
}

impl MulAssign<Natural> for Natural {
    /// Multiplies a [`Natural`] by a [`Natural`] in place, taking the [`Natural`] on the right-hand
    /// side by value.
    ///
    /// $$
    /// x \gets = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ONE;
    /// x *= Natural::from_str("1000").unwrap();
    /// x *= Natural::from_str("2000").unwrap();
    /// x *= Natural::from_str("3000").unwrap();
    /// x *= Natural::from_str("4000").unwrap();
    /// assert_eq!(x.to_string(), "24000000000000");
    /// ```
    fn mul_assign(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (Natural(Small(x)), _) => {
                other.mul_assign_limb(*x);
                *self = other;
            }
            (_, Natural(Small(y))) => self.mul_assign_limb(*y),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                *xs = limbs_mul(xs, ys);
                self.trim();
            }
        }
    }
}

impl<'a> MulAssign<&'a Natural> for Natural {
    /// Multiplies a [`Natural`] by a [`Natural`] in place, taking the [`Natural`] on the right-hand
    /// side by reference.
    ///
    /// $$
    /// x \gets = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ONE;
    /// x *= &Natural::from_str("1000").unwrap();
    /// x *= &Natural::from_str("2000").unwrap();
    /// x *= &Natural::from_str("3000").unwrap();
    /// x *= &Natural::from_str("4000").unwrap();
    /// assert_eq!(x.to_string(), "24000000000000");
    /// ```
    fn mul_assign(&mut self, other: &'a Natural) {
        match (&mut *self, other) {
            (Natural(Small(x)), _) => *self = other.mul_limb_ref(*x),
            (_, Natural(Small(y))) => self.mul_assign_limb(*y),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                *xs = limbs_mul(xs, ys);
                self.trim();
            }
        }
    }
}

impl Product for Natural {
    /// Multiplies together all the [`Natural`]s in an iterator.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `Natural::sum(xs.map(Natural::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Product;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::product(vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().into_iter()),
    ///     210
    /// );
    /// ```
    fn product<I>(xs: I) -> Natural
    where
        I: Iterator<Item = Natural>,
    {
        let mut stack = Vec::new();
        for (i, x) in xs.enumerate().map(|(i, x)| (i + 1, x)) {
            if x == 0 {
                return Natural::ZERO;
            }
            let mut p = x;
            for _ in 0..i.trailing_zeros() {
                p *= stack.pop().unwrap();
            }
            stack.push(p);
        }
        let mut p = Natural::ONE;
        for x in stack.into_iter().rev() {
            p *= x;
        }
        p
    }
}

impl<'a> Product<&'a Natural> for Natural {
    /// Multiplies together all the [`Natural`]s in an iterator of [`Natural`] references.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `Natural::sum(xs.map(Natural::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Product;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::product(vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().iter()),
    ///     210
    /// );
    /// ```
    fn product<I>(xs: I) -> Natural
    where
        I: Iterator<Item = &'a Natural>,
    {
        let mut stack = Vec::new();
        for (i, x) in xs.enumerate().map(|(i, x)| (i + 1, x)) {
            if *x == 0 {
                return Natural::ZERO;
            }
            let mut p = x.clone();
            for _ in 0..i.trailing_zeros() {
                p *= stack.pop().unwrap();
            }
            stack.push(p);
        }
        let mut p = Natural::ONE;
        for x in stack.into_iter().rev() {
            p *= x;
        }
        p
    }
}

/// Code for the Schönhage-Strassen (FFT) multiplication algorithm.
pub mod fft;
/// Code for multiplying a many-limbed [`Natural`] by a single [limb](crate#limbs).
pub mod limb;
/// Code for computing only the lowest [limbs](crate#limbs) of the product of two [`Natural`]s.
pub mod mul_low;
/// Code for multiplying two [`Natural`]s modulo one less than a large power of 2; used by the
/// Schönhage-Strassen algorithm.
pub mod mul_mod;
/// Code for evaluating polynomials at various points; used in Toom-Cook multiplication.
pub mod poly_eval;
/// Code for reconstructing polynomials from their values at various points; used in Toom-Cook
/// multiplication.
pub mod poly_interpolate;
#[cfg(feature = "test_build")]
pub mod product_of_limbs;
#[cfg(not(feature = "test_build"))]
pub(crate) mod product_of_limbs;
/// Code for Toom-Cook multiplication.
pub mod toom;
