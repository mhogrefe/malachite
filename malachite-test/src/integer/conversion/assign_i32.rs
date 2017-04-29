use num;

pub fn num_assign_i32(x: &mut num::BigInt, i: i32) {
    *x = num::BigInt::from(i);
}
