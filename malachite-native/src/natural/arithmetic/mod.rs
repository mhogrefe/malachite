use malachite_base::num::make_u64;
use natural::Natural::{self, Small};

impl Natural {
    //TODO test
    pub fn div_rem_in_place_u32(&mut self, op: u32) -> u32 {
        assert_ne!(op, 0);
        if op == 1 {
            return 0;
        }
        if let Small(ref mut x) = *self {
            let rem = *x % op;
            *x /= op;
            return rem;
        }
        let mut upper = 0u32;
        {
            let xs = self.promote_in_place();
            let xs_len = xs.len();
            for i_rev in 0..xs_len {
                let i = xs_len - i_rev - 1;
                let lower = xs[i];
                let x = make_u64(upper, lower);
                let q = (x / op as u64) as u32;
                xs[i] = q;
                upper = (x % op as u64) as u32;
            }
        }
        self.trim();
        upper
    }
}

pub mod add;
pub mod add_u32;
pub mod add_mul;
pub mod add_mul_u32;
pub mod divisible_by_power_of_2;
pub mod even_odd;
pub mod is_power_of_two;
pub mod mul;
pub mod mul_u32;
pub mod neg;
pub mod shl_u32;
pub mod shr_u32;
pub mod sub;
pub mod sub_u32;
pub mod sub_mul;
pub mod sub_mul_u32;
