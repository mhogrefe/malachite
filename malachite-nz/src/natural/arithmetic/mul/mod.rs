use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use natural::arithmetic::add::limbs_slice_add_greater_in_place_left;
use natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::mul::fft::_limbs_mul_greater_to_out_fft;
use natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use natural::arithmetic::mul::toom::MUL_TOOM33_THRESHOLD_LIMIT;
use natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22, _limbs_mul_greater_to_out_toom_22_scratch_len,
    _limbs_mul_greater_to_out_toom_32, _limbs_mul_greater_to_out_toom_33,
    _limbs_mul_greater_to_out_toom_33_scratch_len, _limbs_mul_greater_to_out_toom_42,
    _limbs_mul_greater_to_out_toom_43, _limbs_mul_greater_to_out_toom_44,
    _limbs_mul_greater_to_out_toom_44_scratch_len, _limbs_mul_greater_to_out_toom_53,
    _limbs_mul_greater_to_out_toom_63, _limbs_mul_greater_to_out_toom_6h,
    _limbs_mul_greater_to_out_toom_6h_scratch_len, _limbs_mul_greater_to_out_toom_8h,
    _limbs_mul_greater_to_out_toom_8h_scratch_len,
    _limbs_mul_same_length_to_out_toom_6h_scratch_len,
    _limbs_mul_same_length_to_out_toom_8h_scratch_len,
};
use natural::arithmetic::square::limbs_square_to_out;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{
    Limb, MUL_FFT_THRESHOLD, MUL_TOOM22_THRESHOLD, MUL_TOOM32_TO_TOOM43_THRESHOLD,
    MUL_TOOM32_TO_TOOM53_THRESHOLD, MUL_TOOM33_THRESHOLD, MUL_TOOM42_TO_TOOM53_THRESHOLD,
    MUL_TOOM42_TO_TOOM63_THRESHOLD, MUL_TOOM44_THRESHOLD, MUL_TOOM6H_THRESHOLD,
    MUL_TOOM8H_THRESHOLD,
};
use std::ops::{Mul, MulAssign};

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
/// the limbs of the product of the `Natural`s. `xs` must be as least as long as `ys` and `ys`
/// cannot be empty.
///
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys` or `ys` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mul::limbs_mul_greater;
/// use malachite_nz::platform::Limb;
///
/// assert_eq!(limbs_mul_greater(&[1, 2, 3], &[6, 7]), &[6, 19, 32, 21, 0]);
/// assert_eq!(limbs_mul_greater(&[100, 101, u32::MAX], &[102, 101, 2]),
///         &[10200, 20402, 10299, 203, 99, 2]);
/// ```
///
/// This is mpn_mul from mpn/generic/mul.c, GMP 6.1.2, where prodp is returned.
pub fn limbs_mul_greater(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let mut out = vec![0; xs.len() + ys.len()];
    limbs_mul_greater_to_out(&mut out, xs, ys);
    out
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
/// the limbs of the product of the `Natural`s. Neither slice can be empty. The length of the
/// resulting slice is always the sum of the lengths of the input slices, so it may have trailing
/// zeros.
///
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if either slice is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mul::limbs_mul;
/// use malachite_nz::platform::Limb;
///
/// assert_eq!(limbs_mul(&[6, 7], &[1, 2, 3]), &[6, 19, 32, 21, 0]);
/// assert_eq!(limbs_mul(&[100, 101, u32::MAX], &[102, 101, 2]),
///         &[10200, 20402, 10299, 203, 99, 2]);
/// ```
///
/// This is mpn_mul from mpn/generic/mul.c, GMP 6.1.2, where un may be less than vn and prodp is
/// returned.
pub fn limbs_mul(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    if xs.len() >= ys.len() {
        limbs_mul_greater(xs, ys)
    } else {
        limbs_mul_greater(ys, xs)
    }
}

// T

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, writes the `2 * xs.len()` least-significant limbs of the product of the `Natural`s
/// to an output slice. The output must be at least as long as `2 * xs.len()`, `xs` must be as long
/// as `ys`, and neither slice can be empty. Returns the result limb at index `2 * xs.len() - 1`
/// (which may be zero).
///
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out` is too short, `xs` and `ys` have different lengths, or either slice is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mul::limbs_mul_same_length_to_out;
/// use malachite_nz::platform::Limb;
///
/// let xs: &mut [Limb] = &mut [10; 4];
/// limbs_mul_same_length_to_out(xs, &[1, 2], &[6, 7]);
/// assert_eq!(xs, &[6, 19, 14, 0]);
///
/// let xs: &mut [Limb] = &mut [10; 6];
/// limbs_mul_same_length_to_out(xs, &[100, 101, u32::MAX], &[102, 101, 2]);
/// assert_eq!(xs, &[10200, 20402, 10299, 203, 99, 2]);
/// ```
///
/// This is mpn_mul_n from mpn/generic/mul_n.c, GMP 6.1.2.
pub fn limbs_mul_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    assert_ne!(len, 0);
    if std::ptr::eq(xs, ys) {
        limbs_square_to_out(out, xs);
    } else if len < MUL_TOOM22_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if len < MUL_TOOM33_THRESHOLD {
        // Allocate workspace of fixed size on stack: fast!
        let scratch =
            &mut [0; _limbs_mul_greater_to_out_toom_22_scratch_len(MUL_TOOM33_THRESHOLD_LIMIT - 1)];
        assert!(MUL_TOOM33_THRESHOLD <= MUL_TOOM33_THRESHOLD_LIMIT);
        _limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else if len < MUL_TOOM44_THRESHOLD {
        let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_len(len)];
        _limbs_mul_greater_to_out_toom_33(out, xs, ys, &mut scratch);
    } else if len < MUL_TOOM6H_THRESHOLD {
        let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_len(len)];
        _limbs_mul_greater_to_out_toom_44(out, xs, ys, &mut scratch);
    } else if len < MUL_TOOM8H_THRESHOLD {
        let mut scratch = vec![0; _limbs_mul_same_length_to_out_toom_6h_scratch_len(len)];
        _limbs_mul_greater_to_out_toom_6h(out, xs, ys, &mut scratch);
    } else if len < MUL_FFT_THRESHOLD {
        let mut scratch = vec![0; _limbs_mul_same_length_to_out_toom_8h_scratch_len(len)];
        _limbs_mul_greater_to_out_toom_8h(out, xs, ys, &mut scratch);
    } else {
        _limbs_mul_greater_to_out_fft(out, xs, ys);
    }
}

// This is TOOM44_OK from mpn/generic/mul.c, GMP 6.1.2.
const fn toom44_ok(xs_len: usize, ys_len: usize) -> bool {
    12 + 3 * xs_len < ys_len << 2
}

// T

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. The output must be at least as long as `xs.len() + ys.len()`, `xs` must be as least as
/// long as `ys`, and `ys` cannot be empty. Returns the result limb at index
/// `xs.len() + ys.len() - 1` (which may be zero).
///
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out` is too short, `xs` is shorter than `ys`, or `ys` is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mul::limbs_mul_greater_to_out;
/// use malachite_nz::platform::Limb;
///
/// let xs: &mut [Limb] = &mut [10; 5];
/// assert_eq!(limbs_mul_greater_to_out(xs, &[1, 2, 3], &[6, 7]), 0);
/// assert_eq!(xs, &[6, 19, 32, 21, 0]);
///
/// let xs: &mut [Limb] = &mut [10; 6];
/// assert_eq!(limbs_mul_greater_to_out(xs, &[100, 101, u32::MAX], &[102, 101, 2]), 2);
/// assert_eq!(xs, &[10200, 20402, 10299, 203, 99, 2]);
/// ```
///
/// This is mpn_mul from mpn/generic/mul.c, GMP 6.1.2.
pub fn limbs_mul_greater_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> Limb {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert_ne!(ys_len, 0);
    if xs_len == ys_len {
        limbs_mul_same_length_to_out(out, xs, ys);
    } else if ys_len < MUL_TOOM22_THRESHOLD {
        // Plain schoolbook multiplication. Unless xs_len is very large, or else if
        // `limbs_mul_same_length_to_out` applies, perform basecase multiply directly.
        _limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if ys_len < MUL_TOOM33_THRESHOLD {
        let toom_x2_scratch_len = 9 * ys_len / 2 + (usize::wrapping_from(Limb::WIDTH) << 1);
        let mut scratch = vec![0; toom_x2_scratch_len];
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
            let four_xs_len = xs_len << 2;
            if four_xs_len < 5 * ys_len {
                _limbs_mul_greater_to_out_toom_22(&mut scratch2, xs, ys, &mut scratch);
            } else if four_xs_len < 7 * ys_len {
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
            let toom_x3_scratch_len = (ys_len << 2) + usize::wrapping_from(Limb::WIDTH);
            let mut scratch = vec![0; toom_x3_scratch_len];
            if xs_len << 1 >= 5 * ys_len {
                // The maximum scratch2 usage is for the `limbs_mul_to_out` result.
                let mut scratch2 = vec![0; (7 * ys_len) >> 1];
                let two_ys_len = ys_len << 1;
                let (xs_lo, mut xs) = xs.split_at(two_ys_len);
                if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                    _limbs_mul_greater_to_out_toom_42(out, xs_lo, ys, &mut scratch);
                } else {
                    _limbs_mul_greater_to_out_toom_63(out, xs_lo, ys, &mut scratch);
                }
                let mut out_offset = two_ys_len;
                // xs_len >= 2.5 * ys_len
                while xs.len() << 1 >= 5 * ys_len {
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
            } else if 6 * xs_len < 7 * ys_len {
                _limbs_mul_greater_to_out_toom_33(out, xs, ys, &mut scratch);
            } else if xs_len << 1 < 3 * ys_len {
                if ys_len < MUL_TOOM32_TO_TOOM43_THRESHOLD {
                    _limbs_mul_greater_to_out_toom_32(out, xs, ys, &mut scratch);
                } else {
                    _limbs_mul_greater_to_out_toom_43(out, xs, ys, &mut scratch);
                }
            } else if 6 * xs_len < 11 * ys_len {
                if xs_len << 2 < 7 * ys_len {
                    if ys_len < MUL_TOOM32_TO_TOOM53_THRESHOLD {
                        _limbs_mul_greater_to_out_toom_32(out, xs, ys, &mut scratch);
                    } else {
                        _limbs_mul_greater_to_out_toom_53(out, xs, ys, &mut scratch);
                    }
                } else if ys_len < MUL_TOOM42_TO_TOOM53_THRESHOLD {
                    _limbs_mul_greater_to_out_toom_42(out, xs, ys, &mut scratch);
                } else {
                    _limbs_mul_greater_to_out_toom_53(out, xs, ys, &mut scratch);
                }
            } else if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                _limbs_mul_greater_to_out_toom_42(out, xs, ys, &mut scratch);
            } else {
                _limbs_mul_greater_to_out_toom_63(out, xs, ys, &mut scratch);
            }
        } else if ys_len < MUL_TOOM6H_THRESHOLD {
            let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_len(xs_len)];
            _limbs_mul_greater_to_out_toom_44(out, xs, ys, &mut scratch);
        } else if ys_len < MUL_TOOM8H_THRESHOLD {
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_len(xs_len, ys_len)];
            _limbs_mul_greater_to_out_toom_6h(out, xs, ys, &mut scratch);
        } else {
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_len(xs_len, ys_len)];
            _limbs_mul_greater_to_out_toom_8h(out, xs, ys, &mut scratch);
        }
    } else {
        _limbs_mul_greater_to_out_fft(out, xs, ys);
    }
    out[xs_len + ys_len - 1]
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. The output must be at least as long as `xs.len() + ys.len()`, and neither slice can be
/// empty. Returns the result limb at index `xs.len() + ys.len() - 1` (which may be zero).
///
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if `out` is too short or either slice is empty.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mul::limbs_mul_to_out;
/// use malachite_nz::platform::Limb;
///
/// let xs: &mut [Limb] = &mut [10; 5];
/// assert_eq!(limbs_mul_to_out(xs, &[6, 7], &[1, 2, 3]), 0);
/// assert_eq!(xs, &[6, 19, 32, 21, 0]);
///
/// let xs: &mut [Limb] = &mut [10; 6];
/// assert_eq!(limbs_mul_to_out(xs, &[100, 101, u32::MAX], &[102, 101, 2]), 2);
/// assert_eq!(xs, &[10200, 20402, 10299, 203, 99, 2]);
/// ```
///
/// This is mpn_mul from mpn/generic/mul.c, GMP 6.1.2, where un may be less than vn.
pub fn limbs_mul_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> Limb {
    if xs.len() >= ys.len() {
        limbs_mul_greater_to_out(out, xs, ys)
    } else {
        limbs_mul_greater_to_out(out, ys, xs)
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
/// This is mpn_mul_basecase from mpn/generic/mul_basecase.c, GMP 6.1.2.
pub fn _limbs_mul_greater_to_out_basecase(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    assert!(out.len() >= xs_len + ys_len);
    // We first multiply by the low order limb. This result can be stored, not added, to out.
    out[xs_len] = limbs_mul_limb_to_out(out, xs, ys[0]);
    // Now accumulate the product of xs and the next higher limb from ys.
    let window_size = xs_len + 1;
    for i in 1..ys_len {
        let (out_last, out_init) = out[i..i + window_size].split_last_mut().unwrap();
        *out_last = limbs_slice_add_mul_limb_same_length_in_place_left(out_init, xs, ys[i]);
    }
}

impl Mul<Natural> for Natural {
    type Output = Natural;

    /// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by value.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()` + `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((Natural::ONE * Natural::from(123u32)).to_string(), "123");
    /// assert_eq!((Natural::from(123u32) * Natural::ZERO).to_string(), "0");
    /// assert_eq!((Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
    /// assert_eq!((Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
    ///            .unwrap()).to_string(), "121932631112635269000000");
    /// ```
    #[inline]
    fn mul(mut self, other: Natural) -> Natural {
        self *= other;
        self
    }
}

impl<'a> Mul<&'a Natural> for Natural {
    type Output = Natural;

    /// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by value and the right
    /// `Natural` by reference.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()` + `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((Natural::ONE * &Natural::from(123u32)).to_string(), "123");
    /// assert_eq!((Natural::from(123u32) * &Natural::ZERO).to_string(), "0");
    /// assert_eq!((Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
    /// assert_eq!((Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
    ///            .unwrap()).to_string(), "121932631112635269000000");
    /// ```
    #[inline]
    fn mul(mut self, other: &'a Natural) -> Natural {
        self *= other;
        self
    }
}

impl<'a> Mul<Natural> for &'a Natural {
    type Output = Natural;

    /// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by reference and the right
    /// `Natural` by value.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()` + `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Natural::ONE * Natural::from(123u32)).to_string(), "123");
    /// assert_eq!((&Natural::from(123u32) * Natural::ZERO).to_string(), "0");
    /// assert_eq!((&Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
    /// assert_eq!((&Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
    ///            .unwrap()).to_string(), "121932631112635269000000");
    /// ```
    #[inline]
    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

impl<'a, 'b> Mul<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by reference.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()` + `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Natural::ONE * &Natural::from(123u32)).to_string(), "123");
    /// assert_eq!((&Natural::from(123u32) * &Natural::ZERO).to_string(), "0");
    /// assert_eq!((&Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
    /// assert_eq!((&Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
    ///            .unwrap()).to_string(), "121932631112635269000000");
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
    /// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by value.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()` + `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
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
    /// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by
    /// reference.
    ///
    /// Time: worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: worst case O(n * log(n))
    ///
    /// where n = `self.significant_bits()` + `other.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
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

pub mod fft;
pub mod limb;
pub mod mul_low;
pub mod mul_mod;
pub mod poly_eval;
pub mod poly_interpolate;
pub mod square_mod;
pub mod toom;
