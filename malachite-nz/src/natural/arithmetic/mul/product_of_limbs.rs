use crate::natural::arithmetic::mul::limb::limbs_slice_mul_limb_in_place;
use crate::natural::arithmetic::mul::{
    limbs_mul_limb_to_out, limbs_mul_to_out, MUL_TOOM22_THRESHOLD,
};
use crate::platform::Limb;

const RECURSIVE_PROD_THRESHOLD: usize = MUL_TOOM22_THRESHOLD;

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `factors.len()`.
//
// This is equivalent to `mpz_prodlimbs` from `mpz/prodlimbs.c`, GMP 6.2.1, except that the output
// buffer is provided.
pub fn limbs_product(out: &mut [Limb], factors: &mut [Limb]) -> usize {
    let mut factors_len = factors.len();
    assert!(factors_len > 1);
    assert!(RECURSIVE_PROD_THRESHOLD > 3);
    if factors_len < RECURSIVE_PROD_THRESHOLD {
        factors_len -= 1;
        let mut size = 1;
        for i in 1..factors_len {
            let factor = factors[i];
            let carry = limbs_slice_mul_limb_in_place(&mut factors[..size], factor);
            factors[size] = carry;
            if carry != 0 {
                size += 1;
            }
        }
        assert!(out.len() > size);
        let carry = limbs_mul_limb_to_out(out, &factors[..size], factors[factors_len]);
        out[size] = carry;
        if carry != 0 {
            size += 1
        }
        size
    } else {
        let half_len = factors_len >> 1;
        let (factors, xs) = factors.split_at_mut(half_len);
        let mut ys = vec![0; xs.len()];
        let ys_len = limbs_product(&mut ys, xs);
        let xs_len = limbs_product(xs, &mut factors[..half_len]);
        let size = xs_len + ys_len;
        assert!(out.len() >= size);
        if limbs_mul_to_out(out, &xs[..xs_len], &ys[..ys_len]) == 0 {
            size - 1
        } else {
            size
        }
    }
}
