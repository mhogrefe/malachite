use std::iter::{repeat, Repeat};

/// Generates random units; repeats `()`.
///
/// $P(()) = 1$.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::tuples::random::random_units;
///
/// assert_eq!(random_units().take(10).collect::<Vec<_>>(), &[(); 10]);
/// ```
pub fn random_units() -> Repeat<()> {
    repeat(())
}
