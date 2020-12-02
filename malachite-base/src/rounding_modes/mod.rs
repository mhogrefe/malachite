use named::Named;

/// An enum that specifies how a value should be rounded.
///
/// For more information, check the module-level documentation.
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

/// A list of all six rounding modes.
pub const ROUNDING_MODES: [RoundingMode; 6] = [
    RoundingMode::Down,
    RoundingMode::Up,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
    RoundingMode::Nearest,
    RoundingMode::Exact,
];

/// This module contains a `Display` impl for `RoundingMode`.
pub mod display;
/// This module contains iterators that generate `RoundingMode`s without repetition.
pub mod exhaustive;
/// This module contains a `FromStr` impl for `RoundingMode`.
pub mod from_str;
/// This module contains a `Neg` impl for `RoundingMode`.
pub mod neg;
/// This module contains iterators that generate `RoundingMode`s randomly.
pub mod random;
