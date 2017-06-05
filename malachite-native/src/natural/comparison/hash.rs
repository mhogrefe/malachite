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
        for i in self.limbs_le() {
            i.hash(state);
        }
    }
}
