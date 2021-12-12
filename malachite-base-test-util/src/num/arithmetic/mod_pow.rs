use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

pub fn naive_mod_pow<T: PrimitiveUnsigned>(x: T, exp: u64, m: T) -> T {
    if m == T::ONE {
        return T::ZERO;
    }
    let data = T::precompute_mod_mul_data(&m);
    let mut out = T::ONE;
    for _ in 0..exp {
        out.mod_mul_precomputed_assign(x, m, &data);
    }
    out
}
