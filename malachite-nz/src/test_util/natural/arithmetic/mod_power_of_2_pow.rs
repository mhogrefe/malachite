use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{ModPowerOf2MulAssign, ModPowerOf2SquareAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::BitIterable;

pub fn simple_binary_mod_power_of_2_pow(x: &Natural, exp: &Natural, pow: u64) -> Natural {
    if pow == 0 {
        return Natural::ZERO;
    }
    let mut out = Natural::ONE;
    for bit in exp.bits().rev() {
        out.mod_power_of_2_square_assign(pow);
        if bit {
            out.mod_power_of_2_mul_assign(x, pow);
        }
    }
    out
}
