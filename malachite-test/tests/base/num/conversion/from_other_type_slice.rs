use std::fmt::Debug;

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{FromOtherTypeSlice, VecFromOtherTypeSlice};
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

fn from_other_type_slice_properties_helper<T: PrimitiveUnsigned + Rand, U: Debug + Eq + Zero>()
where
    U: FromOtherTypeSlice<T> + VecFromOtherTypeSlice<T>,
{
    test_properties(vecs_of_unsigned, |xs| {
        let value = U::from_other_type_slice(xs);
        let ys = U::vec_from_other_type_slice(xs);
        if xs.is_empty() {
            assert_eq!(value, U::ZERO);
        } else {
            assert_eq!(value, ys[0]);
        }
    });
}

#[test]
fn from_other_type_slice_properties() {
    from_other_type_slice_properties_helper::<u8, u8>();
    from_other_type_slice_properties_helper::<u8, u16>();
    from_other_type_slice_properties_helper::<u8, u32>();
    from_other_type_slice_properties_helper::<u8, u64>();
    from_other_type_slice_properties_helper::<u8, u128>();
    from_other_type_slice_properties_helper::<u8, usize>();
    from_other_type_slice_properties_helper::<u16, u8>();
    from_other_type_slice_properties_helper::<u16, u16>();
    from_other_type_slice_properties_helper::<u16, u32>();
    from_other_type_slice_properties_helper::<u16, u64>();
    from_other_type_slice_properties_helper::<u16, u128>();
    from_other_type_slice_properties_helper::<u16, usize>();
    from_other_type_slice_properties_helper::<u32, u8>();
    from_other_type_slice_properties_helper::<u32, u16>();
    from_other_type_slice_properties_helper::<u32, u32>();
    from_other_type_slice_properties_helper::<u32, u64>();
    from_other_type_slice_properties_helper::<u32, u128>();
    from_other_type_slice_properties_helper::<u32, usize>();
    from_other_type_slice_properties_helper::<u64, u8>();
    from_other_type_slice_properties_helper::<u64, u16>();
    from_other_type_slice_properties_helper::<u64, u32>();
    from_other_type_slice_properties_helper::<u64, u64>();
    from_other_type_slice_properties_helper::<u64, u128>();
    from_other_type_slice_properties_helper::<u64, usize>();
    from_other_type_slice_properties_helper::<usize, u8>();
    from_other_type_slice_properties_helper::<usize, u16>();
    from_other_type_slice_properties_helper::<usize, u32>();
    from_other_type_slice_properties_helper::<usize, u64>();
    from_other_type_slice_properties_helper::<usize, u128>();
    from_other_type_slice_properties_helper::<usize, usize>();
}
