/// Inserts several copies of a value to the left (beginning) of a `Vec`. Using this function is
/// more efficient than inserting the values one by one.
///
/// Time: worst case O(n + m)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len()` before the function is called and m = `pad_size`
///
/// # Examples
/// ```
/// use malachite_base::vecs::vec_pad_left;
///
/// let mut xs = vec![1, 2, 3];
/// vec_pad_left::<u32>(&mut xs, 5, 10);
/// assert_eq!(xs, [10, 10, 10, 10, 10, 1, 2, 3]);
/// ```
pub fn vec_pad_left<T: Clone>(xs: &mut Vec<T>, pad_size: usize, pad_value: T) {
    let old_len = xs.len();
    xs.resize(old_len + pad_size, pad_value);
    for i in (0..old_len).rev() {
        xs.swap(i, i + pad_size);
    }
}

/// Deletes several values from the left (beginning) of a `Vec`. Using this function is more
/// efficient than deleting the values one by one.
///
/// Time: worst case O(max(1, n - m))
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` before the function is called and m = `delete_size`
///
/// # Panics
/// Panics if `delete_size` is greater than `xs.len()`.
///
/// # Examples
/// ```
/// use malachite_base::vecs::vec_delete_left;
///
/// let mut xs = vec![1, 2, 3, 4, 5];
/// vec_delete_left::<u32>(&mut xs, 3);
/// assert_eq!(xs, [4, 5]);
/// ```
pub fn vec_delete_left<T: Copy>(xs: &mut Vec<T>, delete_size: usize) {
    let old_len = xs.len();
    xs.copy_within(delete_size..old_len, 0);
    xs.truncate(old_len - delete_size);
}

pub mod from_str;
