use num::PrimitiveUnsigned;

//TODO test other unsigned besides u32
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
/// assert!(limbs_test_zero::<u32>(&[0, 0, 0]));
/// assert!(!limbs_test_zero::<u32>(&[0, 1, 0]));
/// ```
pub fn limbs_test_zero<T: PrimitiveUnsigned>(limbs: &[T]) -> bool {
    limbs.iter().all(|&limb| limb == T::ZERO)
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
/// limbs_set_zero::<u32>(&mut limbs[1..4]);
/// assert_eq!(limbs, [1, 0, 0, 0, 5]);
/// ```
pub fn limbs_set_zero<T: PrimitiveUnsigned>(limbs: &mut [T]) {
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
pub fn limbs_pad_left<T: PrimitiveUnsigned>(limbs: &mut Vec<T>, pad_size: usize, pad_limb: T) {
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
pub fn limbs_delete_left<T: PrimitiveUnsigned>(limbs: &mut Vec<T>, delete_size: usize) {
    assert!(delete_size <= limbs.len());
    let remaining_size = limbs.len() - delete_size;
    for i in 0..remaining_size {
        limbs.swap(i, i + delete_size);
    }
    limbs.truncate(remaining_size);
}

//TODO docs and tests
pub fn limbs_leading_zero_limbs<T: PrimitiveUnsigned>(limbs: &[T]) -> usize {
    limbs.iter().take_while(|&&limb| limb == T::ZERO).count()
}

pub fn limbs_trailing_zero_limbs<T: PrimitiveUnsigned>(limbs: &[T]) -> usize {
    limbs
        .iter()
        .rev()
        .take_while(|&&limb| limb == T::ZERO)
        .count()
}
