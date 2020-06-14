use num::basic::traits::Zero;

/// Tests whether all values in a slice are equal to 0.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_base::slices::slice_test_zero::slice_test_zero;
///
/// assert!(slice_test_zero::<u32>(&[0, 0, 0]));
/// assert!(!slice_test_zero::<u32>(&[0, 1, 0]));
/// ```
///
/// This is mpn_zero_p from gmp.h, GMP 6.1.2.
pub fn slice_test_zero<T: Eq + Zero>(xs: &[T]) -> bool {
    let zero = T::ZERO;
    xs.iter().all(|x| x == &zero)
}
