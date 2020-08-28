use malachite_base::num::arithmetic::traits::{
    EqModPowerOfTwo, IsPowerOfTwo, Parity, Pow, PowAssign, PowerOfTwo, ShrRound, Square,
    SquareAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, SplitInHalf, WrappingFrom};
use malachite_base::num::logic::traits::{BitAccess, CountOnes, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use natural::arithmetic::mul::limb::limbs_slice_mul_limb_in_place;
use natural::arithmetic::mul::limbs_mul_greater_to_out;
use natural::arithmetic::square::limbs_square_to_out;
use natural::logic::significant_bits::limbs_significant_bits;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{DoubleLimb, Limb};
use std::mem::swap;

fn exp_predecessor(exp: u64) -> u64 {
    if exp.even() {
        exp >> 1
    } else {
        exp - 1
    }
}

fn estimated_limb_len_helper(x: Limb, exp: u64) -> usize {
    usize::exact_from(
        (x.significant_bits() * exp).shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling),
    )
}

// Never an underestimate.
pub fn _limb_pow_alt_estimated_out_len(x: Limb, exp: u64) -> usize {
    if exp.even() {
        estimated_limb_len_helper(x, exp >> 1) << 1
    } else {
        estimated_limb_len_helper(x, exp - 1) + 1
    }
}

// Never an underestimate.
#[inline]
pub fn _limb_pow_alt_estimated_scratch_len(x: Limb, exp: u64) -> usize {
    _limb_pow_alt_estimated_out_len(x, exp_predecessor(exp))
}

/// TODO figure out how to find scratch len using mp_bases. x > 1.
///
/// This is mpn_pow_1 from mpn/generic/pow_1.c, GMP 6.1.2, where exp > 1 and bn == 1.
pub fn limb_pow_to_out_alt<'a>(
    mut out: &'a mut [Limb],
    x: Limb,
    exp: u64,
    mut scratch: &'a mut [Limb],
) -> usize {
    assert!(x > 1);
    assert!(exp > 1);
    // Count number of bits in exp, and compute where to put initial square in order to magically
    // get results in the entry out.
    let bits = exp.significant_bits();
    if bits.odd() {
        swap(&mut out, &mut scratch);
    }
    let (s_hi, s_lo) = DoubleLimb::from(x).square().split_in_half();
    out[0] = s_lo;
    out[1] = s_hi;
    let mut out_len = if s_hi == 0 { 1 } else { 2 };
    for i in (0..bits - 1).rev() {
        if exp.get_bit(i) {
            let (out_last, out_init) = out[..out_len + 1].split_last_mut().unwrap();
            *out_last = limbs_slice_mul_limb_in_place(out_init, x);
            if *out_last != 0 {
                out_len += 1;
            }
        }
        if i == 0 {
            break;
        }
        limbs_square_to_out(scratch, &out[..out_len]);
        out_len <<= 1;
        if scratch[out_len - 1] == 0 {
            out_len -= 1;
        }
        swap(&mut out, &mut scratch);
    }
    out_len
}

pub fn limb_pow_alt(x: Limb, exp: u64) -> Vec<Limb> {
    let mut out = vec![0; _limb_pow_alt_estimated_out_len(x, exp)];
    let mut scratch = vec![0; _limb_pow_alt_estimated_scratch_len(x, exp)];
    let out_len = limb_pow_to_out_alt(&mut out, x, exp, &mut scratch);
    assert!(out_len <= out.len());
    out.truncate(out_len);
    out
}

fn estimated_limbs_len_helper(xs: &[Limb], exp: u64) -> usize {
    usize::exact_from(
        (limbs_significant_bits(xs) * exp).shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling),
    )
}

// Never an underestimate.
pub fn _limbs_pow_alt_estimated_out_len(xs: &[Limb], exp: u64) -> usize {
    if exp.even() {
        estimated_limbs_len_helper(xs, exp >> 1) << 1
    } else {
        estimated_limbs_len_helper(xs, exp - 1) + xs.len()
    }
}

// Never an underestimate.
#[inline]
pub fn _limbs_pow_alt_estimated_scratch_len(xs: &[Limb], exp: u64) -> usize {
    _limbs_pow_alt_estimated_out_len(xs, exp_predecessor(exp))
}

/// TODO figure out how to find scratch len using mp_bases.
///
/// This is mpn_pow_1 from mpn/generic/pow_1.c, GMP 6.1.2, where exp > 1, bn > 1, and the last
/// element of xs is nonzero.
pub fn limbs_pow_to_out_alt<'a>(
    mut out: &'a mut [Limb],
    xs: &[Limb],
    exp: u64,
    mut scratch: &'a mut [Limb],
) -> usize {
    let len = xs.len();
    assert!(len > 1);
    assert!(exp > 1);
    // Count number of bits in exp, and compute where to put initial square in order to magically
    // get results in the entry out.
    let bits = exp.significant_bits();
    if bits.eq_mod_power_of_two(CountOnes::count_ones(exp), 1) {
        swap(&mut out, &mut scratch);
    }
    limbs_square_to_out(out, xs);
    let mut out_len = len << 1;
    if out[out_len - 1] == 0 {
        out_len -= 1;
    }
    for i in (0..bits - 1).rev() {
        if exp.get_bit(i) {
            if limbs_mul_greater_to_out(scratch, &out[..out_len], xs) == 0 {
                out_len -= 1;
            }
            out_len += len;
            swap(&mut out, &mut scratch);
        }
        if i == 0 {
            break;
        }
        limbs_square_to_out(scratch, &out[..out_len]);
        out_len <<= 1;
        if scratch[out_len - 1] == 0 {
            out_len -= 1;
        }
        swap(&mut out, &mut scratch);
    }
    out_len
}

pub fn limbs_pow_alt(xs: &[Limb], exp: u64) -> Vec<Limb> {
    let mut out = vec![0; _limbs_pow_alt_estimated_out_len(xs, exp)];
    let mut scratch = vec![0; _limbs_pow_alt_estimated_scratch_len(xs, exp)];
    let out_len = limbs_pow_to_out_alt(&mut out, xs, exp, &mut scratch);
    assert!(out_len <= out.len());
    out.truncate(out_len);
    out
}

impl Pow<u64> for Natural {
    type Output = Natural;

    /// TODO doc
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).pow(100).to_string(),
    ///     "515377520732011331036461129765621272702107522001"
    /// );
    /// assert_eq!(
    ///     Natural::from_str("12345678987654321").unwrap().pow(3).to_string(),
    ///     "1881676411868862234942354805142998028003108518161"
    /// );
    /// ```
    #[inline]
    fn pow(mut self, exp: u64) -> Natural {
        self.pow_assign(exp);
        self
    }
}

impl<'a> Pow<u64> for &'a Natural {
    type Output = Natural;

    /// TODO doc
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).pow(100).to_string(),
    ///     "515377520732011331036461129765621272702107522001"
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("12345678987654321").unwrap()).pow(3).to_string(),
    ///     "1881676411868862234942354805142998028003108518161"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: u64) -> Natural {
        match (self, exp) {
            (_, 0) | (natural_one!(), _) => Natural::ONE,
            (natural_zero!(), _) => Natural::ZERO,
            (x, 1) => x.clone(),
            (x, 2) => x.square(),
            (x, exp) if x.is_power_of_two() => {
                Natural::power_of_two((x.significant_bits() - 1) * exp)
            }
            (Natural(Small(small)), exp) => {
                if small.significant_bits() * exp <= Limb::WIDTH {
                    Natural(Small(small.checked_pow(u32::wrapping_from(exp)).unwrap()))
                } else {
                    Natural::from_owned_limbs_asc(limb_pow_alt(*small, exp))
                }
            }
            (Natural(Large(ref limbs)), exp) => {
                Natural::from_owned_limbs_asc(limbs_pow_alt(limbs, exp))
            }
        }
    }
}

impl PowAssign<u64> for Natural {
    /// TODO doc
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.pow_assign(100);
    /// assert_eq!(x.to_string(), "515377520732011331036461129765621272702107522001");
    ///
    /// let mut x = Natural::from_str("12345678987654321").unwrap();
    /// x.pow_assign(3);
    /// assert_eq!(x.to_string(), "1881676411868862234942354805142998028003108518161");
    /// ```
    fn pow_assign(&mut self, exp: u64) {
        match (&mut *self, exp) {
            (x, 0) => *x = Natural::ONE,
            (_, 1) | (natural_zero!(), _) | (natural_one!(), _) => {}
            (x, 2) => x.square_assign(),
            (x, exp) if x.is_power_of_two() => {
                *x = Natural::power_of_two((x.significant_bits() - 1) * exp)
            }
            (Natural(Small(ref mut small)), exp) => {
                if small.significant_bits() * exp <= Limb::WIDTH {
                    *small = small.checked_pow(u32::wrapping_from(exp)).unwrap();
                } else {
                    *self = Natural::from_owned_limbs_asc(limb_pow_alt(*small, exp))
                }
            }
            (Natural(Large(ref mut limbs)), exp) => {
                *self = Natural::from_owned_limbs_asc(limbs_pow_alt(limbs, exp))
            }
        }
    }
}
