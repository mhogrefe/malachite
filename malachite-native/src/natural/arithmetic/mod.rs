use natural::Natural;
use natural::Natural::*;
use natural::{get_lower, get_upper, make_u64};
use std::cmp::max;
use std::ops::{Add, AddAssign};

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

//TODO test
impl<'a> AddAssign<&'a Natural> for Natural {
    fn add_assign(&mut self, op: &'a Natural) {
        if let Small(y) = *op {
            self.add_assign(y);
            return;
        }
        let mut xs = self.promote();
        let xs_len = xs.len();
        let ys = op.get_u32s_ref();
        let ys_len = ys.len();
        let mut carry = false;
        for i in 0..max(xs_len, ys_len) {
            let y = if i >= ys_len {
                if !carry {
                    break;
                }
                1
            } else {
                ys[i]
            };
            if i == xs_len {
                xs.push(0);
            }
            let (sum, overflow) = xs[i].overflowing_add(y);
            xs[i] = sum;
            if carry {
                xs[i] += 1;
            }
            carry = overflow;
        }
        if carry {
            xs.push(1);
        }
    }
}

//TODO test
impl AddAssign<Natural> for Natural {
    fn add_assign(&mut self, op: Natural) {
        self.add_assign(&op);
    }
}

//TODO test
impl<'a> Add<&'a Natural> for Natural {
    type Output = Natural;

    fn add(mut self, op: &'a Natural) -> Natural {
        AddAssign::<&'a Natural>::add_assign(&mut self, op);
        self
    }
}

//TODO test
impl Add<Natural> for Natural {
    type Output = Natural;

    fn add(self, op: Natural) -> Natural {
        self.add(&op)
    }
}

pub mod add_u32;
pub mod even_odd;
pub mod is_power_of_two;
pub mod sub_u32;
