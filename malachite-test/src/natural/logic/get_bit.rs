use num::BigUint;
use num::FromPrimitive;
use num::Zero;

pub fn get_bit(x: &mut BigUint, index: u64) -> bool {
    x.clone() & (BigUint::from_u32(1).unwrap() << index as usize) != BigUint::zero()
}
