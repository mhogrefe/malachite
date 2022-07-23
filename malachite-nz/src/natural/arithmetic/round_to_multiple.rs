use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode;
use std::cmp::Ordering;

impl RoundToMultiple<Natural> for Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of another [`Natural`], according to a specified
    /// rounding mode. Both [`Natural`]s are taken by value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \N$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(5u32).round_to_multiple(Natural::ZERO, RoundingMode::Down), 0);
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple(Natural::from(4u32), RoundingMode::Down),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple(Natural::from(4u32), RoundingMode::Up),
    ///     12
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple(Natural::from(5u32), RoundingMode::Exact),
    ///     10
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple(Natural::from(3u32), RoundingMode::Nearest),
    ///     9
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32).round_to_multiple(Natural::from(3u32), RoundingMode::Nearest),
    ///     21
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple(Natural::from(4u32), RoundingMode::Nearest),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32).round_to_multiple(Natural::from(4u32), RoundingMode::Nearest),
    ///     16
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: Natural, rm: RoundingMode) -> Natural {
        self.round_to_multiple_assign(other, rm);
        self
    }
}

impl<'a> RoundToMultiple<&'a Natural> for Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of another [`Natural`], according to a specified
    /// rounding mode. The first [`Natural`] is taken by value and the second by reference.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \N$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(5u32).round_to_multiple(&Natural::ZERO, RoundingMode::Down), 0);
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple(&Natural::from(4u32), RoundingMode::Down),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple(&Natural::from(4u32), RoundingMode::Up),
    ///     12
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple(&Natural::from(5u32), RoundingMode::Exact),
    ///     10
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple(&Natural::from(3u32), RoundingMode::Nearest),
    ///     9
    /// );
    /// assert_eq!(
    ///     Natural::from(20u32).round_to_multiple(&Natural::from(3u32), RoundingMode::Nearest),
    ///     21
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).round_to_multiple(&Natural::from(4u32), RoundingMode::Nearest),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(14u32).round_to_multiple(&Natural::from(4u32), RoundingMode::Nearest),
    ///     16
    /// );
    /// ```
    #[inline]
    fn round_to_multiple(mut self, other: &'a Natural, rm: RoundingMode) -> Natural {
        self.round_to_multiple_assign(other, rm);
        self
    }
}

impl<'a> RoundToMultiple<Natural> for &'a Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of another [`Natural`], according to a specified
    /// rounding mode. The first [`Natural`] is taken by reference and the second by value.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \N$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(5u32)).round_to_multiple(Natural::ZERO, RoundingMode::Down), 0);
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple(Natural::from(4u32), RoundingMode::Down),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple(Natural::from(4u32), RoundingMode::Up),
    ///     12
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple(Natural::from(5u32), RoundingMode::Exact),
    ///     10
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple(Natural::from(3u32), RoundingMode::Nearest),
    ///     9
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32)).round_to_multiple(Natural::from(3u32), RoundingMode::Nearest),
    ///     21
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple(Natural::from(4u32), RoundingMode::Nearest),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32)).round_to_multiple(Natural::from(4u32), RoundingMode::Nearest),
    ///     16
    /// );
    /// ```
    fn round_to_multiple(self, other: Natural, rm: RoundingMode) -> Natural {
        match (self, other) {
            (x, y) if *x == y => y,
            (x, natural_zero!()) => match rm {
                RoundingMode::Down | RoundingMode::Floor | RoundingMode::Nearest => Natural::ZERO,
                _ => panic!("Cannot round {} to zero using RoundingMode {}", x, rm),
            },
            (x, y) => {
                let r = x % &y;
                if r == 0 {
                    x.clone()
                } else {
                    let floor = x - &r;
                    match rm {
                        RoundingMode::Down | RoundingMode::Floor => floor,
                        RoundingMode::Up | RoundingMode::Ceiling => floor + y,
                        RoundingMode::Nearest => {
                            match (r << 1u64).cmp(&y) {
                                Ordering::Less => floor,
                                Ordering::Greater => floor + y,
                                Ordering::Equal => {
                                    // The even multiple of y will have more trailing zeros.
                                    if floor == 0 {
                                        floor
                                    } else {
                                        let ceiling = &floor + y;
                                        if floor.trailing_zeros() > ceiling.trailing_zeros() {
                                            floor
                                        } else {
                                            ceiling
                                        }
                                    }
                                }
                            }
                        }
                        RoundingMode::Exact => {
                            panic!("Cannot round {} to {} using RoundingMode {}", x, y, rm)
                        }
                    }
                }
            }
        }
    }
}

impl<'a, 'b> RoundToMultiple<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Rounds a [`Natural`] to a multiple of another [`Natural`], according to a specified
    /// rounding mode. Both [`Natural`]s are taken by reference.
    ///
    /// Let $q = \frac{x}{y}$:
    ///
    /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
    ///
    /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
    ///
    /// $$
    /// f(x, y, \mathrm{Nearest}) = \begin{cases}
    ///     y \lfloor q \rfloor & \text{if} \\quad
    ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
    ///     y \lfloor q \rfloor &
    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
    ///     \\ \text{is even} \\\\
    ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
    ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
    /// \end{cases}
    /// $$
    ///
    /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \N$.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple(other, RoundingMode::Exact)`
    /// - `{ assert!(x.divisible_by(other)); x }`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultiple;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(5u32)).round_to_multiple(&Natural::ZERO, RoundingMode::Down), 0);
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple(&Natural::from(4u32), RoundingMode::Down),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple(&Natural::from(4u32), RoundingMode::Up),
    ///     12
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple(&Natural::from(5u32), RoundingMode::Exact),
    ///     10
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple(&Natural::from(3u32), RoundingMode::Nearest),
    ///     9
    /// );
    /// assert_eq!(
    ///     (&Natural::from(20u32)).round_to_multiple(&Natural::from(3u32), RoundingMode::Nearest),
    ///     21
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).round_to_multiple(&Natural::from(4u32), RoundingMode::Nearest),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(14u32)).round_to_multiple(&Natural::from(4u32), RoundingMode::Nearest),
    ///     16
    /// );
    /// ```
    fn round_to_multiple(self, other: &'b Natural, rm: RoundingMode) -> Natural {
        match (self, other) {
            (x, y) if x == y => x.clone(),
            (x, &natural_zero!()) => match rm {
                RoundingMode::Down | RoundingMode::Floor | RoundingMode::Nearest => Natural::ZERO,
                _ => panic!("Cannot round {} to zero using RoundingMode {}", x, rm),
            },
            (x, y) => {
                let r = x % y;
                if r == 0 {
                    x.clone()
                } else {
                    let floor = x - &r;
                    match rm {
                        RoundingMode::Down | RoundingMode::Floor => floor,
                        RoundingMode::Up | RoundingMode::Ceiling => floor + y,
                        RoundingMode::Nearest => {
                            match (r << 1u64).cmp(y) {
                                Ordering::Less => floor,
                                Ordering::Greater => floor + y,
                                Ordering::Equal => {
                                    // The even multiple of y will have more trailing zeros.
                                    if floor == 0 {
                                        floor
                                    } else {
                                        let ceiling = &floor + y;
                                        if floor.trailing_zeros() > ceiling.trailing_zeros() {
                                            floor
                                        } else {
                                            ceiling
                                        }
                                    }
                                }
                            }
                        }
                        RoundingMode::Exact => {
                            panic!("Cannot round {} to {} using RoundingMode {}", x, y, rm)
                        }
                    }
                }
            }
        }
    }
}

impl RoundToMultipleAssign<Natural> for Natural {
    /// Rounds a [`Natural`] to a multiple of another [`Natural`] in place, according to a
    /// specified rounding mode. The [`Natural`] on the right-hand side is taken by value.
    ///
    /// See the
    /// [`RoundToMultiple`](malachite_base::num::arithmetic::traits::RoundToMultiple) documentation
    /// for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_assign(other, RoundingMode::Exact);`
    /// - `assert!(x.divisible_by(other));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(5u32);
    /// x.round_to_multiple_assign(Natural::ZERO, RoundingMode::Down);
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.round_to_multiple_assign(Natural::from(4u32), RoundingMode::Down);
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.round_to_multiple_assign(Natural::from(4u32), RoundingMode::Up);
    /// assert_eq!(x, 12);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.round_to_multiple_assign(Natural::from(5u32), RoundingMode::Exact);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.round_to_multiple_assign(Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(20u32);
    /// x.round_to_multiple_assign(Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(x, 21);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.round_to_multiple_assign(Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(14u32);
    /// x.round_to_multiple_assign(Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(x, 16);
    /// ```
    fn round_to_multiple_assign(&mut self, other: Natural, rm: RoundingMode) {
        match (&mut *self, other) {
            (x, y) if *x == y => {}
            (x, natural_zero!()) => match rm {
                RoundingMode::Down | RoundingMode::Floor | RoundingMode::Nearest => {
                    *self = Natural::ZERO
                }
                _ => panic!("Cannot round {} to zero using RoundingMode {}", x, rm),
            },
            (x, y) => {
                let r = &*x % &y;
                if r != 0 {
                    *x -= &r;
                    match rm {
                        RoundingMode::Down | RoundingMode::Floor => {}
                        RoundingMode::Up | RoundingMode::Ceiling => *x += y,
                        RoundingMode::Nearest => {
                            match (r << 1u64).cmp(&y) {
                                Ordering::Less => {}
                                Ordering::Greater => *x += y,
                                Ordering::Equal => {
                                    // The even multiple of y will have more trailing zeros.
                                    if *x != 0 {
                                        let ceiling = &*x + y;
                                        if x.trailing_zeros() < ceiling.trailing_zeros() {
                                            *x = ceiling;
                                        }
                                    }
                                }
                            }
                        }
                        RoundingMode::Exact => {
                            panic!("Cannot round {} to {} using RoundingMode {}", x, y, rm)
                        }
                    }
                }
            }
        }
    }
}

impl<'a> RoundToMultipleAssign<&'a Natural> for Natural {
    /// Rounds a [`Natural`] to a multiple of another [`Natural`] in place, according to a
    /// specified rounding mode. The [`Natural`] on the right-hand side is taken by reference.
    ///
    /// See the
    /// [`RoundToMultiple`](malachite_base::num::arithmetic::traits::RoundToMultiple) documentation
    /// for details.
    ///
    /// The following two expressions are equivalent:
    /// - `x.round_to_multiple_assign(other, RoundingMode::Exact);`
    /// - `assert!(x.divisible_by(other));`
    ///
    /// but the latter should be used as it is clearer and more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
    /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::RoundToMultipleAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(5u32);
    /// x.round_to_multiple_assign(&Natural::ZERO, RoundingMode::Down);
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.round_to_multiple_assign(&Natural::from(4u32), RoundingMode::Down);
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.round_to_multiple_assign(&Natural::from(4u32), RoundingMode::Up);
    /// assert_eq!(x, 12);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.round_to_multiple_assign(&Natural::from(5u32), RoundingMode::Exact);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.round_to_multiple_assign(&Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(20u32);
    /// x.round_to_multiple_assign(&Natural::from(3u32), RoundingMode::Nearest);
    /// assert_eq!(x, 21);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.round_to_multiple_assign(&Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(14u32);
    /// x.round_to_multiple_assign(&Natural::from(4u32), RoundingMode::Nearest);
    /// assert_eq!(x, 16);
    /// ```
    fn round_to_multiple_assign(&mut self, other: &'a Natural, rm: RoundingMode) {
        match (&mut *self, other) {
            (x, y) if *x == *y => {}
            (x, natural_zero!()) => match rm {
                RoundingMode::Down | RoundingMode::Floor | RoundingMode::Nearest => {
                    *self = Natural::ZERO
                }
                _ => panic!("Cannot round {} to zero using RoundingMode {}", x, rm),
            },
            (x, y) => {
                let r = &*x % y;
                if r != 0 {
                    *x -= &r;
                    match rm {
                        RoundingMode::Down | RoundingMode::Floor => {}
                        RoundingMode::Up | RoundingMode::Ceiling => *x += y,
                        RoundingMode::Nearest => {
                            match (r << 1u64).cmp(y) {
                                Ordering::Less => {}
                                Ordering::Greater => *x += y,
                                Ordering::Equal => {
                                    // The even multiple of y will have more trailing zeros.
                                    if *x != 0 {
                                        let ceiling = &*x + y;
                                        if x.trailing_zeros() < ceiling.trailing_zeros() {
                                            *x = ceiling;
                                        }
                                    }
                                }
                            }
                        }
                        RoundingMode::Exact => {
                            panic!("Cannot round {} to {} using RoundingMode {}", x, y, rm)
                        }
                    }
                }
            }
        }
    }
}
