use malachite_base::num::arithmetic::traits::{
    DivAssignMod, DivMod, DivRound, DivRoundAssign, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Iverson, One};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use crate::natural::Natural;
use crate::platform::Limb;
use std::cmp::Ordering;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// quotient limbs of a `Limb` divided by the `Natural` and rounded according to a specified
// `RoundingMode`. The limb slice must have at least two elements and cannot have any trailing
// zeros.
//
// This function returns a `None` iff the rounding mode is `Exact` but the remainder of the
// division would be nonzero.
//
// Note that this function may only return `None`, `Some(0)`, or `Some(1)` because of the
// restrictions placed on the input slice.
//
// # Worst-case complexity
// Constant time and additional memory.
pub_test! {limbs_limb_div_round_limbs(n: Limb, ds: &[Limb], rm: RoundingMode) -> Option<Limb> {
    if n == 0 {
        Some(0)
    } else {
        match rm {
            RoundingMode::Down | RoundingMode::Floor => Some(0),
            RoundingMode::Up | RoundingMode::Ceiling => Some(1),
            RoundingMode::Exact => None,
            // 1 if 2 * n > Natural::from_limbs_asc(ds); otherwise, 0
            RoundingMode::Nearest => Some(Limb::iverson(
                ds.len() == 2 && ds[1] == 1 && n.get_highest_bit() && (n << 1) > ds[0],
            )),
        }
    }
}}

// Compares 2x and y
fn double_cmp(x: &Natural, y: &Natural) -> Ordering {
    (x.significant_bits() + 1)
        .cmp(&y.significant_bits())
        .then_with(|| x.cmp_normalized(y))
}

fn div_round_nearest(q: Natural, r: &Natural, d: &Natural) -> Natural {
    let compare = double_cmp(r, d);
    if compare == Ordering::Greater || compare == Ordering::Equal && q.odd() {
        q.add_limb(1)
    } else {
        q
    }
}

fn div_round_assign_nearest(q: &mut Natural, r: &Natural, d: &Natural) {
    let compare = double_cmp(r, d);
    if compare == Ordering::Greater || compare == Ordering::Equal && q.odd() {
        *q += Natural::ONE;
    }
}

impl DivRound<Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value and rounding according
    /// to a specified rounding mode.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $$
    /// f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
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
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
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
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Down), 2);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).div_round(Natural::from(3u32), RoundingMode::Floor),
    ///     333333333333u64
    /// );
    /// assert_eq!(Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Up), 3);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).div_round(Natural::from(3u32), RoundingMode::Ceiling),
    ///     333333333334u64);
    /// assert_eq!(Natural::from(10u32).div_round(Natural::from(5u32), RoundingMode::Exact), 2);
    /// assert_eq!(Natural::from(10u32).div_round(Natural::from(3u32), RoundingMode::Nearest), 3);
    /// assert_eq!(Natural::from(20u32).div_round(Natural::from(3u32), RoundingMode::Nearest), 7);
    /// assert_eq!(Natural::from(10u32).div_round(Natural::from(4u32), RoundingMode::Nearest), 2);
    /// assert_eq!(Natural::from(14u32).div_round(Natural::from(4u32), RoundingMode::Nearest), 4);
    /// ```
    #[inline]
    fn div_round(mut self, other: Natural, rm: RoundingMode) -> Natural {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<&'a Natural> for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and rounding according to a specified rounding mode.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $$
    /// f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
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
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
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
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).div_round(&Natural::from(4u32), RoundingMode::Down), 2);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).div_round(&Natural::from(3u32), RoundingMode::Floor),
    ///     333333333333u64
    /// );
    /// assert_eq!(Natural::from(10u32).div_round(&Natural::from(4u32), RoundingMode::Up), 3);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).div_round(&Natural::from(3u32), RoundingMode::Ceiling),
    ///     333333333334u64);
    /// assert_eq!(Natural::from(10u32).div_round(&Natural::from(5u32), RoundingMode::Exact), 2);
    /// assert_eq!(Natural::from(10u32).div_round(&Natural::from(3u32), RoundingMode::Nearest), 3);
    /// assert_eq!(Natural::from(20u32).div_round(&Natural::from(3u32), RoundingMode::Nearest), 7);
    /// assert_eq!(Natural::from(10u32).div_round(&Natural::from(4u32), RoundingMode::Nearest), 2);
    /// assert_eq!(Natural::from(14u32).div_round(&Natural::from(4u32), RoundingMode::Nearest), 4);
    /// ```
    #[inline]
    fn div_round(mut self, other: &'a Natural, rm: RoundingMode) -> Natural {
        self.div_round_assign(other, rm);
        self
    }
}

impl<'a> DivRound<Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and rounding according to a specified rounding mode.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $$
    /// f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
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
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
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
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(10u32)).div_round(Natural::from(4u32), RoundingMode::Down), 2);
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(Natural::from(3u32), RoundingMode::Floor),
    ///     333333333333u64
    /// );
    /// assert_eq!((&Natural::from(10u32)).div_round(Natural::from(4u32), RoundingMode::Up), 3);
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(Natural::from(3u32), RoundingMode::Ceiling),
    ///     333333333334u64);
    /// assert_eq!((&Natural::from(10u32)).div_round(Natural::from(5u32), RoundingMode::Exact), 2);
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(3u32), RoundingMode::Nearest),
    ///     3
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32)).div_round(Natural::from(3u32), RoundingMode::Nearest),
    ///     7
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(Natural::from(4u32), RoundingMode::Nearest),
    ///     2
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32)).div_round(Natural::from(4u32), RoundingMode::Nearest),
    ///     4
    /// );
    /// ```
    fn div_round(self, other: Natural, rm: RoundingMode) -> Natural {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            self / other
        } else {
            let (q, r) = self.div_mod(&other);
            if r == 0 {
                q
            } else {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => q.add_limb(1),
                    RoundingMode::Exact => panic!("Division is not exact"),
                    RoundingMode::Nearest => div_round_nearest(q, &r, &other),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl<'a, 'b> DivRound<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference and rounding
    /// according to a specified rounding mode.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $$
    /// f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
    /// $$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
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
    /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
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
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRound, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(10u32)).div_round(&Natural::from(4u32), RoundingMode::Down), 2);
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(&Natural::from(3u32), RoundingMode::Floor),
    ///     333333333333u64
    /// );
    /// assert_eq!((&Natural::from(10u32)).div_round(&Natural::from(4u32), RoundingMode::Up), 3);
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).div_round(&Natural::from(3u32), RoundingMode::Ceiling),
    ///     333333333334u64);
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(5u32), RoundingMode::Exact),
    ///     2
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(3u32), RoundingMode::Nearest),
    ///     3
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32)).div_round(&Natural::from(3u32), RoundingMode::Nearest),
    ///     7
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).div_round(&Natural::from(4u32), RoundingMode::Nearest),
    ///     2
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32)).div_round(&Natural::from(4u32), RoundingMode::Nearest),
    ///     4
    /// );
    /// ```
    fn div_round(self, other: &'b Natural, rm: RoundingMode) -> Natural {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            self / other
        } else {
            let (q, r) = self.div_mod(other);
            if r == 0 {
                q
            } else {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => q.add_limb(1),
                    RoundingMode::Exact => panic!("Division is not exact: {} / {}", self, other),
                    RoundingMode::Nearest => div_round_nearest(q, &r, other),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl DivRoundAssign<Natural> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value and rounding according to a specified rounding mode.
    ///
    /// See the [`DivRound`](malachite_base::num::arithmetic::traits::DivRound) documentation for
    /// details.
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
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRoundAssign, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Down);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Floor);
    /// assert_eq!(n, 333333333333u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Up);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Ceiling);
    /// assert_eq!(n, 333333333334u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(5u32), RoundingMode::Exact);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(20u32);
    /// n.div_round_assign(Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n, 7);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(14u32);
    /// n.div_round_assign(Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n, 4);
    /// ```
    fn div_round_assign(&mut self, other: Natural, rm: RoundingMode) {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            *self /= other;
        } else {
            let r = self.div_assign_mod(&other);
            if r != 0 {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => {
                        *self += Natural::ONE;
                    }
                    RoundingMode::Exact => panic!("Division is not exact"),
                    RoundingMode::Nearest => div_round_assign_nearest(self, &r, &other),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl<'a> DivRoundAssign<&'a Natural> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference and rounding according to a specified rounding mode.
    ///
    /// See the [`DivRound`](malachite_base::num::arithmetic::traits::DivRound) documentation for
    /// details.
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
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{DivRoundAssign, Pow};
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Down);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Floor);
    /// assert_eq!(n, 333333333333u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Up);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(10u32).pow(12);
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Ceiling);
    /// assert_eq!(n, 333333333334u64);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(5u32), RoundingMode::Exact);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(20u32);
    /// n.div_round_assign(&Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(n, 7);
    ///
    /// let mut n = Natural::from(10u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n, 2);
    ///
    /// let mut n = Natural::from(14u32);
    /// n.div_round_assign(&Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(n, 4);
    /// ```
    fn div_round_assign(&mut self, other: &'a Natural, rm: RoundingMode) {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down {
            *self /= other;
        } else {
            let r = self.div_assign_mod(other);
            if r != 0 {
                match rm {
                    RoundingMode::Ceiling | RoundingMode::Up => {
                        *self += Natural::ONE;
                    }
                    RoundingMode::Exact => panic!("Division is not exact"),
                    RoundingMode::Nearest => div_round_assign_nearest(self, &r, other),
                    _ => unreachable!(),
                }
            }
        }
    }
}
