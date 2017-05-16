use gmp_mpfr_sys::gmp;
use integer::Integer;
use integer::Integer::*;
use std::cmp::Ordering;
use std::ops::{AddAssign, MulAssign, Sub, SubAssign};
use std::os::raw::c_ulong;

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
impl<'a> AddAssign<&'a Integer> for Integer {
    fn add_assign(&mut self, op: &'a Integer) {
        if *op == 0 {
            return;
        }
        if let Small(ref mut x) = *self {
            if let Small(y) = *op {
                if let Some(sum) = x.checked_add(y) {
                    *x = sum;
                    return;
                }
            }
        }
        let mut x = self.promote_in_place();
        match *op {
            Small(y) => {
                if y > 0 {
                    unsafe {
                        gmp::mpz_add_ui(x, x, y as c_ulong);
                    }
                } else {
                    unsafe {
                        gmp::mpz_sub_ui(x, x, y.wrapping_neg() as c_ulong);
                    }
                }
            }
            Large(y) => unsafe {
                gmp::mpz_add(x, x, &y);
            },
        }
    }
}

//TODO test
impl AddAssign<Integer> for Integer {
    fn add_assign(&mut self, op: Integer) {
        self.add_assign(&op);
    }
}

//TODO test
impl SubAssign<i32> for Integer {
    fn sub_assign(&mut self, op: i32) {
        if op == 0 {
            return;
        }
        let mut promote = false;
        if let Small(ref mut x) = *self {
            match x.checked_sub(op) {
                Some(difference) => *x = difference,
                None => promote = true,
            }
        }
        if promote {
            let x = self.promote_in_place();
            if op > 0 {
                unsafe {
                    gmp::mpz_sub_ui(x, x, op as c_ulong);
                }
            } else {
                unsafe {
                    gmp::mpz_add_ui(x, x, op.wrapping_neg() as c_ulong);
                }
            }
        }
    }
}

//TODO test
impl<'a> SubAssign<&'a Integer> for Integer {
    fn sub_assign(&mut self, op: &'a Integer) {
        if *op == 0 {
            return;
        }
        if let Small(ref mut x) = *self {
            if let Small(y) = *op {
                if let Some(difference) = x.checked_sub(y) {
                    *x = difference;
                    return;
                }
            }
        }
        let mut x = self.promote_in_place();
        match *op {
            Small(y) => {
                if y > 0 {
                    unsafe {
                        gmp::mpz_sub_ui(x, x, y as c_ulong);
                    }
                } else {
                    unsafe {
                        gmp::mpz_add_ui(x, x, y.wrapping_neg() as c_ulong);
                    }
                }
            }
            Large(y) => unsafe {
                gmp::mpz_sub(x, x, &y);
            },
        }
    }
}

//TODO test
impl SubAssign<Integer> for Integer {
    fn sub_assign(&mut self, op: Integer) {
        self.sub_assign(&op);
    }
}

//TODO test
impl<'a> Sub<&'a Integer> for Integer {
    type Output = Integer;
    fn sub(mut self, op: &'a Integer) -> Integer {
        SubAssign::<&'a Integer>::sub_assign(&mut self, op);
        self
    }
}

//TODO test
impl Sub<Integer> for Integer {
    type Output = Integer;
    fn sub(self, op: Integer) -> Integer {
        self.sub(&op)
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
pub mod add_i32;
pub mod add_u32;
pub mod neg;
pub mod sub_u32;
