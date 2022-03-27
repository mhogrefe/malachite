use num::arithmetic::traits::NegAssign;
use rounding_modes::RoundingMode;
use std::ops::Neg;

/// Returns the negative of a `RoundingMode`.
///
/// The negative is defined so that if a `RoundingMode` $m$ is used to round the result of an odd
/// function $f$, $f(x, -m) = -f(-x, m)$. `Floor` and `Ceiling` are swapped, and the other modes are
/// unchanged.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::rounding_modes::RoundingMode;
///
/// assert_eq!(-RoundingMode::Down, RoundingMode::Down);
/// assert_eq!(-RoundingMode::Up, RoundingMode::Up);
/// assert_eq!(-RoundingMode::Floor, RoundingMode::Ceiling);
/// assert_eq!(-RoundingMode::Ceiling, RoundingMode::Floor);
/// assert_eq!(-RoundingMode::Nearest, RoundingMode::Nearest);
/// assert_eq!(-RoundingMode::Exact, RoundingMode::Exact);
/// ```
impl Neg for RoundingMode {
    type Output = RoundingMode;

    #[inline]
    fn neg(self) -> RoundingMode {
        match self {
            RoundingMode::Floor => RoundingMode::Ceiling,
            RoundingMode::Ceiling => RoundingMode::Floor,
            rm => rm,
        }
    }
}

impl NegAssign for RoundingMode {
    /// Replaces a `RoundingMode` with its negative.
    ///
    /// The negative is defined so that if a `RoundingMode` $m$ is used to round the result of an
    /// odd function $f$, $f(x, -m) = -f(-x, m)$. `Floor` and `Ceiling` are swapped, and the other
    /// modes are unchanged.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_base::rounding_modes::RoundingMode;
    ///
    /// let mut rm = RoundingMode::Down;
    /// rm.neg_assign();
    /// assert_eq!(rm, RoundingMode::Down);
    ///
    /// let mut rm = RoundingMode::Floor;
    /// rm.neg_assign();
    /// assert_eq!(rm, RoundingMode::Ceiling);
    /// ```
    #[inline]
    fn neg_assign(&mut self) {
        if *self == RoundingMode::Floor {
            *self = RoundingMode::Ceiling;
        } else if *self == RoundingMode::Ceiling {
            *self = RoundingMode::Floor
        }
    }
}
