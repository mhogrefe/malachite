use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::SaturatingSubAssign;
use malachite_base::num::basic::traits::One;

pub fn factorial_naive(mut n: u64) -> Natural {
    let mut f = Natural::ONE;
    while n != 0 {
        f *= Natural::from(n);
        n -= 1;
    }
    f
}

pub fn double_factorial_naive(mut n: u64) -> Natural {
    let mut f = Natural::ONE;
    while n != 0 {
        f *= Natural::from(n);
        n.saturating_sub_assign(2);
    }
    f
}

pub fn multifactorial_naive(mut n: u64, m: u64) -> Natural {
    assert_ne!(m, 0);
    let mut f = Natural::ONE;
    while n != 0 {
        f *= Natural::from(n);
        n.saturating_sub_assign(m);
    }
    f
}
