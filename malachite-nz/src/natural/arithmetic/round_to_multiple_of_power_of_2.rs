use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;
use crate::natural::logic::bit_access::limbs_get_bit;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, PowerOf2, RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign, ShrRound,
    ShrRoundAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LowMask};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::{slice_set_zero, slice_test_zero};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` rounded down to a multiple of 2<sup>`pow`</sup>.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_round_to_multiple_of_power_of_2_down(xs: &[Limb], pow: u64) -> Vec<Limb> {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        Vec::new()
    } else {
        let mut out = vec![0; xs_len];
        out[clear_count..].copy_from_slice(&xs[clear_count..]);
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            out[clear_count] &= !Limb::low_mask(small_pow);
        }
        out
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` rounded up to a multiple of 2<sup>`pow`</sup>. The limbs should not all
// be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {limbs_round_to_multiple_of_power_of_2_up(xs: &[Limb], pow: u64) -> Vec<Limb> {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    let mut out;
    let small_pow = pow & Limb::WIDTH_MASK;
    if clear_count >= xs_len {
        out = vec![0; clear_count + 1];
        out[clear_count] = Limb::power_of_2(small_pow);
    } else {
        let (xs_lo, xs_hi) = xs.split_at(clear_count);
        let mut exact = slice_test_zero(xs_lo);
        out = vec![0; xs_len];
        let out_hi = &mut out[clear_count..];
        out_hi.copy_from_slice(xs_hi);
        if small_pow != 0 {
            let remainder = out_hi[0].mod_power_of_2(small_pow);
            if remainder != 0 {
                out_hi[0] -= remainder;
                exact = false;
            }
        }
        if !exact && limbs_slice_add_limb_in_place(out_hi, Limb::power_of_2(small_pow)) {
            out.push(1);
        }
    }
    out
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.s
fn limbs_round_to_multiple_of_power_of_2_half_integer_to_even(xs: &[Limb], pow: u64) -> Vec<Limb> {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        Vec::new()
    } else {
        let xs_hi = &xs[clear_count..];
        let mut out = vec![0; xs_len];
        let out_hi = &mut out[clear_count..];
        out_hi.copy_from_slice(xs_hi);
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            out_hi[0] &= !Limb::low_mask(small_pow);
        }
        if xs_hi[0].get_bit(small_pow)
            && limbs_slice_add_limb_in_place(out_hi, Limb::power_of_2(small_pow))
        {
            out.push(1);
        }
        out
    }
}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` rounded to the nearest multiple of 2<sup>`pow`</sup>. If the original
// value is exactly between two multiples, it is rounded to the one whose `pow`th bit is zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {limbs_round_to_multiple_of_power_of_2_nearest(xs: &[Limb], pow: u64) -> Vec<Limb> {
    if pow == 0 {
        xs.to_vec()
    } else if !limbs_get_bit(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_2_down(xs, pow)
    } else if !limbs_divisible_by_power_of_2(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_2_up(xs, pow)
    } else {
        limbs_round_to_multiple_of_power_of_2_half_integer_to_even(xs, pow)
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` rounded to a multiple of 2<sup>`pow`</sup>, using a specified rounding
// format. If the original value is not already a multiple of the power of 2, and the
// `RoundingMode` is `Exact`, `None` is returned. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {limbs_round_to_multiple_of_power_of_2(
    xs: &[Limb],
    pow: u64,
    rm: RoundingMode,
) -> Option<Vec<Limb>> {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => {
            Some(limbs_round_to_multiple_of_power_of_2_down(xs, pow))
        }
        RoundingMode::Up | RoundingMode::Ceiling => {
            Some(limbs_round_to_multiple_of_power_of_2_up(xs, pow))
        }
        RoundingMode::Nearest => Some(limbs_round_to_multiple_of_power_of_2_nearest(xs, pow)),
        RoundingMode::Exact => {
            if limbs_divisible_by_power_of_2(xs, pow) {
                Some(xs.to_vec())
            } else {
                None
            }
        }
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural`, rounded down to a multiple of 2<sup>`pow`</sup>, to the input `Vec`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_round_to_multiple_of_power_of_2_down_in_place(xs: &mut Vec<Limb>, pow: u64) {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        xs.clear();
    } else {
        slice_set_zero(&mut xs[..clear_count]);
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            xs[clear_count] &= !Limb::low_mask(small_pow);
        }
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural`, rounded up to a multiple of 2<sup>`pow`</sup>, to the input `Vec`. The
// limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {limbs_round_to_multiple_of_power_of_2_up_in_place(xs: &mut Vec<Limb>, pow: u64) {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    let small_pow = pow & Limb::WIDTH_MASK;
    if clear_count >= xs_len {
        *xs = vec![0; clear_count + 1];
        xs[clear_count] = Limb::power_of_2(small_pow);
    } else {
        let (xs_lo, xs_hi) = xs.split_at_mut(clear_count);
        let mut exact = slice_test_zero(xs_lo);
        slice_set_zero(xs_lo);
        if small_pow != 0 {
            let remainder = xs_hi[0].mod_power_of_2(small_pow);
            if remainder != 0 {
                xs_hi[0] -= remainder;
                exact = false;
            }
        }
        if !exact && limbs_slice_add_limb_in_place(xs_hi, Limb::power_of_2(small_pow)) {
            xs.push(1);
        }
    }
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
fn limbs_round_to_multiple_of_power_of_2_half_integer_to_even_in_place(
    xs: &mut Vec<Limb>,
    pow: u64,
) {
    let clear_count = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let xs_len = xs.len();
    if clear_count >= xs_len {
        xs.clear();
    } else {
        let (xs_lo, xs_hi) = xs.split_at_mut(clear_count);
        if let Some(last) = xs_lo.last_mut() {
            *last = 0;
        }
        let small_pow = pow & Limb::WIDTH_MASK;
        if small_pow != 0 {
            xs_hi[0] &= !Limb::low_mask(small_pow);
        }
        if xs_hi[0].get_bit(small_pow)
            && limbs_slice_add_limb_in_place(xs_hi, Limb::power_of_2(small_pow))
        {
            xs.push(1);
        }
    }
}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural`, rounded to the nearest multiple of 2<sup>`pow`</sup>, to the input
// `Vec`. If the original value is exactly between two multiples, it is rounded to the one whose
// `pow`th bit is zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {limbs_round_to_multiple_of_power_of_2_nearest_in_place(xs: &mut Vec<Limb>, pow: u64) {
    if pow == 0 {
    } else if !limbs_get_bit(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_2_down_in_place(xs, pow);
    } else if !limbs_divisible_by_power_of_2(xs, pow - 1) {
        limbs_round_to_multiple_of_power_of_2_up_in_place(xs, pow);
    } else {
        limbs_round_to_multiple_of_power_of_2_half_integer_to_even_in_place(xs, pow);
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` rounded to the nearest multiple of 2<sup>`pow`</sup> to the input `Vec`,
// using a specified rounding format. If the original value is not already a multiple of the power
// of two, and the `RoundingMode` is `Exact`, the value of `xs` becomes unspecified and `false` is
// returned. Otherwise, `true` is returned. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), pow / Limb::WIDTH)`.
pub_test! {limbs_round_to_multiple_of_power_of_2_in_place(
    xs: &mut Vec<Limb>,
    pow: u64,
    rm: RoundingMode,
) -> bool {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => {
            limbs_round_to_multiple_of_power_of_2_down_in_place(xs, pow);
            true
        }
        RoundingMode::Up | RoundingMode::Ceiling => {
            limbs_round_to_multiple_of_power_of_2_up_in_place(xs, pow);
            true
        }
        RoundingMode::Nearest => {
            limbs_round_to_multiple_of_power_of_2_nearest_in_place(xs, pow);
            true
        }
        RoundingMode::Exact => limbs_divisible_by_power_of_2(xs, pow),
    }
}}

impl RoundToMultipleOfPowerOf2<u64> for Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of $2^k$ according to a specified rounding mode. The
    /// [`Natural`] is taken by value.
    ///
    /// Let $q = \frac{x}{2^k}$:
    ///
    /// $f(x, k, \mathrm{Down}) = f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
    ///
    /// $f(x, k, \mathrm{Up}) = f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
    ///
    /// $$
    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
    ///     2^k \lfloor q \rfloor & \text{if} \\quad
    ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     2^k \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     2^k \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_of_power_of_2(pow, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by_power_of_2(pow)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), pow / Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple_of_power_of_2(2, RoundingMode::Floor),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple_of_power_of_2(2, RoundingMode::Ceiling),
    ///     12
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple_of_power_of_2(2, RoundingMode::Down),
    ///     8
    /// );
    /// assert_eq!(Natural::from(10u32).round_to_multiple_of_power_of_2(2, RoundingMode::Up), 12);
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple_of_power_of_2(2, RoundingMode::Nearest),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(12u32).round_to_multiple_of_power_of_2(2, RoundingMode::Exact),
    ///     12
    /// );
    /// ```
    #[inline]
    fn round_to_multiple_of_power_of_2(mut self, pow: u64, rm: RoundingMode) -> Natural {
        self.round_to_multiple_of_power_of_2_assign(pow, rm);
        self
    }
}

impl<'a> RoundToMultipleOfPowerOf2<u64> for &'a Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of $2^k$ according to a specified rounding mode. The
    /// [`Natural`] is taken by reference.
    ///
    /// Let $q = \frac{x}{2^k}$:
    ///
    /// $f(x, k, \mathrm{Down}) = f(x, k, \mathrm{Floor}) = 2^k \lfloor q \rfloor.$
    ///
    /// $f(x, k, \mathrm{Up}) = f(x, k, \mathrm{Ceiling}) = 2^k \lceil q \rceil.$
    ///
    /// $$
    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
    ///     2^k \lfloor q \rfloor & \text{if} \\quad
    ///     q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     2^k \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     2^k \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     2^k \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor =
    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, k, \mathrm{Exact}) = 2^k q$, but panics if $q \notin \Z$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_of_power_of_2(pow, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by_power_of_2(pow)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), pow / Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple_of_power_of_2(2, RoundingMode::Floor),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple_of_power_of_2(2, RoundingMode::Ceiling),
    ///     12
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple_of_power_of_2(2, RoundingMode::Down),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple_of_power_of_2(2, RoundingMode::Up),
    ///     12
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple_of_power_of_2(2, RoundingMode::Nearest),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(12u32)).round_to_multiple_of_power_of_2(2, RoundingMode::Exact),
    ///     12
    /// );
    /// ```
    fn round_to_multiple_of_power_of_2(self, pow: u64, rm: RoundingMode) -> Natural {
        match (self, pow) {
            (_, 0) | (natural_zero!(), _) => self.clone(),
            (Natural(Small(small)), pow) => Natural::from(small.shr_round(pow, rm)) << pow,
            (Natural(Large(ref limbs)), pow) => {
                if let Some(result_limbs) = limbs_round_to_multiple_of_power_of_2(limbs, pow, rm) {
                    Natural::from_owned_limbs_asc(result_limbs)
                } else {
                    panic!("Rounding {} to multiple of 2^{} is not exact", self, pow);
                }
            }
        }
    }
}

impl RoundToMultipleOfPowerOf2Assign<u64> for Natural {
    /// Rounds a [`Natural`] to a multiple of $2^k$ in place, according to a specified rounding
    /// mode.
    ///
    /// See the [`RoundToMultipleOfPowerOf2`](RoundToMultipleOfPowerOf2) documentation for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_of_power_of_2_assign(pow, RoundingMode::Exact);`
    /// - `assert!(x.divisible_by_power_of_2(pow));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), pow / Limb::WIDTH)`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, but `self` is not a multiple of the power of 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleOfPowerOf2Assign;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Floor);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Ceiling);
    /// assert_eq!(n, 12);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Down);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Up);
    /// assert_eq!(n, 12);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Nearest);
    /// assert_eq!(n, 8);
    ///
    /// let mut n = Natural::from(12u32);
    /// n.round_to_multiple_of_power_of_2_assign(2, RoundingMode::Exact);
    /// assert_eq!(n, 12);
    /// ```
    fn round_to_multiple_of_power_of_2_assign(&mut self, pow: u64, rm: RoundingMode) {
        match (&mut *self, pow) {
            (_, 0) | (natural_zero!(), _) => {}
            (Natural(Small(ref mut small)), pow) => {
                small.shr_round_assign(pow, rm);
                *self <<= pow;
            }
            (Natural(Large(ref mut limbs)), pow) => {
                if limbs_round_to_multiple_of_power_of_2_in_place(limbs, pow, rm) {
                    self.trim();
                } else {
                    panic!("Rounding is not exact");
                }
            }
        }
    }
}
