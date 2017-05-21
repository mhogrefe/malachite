use natural::Natural;
use natural::Natural::*;
use natural::{get_lower, get_upper, make_u64};

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
            let xs = self.promote();
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

    pub fn mul_in_place_u32(&mut self, op: u32) {
        if let Small(ref mut x) = *self {
            let product = *x as u64 * op as u64;
            if get_upper(product) == 0 {
                *x = product as u32;
                return;
            }
        }
        let mut xs = self.promote();
        let mut carry = 0;
        for x in xs.iter_mut() {
            let product = *x as u64 * op as u64 + carry as u64;
            *x = get_lower(product);
            carry = get_upper(product);
        }
        if carry != 0 {
            xs.push(carry);
        }
    }
}

pub mod add;
pub mod add_u32;
pub mod even_odd;
pub mod is_power_of_two;
pub mod shl_u32;
pub mod sub;
pub mod sub_u32;
