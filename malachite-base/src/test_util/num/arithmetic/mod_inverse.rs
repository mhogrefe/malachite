use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;
use test_util::num::arithmetic::extended_gcd::extended_gcd_unsigned_euclidean;

pub fn mod_inverse_euclidean<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
    m: U,
) -> Option<U> {
    assert_ne!(x, U::ZERO);
    assert!(x < m);
    let (gcd, inverse, _) = extended_gcd_unsigned_euclidean::<U, S>(x, m);
    if gcd == U::ONE {
        Some(if inverse >= S::ZERO {
            U::wrapping_from(inverse)
        } else {
            U::wrapping_from(inverse).wrapping_add(m)
        })
    } else {
        None
    }
}
