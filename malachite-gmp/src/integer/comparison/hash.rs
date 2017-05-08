use integer::Integer;
use std::hash::{Hash, Hasher};

/// Hashes an `Integer`.
impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self >= 0).hash(state);
        self.clone().unsigned_abs().hash(state);
    }
}
