use std::fmt::{self, Display, Formatter};
use std::ops::Neg;
use std::str::FromStr;

/// A `RoundingMode` is used to specify the rounding behavior of a function whose codomain is
/// restricted. For example, consider a function which divides two integers. The natural codomain
/// of this function would be the set of rational numbers, but sometimes we want to perform what is
/// called integer division, where the codomain is the set of integers. This involves making a
/// a decision about how to round the rational that would result from the ordinary division to an
/// integer. There are six available rounding modes:
///
/// - `Down` rounds toward zero; this is also called truncation. 2.2 is rounded to 2 and -2.2 to
///   -2.
/// - `Up` rounds away from zero. 2.2 is rounded to 3 and -2.2 to -3.
/// - `Floor` rounds toward negative infinity. 2.2 is rounded to 2 and -2.2 to -3.
/// - `Ceiling` rounds toward positive infinity. 2.2 is rounded to 3 and -2.2 to -2.
/// - `Nearest` rounds toward the closest value, so 2.2 is rounded to 2 and 2.8 to 3. When there is
///   a tie between the two closest values, the round-to-even rule (also called bankers' rounding)
///   is used, which rounds toward the even number; so 2.5 rounds to 2 and 3.5 to 4.
/// - `Exact` asserts that the function requires no rounding at a particular value. Dividing 12 by
///   4 using this mode would give 3, but dividing 12 by 5 would panic.
///
/// Sometimes a `RoundingMode` is used in an unusual context, such as rounding _to_ a floating-point
/// number, in which case further explanation of its behavior is provided at the usage site.
///
/// A `RoundingMode` takes up 1 byte of space.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RoundingMode {
    Down,
    Up,
    Floor,
    Ceiling,
    Nearest,
    Exact,
}

/// Returns the negative of a `RoundingMode`. The negative is defined so that for an odd function f,
/// f(x, -rm) = -f(-x, rm). Floor and ceiling are swapped, and the other modes are unchanged.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_base::round::RoundingMode;
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

impl Display for RoundingMode {
    /// Converts a `RoundingMode` to a `String`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::round::RoundingMode;
    ///
    /// assert_eq!(RoundingMode::Down.to_string(), "Down");
    /// assert_eq!(RoundingMode::Up.to_string(), "Up");
    /// assert_eq!(RoundingMode::Floor.to_string(), "Floor");
    /// assert_eq!(RoundingMode::Ceiling.to_string(), "Ceiling");
    /// assert_eq!(RoundingMode::Nearest.to_string(), "Nearest");
    /// assert_eq!(RoundingMode::Exact.to_string(), "Exact");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl FromStr for RoundingMode {
    type Err = String;

    /// Converts a `String` to a `RoundingMode`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `src.len()`
    ///
    /// # Examples
    /// ```
    /// use malachite_base::round::RoundingMode;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(RoundingMode::from_str("Down"), Ok(RoundingMode::Down));
    /// assert_eq!(RoundingMode::from_str("Up"), Ok(RoundingMode::Up));
    /// assert_eq!(RoundingMode::from_str("Floor"), Ok(RoundingMode::Floor));
    /// assert_eq!(RoundingMode::from_str("Ceiling"), Ok(RoundingMode::Ceiling));
    /// assert_eq!(RoundingMode::from_str("Nearest"), Ok(RoundingMode::Nearest));
    /// assert_eq!(RoundingMode::from_str("Exact"), Ok(RoundingMode::Exact));
    /// assert_eq!(RoundingMode::from_str("abc"), Err("abc".to_string()));
    /// ```
    #[inline]
    fn from_str(src: &str) -> Result<RoundingMode, String> {
        match src {
            "Down" => Ok(RoundingMode::Down),
            "Up" => Ok(RoundingMode::Up),
            "Floor" => Ok(RoundingMode::Floor),
            "Ceiling" => Ok(RoundingMode::Ceiling),
            "Nearest" => Ok(RoundingMode::Nearest),
            "Exact" => Ok(RoundingMode::Exact),
            _ => Err(src.to_owned()),
        }
    }
}
