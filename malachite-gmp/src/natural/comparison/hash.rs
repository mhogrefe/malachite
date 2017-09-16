use natural::Natural;
use std::hash::{Hash, Hasher};

/// Hashes a `Natural`.
impl Hash for Natural {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.to_limbs_le() {
            i.hash(state);
        }
    }
}
