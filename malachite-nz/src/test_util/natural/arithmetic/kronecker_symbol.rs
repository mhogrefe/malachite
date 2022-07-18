use malachite_base::num::arithmetic::traits::{ModPowerOf2, NegAssign, Parity};
use malachite_base::num::conversion::traits::WrappingInto;
use malachite_base::num::logic::traits::BitAccess;
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
