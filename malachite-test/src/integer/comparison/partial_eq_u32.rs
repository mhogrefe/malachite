use num;

pub fn num_partial_eq_u32(x: &num::BigInt, u: u32) -> bool {
    *x == num::BigInt::from(u)
}
