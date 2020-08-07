use num::basic::traits::Zero;

/// Generates all the nonzero values of a provided iterator.
///
/// This `struct` is created by the `nonzero_values` method. See its documentation for more.
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

/// Returns an iterator that generates all the nonzero values of a provided iterator.
///
/// `nonzero_values(xs)` generates the same values as `xs.filter(|x| x != T::ZERO)`, but its type is
/// easier to work with.
///
/// This iterator will hang if given an iterator that produces an infinite suffix of zeros.
///
/// The output length is the number of nonzero values produced by `xs`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::nonzero_values;
///
/// assert_eq!(
///     nonzero_values([-3i8, -2, -1, 0, 1, 2, 3].iter().cloned()).collect::<Vec<_>>(),
///     &[-3, -2, -1, 1, 2, 3]
/// )
/// ```
#[inline]
pub fn nonzero_values<I: Iterator>(xs: I) -> NonzeroValues<I>
where
    I::Item: Eq + Zero,
{
    NonzeroValues(xs)
}

/// This module contains functions that compare adjacent iterator elements.
pub mod comparison;
