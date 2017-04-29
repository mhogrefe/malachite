use num;

pub fn num_partial_eq_u32(x: &num::BigUint, u: u32) -> bool {
    *x == num::BigUint::from(u)
}
