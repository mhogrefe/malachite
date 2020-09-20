/// Generates all singletons (1-element tuples) with values from a given iterator.
///
/// This `struct` is created by the `singletons` method. See its documentation for more.
pub struct Singletons<I: Iterator> {
    xs: I,
}

impl<I: Iterator> Iterator for Singletons<I> {
    type Item = (I::Item,);

    #[inline]
    fn next(&mut self) -> Option<(I::Item,)> {
        self.xs.next().map(|x| (x,))
    }
}

/// Generates all singletons (1-element tuples) with values from a given iterator.
///
/// The elements appear in the same order as they do in the given iterator, but wrapped in `(_,)`.
///
/// The output length is `xs.count()`.
///
/// # Complexity per iteration
///
/// Same as the time and additional memory complexity of iterating `xs`.
///
/// # Examples
/// ```
/// use malachite_base::tuples::singletons;
///
/// assert_eq!(singletons([1, 2, 3].iter().cloned()).collect::<Vec<_>>(), &[(1,), (2,), (3,)]);
/// ```
#[inline]
pub fn singletons<I: Iterator>(xs: I) -> Singletons<I> {
    Singletons { xs }
}
