/// Given a slice `xs` and an starting index, copies the contents of `&xs[starting_index..]` to
/// `&xs[..xs.len() - starting_index]`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `amount` is greater than the length of `xs`.
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_move_left::slice_move_left;
///
/// let xs = &mut [1, 2, 3, 4, 5, 6];
/// slice_move_left::<u32>(xs, 2);
/// assert_eq!(xs, &[3, 4, 5, 6, 5, 6]);
/// ```
#[inline]
pub fn slice_move_left<T: Copy>(xs: &mut [T], starting_index: usize) {
    xs.copy_within(starting_index..xs.len(), 0);
}
