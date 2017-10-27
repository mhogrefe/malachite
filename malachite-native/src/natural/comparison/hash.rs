use natural::Natural;
use std::hash::{Hash, Hasher};

/// Hashes a `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
impl Hash for Natural {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Small(small) => small.hash(state),
            Large(ref limbs) => {
                for limb in limbs {
                    limb.hash(state);
                }
            }
        }
    }
}
