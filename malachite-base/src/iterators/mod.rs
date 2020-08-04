use num::basic::traits::Zero;

/// Filters out zeros from an iterator.
#[derive(Clone, Debug)]
pub struct NonzeroValues<I: Iterator>(I)
where
    I::Item: Eq + Zero;

impl<I: Iterator> Iterator for NonzeroValues<I>
where
    I::Item: Eq + Zero,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        loop {
            let x = self.0.next();
            if x != Some(I::Item::ZERO) {
                return x;
            }
        }
    }
}

/// Filters out zeros from an iterator.
///
/// This iterator will hang if given an iterator that produces an infinite suffix of zeros.
///
/// Length is the number of nonzero values produced by `xs`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::nonzero_values;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_primitive_integers;
///
/// assert_eq!(
///     nonzero_values(random_primitive_integers::<u8>(EXAMPLE_SEED)).take(10)
///         .collect::<Vec<_>>(),
///     &[113, 239, 69, 108, 228, 210, 168, 161, 87, 32]
/// )
/// ```
#[inline]
pub fn nonzero_values<I: Iterator>(xs: I) -> NonzeroValues<I>
where
    I::Item: Eq + Zero,
{
    NonzeroValues(xs)
}

pub mod comparison;
