use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{
    DivAssignMod, DivMod, DivRound, DivRoundAssign, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// quotient limbs of a `Limb` divided by the `Natural` and rounded according to a specified
// `RoundingMode`. The limb slice must have at least two elements and cannot have any trailing
// zeros. An `Ordering` is also returned, indicating whether the returned value is less than, equal
// to, or greater than the exact value.
//
// This function returns a `None` iff the rounding mode is `Exact` but the remainder of the
// division would be nonzero.
//
// Note that this function may only return `None`, `Some((0, Less))`, or `Some((1, Greater))`
// because of the restrictions placed on the input slice.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {limbs_limb_div_round_limbs(n: Limb, ds: &[Limb], rm: RoundingMode)
        -> Option<(Limb, Ordering)> {
    if n == 0 {
        Some((0, Ordering::Equal))
    } else {
        match rm {
            RoundingMode::Down | RoundingMode::Floor => Some((0, Ordering::Less)),
            RoundingMode::Up | RoundingMode::Ceiling => Some((1, Ordering::Greater)),
            RoundingMode::Exact => None,
            // 1 if 2 * n > Natural::from_limbs_asc(ds); otherwise, 0
            RoundingMode::Nearest => Some(
                if ds.len() == 2 && ds[1] == 1 && n.get_highest_bit() && (n << 1) > ds[0] {
                    (1, Ordering::Greater)
                } else {
                    (0, Ordering::Less)
                },
            ),
        }
    }
}}

// Compares 2x and y
pub(crate) fn double_cmp(x: &Natural, y: &Natural) -> Ordering {
    (x.significant_bits() + 1)
        .cmp(&y.significant_bits())
        .then_with(|| x.cmp_normalized(y))
}

// assumes r != 0
fn div_round_nearest(q: Natural, r: &Natural, d: &Natural) -> (Natural, Ordering) {
    let compare = double_cmp(r, d);
    if compare == Ordering::Greater || compare == Ordering::Equal && q.odd() {
        (q.add_limb(1), Ordering::Greater)
    } else {
        (q, Ordering::Less)
    }
}

// assumes r != 0
fn div_round_assign_nearest(q: &mut Natural, r: &Natural, d: &Natural) -> Ordering {
    let compare = double_cmp(r, d);
    if compare == Ordering::Greater || compare == Ordering::Equal && q.odd() {
        *q += Natural::ONE;
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

impl DivRound<Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value and rounding according
    /// to a specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// returned value is less than, equal to, or greater than the exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & \text{if} \\quad  q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad  q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
    ///
    /// Then
    /// $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), q))$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use core::cmp::Ordering;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Down),
    ///     (Natural::from(2u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).div_round(Natural::from(3u32), RoundingMode::Floor),
    ///     (Natural::from(333333333333u64), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Up),
    ///     (Natural::from(3u32), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).div_round(Natural::from(3u32), RoundingMode::Ceiling),
    ///     (Natural::from(333333333334u64), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(5u32), RoundingMode::Exact),
    ///     (Natural::from(2u32), Ordering::Equal)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(3u32), RoundingMode::Nearest),
    ///     (Natural::from(3u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32).div_round(Natural::from(3u32), RoundingMode::Nearest),
    ///     (Natural::from(7u32), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Nearest),
    ///     (Natural::from(2u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32).div_round(Natural::from(4u32), RoundingMode::Nearest),
    ///     (Natural::from(4u32), Ordering::Greater)
    /// );
    /// ```
    #[inline]
    fn div_round(mut self, other: Natural, rm: RoundingMode) -> (Natural, Ordering) {
        let o = self.div_round_assign(other, rm);
        (self, o)
    }
}

impl<'a> DivRound<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and rounding according to a specified rounding mode. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & \text{if} \\quad  q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad  q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
    ///
    /// Then
    /// $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), q))$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use core::cmp::Ordering;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(4u32), RoundingMode::Down),
    ///     (Natural::from(2u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).div_round(&Natural::from(3u32), RoundingMode::Floor),
    ///     (Natural::from(333333333333u64), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(4u32), RoundingMode::Up),
    ///     (Natural::from(3u32), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).div_round(&Natural::from(3u32), RoundingMode::Ceiling),
    ///     (Natural::from(333333333334u64), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(5u32), RoundingMode::Exact),
    ///     (Natural::from(2u32), Ordering::Equal)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(3u32), RoundingMode::Nearest),
    ///     (Natural::from(3u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32).div_round(&Natural::from(3u32), RoundingMode::Nearest),
    ///     (Natural::from(7u32), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).div_round(&Natural::from(4u32), RoundingMode::Nearest),
    ///     (Natural::from(2u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32).div_round(&Natural::from(4u32), RoundingMode::Nearest),
    ///     (Natural::from(4u32), Ordering::Greater)
    /// );
    /// ```
    #[inline]
    fn div_round(mut self, other: &'a Natural, rm: RoundingMode) -> (Natural, Ordering) {
        let o = self.div_round_assign(other, rm);
        (self, o)
    }
}

impl<'a> DivRound<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and rounding according to a specified rounding mode. An [`Ordering`] is also
    /// returned, indicating whether the returned value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & \text{if} \\quad  q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad  q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
    ///
    /// Then
    /// $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), q))$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use core::cmp::Ordering;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(4u32), RoundingMode::Down),
    ///     (Natural::from(2u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(Natural::from(3u32), RoundingMode::Floor),
    ///     (Natural::from(333333333333u64), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(4u32), RoundingMode::Up),
    ///     (Natural::from(3u32), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(Natural::from(3u32), RoundingMode::Ceiling),
    ///     (Natural::from(333333333334u64), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(5u32), RoundingMode::Exact),
    ///     (Natural::from(2u32), Ordering::Equal)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(3u32), RoundingMode::Nearest),
    ///     (Natural::from(3u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32)).div_round(Natural::from(3u32), RoundingMode::Nearest),
    ///     (Natural::from(7u32), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(4u32), RoundingMode::Nearest),
    ///     (Natural::from(2u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32)).div_round(Natural::from(4u32), RoundingMode::Nearest),
    ///     (Natural::from(4u32), Ordering::Greater)
    /// );
    /// ```
    fn div_round(self, other: Natural, rm: RoundingMode) -> (Natural, Ordering) {
        let (q, r) = self.div_mod(&other);
        if r == 0 {
            (q, Ordering::Equal)
        } else {
            match rm {
                RoundingMode::Floor | RoundingMode::Down => (q, Ordering::Less),
                RoundingMode::Ceiling | RoundingMode::Up => (q.add_limb(1), Ordering::Greater),
                RoundingMode::Exact => panic!("Division is not exact"),
                RoundingMode::Nearest => div_round_nearest(q, &r, &other),
            }
        }
    }
}

impl<'a, 'b> DivRound<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference and rounding
    /// according to a specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the exact value.
    ///
    /// Let $q = \frac{x}{y}$, and let $g$ be the function that just returns the first element of
    /// the pair, without the [`Ordering`]:
    ///
    /// $$
    /// g(x, y, \mathrm{Down}) = g(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Up}) = g(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// g(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
    ///     \lceil q \rceil & \text{if} \\quad  q - \lfloor q \rfloor > \frac{1}{2}, \\\\
    ///     \lfloor q \rfloor &
    ///     \text{if} \\quad  q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is even}, \\\\
    ///     \lceil q \rceil &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $g(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
    ///
    /// Then
    /// $f(x, y, r) = (g(x, y, r), \operatorname{cmp}(g(x, y, r), q))$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use core::cmp::Ordering;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(4u32), RoundingMode::Down),
    ///     (Natural::from(2u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(&Natural::from(3u32), RoundingMode::Floor),
    ///     (Natural::from(333333333333u64), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(4u32), RoundingMode::Up),
    ///     (Natural::from(3u32), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(&Natural::from(3u32), RoundingMode::Ceiling),
    ///     (Natural::from(333333333334u64), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(5u32), RoundingMode::Exact),
    ///     (Natural::from(2u32), Ordering::Equal)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(3u32), RoundingMode::Nearest),
    ///     (Natural::from(3u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32)).div_round(&Natural::from(3u32), RoundingMode::Nearest),
    ///     (Natural::from(7u32), Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(4u32), RoundingMode::Nearest),
    ///     (Natural::from(2u32), Ordering::Less)
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32)).div_round(&Natural::from(4u32), RoundingMode::Nearest),
    ///     (Natural::from(4u32), Ordering::Greater)
    /// );
    /// ```
    fn div_round(self, other: &'b Natural, rm: RoundingMode) -> (Natural, Ordering) {
        let (q, r) = self.div_mod(other);
        if r == 0 {
            (q, Ordering::Equal)
        } else {
            match rm {
                RoundingMode::Floor | RoundingMode::Down => (q, Ordering::Less),
                RoundingMode::Ceiling | RoundingMode::Up => (q.add_limb(1), Ordering::Greater),
                RoundingMode::Exact => panic!("Division is not exact: {self} / {other}"),
                RoundingMode::Nearest => div_round_nearest(q, &r, other),
            }
        }
    }
}

impl DivRoundAssign<Natural> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value and rounding according to a specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the assigned value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See the [`DivRound`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{DivRoundAssign, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use core::cmp::Ordering;
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(Natural::from(4u32), RoundingMode::Down), Ordering::Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(Natural::from(3u32), RoundingMode::Floor), Ordering::Less);
    /// assert_eq!(n, 333333333333u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(Natural::from(4u32), RoundingMode::Up), Ordering::Greater);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// assert_eq!(
    ///     n.div_round_assign(&Natural::from(3u32), RoundingMode::Ceiling),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(n, 333333333334u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(Natural::from(5u32), RoundingMode::Exact), Ordering::Equal);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest), Ordering::Less);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(20u32);
    /// assert_eq!(
    ///     n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(n, 7);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest), Ordering::Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(14u32);
    /// assert_eq!(
    ///     n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(n, 4);
    /// ```
    fn div_round_assign(&mut self, other: Natural, rm: RoundingMode) -> Ordering {
        let r = self.div_assign_mod(&other);
        if r == 0 {
            Ordering::Equal
        } else {
            match rm {
                RoundingMode::Floor | RoundingMode::Down => Ordering::Less,
                RoundingMode::Ceiling | RoundingMode::Up => {
                    *self += Natural::ONE;
                    Ordering::Greater
                }
                RoundingMode::Exact => panic!("Division is not exact"),
                RoundingMode::Nearest => div_round_assign_nearest(self, &r, &other),
            }
        }
    }
}

impl<'a> DivRoundAssign<&'a Natural> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference and rounding according to a specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the assigned value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See the [`DivRound`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{DivRoundAssign, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    /// use core::cmp::Ordering;
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(&Natural::from(4u32), RoundingMode::Down), Ordering::Less);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// assert_eq!(n.div_round_assign(&Natural::from(3u32), RoundingMode::Floor), Ordering::Less);
    /// assert_eq!(n, 333333333333u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(&Natural::from(4u32), RoundingMode::Up), Ordering::Greater);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// assert_eq!(
    ///     n.div_round_assign(&Natural::from(3u32), RoundingMode::Ceiling),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(n, 333333333334u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(n.div_round_assign(&Natural::from(5u32), RoundingMode::Exact), Ordering::Equal);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(
    ///     n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest),
    ///     Ordering::Less
    /// );
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(20u32);
    /// assert_eq!(
    ///     n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(n, 7);
    ///
    /// let mut n = Natural::from(10u32);
    /// assert_eq!(
    ///     n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest),
    ///     Ordering::Less
    /// );
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(14u32);
    /// assert_eq!(
    ///     n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest),
    ///     Ordering::Greater
    /// );
    /// assert_eq!(n, 4);
    /// ```
    fn div_round_assign(&mut self, other: &'a Natural, rm: RoundingMode) -> Ordering {
        let r = self.div_assign_mod(other);
        if r == 0 {
            Ordering::Equal
        } else {
            match rm {
                RoundingMode::Floor | RoundingMode::Down => Ordering::Less,
                RoundingMode::Ceiling | RoundingMode::Up => {
                    *self += Natural::ONE;
                    Ordering::Greater
                }
                RoundingMode::Exact => panic!("Division is not exact"),
                RoundingMode::Nearest => div_round_assign_nearest(self, &r, other),
            }
        }
    }
}
