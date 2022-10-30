use crate::num::basic::unsigneds::PrimitiveUnsigned;

pub fn checked_primorial_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    let n = T::try_from(n).ok()?;
    let mut f = T::ONE;
    for p in T::primes().take_while(|&p| p <= n) {
        f = f.checked_mul(p)?;
    }
    Some(f)
}

pub fn checked_product_of_first_n_primes_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    let mut f = T::ONE;
    for p in T::primes().take(usize::try_from(n).ok()?) {
        f = f.checked_mul(p)?;
    }
    Some(f)
}
