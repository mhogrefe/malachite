use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, HasHalf, JoinHalves, SplitInHalf};

pub fn limbs_invert_limb_naive<
    T: CheckedFrom<DT> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    x: T,
) -> T {
    T::exact_from(DT::MAX / DT::from(x) - DT::power_of_2(T::WIDTH))
}
