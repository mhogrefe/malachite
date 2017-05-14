use num;

pub fn num_add_u32(mut x: num::BigInt, u: u32) -> num::BigInt {
    x = x + num::BigInt::from(u);
    x
}
