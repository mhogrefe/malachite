use fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CheckedLogTwo, DivAssignMod, DivMod, DivisibleByPowerOfTwo, ModPowerOfTwoAssign, Parity,
    ShrRound, ShrRoundAssign, XMulYIsZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{Digits, ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{LeadingZeros, TrailingZeros};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::slice_set_zero;
use natural::arithmetic::div_exact::limbs_div_exact_limb_in_place;
use natural::arithmetic::div_mod::{limbs_div_mod_extra_in_place, limbs_div_mod_to_out};
use natural::arithmetic::mul::limb::{limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place};
use natural::arithmetic::mul::toom::TUNE_PROGRAM_BUILD;
use natural::arithmetic::square::limbs_square_to_out;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::Natural;
use platform::{
    Limb, BASES, MP_BASES_BIG_BASE_10, MP_BASES_BIG_BASE_INVERTED_10, MP_BASES_CHARS_PER_LIMB_10,
    MP_BASES_NORMALIZATION_STEPS_10,
};
use std::cmp::Ordering;

const GET_STR_THRESHOLD_LIMIT: usize = 150;

pub const GET_STR_PRECOMPUTE_THRESHOLD: usize = 29;

#[inline]
pub const fn get_chars_per_limb(base: u64) -> usize {
    BASES[base as usize].0
}

const fn get_log_base_of_2(base: u64) -> Limb {
    BASES[base as usize].1
}

const fn get_big_base(base: u64) -> Limb {
    BASES[base as usize].3
}

const fn get_big_base_inverted(base: u64) -> Limb {
    BASES[base as usize].4
}

/// Compute the number of base-b digits corresponding to nlimbs limbs, rounding down.
///
/// This is DIGITS_IN_BASE_PER_LIMB from gmp-impl.h, where res is returned.
fn digits_in_base_per_limb(nlimbs: usize, b: u64) -> u64 {
    u64::exact_from(
        Limb::x_mul_y_is_zz(
            get_log_base_of_2(b),
            Limb::exact_from(nlimbs) << Limb::LOG_WIDTH,
        )
        .0,
    )
}

/// This is DIGITS_IN_BASEGT2_FROM_BITS from gmp-impl.h, GMP 6.2.1, where res is returned and base
/// is not a power of two.
fn limbs_digit_count_helper(nbits: u64, base: u64) -> u64 {
    u64::exact_from(Limb::x_mul_y_is_zz(get_log_base_of_2(base) + 1, Limb::exact_from(nbits)).0)
        .checked_add(1)
        .unwrap()
}

/// The result is either exact or one too big.
///
/// To be exact always it'd be necessary to examine all the limbs of the
/// operand, since numbers like 100..000 and 99...999 generally differ only
/// in the lowest limb.  It'd be possible to examine just a couple of high
/// limbs to increase the probability of being exact, but that doesn't seem
/// worth bothering with.
///
/// This is MPN_SIZEINBASE from gmp-impl.h, GMP 6.2.1, where result is returned and base is not a
/// power of two.
pub fn limbs_digit_count(xs: &[Limb], base: u64) -> u64 {
    assert!(base > 2);
    assert!(base < u64::wrapping_from(BASES.len()));
    assert!(!base.is_power_of_two());
    let size = xs.len();
    if size == 0 {
        1
    } else {
        limbs_digit_count_helper(
            (u64::exact_from(size) << Limb::LOG_WIDTH)
                - LeadingZeros::leading_zeros(*xs.last().unwrap()),
            base,
        )
    }
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
pub fn _limbs_to_digits_small_base_basecase(
    out: &mut [u8],
    len: usize,
    xs: &[Limb],
    base: u64,
) -> usize {
    assert!(base > 2);
    assert!(base < 256);
    assert!(out.len() >= len);
    let mut xs_len = xs.len();
    assert!(xs_len < GET_STR_PRECOMPUTE_THRESHOLD);
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

/// This is powers from from gmp-impl.c, GMP 6.2.1.
#[derive(Clone, Copy, Default)]
struct PowerTableIndicesRow {
    start: usize, // actual power value
    len: usize,
    shift: usize,          // weight of lowest limb, in limb base B
    digits_in_base: usize, // number of corresponding digits
}

pub struct PowerTableRow<'a> {
    power: &'a [Limb],
    shift: usize,          // weight of lowest limb, in limb base B
    digits_in_base: usize, // number of corresponding digits
}

const DIV_1_VS_MUL_1_PERCENT: usize = 150;

const HAVE_MPN_COMPUTE_POWTAB_MUL: bool = DIV_1_VS_MUL_1_PERCENT > 120;

const HAVE_MPN_COMPUTE_POWTAB_DIV: bool = DIV_1_VS_MUL_1_PERCENT < 275;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PowerTableAlgorithm {
    Mul,
    Div,
}

/// This is powtab_decide from mpn/compute_powtab.c, GMP 6.2.1.
pub fn _limbs_choose_power_table_algorithm(
    exptab: &mut [usize],
    xs_len: usize,
    base: u64,
) -> (usize, PowerTableAlgorithm) {
    let chars_per_limb = get_chars_per_limb(base);
    let mut number_of_powers = 0;
    let mut power: usize = xs_len.shr_round(1, RoundingMode::Ceiling);
    while power != 1 {
        exptab[number_of_powers] = power * chars_per_limb;
        number_of_powers += 1;
        power = (power + 1) >> 1;
    }
    exptab[number_of_powers] = chars_per_limb;
    if HAVE_MPN_COMPUTE_POWTAB_MUL && HAVE_MPN_COMPUTE_POWTAB_DIV {
        let power = xs_len - 1;
        let n = xs_len.shr_round(1, RoundingMode::Ceiling);
        let mut mul_cost = 1;
        let mut div_cost = 1;
        for i in (1..number_of_powers).rev() {
            let pow = (power >> i) + 1;
            if n != pow << (i - 1) {
                if pow.odd() {
                    div_cost += pow;
                }
                mul_cost += if pow > 2 && pow.even() { pow << 1 } else { pow };
            } else if pow.odd() {
                mul_cost += pow;
                div_cost += pow;
            }
        }
        div_cost = div_cost * DIV_1_VS_MUL_1_PERCENT / 100;
        (
            number_of_powers,
            if mul_cost <= div_cost {
                PowerTableAlgorithm::Mul
            } else {
                PowerTableAlgorithm::Div
            },
        )
    } else if HAVE_MPN_COMPUTE_POWTAB_MUL {
        (number_of_powers, PowerTableAlgorithm::Mul)
    } else if HAVE_MPN_COMPUTE_POWTAB_DIV {
        (number_of_powers, PowerTableAlgorithm::Div)
    } else {
        panic!("no powtab function available");
    }
}

/// This is mpn_str_powtab_alloc from gmp-impl.h, GMP 6.2.1.
const fn _limbs_digits_power_table_scratch_len(xs_len: usize) -> usize {
    xs_len + ((Limb::WIDTH as usize) << 1)
}

/// This is mpn_dc_get_str_itch from gmp-impl.h, GMP 6.2.1.
const fn _limbs_to_digits_small_base_divide_and_conquer_scratch_len(xs_len: usize) -> usize {
    xs_len + (Limb::WIDTH as usize)
}

/// This is mpn_compute_powtab_mul from mpn/compute_powtab.c, GMP 6.2.1.
pub fn _limbs_compute_power_table_using_mul<'a>(
    power_table_memory: &'a mut [Limb],
    base: u64,
    exponents: &[usize],
    power_len: usize,
) -> Vec<PowerTableRow<'a>> {
    let mut power_indices = Vec::new();
    let big_base = get_big_base(base);
    let chars_per_limb = get_chars_per_limb(base);
    let mut digits_in_base = chars_per_limb;
    let (head, mut remainder) = power_table_memory.split_first_mut().unwrap();
    *head = big_base;
    let (hi, lo) = Limb::x_mul_y_is_zz(big_base, big_base);
    remainder[0] = lo;
    remainder[1] = hi;
    power_indices.push(PowerTableIndicesRow {
        start: 0,
        len: 1,
        digits_in_base,
        shift: 0,
    });
    // `a` and `n` are the start index and length of a power subslice.
    let (mut start, mut len, mut shift) = if lo == 0 { (2, 1, 1) } else { (1, 2, 0) };
    digits_in_base <<= 1;
    power_indices.push(PowerTableIndicesRow {
        start,
        len,
        digits_in_base,
        shift,
    });
    let start_index;
    start_index = if exponents[0] == chars_per_limb << power_len {
        let (power, next_remainder) = remainder[shift..].split_at_mut(len);
        remainder = next_remainder;
        limbs_square_to_out(remainder, power);
        start = 3;
        power_len - 2
    } else {
        if (digits_in_base + chars_per_limb) << (power_len - 2) <= exponents[0] {
            // a = 3, sometimes adjusted to 4.
            let (power, next_remainder) = remainder[shift..].split_at_mut(len);
            remainder = next_remainder;
            let carry = limbs_mul_limb_to_out(remainder, power, big_base);
            remainder[len] = carry;
            if carry != 0 {
                len += 1;
            }
            start = 3;
            digits_in_base += chars_per_limb;
            if remainder[1] == 0 {
                start = 4;
                len -= 1;
                shift += 1;
            }
            power_indices.push(PowerTableIndicesRow {
                start,
                len,
                digits_in_base,
                shift,
            });
            let (power, next_remainder) = remainder[start - 3..].split_at_mut(7 - start);
            remainder = next_remainder;
            limbs_square_to_out(remainder, &power[..len]);
            start = 7;
        } else {
            remainder[2] = remainder[start - 1];
            remainder[3] = remainder[start];
            remainder = &mut remainder[2..];
            power_indices.push(PowerTableIndicesRow {
                start: 3,
                len,
                digits_in_base,
                shift,
            });
            let (power, next_remainder) = remainder.split_at_mut(3);
            remainder = next_remainder;
            limbs_square_to_out(remainder, &power[..len]);
            start = 6;
        }
        power_len - 3
    };
    for i in (0..=start_index).rev() {
        let increment = (len + 1) << 1;
        digits_in_base <<= 1;
        len <<= 1;
        if remainder[len - 1] == 0 {
            len -= 1;
        }
        shift <<= 1;
        let mut adjust = 0;
        if remainder[0] == 0 {
            len -= 1;
            shift += 1;
            remainder = &mut remainder[1..];
            adjust += 1;
        }
        // Adjust new value if it is too small as input to the next squaring.
        if (digits_in_base + chars_per_limb) << i <= exponents[0] {
            let carry = limbs_slice_mul_limb_in_place(&mut remainder[..len], big_base);
            remainder[len] = carry;
            if carry != 0 {
                len += 1;
            }
            digits_in_base += chars_per_limb;
            if remainder[0] == 0 {
                len -= 1;
                shift += 1;
                adjust += 1;
                remainder = &mut remainder[1..];
            }
        }
        power_indices.push(PowerTableIndicesRow {
            start: start + adjust,
            len,
            digits_in_base,
            shift,
        });
        start += increment;
        let (power, next_remainder) = remainder.split_at_mut(increment - adjust);
        remainder = next_remainder;
        if i != 0 {
            limbs_square_to_out(remainder, &power[..len]);
        }
    }
    for (&exponent, row) in exponents[1..start_index + 2]
        .iter()
        .rev()
        .zip(power_indices[power_len - start_index - 1..].iter_mut())
    {
        if row.digits_in_base < exponent {
            let start = row.start;
            let end = start + row.len;
            let carry =
                limbs_slice_mul_limb_in_place(&mut power_table_memory[start..end], big_base);
            power_table_memory[end] = carry;
            if carry != 0 {
                row.len += 1;
            }
            assert!(row.digits_in_base + chars_per_limb == exponent);
            row.digits_in_base = exponent;
            if power_table_memory[start] == 0 {
                row.start += 1;
                row.len -= 1;
                row.shift += 1;
            }
        }
    }
    let mut powers = Vec::with_capacity(power_indices.len());
    let mut remainder: &mut [Limb] = power_table_memory;
    let mut consumed_len = 0;
    for row in power_indices {
        remainder = &mut remainder[row.start - consumed_len..];
        let (power, new_remainder) = remainder.split_at_mut(row.len);
        consumed_len = row.start + power.len();
        powers.push(PowerTableRow {
            power,
            digits_in_base: row.digits_in_base,
            shift: row.shift,
        });
        remainder = new_remainder;
    }
    powers
}

/// This is mpn_compute_powtab_div from mpn/compute_powtab.c, GMP 6.2.1.
pub fn _limbs_compute_power_table_using_div<'a>(
    power_table_memory: &'a mut [Limb],
    base: u64,
    exponents: &[usize],
    power_len: usize,
) -> Vec<PowerTableRow<'a>> {
    let big_base = get_big_base(base);
    let chars_per_limb = get_chars_per_limb(base);
    let big_base_trailing_zeros = TrailingZeros::trailing_zeros(big_base);
    power_table_memory[0] = big_base;
    let mut powers = Vec::with_capacity(power_len + 1);
    let (mut power, mut remainder) = power_table_memory.split_at_mut(1);
    powers.push(PowerTableRow {
        power: &*power,
        digits_in_base: chars_per_limb,
        shift: 0,
    });
    let mut digits_in_base = chars_per_limb;
    let mut len = 1;
    let mut shift = 0;
    for &exp in exponents[..power_len].iter().rev() {
        let two_n = len << 1;
        limbs_square_to_out(remainder, power);
        len = two_n - 1;
        if remainder[len] != 0 {
            len += 1;
        }
        digits_in_base <<= 1;
        if digits_in_base != exp {
            limbs_div_exact_limb_in_place(&mut remainder[..len], big_base);
            if remainder[len - 1] == 0 {
                len -= 1;
            }
            digits_in_base -= chars_per_limb;
        }
        shift <<= 1;
        // Strip low zero limbs, but be careful to keep the result divisible by big_base.
        let mut adjust = 0;
        while remainder[adjust] == 0
            && remainder[adjust + 1].divisible_by_power_of_two(big_base_trailing_zeros)
        {
            adjust += 1;
        }
        len -= adjust;
        shift += adjust;
        remainder = &mut remainder[adjust..];
        let (next_power, new_remainder) = remainder.split_at_mut(two_n);
        power = &mut next_power[..len];
        remainder = new_remainder;
        powers.push(if power[0] == 0 {
            PowerTableRow {
                power: &power[1..],
                digits_in_base,
                shift: shift + 1,
            }
        } else {
            PowerTableRow {
                power,
                digits_in_base,
                shift,
            }
        });
    }
    powers
}

/// This is mpn_compute_powtab from mpn/compute_powtab.c, GMP 6.2.1.
pub fn _limbs_compute_power_table(
    power_table_memory: &mut [Limb],
    xs_len: usize,
    base: u64,
    forced_algorithm: Option<PowerTableAlgorithm>,
) -> (usize, Vec<PowerTableRow>) {
    let mut exponents = [0; Limb::WIDTH as usize];
    let (power_len, auto_algorithm) =
        _limbs_choose_power_table_algorithm(&mut exponents, xs_len, base);
    let algorithm = forced_algorithm.unwrap_or(auto_algorithm);
    let powers = match algorithm {
        PowerTableAlgorithm::Mul => {
            _limbs_compute_power_table_using_mul(power_table_memory, base, &exponents, power_len)
        }
        PowerTableAlgorithm::Div => {
            _limbs_compute_power_table_using_div(power_table_memory, base, &exponents, power_len)
        }
    };
    (power_len, powers)
}

const GET_STR_DC_THRESHOLD: usize = 15;

/// Convert {UP,UN} to a string with a base as represented in POWTAB, and put
/// the string in STR.  Generate LEN characters, possibly padding with zeros to
/// the left.  If LEN is zero, generate as many characters as required.
/// Return a pointer immediately after the last digit of the result string.
/// This uses divide-and-conquer and is intended for large conversions.
///
/// This is mpn_dc_get_str from mpn/generic/get_str.c, GMP 6.2.1.
pub fn _limbs_to_digits_small_base_divide_and_conquer(
    out: &mut [u8],
    mut len: usize,
    xs: &mut [Limb],
    base: u64,
    powers: &[PowerTableRow],
    i: usize,
    scratch: &mut [Limb],
) -> usize {
    let xs_len = xs.len();
    if xs_len < GET_STR_DC_THRESHOLD {
        if xs_len != 0 {
            _limbs_to_digits_small_base_basecase(out, len, xs, base)
        } else {
            fail_on_untested_path("_limbs_to_digits_small_base_divide_and_conquer, xs_len == 0");
            slice_set_zero(&mut out[..len]);
            len
        }
    } else {
        let power = &powers[i];
        let power_len = power.power.len();
        let shift = power.shift;
        let total_len = power_len + shift;
        if xs_len < total_len
            || xs_len == total_len
                && limbs_cmp_same_length(&xs[shift..], power.power) == Ordering::Less
        {
            fail_on_untested_path(
                "_limbs_to_digits_small_base_divide_and_conquer, \
                xs_len < pwn + sn || \
                xs_len == pwn + sn && \
                limbs_cmp_same_length(&xs[sn..xs_len], &powtab_mem[pwp..pwp + xs_len - sn]) == \
                Ordering::Less",
            );
            _limbs_to_digits_small_base_divide_and_conquer(
                out,
                len,
                xs,
                base,
                powers,
                i - 1,
                scratch,
            )
        } else {
            let power = &powers[i];
            //TODO manage memory better
            let xs_copy = xs[shift..].to_vec();
            limbs_div_mod_to_out(scratch, &mut xs[shift..], &xs_copy, power.power);
            let mut q_len = xs_len - total_len;
            if scratch[q_len] != 0 {
                q_len += 1;
            }
            assert!(
                q_len < total_len
                    || q_len == total_len
                        && limbs_cmp_same_length(&scratch[shift..total_len], power.power)
                            == Ordering::Less
            );
            if len != 0 {
                len -= powers[i].digits_in_base;
            }
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(q_len);
            let next_index = _limbs_to_digits_small_base_divide_and_conquer(
                out,
                len,
                scratch_lo,
                base,
                powers,
                i - 1,
                scratch_hi,
            );
            _limbs_to_digits_small_base_divide_and_conquer(
                &mut out[next_index..],
                power.digits_in_base,
                &mut xs[..total_len],
                base,
                powers,
                i - 1,
                scratch,
            ) + next_index
        }
    }
}

/// This is mpn_get_str from mpn/generic/get_str.c, GMP 6.2.1, where un != 0 and base is not a power
/// of two.
pub fn _limbs_to_digits_small_base(
    out: &mut [u8],
    base: u64,
    xs: &mut [Limb],
    forced_algorithm: Option<PowerTableAlgorithm>,
) -> usize {
    let xs_len = xs.len();
    if xs_len == 0 {
        out[0] = 0;
        1
    } else if xs_len < GET_STR_PRECOMPUTE_THRESHOLD {
        _limbs_to_digits_small_base_basecase(out, 0, xs, base)
    } else {
        // Allocate one large block for the powers of big_base.
        let mut power_table_memory = vec![0; _limbs_digits_power_table_scratch_len(xs_len)];
        // Compute a table of powers, were the largest power is >= sqrt(U).
        let digits_len = digits_in_base_per_limb(xs_len, base);
        let len = 1 + usize::exact_from(digits_len) / get_chars_per_limb(base);
        let (power_len, powers) =
            _limbs_compute_power_table(&mut power_table_memory, len, base, forced_algorithm);
        // Using our precomputed powers, convert our number.
        let mut scratch =
            vec![0; _limbs_to_digits_small_base_divide_and_conquer_scratch_len(xs_len)];
        _limbs_to_digits_small_base_divide_and_conquer(
            out,
            0,
            xs,
            base,
            &powers,
            power_len,
            &mut scratch,
        )
    }
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
