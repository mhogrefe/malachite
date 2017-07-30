use gmp_mpfr_sys::gmp;
use integer::Integer;
use integer::Integer::*;
use std::cmp::Ordering;

impl Integer {
    //TODO test
    pub fn find_one(&self, start: u64) -> Option<u64> {
        match *self {
            Small(x) => {
                let shifted = x >> start;
                if shifted == 0 {
                    None
                } else {
                    Some(shifted.trailing_zeros().into())
                }
            }
            Large(x) => {
                if *self == 0 {
                    None
                } else {
                    Some(unsafe { gmp::mpz_scan1(&x, start) })
                }
            }
        }
    }

    //TODO test
    pub fn count_ones(&self) -> Option<u64> {
        if self.sign() == Ordering::Less {
            None
        } else {
            Some(match *self {
                     Small(x) => x.count_ones().into(),
                     Large(x) => unsafe { gmp::mpz_popcount(&x) },
                 })
        }
    }

    //TODO test
    pub fn set_bit(&mut self, index: u64) {
        if let Small(ref mut x) = *self {
            if index < 31 {
                *x |= 1 << index;
                return;
            }
        }
        self.promote_in_place();
        if let Large(ref mut x) = *self {
            unsafe {
                gmp::mpz_setbit(x, index.into());
            }
        }
    }

    //TODO test
    pub fn clear_bit(&mut self, index: u64) {
        match *self {
            Small(ref mut x) => {
                if index < 31 {
                    *x &= !(1 << index);
                }
            }
            Large(ref mut x) => unsafe {
                gmp::mpz_clrbit(x, index.into());
            },
        }
        self.demote_if_small();
    }

    //TODO test
    pub fn assign_bit(&mut self, index: u64, val: bool) {
        if val {
            self.set_bit(index);
        } else {
            self.clear_bit(index);
        }
    }
}

pub mod from_sign_and_limbs;
pub mod get_bit;
pub mod sign_and_limbs;
pub mod significant_bits;
pub mod not;
