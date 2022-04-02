use num::{BigInt, BigUint};

pub fn neg_num(u: BigUint) -> BigInt {
    -BigInt::from(u)
}
