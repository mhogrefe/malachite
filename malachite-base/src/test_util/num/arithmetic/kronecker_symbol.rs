use num::arithmetic::traits::{NegAssign, Parity};
use num::basic::unsigneds::PrimitiveUnsigned;
use std::mem::swap;

// This is equivalent to `n_jacobi_unsigned` from `ulong_extras/jacobi.c`, FLINT 2.7.1.
pub fn jacobi_symbol_unsigned_fast_1<T: PrimitiveUnsigned>(x: T, y: T) -> i8 {
    let mut a = x;
    let mut b = y;
    let mut s = 1;
    if a < b && b != T::ONE {
        if a == T::ZERO {
            return 0;
        }
        swap(&mut a, &mut b);
        let exp = b.trailing_zeros();
        b >>= exp;
        if T::wrapping_from(exp)
            .wrapping_mul(a.wrapping_square() - T::ONE)
            .get_bit(3)
            != (a - T::ONE).wrapping_mul(b - T::ONE).get_bit(2)
        {
            s.neg_assign();
        }
    }
    while b != T::ONE {
        if a >> 2 < b {
            let temp = a - b;
            a = b;
            b = if temp < b {
                temp
            } else if temp < b << 1 {
                temp - a
            } else {
                temp - (a << 1)
            }
        } else {
            a %= b;
            swap(&mut a, &mut b);
        }
        if b == T::ZERO {
            return 0;
        }
        let exp = b.trailing_zeros();
        b >>= exp;
        if T::wrapping_from(exp)
            .wrapping_mul(a.wrapping_square() - T::ONE)
            .get_bit(3)
            != (a - T::ONE).wrapping_mul(b - T::ONE).get_bit(2)
        {
            s.neg_assign();
        }
    }
    s
}

// This is equivalent to `mpn_jacobi_base` from `mpn/jacbase.c`, GMP 6.2.1, where
// `JACOBI_BASE_METHOD == 1` and `result_bit_1` is false.
pub fn jacobi_symbol_unsigned_fast_2_1<T: PrimitiveUnsigned>(mut a: T, mut b: T) -> i8 {
    assert!(b.odd());
    if b == T::ONE {
        return 1;
    } else if a == T::ZERO {
        return 0;
    }
    let mut s = 1;
    let a_twos = a.trailing_zeros();
    if a_twos.odd() && ((b >> 1u32) ^ b).get_bit(1) {
        s.neg_assign();
    }
    a >>= a_twos;
    if a == T::ONE {
        return s;
    }
    if a < b {
        if (a & b).get_bit(1) {
            s.neg_assign();
        }
        swap(&mut a, &mut b);
    }
    loop {
        assert!(a.odd());
        assert!(b.odd());
        assert!(a >= b);
        a -= b;
        if a == T::ZERO {
            return 0;
        }
        let a_twos = a.trailing_zeros();
        if a_twos.odd() && ((b >> 1u32) ^ b).get_bit(1) {
            s.neg_assign();
        }
        a >>= a_twos;
        if a == T::ONE {
            return s;
        }
        if a < b {
            if (a & b).get_bit(1) {
                s.neg_assign();
            }
            swap(&mut a, &mut b);
        }
    }
}

// This is equivalent to `mpn_jacobi_base` from `mpn/jacbase.c`, GMP 6.2.1, where
// `JACOBI_BASE_METHOD == 3` and `result_bit_1` is false.
pub fn jacobi_symbol_unsigned_fast_2_3<T: PrimitiveUnsigned>(mut a: T, mut b: T) -> i8 {
    assert!(b.odd());
    if b == T::ONE {
        return 1;
    } else if a == T::ZERO {
        return 0;
    }
    let mut s = 1;
    let two = (b >> 1u32) ^ b;
    let shift = !a & T::ONE;
    let shift_8: u8 = shift.wrapping_into();
    a >>= shift_8;
    let mask = shift << 1u32;
    if (two & mask).get_bit(1) {
        s.neg_assign();
    }
    let two_bit = two.get_bit(1);
    while a.even() {
        a >>= 1;
        if two_bit {
            s.neg_assign();
        }
    }
    if a == T::ONE {
        return s;
    }
    if a < b {
        if (a & b).get_bit(1) {
            s.neg_assign();
        }
        swap(&mut a, &mut b);
    }
    loop {
        assert!(a.odd());
        assert!(b.odd());
        assert!(a >= b);
        a -= b;
        if a == T::ZERO {
            return 0;
        }
        let two = (b >> 1u32) ^ b;
        let mask = !a & T::TWO;
        a >>= 1;
        if a.even() {
            a >>= 1;
        }
        if (two ^ (two & mask)).get_bit(1) {
            s.neg_assign();
        }
        while a.even() {
            a >>= 1;
            if two.get_bit(1) {
                s.neg_assign();
            }
        }
        if a == T::ONE {
            return s;
        }
        if a < b {
            if (a & b).get_bit(1) {
                s.neg_assign();
            }
            swap(&mut a, &mut b);
        }
    }
}

// This is equivalent to `mpn_jacobi_base` from `mpn/jacbase.c`, GMP 6.2.1, where
// `JACOBI_BASE_METHOD == 4` and `result_bit_1` is false.
pub fn jacobi_symbol_unsigned_fast_2_4<T: PrimitiveUnsigned>(mut a: T, mut b: T) -> i8 {
    assert!(b.odd());
    if b == T::ONE {
        return 1;
    } else if a == T::ZERO {
        return 0;
    }
    let mut s = 1;
    b >>= 1u32;
    let c = a.trailing_zeros();
    if (T::wrapping_from(c) & (b ^ (b >> 1))).odd() {
        s.neg_assign();
    }
    // We may have c == T::WIDTH - 1, so we can't use a >> (c + 1).
    a >>= c;
    a >>= 1;
    while b != T::ZERO {
        let t = a.wrapping_sub(b);
        if t == T::ZERO {
            return 0;
        }
        let bgta = if t.get_highest_bit() { T::MAX } else { T::ZERO };
        // If b > a, invoke reciprocity
        if (bgta & a & b).odd() {
            s.neg_assign();
        }
        // b <-- min (a, b)
        b.wrapping_add_assign(bgta & t);
        // a <-- |a - b|
        a = (t ^ bgta).wrapping_sub(bgta);
        // Number of trailing zeros is the same no matter if we look at t or a, but using t gives
        // more parallelism.
        let c = t.trailing_zeros() + 1;
        // (2/b) = -1 if b = 3 or 5 mod 8
        if (T::wrapping_from(c) & (b ^ (b >> 1))).odd() {
            s.neg_assign();
        }
        a >>= c;
    }
    s
}
