use malachite_base::num::arithmetic::traits::{Parity, ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::slice_test_zero;
use malachite_base::vecs::vec_delete_left;
use natural::arithmetic::add::limbs_vec_add_limb_in_place;
use natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;
use natural::arithmetic::shr::{limbs_shr, limbs_slice_shr_in_place, limbs_vec_shr_in_place};
use natural::logic::bit_access::limbs_get_bit;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::ops::{Shl, ShlAssign};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` right-shifted by a `Limb`, rounding up. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(1, xs.len() - bits / Limb::WIDTH)`.
//
// This is equivalent to `cfdiv_q_2exp` from `mpz/cfdiv_q_2exp.c`, GMP 6.2.1, where `u` is
// non-negative, `dir == 1`, and the result is returned.
pub_test! {limbs_shr_round_up(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        vec![1]
    } else {
        let (xs_lo, xs_hi) = xs.split_at(delete_count);
        let mut exact = slice_test_zero(xs_lo);
        let mut out = xs_hi.to_vec();
        let small_bits = bits & Limb::WIDTH_MASK;
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(&mut out, small_bits) == 0;
        }
        if !exact {
            limbs_vec_add_limb_in_place(&mut out, 1);
        }
        out
    }
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(1, xs.len() - bits / Limb::WIDTH)`.
fn limbs_shr_round_half_integer_to_even(xs: &[Limb], bits: u64) -> Vec<Limb> {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        Vec::new()
    } else {
        let small_bits = bits & Limb::WIDTH_MASK;
        let mut out = xs[delete_count..].to_vec();
        if small_bits != 0 {
            limbs_slice_shr_in_place(&mut out, small_bits);
        }
        if !out.is_empty() && out[0].odd() {
            limbs_vec_add_limb_in_place(&mut out, 1);
        }
        out
    }
}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` right-shifted by a `Limb`, rounding to the `Natural` nearest to the
// actual value of `self` divided by 2<sup>`bits`</sup>. If the actual value is exactly between
// two integers, it is rounded to the even one.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is
// `max(1, xs.len() - bits / Limb::WIDTH)`.
pub_test! {limbs_shr_round_nearest(xs: &[Limb], bits: u64) -> Vec<Limb> {
    if bits == 0 {
        xs.to_vec()
    } else if !limbs_get_bit(xs, bits - 1) {
        limbs_shr(xs, bits)
    } else if !limbs_divisible_by_power_of_2(xs, bits - 1) {
        limbs_shr_round_up(xs, bits)
    } else {
        limbs_shr_round_half_integer_to_even(xs, bits)
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` right-shifted by a `Limb`, if the shift is exact (doesn't remove any
// `true` bits). If the shift is inexact, `None` is returned. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is
// `max(1, xs.len() - bits / Limb::WIDTH)`.
pub_test! {limbs_shr_exact(xs: &[Limb], bits: u64) -> Option<Vec<Limb>> {
    if limbs_divisible_by_power_of_2(xs, bits) {
        Some(limbs_shr(xs, bits))
    } else {
        None
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` right-shifted by a `Limb`, rounded using a specified rounding format. The
// limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is
// `max(1, xs.len() - bits / Limb::WIDTH)`.
pub_test! {limbs_shr_round(xs: &[Limb], bits: u64, rm: RoundingMode) -> Option<Vec<Limb>> {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => Some(limbs_shr(xs, bits)),
        RoundingMode::Up | RoundingMode::Ceiling => Some(limbs_shr_round_up(xs, bits)),
        RoundingMode::Nearest => Some(limbs_shr_round_nearest(xs, bits)),
        RoundingMode::Exact => limbs_shr_exact(xs, bits),
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` right-shifted by a `Limb`, rounding up, to the input `Vec`. The limbs
// should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(1, xs.len() - bits / Limb::WIDTH)`.
//
// This is equivalent to `cfdiv_q_2exp` from `mpz/cfdiv_q_2exp.c`, GMP 6.2.1, where `u` is
// non-negative, `dir == 1`, and `w == u`.
pub_test! {limbs_vec_shr_round_up_in_place(xs: &mut Vec<Limb>, bits: u64) {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        xs.truncate(1);
        xs[0] = 1;
    } else {
        let mut exact = slice_test_zero(&xs[..delete_count]);
        let small_bits = bits & Limb::WIDTH_MASK;
        vec_delete_left(xs, delete_count);
        if small_bits != 0 {
            exact &= limbs_slice_shr_in_place(xs, small_bits) == 0;
        }
        if !exact {
            limbs_vec_add_limb_in_place(xs, 1);
        }
    }
}}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(1, xs.len() - bits / Limb::WIDTH)`.
fn limbs_vec_shr_round_half_integer_to_even_in_place(xs: &mut Vec<Limb>, bits: u64) {
    let delete_count = usize::exact_from(bits >> Limb::LOG_WIDTH);
    if delete_count >= xs.len() {
        xs.clear();
    } else {
        let small_bits = bits & Limb::WIDTH_MASK;
        vec_delete_left(xs, delete_count);
        if small_bits != 0 {
            limbs_slice_shr_in_place(xs, small_bits);
        }
        if !xs.is_empty() && xs[0].odd() {
            limbs_vec_add_limb_in_place(xs, 1);
        }
    }
}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, rounding to the `Natural`
// nearest to the actual value of `self` divided by 2<sup>`bits`</sup>. If the actual value is
// exactly between two integers, it is rounded to the even one.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `xs.len()`.
pub_test! {limbs_vec_shr_round_nearest_in_place(xs: &mut Vec<Limb>, bits: u64) {
    if bits == 0 {
    } else if !limbs_get_bit(xs, bits - 1) {
        limbs_vec_shr_in_place(xs, bits)
    } else if !limbs_divisible_by_power_of_2(xs, bits - 1) {
        limbs_vec_shr_round_up_in_place(xs, bits)
    } else {
        limbs_vec_shr_round_half_integer_to_even_in_place(xs, bits)
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, if the shift is exact
// (doesn't remove any `true` bits). Returns whether the shift was exact. The limbs should not all
// be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `xs.len()`.
pub_test! {limbs_vec_shr_exact_in_place(xs: &mut Vec<Limb>, bits: u64) -> bool {
    if limbs_divisible_by_power_of_2(xs, bits) {
        limbs_vec_shr_in_place(xs, bits);
        true
    } else {
        false
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` right-shifted by a `Limb` to the input `Vec`, rounded using a specified
// rounding format. If the shift is inexact (removes some `true` bits) and the `RoundingMode` is
// `Exact`, the value of `xs` becomes unspecified and `false` is returned. Otherwise, `true` is
// returned. The limbs should not all be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `xs.len()`.
pub_test! {limbs_vec_shr_round_in_place(xs: &mut Vec<Limb>, bits: u64, rm: RoundingMode) -> bool {
    match rm {
        RoundingMode::Down | RoundingMode::Floor => {
            limbs_vec_shr_in_place(xs, bits);
            true
        }
        RoundingMode::Up | RoundingMode::Ceiling => {
            limbs_vec_shr_round_up_in_place(xs, bits);
            true
        }
        RoundingMode::Nearest => {
            limbs_vec_shr_round_nearest_in_place(xs, bits);
            true
        }
        RoundingMode::Exact => limbs_vec_shr_exact_in_place(xs, bits),
    }
}}

fn shr_round_unsigned_ref_n<T: PrimitiveUnsigned>(x: &Natural, bits: T, rm: RoundingMode) -> Natural
where
    u64: ExactFrom<T>,
    Limb: ShrRound<T, Output = Limb>,
{
    match (x, bits) {
        (natural_zero!(), _) => x.clone(),
        (_, bits) if bits == T::ZERO => x.clone(),
        (Natural(Small(ref small)), bits) => Natural(Small(small.shr_round(bits, rm))),
        (Natural(Large(ref limbs)), bits) => {
            if let Some(out) = limbs_shr_round(limbs, u64::exact_from(bits), rm) {
                Natural::from_owned_limbs_asc(out)
            } else {
                panic!("Right shift is not exact: {} >> {}", x, bits);
            }
        }
    }
}

fn shr_round_assign_unsigned_n<T: PrimitiveUnsigned>(x: &mut Natural, bits: T, rm: RoundingMode)
where
    u64: ExactFrom<T>,
    Limb: ShrRoundAssign<T>,
{
    match (&mut *x, bits) {
        (natural_zero!(), _) => {}
        (_, bits) if bits == T::ZERO => {}
        (Natural(Small(ref mut small)), bits) => {
            small.shr_round_assign(bits, rm);
        }
        (Natural(Large(ref mut limbs)), bits) => {
            if !limbs_vec_shr_round_in_place(limbs, u64::exact_from(bits), rm) {
                panic!("Right shift is not exact.");
            }
            x.trim();
        }
    }
}

macro_rules! impl_natural_shr_round_unsigned {
    ($t:ident) => {
        impl ShrRound<$t> for Natural {
            type Output = Natural;

            /// Shifts a [`Natural`] right (divides it by a power of 2), taking it by value, and
            /// rounds according to the specified rounding mode.
            ///
            /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`.
            /// To test whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$:
            ///
            /// $f(x, k, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// f(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if}
            ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if}
            ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///         \\ \lfloor q \rfloor \\ \text{is odd}.
            /// \end{cases}
            /// $$
            ///
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if `rm` is `RoundingMode::Exact` but `self` is not
            /// divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round).
            #[inline]
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> Natural {
                self.shr_round_assign(bits, rm);
                self
            }
        }

        impl<'a> ShrRound<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a [`Natural`] right (divides it by a power of 2), taking it by reference,
            /// and rounds according to the specified rounding mode.
            ///
            /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`.
            /// To test whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$:
            ///
            /// $f(x, k, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// f(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if}
            ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if}
            ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///         \\ \lfloor q \rfloor \\ \text{is odd}.
            /// \end{cases}
            /// $$
            ///
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(m) = O(m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(1, self.significant_bits() - bits)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if `rm` is `RoundingMode::Exact` but `self` is not
            /// divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round).
            #[inline]
            fn shr_round(self, bits: $t, rm: RoundingMode) -> Natural {
                shr_round_unsigned_ref_n(self, bits, rm)
            }
        }

        impl ShrRoundAssign<$t> for Natural {
            /// Shifts a [`Natural`] right (divides it by a power of 2) and rounds according to the
            /// specified rounding mode, in place. Passing `RoundingMode::Floor` or
            /// `RoundingMode::Down` is equivalent to using `>>=`. To test whether
            /// `RoundingMode::Exact` can be passed, use `self.divisible_by_power_of_2(bits)`.
            ///
            /// See the [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound)
            /// documentation for details.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if `rm` is `RoundingMode::Exact` but `self` is not
            /// divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round_assign).
            #[inline]
            fn shr_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                shr_round_assign_unsigned_n(self, bits, rm);
            }
        }
    };
}
apply_to_unsigneds!(impl_natural_shr_round_unsigned);

fn shr_round_signed_ref_n<'a, U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &'a Natural,
    bits: S,
    rm: RoundingMode,
) -> Natural
where
    &'a Natural: Shl<U, Output = Natural> + ShrRound<U, Output = Natural>,
{
    if bits >= S::ZERO {
        x.shr_round(bits.unsigned_abs(), rm)
    } else {
        x << bits.unsigned_abs()
    }
}

fn shr_round_assign_signed_n<U, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: &mut Natural,
    bits: S,
    rm: RoundingMode,
) where
    Natural: ShlAssign<U> + ShrRoundAssign<U>,
{
    if bits >= S::ZERO {
        x.shr_round_assign(bits.unsigned_abs(), rm);
    } else {
        *x <<= bits.unsigned_abs()
    }
}

macro_rules! impl_natural_shr_round_signed {
    ($t:ident) => {
        impl ShrRound<$t> for Natural {
            type Output = Natural;

            /// Shifts a [`Natural`] right (divides or multiplies it by a power of 2), taking it by
            /// value, and rounds according to the specified rounding mode.
            ///
            /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`.
            /// To test whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$:
            ///
            /// $f(x, k, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// f(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if}
            ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if}
            ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///         \\ \lfloor q \rfloor \\ \text{is odd}.
            /// \end{cases}
            /// $$
            ///
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(-bits, 0)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if $k$ is positive and `rm` is `RoundingMode::Exact` but
            /// `self` is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round).
            #[inline]
            fn shr_round(mut self, bits: $t, rm: RoundingMode) -> Natural {
                self.shr_round_assign(bits, rm);
                self
            }
        }

        impl<'a> ShrRound<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a [`Natural`] right (divides or multiplies it by a power of 2), taking it by
            /// reference, and rounds according to the specified rounding mode.
            ///
            /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`.
            /// To test whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_2(bits)`.
            ///
            /// Let $q = \frac{x}{2^k}$:
            ///
            /// $f(x, k, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, k, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// f(x, k, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if}
            ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if}
            ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///         \\ \lfloor q \rfloor \\ \text{is odd}.
            /// \end{cases}
            /// $$
            ///
            /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(-bits, 0)`.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(-bits, 0)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if $k$ is positive and `rm` is `RoundingMode::Exact` but
            /// `self` is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round).
            #[inline]
            fn shr_round(self, bits: $t, rm: RoundingMode) -> Natural {
                shr_round_signed_ref_n(self, bits, rm)
            }
        }

        impl ShrRoundAssign<$t> for Natural {
            /// Shifts a [`Natural`] right (divides or multiplies it by a power of 2) and rounds
            /// according to the specified rounding mode, in place.
            ///
            /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using `>>`.
            /// To test whether `RoundingMode::Exact` can be passed, use
            /// `self.divisible_by_power_of_2(bits)`.
            ///
            /// See the [`ShrRound`](malachite_base::num::arithmetic::traits::ShrRound)
            /// documentation for details.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(-bits, 0)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if $k$ is positive and `rm` is `RoundingMode::Exact` but
            /// `self` is not divisible by $2^k$.
            ///
            /// # Examples
            /// See [here](super::shr_round#shr_round_assign).
            #[inline]
            fn shr_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                shr_round_assign_signed_n(self, bits, rm);
            }
        }
    };
}
apply_to_signeds!(impl_natural_shr_round_signed);
