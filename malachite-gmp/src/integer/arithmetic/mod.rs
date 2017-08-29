use gmp_mpfr_sys::gmp;
use integer::Integer;
use integer::Integer::*;
use std::cmp::Ordering;

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

pub mod abs;
pub mod add;
pub mod add_i32;
pub mod add_u32;
pub mod even_odd;
pub mod mul;
pub mod mul_i32;
pub mod mul_u32;
pub mod neg;
pub mod shl_u32;
pub mod sub;
pub mod sub_i32;
pub mod sub_u32;
