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
