use std::ops::Neg;

use rounding_modes::RoundingMode;

/// Returns the negative of a `RoundingMode`. The negative is defined so that for an odd function f,
/// f(x, -rm) = -f(-x, rm). Floor and ceiling are swapped, and the other modes are unchanged.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
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
