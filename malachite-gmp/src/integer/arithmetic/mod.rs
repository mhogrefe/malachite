use gmp_mpfr_sys::gmp;
use integer::Integer;
use integer::Integer::*;
use std::cmp::Ordering;
use std::ops::MulAssign;

impl Integer {
    //TODO test
    pub fn div_rem_in_place(&mut self, val: &mut Integer) {
        assert_ne!(val.sign(), Ordering::Equal, "division by zero");
        match *self {
            Small(ref mut x) => {
                match *val {
                    Small(y) => {
                        let quotient = *x / y;
                        *val = Small(*x % y);
                        *x = quotient;
                    }
                    Large(_) => {
                        *x = 0;
                    }
                }
            }
            Large(ref mut x) => {
                match *val {
                    Small(y) => {
                        let mut r = Integer::new_mpz_t();
                        unsafe {
                            gmp::mpz_tdiv_qr_ui(x, &mut r, x, y as u64);
                        };
                        val.assign_mpz_t(r);
                    }
                    Large(ref mut y) => unsafe {
                        gmp::mpz_tdiv_qr(x, y, x, y);
                    },
                }
            }
        }
    }
}

//TODO test
impl MulAssign<i32> for Integer {
    fn mul_assign(&mut self, op: i32) {
        if op == 1 || *self == 0 {
            return;
        }
        if op == 0 {
            *self = Small(0);
        }
        let mut promote = false;
        if let Small(ref mut x) = *self {
            match x.checked_mul(op) {
                Some(product) => *x = product,
                None => promote = true,
            }
        }
        if promote {
            let x = self.promote_in_place();
            unsafe {
                gmp::mpz_mul_si(x, x, op.into());
            }
        }
    }
}

//TODO test
impl<'a> MulAssign<&'a Integer> for Integer {
    fn mul_assign(&mut self, op: &'a Integer) {
        if *op == 1 || *self == 0 {
            return;
        }
        if *op == 0 {
            *self = Small(0);
        }
        if let Small(ref mut x) = *self {
            if let Small(y) = *op {
                if let Some(product) = x.checked_mul(y) {
                    *x = product;
                    return;
                }
            }
        }
        let mut x = self.promote_in_place();
        match *op {
            Small(y) => unsafe {
                gmp::mpz_mul_si(x, x, y.into());
            },
            Large(y) => unsafe {
                gmp::mpz_mul(x, x, &y);
            },
        }
    }
}

//TODO test
impl MulAssign<Integer> for Integer {
    fn mul_assign(&mut self, op: Integer) {
        self.mul_assign(&op);
    }
}

pub mod abs;
pub mod add;
pub mod add_i32;
pub mod add_u32;
pub mod even_odd;
pub mod neg;
pub mod shl_u32;
pub mod sub;
pub mod sub_i32;
pub mod sub_u32;
