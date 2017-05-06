use integer::Integer;
use std::hash::{Hash, Hasher};

impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sign().hash(state);
        for i in self.to_u32s() {
            i.hash(state);
        }
    }
}

pub mod eq_integer;
pub mod ord_integer;
pub mod partial_eq_i32;
pub mod partial_eq_natural;
pub mod partial_eq_u32;
pub mod partial_ord_i32;
pub mod partial_ord_u32;
pub mod sign;
