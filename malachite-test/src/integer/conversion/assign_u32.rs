use num;

pub fn num_assign_u32(x: &mut num::BigInt, u: u32) {
    *x = num::BigInt::from(u);
}
