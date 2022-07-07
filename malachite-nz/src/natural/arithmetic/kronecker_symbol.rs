use malachite_base::num::arithmetic::traits::{
    JacobiSymbol, KroneckerSymbol, LegendreSymbol, ModPowerOf2, NegAssign, Parity,
};
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::conversion::traits::WrappingInto;
use malachite_base::num::logic::traits::BitAccess;
use natural::InnerNatural::Small;
use natural::Natural;
use std::mem::swap;

pub fn jacobi_symbol_simple(mut a: Natural, mut n: Natural) -> i8 {
    assert_ne!(n, 0u32);
    assert!(n.odd());
    a %= &n;
    let mut t = 1i8;
    while a != 0u32 {
        while a.even() {
            a >>= 1u32;
            let r: u8 = (&(&n).mod_power_of_2(3)).wrapping_into();
            if r == 3 || r == 5 {
                t.neg_assign();
            }
        }
        swap(&mut a, &mut n);
        if a.get_bit(1) && n.get_bit(1) {
            t.neg_assign();
        }
        a %= &n;
    }
    if n == 1u32 {
        t
    } else {
        0
    }
}

impl LegendreSymbol<Natural> for Natural {
    #[inline]
    fn legendre_symbol(self, other: Natural) -> i8 {
        self.jacobi_symbol(other)
    }
}

impl<'a> LegendreSymbol<&'a Natural> for Natural {
    #[inline]
    fn legendre_symbol(self, other: &'a Natural) -> i8 {
        self.jacobi_symbol(other)
    }
}

impl<'a> LegendreSymbol<Natural> for &'a Natural {
    #[inline]
    fn legendre_symbol(self, other: Natural) -> i8 {
        self.jacobi_symbol(other)
    }
}

impl<'a, 'b> LegendreSymbol<&'a Natural> for &'b Natural {
    #[inline]
    fn legendre_symbol(self, other: &'a Natural) -> i8 {
        self.jacobi_symbol(other)
    }
}

impl JacobiSymbol<Natural> for Natural {
    #[inline]
    fn jacobi_symbol(self, other: Natural) -> i8 {
        jacobi_symbol_simple(self, other)
    }
}

impl<'a> JacobiSymbol<&'a Natural> for Natural {
    #[inline]
    fn jacobi_symbol(self, other: &'a Natural) -> i8 {
        jacobi_symbol_simple(self, other.clone())
    }
}

impl<'a> JacobiSymbol<Natural> for &'a Natural {
    #[inline]
    fn jacobi_symbol(self, other: Natural) -> i8 {
        jacobi_symbol_simple(self.clone(), other)
    }
}

impl<'a, 'b> JacobiSymbol<&'a Natural> for &'b Natural {
    #[inline]
    fn jacobi_symbol(self, other: &'a Natural) -> i8 {
        jacobi_symbol_simple(self.clone(), other.clone())
    }
}

impl KroneckerSymbol<Natural> for Natural {
    #[inline]
    fn kronecker_symbol(self, other: Natural) -> i8 {
        if let Natural(Small(x)) = self {
            if let Natural(Small(y)) = other {
                return x.kronecker_symbol(y);
            }
        }
        if other == 0u32 {
            i8::iverson(self == 1u32)
        } else if self.even() && other.even() {
            0
        } else {
            let other_twos = other.trailing_zeros().unwrap();
            let mut s = (&self).jacobi_symbol(other >> other_twos);
            if other_twos.odd() {
                let m: u32 = (&self.mod_power_of_2(3)).wrapping_into();
                if m == 3 || m == 5 {
                    s.neg_assign();
                }
            }
            s
        }
    }
}

impl<'a> KroneckerSymbol<&'a Natural> for Natural {
    #[inline]
    fn kronecker_symbol(self, other: &'a Natural) -> i8 {
        if let Natural(Small(x)) = self {
            if let Natural(Small(y)) = other {
                return x.kronecker_symbol(*y);
            }
        }
        if *other == 0u32 {
            i8::iverson(self == 1u32)
        } else if self.even() && other.even() {
            0
        } else {
            let other_twos = other.trailing_zeros().unwrap();
            let mut s = (&self).jacobi_symbol(other >> other_twos);
            if other_twos.odd() {
                let m: u32 = (&self.mod_power_of_2(3)).wrapping_into();
                if m == 3 || m == 5 {
                    s.neg_assign();
                }
            }
            s
        }
    }
}

impl<'a> KroneckerSymbol<Natural> for &'a Natural {
    #[inline]
    fn kronecker_symbol(self, other: Natural) -> i8 {
        if let Natural(Small(x)) = self {
            if let Natural(Small(y)) = other {
                return x.kronecker_symbol(y);
            }
        }
        if other == 0u32 {
            i8::iverson(*self == 1u32)
        } else if self.even() && other.even() {
            0
        } else {
            let other_twos = other.trailing_zeros().unwrap();
            let mut s = self.jacobi_symbol(other >> other_twos);
            if other_twos.odd() {
                let m: u32 = (&self.mod_power_of_2(3)).wrapping_into();
                if m == 3 || m == 5 {
                    s.neg_assign();
                }
            }
            s
        }
    }
}

impl<'a, 'b> KroneckerSymbol<&'a Natural> for &'b Natural {
    #[inline]
    fn kronecker_symbol(self, other: &'a Natural) -> i8 {
        if let Natural(Small(x)) = self {
            if let Natural(Small(y)) = other {
                return x.kronecker_symbol(*y);
            }
        }
        if *other == 0u32 {
            i8::iverson(*self == 1u32)
        } else if self.even() && other.even() {
            0
        } else {
            let other_twos = other.trailing_zeros().unwrap();
            let mut s = self.jacobi_symbol(other >> other_twos);
            if other_twos.odd() {
                let m: u32 = (&self.mod_power_of_2(3)).wrapping_into();
                if m == 3 || m == 5 {
                    s.neg_assign();
                }
            }
            s
        }
    }
}
