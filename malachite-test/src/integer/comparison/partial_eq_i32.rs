use num;

pub fn num_partial_eq_i32(x: &num::BigInt, i: i32) -> bool {
    *x == num::BigInt::from(i)
}
