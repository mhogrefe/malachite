use gmp_mpfr_sys::gmp;
use integer::Integer;
use integer::Integer::*;
use std::cmp::{Ordering, PartialOrd};
use std::hash::{Hash, Hasher};

impl Hash for Integer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sign().hash(state);
        for i in self.to_u32s() {
            i.hash(state);
        }
    }
}

//TODO test
impl Ord for Integer {
    fn cmp(&self, other: &Integer) -> Ordering {
        match self {
            &Small(x) => {
                match other {
                    &Small(y) => x.cmp(&y),
                    &Large(y) => {
                        let ord = unsafe { gmp::mpz_cmp_si(&y, x.into()) };
                        0.cmp(&ord)
                    }
                }
            }
            &Large(x) => {
                match other {
                    &Small(y) => {
                        let ord = unsafe { gmp::mpz_cmp_si(&x, y.into()) };
                        ord.cmp(&0)
                    }
                    &Large(y) => {
                        let ord = unsafe { gmp::mpz_cmp(&x, &y) };
                        ord.cmp(&0)
                    }
                }
            }
        }
    }
}

//TODO test
impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub mod eq_integer;
pub mod partial_eq_i32;
pub mod partial_eq_natural;
pub mod partial_eq_u32;
pub mod partial_ord_i32;
pub mod partial_ord_u32;
pub mod sign;
