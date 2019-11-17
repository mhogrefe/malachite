use num::basic::traits::Zero;

/// Tests whether all limbs in a slice are equal to 0.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Examples
/// ```
/// use malachite_base::limbs::limbs_test_zero;
///
/// assert!(limbs_test_zero::<u32>(&[0, 0, 0]));
/// assert!(!limbs_test_zero::<u32>(&[0, 1, 0]));
/// ```
///
/// This is mpn_zero_p from gmp.h.
pub fn limbs_test_zero<T: Copy + Eq + Zero>(limbs: &[T]) -> bool {
    limbs.iter().all(|&limb| limb == T::ZERO)
}

/// Sets all limbs in a slice to 0.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Examples
/// ```
/// use malachite_base::limbs::limbs_set_zero;
///
/// let mut limbs = [1, 2, 3, 4, 5];
/// limbs_set_zero::<u32>(&mut limbs[1..4]);
/// assert_eq!(limbs, [1, 0, 0, 0, 5]);
/// ```
///
/// This is mpn_zero from mpn/generic/zero.c. Note that this is needed less often in Malachite than
/// in GMP, since Malachite generally initializes new memory with zeros.
pub fn limbs_set_zero<T: Zero>(limbs: &mut [T]) {
    for limb in limbs.iter_mut() {
        *limb = T::ZERO;
    }
}

/// Inserts several copies of a limb to the left (beginning) of a `Vec` of limbs. Using this
/// function is more efficient than inserting the limbs one by one.
///
/// Time: worst case O(n + m)
///
/// Additional memory: worst case O(m)
///
/// where n = `limbs.len()` before the function is called and m = `pad_size`
///
/// # Examples
/// ```
/// use malachite_base::limbs::limbs_pad_left;
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_pad_left::<u32>(&mut limbs, 5, 10);
/// assert_eq!(limbs, [10, 10, 10, 10, 10, 1, 2, 3]);
/// ```
pub fn limbs_pad_left<T: Clone>(limbs: &mut Vec<T>, pad_size: usize, pad_limb: T) {
    let old_len = limbs.len();
    limbs.resize(old_len + pad_size, pad_limb);
    for i in (0..old_len).rev() {
        limbs.swap(i, i + pad_size);
    }
}

/// Deletes several limbs from the left (beginning) of a `Vec` of limbs. Using this function is more
/// efficient than deleting the limbs one by one.
///
/// Time: worst case O(max(1, n - m))
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()` before the function is called and m = `delete_size`
///
/// # Panics
/// Panics if `delete_size` is greater than `limbs.len()`.
///
/// # Examples
/// ```
/// use malachite_base::limbs::limbs_delete_left;
///
/// let mut limbs = vec![1, 2, 3, 4, 5];
/// limbs_delete_left::<u32>(&mut limbs, 3);
/// assert_eq!(limbs, [4, 5]);
/// ```
pub fn limbs_delete_left<T: Copy>(limbs: &mut Vec<T>, delete_size: usize) {
    let old_len = limbs.len();
    limbs.copy_within(delete_size..old_len, 0);
    limbs.truncate(old_len - delete_size);
}

/// Counts the number of zero limbs that a slice starts with.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Examples
/// ```
/// use malachite_base::limbs::limbs_leading_zero_limbs;
///
/// assert_eq!(limbs_leading_zero_limbs::<u32>(&[1, 2, 3]), 0);
/// assert_eq!(limbs_leading_zero_limbs::<u32>(&[0, 0, 0, 1, 2, 3]), 3);
/// ```
pub fn limbs_leading_zero_limbs<T: Copy + Eq + Zero>(limbs: &[T]) -> usize {
    limbs.iter().take_while(|&&limb| limb == T::ZERO).count()
}

/// Counts the number of zero limbs that a slice ends with.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Examples
/// ```
/// use malachite_base::limbs::limbs_trailing_zero_limbs;
///
/// assert_eq!(limbs_trailing_zero_limbs::<u32>(&[1, 2, 3]), 0);
/// assert_eq!(limbs_trailing_zero_limbs::<u32>(&[1, 2, 3, 0, 0, 0]), 3);
/// ```
pub fn limbs_trailing_zero_limbs<T: Copy + Eq + Zero>(limbs: &[T]) -> usize {
    limbs
        .iter()
        .rev()
        .take_while(|&&limb| limb == T::ZERO)
        .count()
}

/// Given a slice of limbs and an amount, copies the contents of `&limbs[amount..]` to
/// `&limbs[..limbs.len() - amount]`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `amount` is greater than the length of `limbs`.
///
/// # Examples
/// ```
/// use malachite_base::limbs::limbs_move_left;
///
/// let limbs = &mut [1, 2, 3, 4, 5, 6];
/// limbs_move_left::<u32>(limbs, 2);
/// assert_eq!(limbs, &[3, 4, 5, 6, 5, 6]);
/// ```
#[inline]
pub fn limbs_move_left<T: Copy>(limbs: &mut [T], amount: usize) {
    limbs.copy_within(amount..limbs.len(), 0);
}
