use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, HasHalf, JoinHalves, SplitInHalf};

pub fn _limbs_invert_limb_naive<
    T: PrimitiveUnsigned,
    DT: JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    x: T,
) -> T
where
    T: CheckedFrom<DT>,
    DT: From<T> + HasHalf<Half = T>,
{
    T::exact_from(DT::MAX / DT::from(x) - DT::power_of_two(T::WIDTH))
}
