use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::HasHalf;

pub fn limbs_invert_limb_naive<
    T: TryFrom<DT> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + PrimitiveUnsigned,
>(
    x: T,
) -> T {
    T::exact_from(DT::MAX / DT::from(x) - DT::power_of_2(T::WIDTH))
}
