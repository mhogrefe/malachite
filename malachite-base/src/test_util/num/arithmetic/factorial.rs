use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::NotAssign;

pub fn checked_factorial_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    let mut f = T::ONE;
    let mut n = T::try_from(n).ok()?;
    while n != T::ZERO {
        f = f.checked_mul(n)?;
        n -= T::ONE;
    }
    Some(f)
}

pub fn checked_double_factorial_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    let mut f = T::ONE;
    let mut n = T::try_from(n).ok()?;
    while n != T::ZERO {
        f = f.checked_mul(n)?;
        n.saturating_sub_assign(T::TWO);
    }
    Some(f)
}

pub fn checked_subfactorial_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    let mut f = T::ONE;
    let mut b = true;
    for i in 1..=n {
        f = f.checked_mul(T::try_from(i).ok()?)?;
        if b {
            f -= T::ONE;
        } else {
            f = f.checked_add(T::ONE)?;
        }
        b.not_assign();
    }
    Some(f)
}
