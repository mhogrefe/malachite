use malachite_base::num::arithmetic::traits::{
    CheckedLogTwo, DivAssignMod, DivMod, ModPowerOfTwoAssign, ShrRoundAssign, XMulYIsZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{Digits, ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::slice_set_zero;
use natural::arithmetic::div_mod::limbs_div_mod_extra_in_place;
use natural::arithmetic::mul::toom::TUNE_PROGRAM_BUILD;
use natural::Natural;
use platform::{
    Limb, BASES, MP_BASES_BIG_BASE_10, MP_BASES_BIG_BASE_INVERTED_10, MP_BASES_CHARS_PER_LIMB_10,
    MP_BASES_NORMALIZATION_STEPS_10,
};

const GET_STR_THRESHOLD_LIMIT: usize = 150;

pub const GET_STR_PRECOMPUTE_THRESHOLD: usize = 29;

pub const fn get_chars_per_limb(base: u64) -> usize {
    BASES[base as usize].0
}

const fn get_big_base(base: u64) -> Limb {
    BASES[base as usize].3
}

const fn get_big_base_inverted(base: u64) -> Limb {
    BASES[base as usize].4
}

macro_rules! base_10_normalization_step {
    ($j: expr, $buffer: ident, $i: ident, $frac: ident) => {
        if MP_BASES_NORMALIZATION_STEPS_10 <= $j {
            let (digit, new_frac) = Limb::x_mul_y_is_zz($frac, 10);
            $frac = new_frac;
            $buffer[$i] = u8::wrapping_from(digit);
            $i += 1;
        }
    };
}

/// Convert `xs` to digits in base `base`, and put the result in `out`. Generate `len` digits,
/// possibly padding with zeros to the left. If `len` is zero, generate as many characters as
/// required. Return the number of significant digits. Complexity is quadratic; intended for small
/// conversions.
///
/// `base` must not be a power of two, and 2 < `base` < 256.
/// `xs.len()` < `GET_STR_PRECOMPUTE_THRESHOLD`.
/// `len` must be at least as large as the actual number of digits.
///
/// This is mpn_bc_get_str from mpn/generic/get_str.c, GMP 6.2.1.
pub fn _limbs_to_digits_asc_basecase(out: &mut [u8], len: usize, xs: &[Limb], base: u64) -> usize {
    assert!(base > 2);
    assert!(base < 256);
    assert!(out.len() >= len);
    let mut xs_len = xs.len();
    // Allocate memory for largest possible string, given that we only get here for operands with
    // `xs_len` < GET_STR_PRECOMPUTE_THRESHOLD and that the smallest base is 3. 7 / 11 is an
    // approximation to 1 / log_2(3).
    const RP_LEN: usize = if TUNE_PROGRAM_BUILD {
        GET_STR_THRESHOLD_LIMIT
    } else {
        GET_STR_PRECOMPUTE_THRESHOLD
    };
    const BUFFER_LEN: usize = (RP_LEN << Limb::LOG_WIDTH) * 7 / 11;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut rs = [0; RP_LEN];
    let mut i = BUFFER_LEN;
    if base == 10 {
        // Special case code for base 10 so that the compiler has a chance to optimize things.
        const DIGIT_SHIFT: u64 = Limb::WIDTH - 4;
        const LIMIT: usize = MP_BASES_CHARS_PER_LIMB_10
            - 4usize.wrapping_sub(MP_BASES_NORMALIZATION_STEPS_10 as usize);
        rs[1..xs_len + 1].copy_from_slice(xs);
        while xs_len > 1 {
            limbs_div_mod_extra_in_place(
                &mut rs[..xs_len + 1],
                1,
                MP_BASES_BIG_BASE_10,
                MP_BASES_BIG_BASE_INVERTED_10,
                MP_BASES_NORMALIZATION_STEPS_10,
            );
            if rs[xs_len] == 0 {
                xs_len -= 1;
            }
            let mut frac = rs[0].wrapping_add(1);
            i -= MP_BASES_CHARS_PER_LIMB_10;
            // Use the fact that 10 in binary is 1010, with the lowest bit 0. After a few
            // `x_mul_y_is_zz`s, we will have accumulated enough low zeros to use a plain multiply.
            base_10_normalization_step!(0, buffer, i, frac);
            base_10_normalization_step!(1, buffer, i, frac);
            base_10_normalization_step!(2, buffer, i, frac);
            base_10_normalization_step!(3, buffer, i, frac);
            frac.shr_round_assign(4, RoundingMode::Ceiling);
            for _ in 0..LIMIT {
                frac *= 10;
                let digit = frac >> DIGIT_SHIFT;
                buffer[i] = u8::wrapping_from(digit);
                i += 1;
                frac.mod_power_of_two_assign(DIGIT_SHIFT);
            }
            i -= MP_BASES_CHARS_PER_LIMB_10;
        }
        let mut r = rs[1];
        while r != 0 {
            let (new_r, d) = r.div_mod(10);
            r = new_r;
            i -= 1;
            buffer[i] = u8::wrapping_from(d);
        }
    } else {
        // not base 10
        let chars_per_limb = get_chars_per_limb(base);
        let big_base = get_big_base(base);
        let big_base_inverted = get_big_base_inverted(base);
        let normalization_steps = LeadingZeros::leading_zeros(big_base);
        let limb_base = Limb::wrapping_from(base);
        rs[1..xs_len + 1].copy_from_slice(&xs[..xs_len]);
        while xs_len > 1 {
            limbs_div_mod_extra_in_place(
                &mut rs[..xs_len + 1],
                1,
                big_base,
                big_base_inverted,
                normalization_steps,
            );
            if rs[xs_len] == 0 {
                xs_len -= 1;
            }
            let mut frac = rs[0].wrapping_add(1);
            let old_i = i;
            i -= chars_per_limb;
            for d in buffer[i..old_i].iter_mut() {
                let (digit, new_frac) = Limb::x_mul_y_is_zz(frac, limb_base);
                frac = new_frac;
                *d = u8::wrapping_from(digit);
            }
        }
        let mut r = rs[1];
        while r != 0 {
            let (new_r, digit) = r.div_mod(limb_base);
            r = new_r;
            i -= 1;
            buffer[i] = u8::wrapping_from(digit);
        }
    }
    let nonzero_len = BUFFER_LEN - i;
    let zero_len = len.saturating_sub(nonzero_len); // Accounts for len == 0 case
    let (out_zero, out_nonzero) = out.split_at_mut(zero_len);
    slice_set_zero(out_zero);
    out_nonzero[..nonzero_len].copy_from_slice(&buffer[i..]);
    zero_len + nonzero_len
}

pub fn _to_digits_asc_naive<
    D: ExactFrom<Natural> + PrimitiveUnsigned,
    B: CheckedLogTwo + Copy + One + Ord,
>(
    x: &Natural,
    base: B,
) -> Vec<D>
where
    Natural: From<B>,
{
    assert!(base > B::ONE);
    let mut digits = Vec::new();
    let mut remainder = x.clone();
    let nat_base = Natural::from(base);
    while remainder != 0 {
        digits.push(D::exact_from(remainder.div_assign_mod(&nat_base)));
    }
    digits
}

macro_rules! digits_unsigned {
    ($d: ident) => {
        impl Digits<$d, u64> for Natural {
            #[inline]
            fn to_digits_asc(&self, base: u64) -> Vec<$d> {
                _to_digits_asc_naive(self, base)
            }

            #[inline]
            fn to_digits_desc(&self, _base: u64) -> Vec<$d> {
                unimplemented!()
            }

            #[inline]
            fn from_digits_asc<I: Iterator<Item = $d>>(_base: u64, _digits: I) -> Natural {
                unimplemented!()
            }

            #[inline]
            fn from_digits_desc<I: Iterator<Item = $d>>(_base: u64, _digits: I) -> Natural {
                unimplemented!()
            }
        }
    };
}
apply_to_unsigneds!(digits_unsigned);
