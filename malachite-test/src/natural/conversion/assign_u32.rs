use num;

pub fn num_assign_u32(x: &mut num::BigUint, u: u32) {
    *x = num::BigUint::from(u);
}
