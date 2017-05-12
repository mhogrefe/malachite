use num;

pub fn num_add_u32(mut x: num::BigUint, u: u32) -> num::BigUint {
    x = x + num::BigUint::from(u);
    x
}
