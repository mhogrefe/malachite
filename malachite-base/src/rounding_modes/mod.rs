use named::Named;

/// An enum that specifies how a value should be rounded.
///
/// A `RoundingMode` can often be specified when a function conceptually returns a value of one
/// type, but must be rounded to another type. The most common case is a conceptually real-valued
/// function whose result must be rounded to an integer, like `div_round`.
///
/// When converting a real value to an integer, the different `RoundingMode`s act as follows:
/// - `Floor` applies the floor function: $x \mapsto \lfloor x \rfloor$. In other words, the value
///   is rounded towards $-\infty$.
/// - `Ceiling` applies the ceiling function: $x \mapsto \lceil x \rceil$. In other words, the value
///   is rounded towards $\infty$.
/// - `Down` applies the function $x \mapsto \operatorname{sgn}(x) \lfloor |x| \rfloor$. In other
///   words, the value is rounded towards $0$.
/// - `Up` applies the function $x \mapsto \operatorname{sgn}(x) \lceil |x| \rceil$. In other words,
///   the value is rounded away from $0$.
/// - `Nearest` applies the function
///   $$
///     x \mapsto \\begin{cases}
///         \lfloor x \rfloor & x - \lfloor x \rfloor < \frac{1}{2} \\\\
///         \lceil x \rceil & x - \lfloor x \rfloor > \frac{1}{2} \\\\
///         \lfloor x \rfloor &
///    x - \lfloor x \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor x \rfloor \\ \text{is even} \\\\
///         \lceil x \rceil &
///    x - \lfloor x \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor x \rfloor \\ \text{is odd.}
///     \\end{cases}
///   $$
///   In other words, it rounds to the nearest integer, and when there's a tie, it rounds to the
///   nearest even integer. This is also called _bankers' rounding_ and is often used as a default.
/// - `Exact` panics if the value is not already rounded.
///
/// # Examples
/// Here are some examples of how floating-point values would be rounded to integer values using the
/// different `RoundingMode`s.
///
/// | x    | `Floor` | `Ceiling` | `Down` | `Up` | `Nearest` | `Exact`    |
/// |------|---------|-----------|--------|------|-----------|------------|
/// |  3.0 |       3 |         3 |      3 |    3 |         3 |          3 |
/// |  3.2 |       3 |         4 |      3 |    4 |         3 | `panic!()` |
/// |  3.8 |       3 |         4 |      3 |    4 |         4 | `panic!()` |
/// |  3.5 |       3 |         4 |      3 |    4 |         4 | `panic!()` |
/// |  4.5 |       4 |         5 |      4 |    5 |         4 | `panic!()` |
/// | -3.2 |      -4 |        -3 |     -3 |   -4 |        -3 | `panic!()` |
/// | -3.8 |      -4 |        -3 |     -3 |   -4 |        -4 | `panic!()` |
/// | -3.5 |      -4 |        -3 |     -3 |   -4 |        -4 | `panic!()` |
/// | -4.5 |      -5 |        -4 |     -4 |   -5 |        -4 | `panic!()` |
///
/// Sometimes a `RoundingMode` is used in an unusual context, such as rounding an integer to a
/// floating-point number, in which case further explanation of its behavior is provided at the
/// usage site.
///
/// A `RoundingMode` takes up 1 byte of space.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RoundingMode {
    Down,
    Up,
    Floor,
    Ceiling,
    Nearest,
    Exact,
}

impl_named!(RoundingMode);

/// A list of all six rounding modes.
pub const ROUNDING_MODES: [RoundingMode; 6] = [
    RoundingMode::Down,
    RoundingMode::Up,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
    RoundingMode::Nearest,
    RoundingMode::Exact,
];

/// The `Display` impl for `RoundingMode`.
pub mod display;
/// Iterators that generate `RoundingMode`s without repetition.
pub mod exhaustive;
/// The `FromStr` impl for `RoundingMode`.
pub mod from_str;
/// The `Neg` and `NegAssign` impls for `RoundingMode`.
pub mod neg;
/// Iterators that generate `RoundingMode`s randomly.
pub mod random;
