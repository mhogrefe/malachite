use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoPow, ModPowerOfTwoPowAssign, PowerOfTwo, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::rounding_modes::RoundingMode;
use natural::arithmetic::mod_pow::{get_bits, get_window_size};
use natural::arithmetic::mod_power_of_two::limbs_vec_mod_power_of_two_in_place;
use natural::arithmetic::mod_power_of_two_square::limbs_square_low;
use natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use natural::logic::bit_access::limbs_get_bit;
use natural::logic::significant_bits::limbs_significant_bits;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Raise an n-limb number to a power and return the lowest n limbs of the result.
///
/// //TODO complexity
///
/// This is mpn_powlo from mpn/generic/powlo.c, GMP 6.1.2, where rp == bp.
pub fn limbs_pow_low(xs: &mut [Limb], es: &[Limb], scratch: &mut [Limb]) {
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    let scratch = &mut scratch[..xs_len];
    let es_len = es.len();
    assert_ne!(es_len, 0);
    assert_ne!(es[es_len - 1], 0);
    assert!(es_len > 1 || es_len == 1 && es[0] > 1);
    let mut bit_index = limbs_significant_bits(es);
    let window_size = get_window_size(bit_index);
    assert!(window_size < bit_index);
    let mut powers = vec![0; xs_len << (window_size - 1)];
    let mut powers: Vec<&mut [Limb]> = powers.chunks_mut(xs_len).collect();
    powers[0].copy_from_slice(xs);
    // Store x ^ 2 in scratch.
    limbs_square_low(scratch, xs);
    // Precompute odd powers of x and put them in `powers`.
    for i in 1..usize::power_of_two(window_size - 1) {
        let (powers_lo, powers_hi) = powers.split_at_mut(i);
        limbs_mul_low_same_length(powers_hi[0], powers_lo[i - 1], scratch);
    }
    let mut exp_bits = get_bits(es, bit_index, window_size);
    let trailing_zeros = TrailingZeros::trailing_zeros(Limb::exact_from(exp_bits));
    bit_index += trailing_zeros;
    bit_index -= window_size;
    xs.copy_from_slice(powers[exp_bits >> trailing_zeros >> 1]);
    while bit_index != 0 {
        while bit_index != 0 && !limbs_get_bit(es, bit_index - 1) {
            limbs_square_low(scratch, xs);
            xs.copy_from_slice(scratch);
            bit_index -= 1;
        }
        if bit_index == 0 {
            break;
        }
        // The next bit of the exponent is 1. Now extract the largest block of bits <= window_size,
        // and such that the least significant bit is 1.
        exp_bits = get_bits(es, bit_index, window_size);
        let mut this_windowsize = window_size;
        if bit_index < window_size {
            this_windowsize -= window_size - bit_index;
            bit_index = 0;
        } else {
            bit_index -= window_size;
        }
        let trailing_zeros = TrailingZeros::trailing_zeros(Limb::exact_from(exp_bits));
        this_windowsize -= trailing_zeros;
        bit_index += trailing_zeros;
        while this_windowsize > 1 {
            limbs_square_low(scratch, xs);
            limbs_square_low(xs, scratch);
            this_windowsize -= 2;
        }
        if this_windowsize == 1 {
            limbs_square_low(scratch, xs);
        } else {
            scratch.copy_from_slice(xs);
        }
        limbs_mul_low_same_length(xs, scratch, powers[exp_bits >> trailing_zeros >> 1]);
    }
}

/// Interpreting a `Vec<Limb>` and a `&[Limb]` as the limbs (in ascending order) of two `Natural`s,
/// writes the limbs of the first `Natural` raised to the second, mod 2<sup>`pow`</sup>, to the
/// input `Vec`. Assumes the input is already reduced mod 2<sup>`pow`</sup>. Neither input may be
/// empty or have trailing zeros, and the exponent must be greater than 1.
///
/// TODO complexity
///
/// # Panics
/// Panics if the exponent has trailing zeros or is 1.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_two_pow::limbs_mod_power_of_two_pow;
///
/// let mut xs = vec![3];
/// limbs_mod_power_of_two_pow(&mut xs, &[3], 4);
/// assert_eq!(xs, &[11]);
///
/// let mut xs = vec![123, 456];
/// limbs_mod_power_of_two_pow(&mut xs, &[789, 987], 42);
/// assert_eq!(xs, &[426102667, 987]);
/// ```
pub fn limbs_mod_power_of_two_pow(xs: &mut Vec<Limb>, es: &[Limb], pow: u64) {
    let out_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling));
    xs.resize(out_len, 0);
    let mut scratch = vec![0; out_len];
    limbs_pow_low(xs, es, &mut scratch);
    limbs_vec_mod_power_of_two_in_place(xs, pow);
}

impl ModPowerOfTwoPow<Natural> for Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup>, taking both `Natural`s by
    /// value. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).mod_power_of_two_pow(Natural::from(10u32), 8), 169);
    /// assert_eq!(
    ///     Natural::from(11u32).mod_power_of_two_pow(Natural::from(1000u32), 30),
    ///     289109473
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_pow(mut self, exp: Natural, pow: u64) -> Natural {
        self.mod_power_of_two_pow_assign(exp, pow);
        self
    }
}

impl<'a> ModPowerOfTwoPow<&'a Natural> for Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup>, taking the base by value and
    /// the exponent by reference. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).mod_power_of_two_pow(&Natural::from(10u32), 8), 169);
    /// assert_eq!(
    ///     Natural::from(11u32).mod_power_of_two_pow(&Natural::from(1000u32), 30),
    ///     289109473
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_pow(mut self, exp: &Natural, pow: u64) -> Natural {
        self.mod_power_of_two_pow_assign(exp, pow);
        self
    }
}

impl<'a> ModPowerOfTwoPow<Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup>, taking the base by reference
    /// and the exponent by value. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).mod_power_of_two_pow(Natural::from(10u32), 8), 169);
    /// assert_eq!(
    ///     (&Natural::from(11u32)).mod_power_of_two_pow(Natural::from(1000u32), 30),
    ///     289109473
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_pow(self, exp: Natural, pow: u64) -> Natural {
        self.mod_power_of_two_pow(&exp, pow)
    }
}

impl<'a, 'b> ModPowerOfTwoPow<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup>, taking both `Natural`s by
    /// reference. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).mod_power_of_two_pow(&Natural::from(10u32), 8), 169);
    /// assert_eq!(
    ///     (&Natural::from(11u32)).mod_power_of_two_pow(&Natural::from(1000u32), 30),
    ///     289109473
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_pow(self, exp: &Natural, pow: u64) -> Natural {
        match (self, exp) {
            _ if pow == 0 => Natural::ZERO,
            (_, natural_zero!()) => Natural::ONE,
            (natural_zero!(), _) | (natural_one!(), _) | (_, natural_one!()) => self.clone(),
            (Natural(Small(x)), Natural(Small(e)))
                if pow <= Limb::WIDTH && u64::convertible_from(*e) =>
            {
                Natural(Small(x.mod_power_of_two_pow(u64::wrapping_from(*e), pow)))
            }
            (_, Natural(Small(e))) => {
                let mut xs = self.to_limbs_asc();
                limbs_mod_power_of_two_pow(&mut xs, &[*e], pow);
                Natural::from_owned_limbs_asc(xs)
            }
            (_, Natural(Large(ref es))) => {
                let mut xs = self.to_limbs_asc();
                limbs_mod_power_of_two_pow(&mut xs, es, pow);
                Natural::from_owned_limbs_asc(xs)
            }
        }
    }
}

impl ModPowerOfTwoPowAssign<Natural> for Natural {
    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup> in place, taking the exponent
    /// by value. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_two_pow_assign(Natural::from(10u32), 8);
    /// assert_eq!(x, 169);
    ///
    /// let mut x = Natural::from(11u32);
    /// x.mod_power_of_two_pow_assign(Natural::from(1000u32), 30);
    /// assert_eq!(x, 289109473);
    /// ```
    #[inline]
    fn mod_power_of_two_pow_assign(&mut self, exp: Natural, pow: u64) {
        self.mod_power_of_two_pow_assign(&exp, pow);
    }
}

impl<'a> ModPowerOfTwoPowAssign<&'a Natural> for Natural {
    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup> in place, taking the exponent
    /// by reference. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_two_pow_assign(&Natural::from(10u32), 8);
    /// assert_eq!(x, 169);
    ///
    /// let mut x = Natural::from(11u32);
    /// x.mod_power_of_two_pow_assign(&Natural::from(1000u32), 30);
    /// assert_eq!(x, 289109473);
    /// ```
    fn mod_power_of_two_pow_assign(&mut self, exp: &Natural, pow: u64) {
        match (&mut *self, exp) {
            _ if pow == 0 => *self = Natural::ZERO,
            (_, natural_zero!()) => *self = Natural::ONE,
            (natural_zero!(), _) | (natural_one!(), _) | (_, natural_one!()) => {}
            (Natural(Small(ref mut x)), Natural(Small(e)))
                if pow <= Limb::WIDTH && u64::convertible_from(*e) =>
            {
                x.mod_power_of_two_pow_assign(u64::wrapping_from(*e), pow)
            }
            (_, Natural(Small(e))) => {
                let xs = self.promote_in_place();
                limbs_mod_power_of_two_pow(xs, &[*e], pow);
                self.trim();
            }
            (_, Natural(Large(ref es))) => {
                let xs = self.promote_in_place();
                limbs_mod_power_of_two_pow(xs, es, pow);
                self.trim();
            }
        }
    }
}
