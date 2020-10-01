use std::iter::{once, Once};

/// Generates the only unit: `()`.
///
/// The output length is 1.
///
/// # Worst-case complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::tuples::exhaustive::exhaustive_units;
///
/// assert_eq!(exhaustive_units().collect::<Vec<_>>(), &[()]);
/// ```
pub fn exhaustive_units() -> Once<()> {
    once(())
}
