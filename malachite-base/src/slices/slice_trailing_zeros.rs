use num::basic::traits::Zero;

/// Counts the number of zeros that a slice ends with.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_trailing_zeros::slice_trailing_zeros;
///
/// assert_eq!(slice_trailing_zeros::<u32>(&[1, 2, 3]), 0);
/// assert_eq!(slice_trailing_zeros::<u32>(&[1, 2, 3, 0, 0, 0]), 3);
/// ```
pub fn slice_trailing_zeros<T: Eq + Zero>(xs: &[T]) -> usize {
    let zero = T::ZERO;
    xs.iter().rev().take_while(|&x| x == &zero).count()
}
