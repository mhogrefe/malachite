use malachite_base::misc::Max;
use malachite_base::num::PrimitiveInteger;
use natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_slice_add_greater_in_place_left,
};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::add_mul_limb::mpn_addmul_1;
use natural::arithmetic::mul::fft::{mpn_fft_best_k, mpn_fft_next_size, MUL_FFT_THRESHOLD};
use natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22, _limbs_mul_greater_to_out_toom_32,
    _limbs_mul_greater_to_out_toom_33, _limbs_mul_greater_to_out_toom_33_scratch_size,
    _limbs_mul_greater_to_out_toom_42, _limbs_mul_greater_to_out_toom_43,
    _limbs_mul_greater_to_out_toom_44, _limbs_mul_greater_to_out_toom_44_scratch_size,
    _limbs_mul_greater_to_out_toom_53, _limbs_mul_greater_to_out_toom_63,
    _limbs_mul_greater_to_out_toom_6h, _limbs_mul_greater_to_out_toom_6h_scratch_size,
    _limbs_mul_greater_to_out_toom_8h, _limbs_mul_greater_to_out_toom_8h_scratch_size,
    _limbs_mul_same_length_to_out_toom_6h_scratch_size,
    _limbs_mul_same_length_to_out_toom_8h_scratch_size, MUL_TOOM22_THRESHOLD,
    MUL_TOOM32_TO_TOOM43_THRESHOLD, MUL_TOOM32_TO_TOOM53_THRESHOLD, MUL_TOOM33_THRESHOLD,
    MUL_TOOM33_THRESHOLD_LIMIT, MUL_TOOM42_TO_TOOM53_THRESHOLD, MUL_TOOM42_TO_TOOM63_THRESHOLD,
    MUL_TOOM44_THRESHOLD, MUL_TOOM6H_THRESHOLD, MUL_TOOM8H_THRESHOLD,
};
use natural::arithmetic::mul_limb::limbs_mul_limb_to_out;
use natural::arithmetic::sub::limbs_sub_same_length_to_out;
use natural::Natural::{self, Large, Small};
use platform::Limb;
use std::ops::{Mul, MulAssign};

// This doesn't use `chunks_exact` because sometimes `xs_last` is longer than `n`.
macro_rules! split_into_chunks {
    ($xs: expr, $n: expr, $last_chunk_size: ident, [$($xs_i: ident),*], $xs_last: ident) => {
        let remainder = &$xs;
        $(
            let ($xs_i, remainder) = remainder.split_at($n);
        )*
        let $xs_last = remainder;
        let $last_chunk_size = $xs_last.len();
    }
}

// This doesn't use `chunks_exact_mut` because sometimes `xs_last` is longer than `n`.
macro_rules! split_into_chunks_mut {
    ($xs: expr, $n: expr, [$($xs_i: ident),*], $xs_last: ident) => {
        let remainder = &mut $xs[..];
        $(
            let ($xs_i, remainder) = remainder.split_at_mut($n);
        )*
        let $xs_last = remainder;
    }
}

//TODO use better algorithms

//TODO test
// docs preserved
// Inputs are ap and bp; output is rp, with ap, bp and rp all the same length, computation is mod
// B ^ rn - 1, and values are semi-normalised; zero is represented as either 0 or B ^ n - 1. Needs a
// scratch of 2rn limbs at tp.
// mpn_bc_mulmod_bnm1 from mpn/generic/mulmod_bnm1.c
pub fn mpn_bc_mulmod_bnm1(rp: &mut [Limb], ap: &[Limb], bp: &[Limb], tp: &mut [Limb]) {
    let rn = ap.len();
    assert_ne!(rn, 0);
    limbs_mul_same_length_to_out(tp, ap, bp);
    let cy = if limbs_add_same_length_to_out(rp, &tp[..rn], &tp[rn..2 * rn]) {
        1
    } else {
        0
    };
    // If cy == 1, then the value of rp is at most B ^ rn - 2, so there can be no overflow when
    // adding in the carry.
    limbs_slice_add_limb_in_place(&mut rp[..rn], cy);
}

//TODO test
// docs preserved
// Inputs are ap and bp; output is rp, with ap, bp and rp all the same length, in semi-normalised
// representation, computation is mod B ^ rn + 1. Needs a scratch area of 2rn + 2 limbs at tp.
// Output is normalised.
// mpn_bc_mulmod_bnp1 from mpn/generic/mulmod_bnm1.c
pub fn mpn_bc_mulmod_bnp1(rp: &mut [Limb], ap: &[Limb], bp: &[Limb], tp: &mut [Limb]) {
    let rn = ap.len() - 1;
    assert_ne!(rn, 0);
    limbs_mul_same_length_to_out(tp, ap, bp);
    assert_eq!(tp[2 * rn + 1], 0);
    assert!(tp[2 * rn] < Limb::MAX);
    let cy = tp[2 * rn]
        + if limbs_sub_same_length_to_out(rp, &tp[..rn], &tp[rn..2 * rn]) {
            1
        } else {
            0
        };
    rp[rn] = 0;
    limbs_slice_add_limb_in_place(&mut rp[..=rn], cy);
}

//TODO PASTE A

//TODO tune
const MULMOD_BNM1_THRESHOLD: usize = 13;
const MUL_FFT_MODF_THRESHOLD: usize = MUL_TOOM33_THRESHOLD * 3;

//TODO test
// checked
// docs preserved
// mpn_mulmod_bnm1_next_size from mpn/generic/mulmod_bnm1.c
pub fn mpn_mulmod_bnm1_next_size(n: usize) -> usize {
    if n < MULMOD_BNM1_THRESHOLD {
        return n;
    } else if n < 4 * (MULMOD_BNM1_THRESHOLD - 1) + 1 {
        return (n + (2 - 1)) & 2_usize.wrapping_neg();
    } else if n < 8 * (MULMOD_BNM1_THRESHOLD - 1) + 1 {
        return (n + (4 - 1)) & 4_usize.wrapping_neg();
    }
    let nh = (n + 1) >> 1;
    if nh < MUL_FFT_MODF_THRESHOLD {
        (n + (8 - 1)) & 8_usize.wrapping_neg()
    } else {
        2 * mpn_fft_next_size(nh, mpn_fft_best_k(nh, 0))
    }
}

//TODO test
// checked
// docs preserved
// mpn_mulmod_bnm1_itch from gmp-impl.h
pub fn mpn_mulmod_bnm1_itch(rn: usize, an: usize, bn: usize) -> usize {
    let n = rn >> 1;
    rn + 4
        + if an > n {
            if bn > n {
                rn
            } else {
                n
            }
        } else {
            0
        }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. The output must be at least as long as `xs.len() + ys.len()`, `xs` must be as least as
/// long as `ys`, and `ys` cannot be empty. Returns the result limb at index
/// `xs.len() + ys.len() - 1` (which may be zero).
///
/// This uses the basecase, quadratic, schoolbook algorithm, and it is most critical code for
/// multiplication. All multiplies rely on this, both small and huge. Small ones arrive here
/// immediately, and huge ones arrive here as this is the base case for Karatsuba's recursive
/// algorithm.
///
/// Time: worst case O(n<sup>2</sup>)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// Panics if `out` is too short, `xs` is shorter than `ys`, or `ys` is empty.
///
/// This is mpn_mul_basecase from mpn/generic/mul_basecase.c.
pub fn _limbs_mul_greater_to_out_basecase(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    assert!(out.len() >= xs_len + ys_len);
    // We first multiply by the low order limb. This result can be stored, not added, to out.
    out[xs_len] = limbs_mul_limb_to_out(out, xs, ys[0]);
    // Now accumulate the product of xs and the next higher limb from ys.
    for i in 1..ys_len {
        let out = &mut out[i..];
        out[xs_len] = mpn_addmul_1(out, xs, ys[i]);
    }
}

//TODO test
// multiply natural numbers.
// mpn_mul_n from mpn/generic/mul_n.c
pub fn limbs_mul_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    assert_ne!(len, 0);

    if len < MUL_TOOM22_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if len < MUL_TOOM33_THRESHOLD {
        // TODO once const fn is stable, make this
        // _limbs_mul_greater_to_out_toom_22_scratch_size(MUL_TOOM33_THRESHOLD_LIMIT - 1)

        // Allocate workspace of fixed size on stack: fast!
        let scratch = &mut [0; 2 * (MUL_TOOM33_THRESHOLD_LIMIT - 1 + Limb::WIDTH as usize)];
        assert!(MUL_TOOM33_THRESHOLD <= MUL_TOOM33_THRESHOLD_LIMIT);
        _limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else if len < MUL_TOOM44_THRESHOLD {
        let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(len)];
        _limbs_mul_greater_to_out_toom_33(out, xs, ys, &mut scratch);
    } else if len < MUL_TOOM6H_THRESHOLD {
        let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(len)];
        _limbs_mul_greater_to_out_toom_44(out, xs, ys, &mut scratch);
    } else if len < MUL_TOOM8H_THRESHOLD {
        let mut scratch = vec![0; _limbs_mul_same_length_to_out_toom_6h_scratch_size(len)];
        _limbs_mul_greater_to_out_toom_6h(out, xs, ys, &mut scratch);
    } else if len < MUL_FFT_THRESHOLD {
        let mut scratch = vec![0; _limbs_mul_same_length_to_out_toom_8h_scratch_size(len)];
        _limbs_mul_greater_to_out_toom_8h(out, xs, ys, &mut scratch);
    } else {
        // The current FFT code allocates its own space. That should probably change.
        //TODO mpn_fft_mul (out, xs, len, ys, len);
        _limbs_mul_greater_to_out_basecase(out, xs, ys);
    }
}

// This is TOOM44_OK from mpn/generic/mul.c.
fn toom44_ok(xs_len: usize, ys_len: usize) -> bool {
    12 + 3 * xs_len < 4 * ys_len
}

// Multiply two natural numbers.
// This is mpn_mul from mpn/generic/mul.c.
pub fn limbs_mul_greater_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> Limb {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert_ne!(ys_len, 0);

    if xs_len == ys_len {
        //TODO if xs as *const [Limb] == ys as *const [Limb] {
        //TODO     mpn_sqr(out, xs, xs_len);
        //TODO } else {
        //TODO     mpn_mul_n(out, xs, ys);
        //TODO }
        limbs_mul_same_length_to_out(out, xs, ys);
    } else if ys_len < MUL_TOOM22_THRESHOLD {
        // Plain schoolbook multiplication. Unless xs_len is very large, or else if
        // `limbs_mul_same_length_to_out` applies, perform basecase multiply directly.
        _limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if ys_len < MUL_TOOM33_THRESHOLD {
        let toom_x2_scratch_size = 9 * ys_len / 2 + Limb::WIDTH as usize * 2;
        let mut scratch = vec![0; toom_x2_scratch_size];
        if xs_len >= 3 * ys_len {
            _limbs_mul_greater_to_out_toom_42(out, &xs[..ys_len << 1], ys, &mut scratch);
            let two_ys_len = ys_len << 1;
            let three_ys_len = two_ys_len + ys_len;
            // The maximum `scratch2` usage is for the `_limbs_mul_greater_to_out_toom_x2` result.
            let mut scratch2 = vec![0; two_ys_len << 1];
            let mut xs = &xs[two_ys_len..];
            let mut out_offset = two_ys_len;
            while xs.len() >= three_ys_len {
                let out = &mut out[out_offset..];
                let (xs_lo, xs_hi) = xs.split_at(two_ys_len);
                _limbs_mul_greater_to_out_toom_42(&mut scratch2, xs_lo, ys, &mut scratch);
                let (scratch2_lo, scratch2_hi) = scratch2.split_at(ys_len);
                out[ys_len..three_ys_len].copy_from_slice(&scratch2_hi[..two_ys_len]);
                assert!(!limbs_slice_add_greater_in_place_left(out, scratch2_lo));
                xs = xs_hi;
                out_offset += two_ys_len;
            }
            let xs_len = xs.len();
            let out = &mut out[out_offset..];
            // ys_len <= xs_len < 3 * ys_len
            if 4 * xs_len < 5 * ys_len {
                _limbs_mul_greater_to_out_toom_22(&mut scratch2, xs, ys, &mut scratch);
            } else if 4 * xs_len < 7 * ys_len {
                _limbs_mul_greater_to_out_toom_32(&mut scratch2, xs, ys, &mut scratch);
            } else {
                _limbs_mul_greater_to_out_toom_42(&mut scratch2, xs, ys, &mut scratch);
            }
            let (scratch2_lo, scratch2_hi) = scratch2.split_at(ys_len);
            out[ys_len..ys_len + xs_len].copy_from_slice(&scratch2_hi[..xs_len]);
            assert!(!limbs_slice_add_greater_in_place_left(out, scratch2_lo));
        } else if 4 * xs_len < 5 * ys_len {
            _limbs_mul_greater_to_out_toom_22(out, xs, ys, &mut scratch);
        } else if 4 * xs_len < 7 * ys_len {
            _limbs_mul_greater_to_out_toom_32(out, xs, ys, &mut scratch);
        } else {
            _limbs_mul_greater_to_out_toom_42(out, xs, ys, &mut scratch);
        }
    } else if (xs_len + ys_len) >> 1 < MUL_FFT_THRESHOLD || 3 * ys_len < MUL_FFT_THRESHOLD {
        // Handle the largest operands that are not in the FFT range. The 2nd condition makes very
        // unbalanced operands avoid the FFT code (except perhaps as coefficient products of the
        // Toom code).
        if ys_len < MUL_TOOM44_THRESHOLD || !toom44_ok(xs_len, ys_len) {
            // Use ToomX3 variants
            let toom_x3_scratch_size = 4 * ys_len + Limb::WIDTH as usize;
            let mut scratch = vec![0; toom_x3_scratch_size];
            if 2 * xs_len >= 5 * ys_len {
                // The maximum scratch2 usage is for the `limbs_mul_to_out` result.
                let mut scratch2 = vec![0; 7 * ys_len >> 1];
                if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                    _limbs_mul_greater_to_out_toom_42(out, &xs[..2 * ys_len], ys, &mut scratch);
                } else {
                    _limbs_mul_greater_to_out_toom_63(out, &xs[..2 * ys_len], ys, &mut scratch);
                }
                let two_ys_len = ys_len << 1;
                let mut xs = &xs[two_ys_len..];
                let mut out_offset = two_ys_len;
                // xs_len >= 2.5 * ys_len
                while 2 * xs.len() >= 5 * ys_len {
                    let out = &mut out[out_offset..];
                    let (xs_lo, xs_hi) = xs.split_at(two_ys_len);
                    if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                        _limbs_mul_greater_to_out_toom_42(&mut scratch2, xs_lo, ys, &mut scratch);
                    } else {
                        _limbs_mul_greater_to_out_toom_63(&mut scratch2, xs_lo, ys, &mut scratch);
                    }
                    let (scratch2_lo, scratch2_hi) = scratch2.split_at(ys_len);
                    out[ys_len..ys_len + two_ys_len].copy_from_slice(&scratch2_hi[..two_ys_len]);
                    assert!(!limbs_slice_add_greater_in_place_left(out, scratch2_lo));
                    xs = xs_hi;
                    out_offset += two_ys_len;
                }
                let xs_len = xs.len();
                let out = &mut out[out_offset..];
                // ys_len / 2 <= xs_len < 2.5 * ys_len
                limbs_mul_to_out(&mut scratch2, xs, ys);
                let (scratch2_lo, scratch2_hi) = scratch2.split_at(ys_len);
                out[ys_len..xs_len + ys_len].copy_from_slice(&scratch2_hi[..xs_len]);
                assert!(!limbs_slice_add_greater_in_place_left(out, scratch2_lo));
            } else {
                if 6 * xs_len < 7 * ys_len {
                    _limbs_mul_greater_to_out_toom_33(out, xs, ys, &mut scratch);
                } else if 2 * xs_len < 3 * ys_len {
                    if ys_len < MUL_TOOM32_TO_TOOM43_THRESHOLD {
                        _limbs_mul_greater_to_out_toom_32(out, xs, ys, &mut scratch);
                    } else {
                        _limbs_mul_greater_to_out_toom_43(out, xs, ys, &mut scratch);
                    }
                } else if 6 * xs_len < 11 * ys_len {
                    if 4 * xs_len < 7 * ys_len {
                        if ys_len < MUL_TOOM32_TO_TOOM53_THRESHOLD {
                            _limbs_mul_greater_to_out_toom_32(out, xs, ys, &mut scratch);
                        } else {
                            _limbs_mul_greater_to_out_toom_53(out, xs, ys, &mut scratch);
                        }
                    } else {
                        if ys_len < MUL_TOOM42_TO_TOOM53_THRESHOLD {
                            _limbs_mul_greater_to_out_toom_42(out, xs, ys, &mut scratch);
                        } else {
                            _limbs_mul_greater_to_out_toom_53(out, xs, ys, &mut scratch);
                        }
                    }
                } else {
                    if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                        _limbs_mul_greater_to_out_toom_42(out, xs, ys, &mut scratch);
                    } else {
                        _limbs_mul_greater_to_out_toom_63(out, xs, ys, &mut scratch);
                    }
                }
            }
        } else {
            if ys_len < MUL_TOOM6H_THRESHOLD {
                let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(xs_len)];
                _limbs_mul_greater_to_out_toom_44(out, xs, ys, &mut scratch);
            } else if ys_len < MUL_TOOM8H_THRESHOLD {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_size(xs_len, ys_len)];
                _limbs_mul_greater_to_out_toom_6h(out, xs, ys, &mut scratch);
            } else {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_size(xs_len, ys_len)];
                _limbs_mul_greater_to_out_toom_8h(out, xs, ys, &mut scratch);
            }
        }
    } else {
        //TODO replace with FFT
        _limbs_mul_greater_to_out_basecase(out, xs, ys);
    }
    out[xs_len + ys_len - 1]
}

pub fn limbs_mul_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> Limb {
    if xs.len() >= ys.len() {
        limbs_mul_greater_to_out(out, xs, ys)
    } else {
        limbs_mul_greater_to_out(out, ys, xs)
    }
}

// In GMP this is hardcoded to 500
pub const MUL_BASECASE_MAX_UN: usize = 500;

//TODO update docs
// 1 < ys.len() < MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN < xs.len()
//
// This is currently not measurably better than just basecase.
fn limbs_mul_greater_to_out_basecase_mem_opt(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(1 < ys_len);
    assert!(ys_len < MUL_TOOM22_THRESHOLD);
    assert!(MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN);
    assert!(MUL_BASECASE_MAX_UN < xs_len);
    let mut triangle_buffer = [0; MUL_TOOM22_THRESHOLD];
    let mut offset = 0;
    for chunk in xs.chunks(MUL_BASECASE_MAX_UN) {
        let mut out = &mut out[offset..];
        if chunk.len() >= ys_len {
            _limbs_mul_greater_to_out_basecase(out, chunk, ys);
        } else {
            _limbs_mul_greater_to_out_basecase(out, ys, chunk);
        }
        if offset != 0 {
            limbs_slice_add_greater_in_place_left(out, &triangle_buffer[..ys_len]);
        }
        offset += MUL_BASECASE_MAX_UN;
        if offset < xs_len {
            triangle_buffer[..ys_len]
                .copy_from_slice(&out[MUL_BASECASE_MAX_UN..MUL_BASECASE_MAX_UN + ys_len]);
        }
    }
}

//TODO update docs
fn limbs_mul_greater_to_out_basecase_or_mem_opt(prod: &mut [Limb], u: &[Limb], v: &[Limb]) {
    let u_len = u.len();
    let v_len = v.len();
    assert!(u_len >= v_len);
    if v_len > 1 && v_len < MUL_TOOM22_THRESHOLD && u.len() > MUL_BASECASE_MAX_UN {
        limbs_mul_greater_to_out_basecase_mem_opt(prod, u, v)
    } else {
        _limbs_mul_greater_to_out_basecase(prod, u, v);
    }
}

pub fn limbs_mul_greater(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    limbs_mul_greater_to_out(&mut product_limbs, xs, ys);
    product_limbs
}

pub fn limbs_mul(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    if xs.len() >= ys.len() {
        limbs_mul_greater(xs, ys)
    } else {
        limbs_mul_greater(ys, xs)
    }
}

fn limbs_mul_basecase_mem_opt(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        limbs_mul_greater_to_out_basecase_or_mem_opt(&mut product_limbs, xs, ys);
    } else {
        limbs_mul_greater_to_out_basecase_or_mem_opt(&mut product_limbs, ys, xs);
    }
    product_limbs
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by value.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ONE * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * Natural::ZERO).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl Mul<Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ONE * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * &Natural::ZERO).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<&'a Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: &'a Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by reference and the right
/// `Natural` by value.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ONE * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * Natural::ZERO).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((&Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<Natural> for &'a Natural {
    type Output = Natural;

    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by reference.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ONE * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * &Natural::ZERO).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((&Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a, 'b> Mul<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn mul(self, other: &'a Natural) -> Natural {
        if let Small(y) = *other {
            self * y
        } else if let Small(x) = *self {
            other * x
        } else {
            match (self, other) {
                (&Large(ref xs), &Large(ref ys)) => {
                    let mut product = Large(limbs_mul(xs, ys));
                    product.trim();
                    product
                }
                _ => unreachable!(),
            }
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by value.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::One;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x *= Natural::from_str("1000").unwrap();
///     x *= Natural::from_str("2000").unwrap();
///     x *= Natural::from_str("3000").unwrap();
///     x *= Natural::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "24000000000000");
/// }
/// ```
impl MulAssign<Natural> for Natural {
    fn mul_assign(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref mut ys)) => {
                    *xs = limbs_mul(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::One;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x *= &Natural::from_str("1000").unwrap();
///     x *= &Natural::from_str("2000").unwrap();
///     x *= &Natural::from_str("3000").unwrap();
///     x *= &Natural::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "24000000000000");
/// }
/// ```
impl<'a> MulAssign<&'a Natural> for Natural {
    fn mul_assign(&mut self, other: &'a Natural) {
        if let Small(y) = *other {
            *self *= y;
        } else if let Small(x) = *self {
            *self = other * x;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), &Large(ref ys)) => {
                    *xs = limbs_mul(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

impl Natural {
    pub fn _mul_assign_basecase_mem_opt(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref ys)) => *xs = limbs_mul_basecase_mem_opt(xs, ys),
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

pub mod fft;
pub mod poly_eval;
pub mod poly_interpolate;
pub mod toom;
