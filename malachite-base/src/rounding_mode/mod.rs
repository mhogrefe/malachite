use named::Named;

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

impl_named!(RoundingMode);

pub mod display;
pub mod from_str;
pub mod neg;
