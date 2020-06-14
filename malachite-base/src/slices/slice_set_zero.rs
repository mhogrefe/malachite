use num::basic::traits::Zero;

/// Sets all values in a slice to 0.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_set_zero::slice_set_zero;
///
/// let mut xs = [1, 2, 3, 4, 5];
/// slice_set_zero::<u32>(&mut xs[1..4]);
/// assert_eq!(xs, [1, 0, 0, 0, 5]);
/// ```
///
/// This is mpn_zero from mpn/generic/zero.c, GMP 6.1.2. Note that this is needed less often in
/// Malachite than in GMP, since Malachite generally initializes new memory with zeros.
pub fn slice_set_zero<T: Zero>(xs: &mut [T]) {
    for x in xs.iter_mut() {
        *x = T::ZERO;
    }
}
