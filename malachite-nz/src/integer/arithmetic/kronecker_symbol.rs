use integer::Integer;
use malachite_base::num::arithmetic::traits::{
    Abs, JacobiSymbol, KroneckerSymbol, LegendreSymbol, ModPowerOf2, NegAssign, Parity, UnsignedAbs,
};
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::conversion::traits::WrappingInto;
use malachite_base::num::logic::traits::BitAccess;

impl LegendreSymbol<Integer> for Integer {
    #[inline]
    fn legendre_symbol(self, other: Integer) -> i8 {
        self.jacobi_symbol(other)
    }
}

impl<'a> LegendreSymbol<&'a Integer> for Integer {
    #[inline]
    fn legendre_symbol(self, other: &'a Integer) -> i8 {
        self.jacobi_symbol(other)
    }
}

impl<'a> LegendreSymbol<Integer> for &'a Integer {
    #[inline]
    fn legendre_symbol(self, other: Integer) -> i8 {
        self.jacobi_symbol(other)
    }
}

impl<'a, 'b> LegendreSymbol<&'a Integer> for &'b Integer {
    #[inline]
    fn legendre_symbol(self, other: &'a Integer) -> i8 {
        self.jacobi_symbol(other)
    }
}

impl JacobiSymbol<Integer> for Integer {
    #[inline]
    fn jacobi_symbol(self, other: Integer) -> i8 {
        assert!(other > 0u32);
        assert!(other.odd());
        (if self < 0u32 && other.get_bit(1) {
            -1
        } else {
            1
        }) * self.unsigned_abs().jacobi_symbol(other.unsigned_abs())
    }
}

impl<'a> JacobiSymbol<&'a Integer> for Integer {
    #[inline]
    fn jacobi_symbol(self, other: &'a Integer) -> i8 {
        assert!(*other > 0u32);
        assert!(other.odd());
        (if self < 0u32 && other.get_bit(1) {
            -1
        } else {
            1
        }) * self.unsigned_abs().jacobi_symbol(other.unsigned_abs_ref())
    }
}

impl<'a> JacobiSymbol<Integer> for &'a Integer {
    #[inline]
    fn jacobi_symbol(self, other: Integer) -> i8 {
        assert!(other > 0u32);
        assert!(other.odd());
        (if *self < 0u32 && other.get_bit(1) {
            -1
        } else {
            1
        }) * self.unsigned_abs_ref().jacobi_symbol(other.unsigned_abs())
    }
}

impl<'a, 'b> JacobiSymbol<&'a Integer> for &'b Integer {
    #[inline]
    fn jacobi_symbol(self, other: &'a Integer) -> i8 {
        assert!(*other > 0u32);
        assert!(other.odd());
        (if *self < 0u32 && other.get_bit(1) {
            -1
        } else {
            1
        }) * self
            .unsigned_abs_ref()
            .jacobi_symbol(other.unsigned_abs_ref())
    }
}

impl KroneckerSymbol<Integer> for Integer {
    #[inline]
    fn kronecker_symbol(self, other: Integer) -> i8 {
        if other == 0u32 {
            i8::iverson(self == 1u32 || self == -1i32)
        } else if self.even() && other.even() {
            0
        } else {
            let other_twos = other.trailing_zeros().unwrap();
            let mut s = 1i8;
            if other_twos.odd() {
                let m: u32 = (&(&self).mod_power_of_2(3)).wrapping_into();
                if m == 3 || m == 5 {
                    s = -1;
                }
            }
            if self < 0u32 && other < 0u32 {
                s.neg_assign();
            }
            s * self.jacobi_symbol((other >> other_twos).abs())
        }
    }
}

impl<'a> KroneckerSymbol<&'a Integer> for Integer {
    #[inline]
    fn kronecker_symbol(self, other: &'a Integer) -> i8 {
        if *other == 0u32 {
            i8::iverson(self == 1u32 || self == -1i32)
        } else if self.even() && other.even() {
            0
        } else {
            let other_twos = other.trailing_zeros().unwrap();
            let mut s = 1i8;
            if other_twos.odd() {
                let m: u32 = (&(&self).mod_power_of_2(3)).wrapping_into();
                if m == 3 || m == 5 {
                    s = -1;
                }
            }
            if self < 0u32 && *other < 0u32 {
                s.neg_assign();
            }
            s * self.jacobi_symbol((other >> other_twos).abs())
        }
    }
}

impl<'a> KroneckerSymbol<Integer> for &'a Integer {
    #[inline]
    fn kronecker_symbol(self, other: Integer) -> i8 {
        if other == 0u32 {
            i8::iverson(*self == 1u32 || *self == -1i32)
        } else if self.even() && other.even() {
            0
        } else {
            let other_twos = other.trailing_zeros().unwrap();
            let mut s = 1i8;
            if other_twos.odd() {
                let m: u32 = (&self.mod_power_of_2(3)).wrapping_into();
                if m == 3 || m == 5 {
                    s = -1;
                }
            }
            if *self < 0u32 && other < 0u32 {
                s.neg_assign();
            }
            s * self.jacobi_symbol((other >> other_twos).abs())
        }
    }
}

impl<'a, 'b> KroneckerSymbol<&'a Integer> for &'b Integer {
    #[inline]
    fn kronecker_symbol(self, other: &'a Integer) -> i8 {
        if *other == 0u32 {
            i8::iverson(*self == 1u32 || *self == -1i32)
        } else if self.even() && other.even() {
            0
        } else {
            let other_twos = other.trailing_zeros().unwrap();
            let mut s = 1i8;
            if other_twos.odd() {
                let m: u32 = (&self.mod_power_of_2(3)).wrapping_into();
                if m == 3 || m == 5 {
                    s = -1;
                }
            }
            if *self < 0u32 && *other < 0u32 {
                s.neg_assign();
            }
            s * self.jacobi_symbol((other >> other_twos).abs())
        }
    }
}
