/// Tests whether all limbs in a slice are equal to 0.
///
/// Equivalent to GMP's `mpn_zero_p`.
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
/// assert!(limbs_test_zero(&[0, 0, 0]));
/// assert!(!limbs_test_zero(&[0, 1, 0]));
/// ```
pub fn limbs_test_zero(limbs: &[u32]) -> bool {
    limbs.iter().all(|&limb| limb == 0)
}

/// Sets all limbs in a slice to 0.
///
/// Equivalent to GMP's `mpn_zero`. Note that this is needed less often in Malachite than in GMP,
/// since Malachite generally initializes new memory with zeros.
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
/// limbs_set_zero(&mut limbs[1..4]);
/// assert_eq!(limbs, [1, 0, 0, 0, 5]);
/// ```
pub fn limbs_set_zero(limbs: &mut [u32]) {
    for limb in limbs.iter_mut() {
        *limb = 0;
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
/// limbs_pad_left(&mut limbs, 5, 10);
/// assert_eq!(limbs, [10, 10, 10, 10, 10, 1, 2, 3]);
/// ```
pub fn limbs_pad_left(limbs: &mut Vec<u32>, pad_size: usize, pad_limb: u32) {
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
/// limbs_delete_left(&mut limbs, 3);
/// assert_eq!(limbs, [4, 5]);
/// ```
pub fn limbs_delete_left(limbs: &mut Vec<u32>, delete_size: usize) {
    assert!(delete_size <= limbs.len());
    let remaining_size = limbs.len() - delete_size;
    for i in 0..remaining_size {
        limbs.swap(i, i + delete_size);
    }
    limbs.truncate(remaining_size);
}
