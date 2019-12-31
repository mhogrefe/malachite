use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    FromOtherTypeSlice, VecFromOtherType, VecFromOtherTypeSlice,
};
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::unsigneds;

fn vec_from_other_type_properties_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>()
where
    U: VecFromOtherType<T>,
    U: VecFromOtherTypeSlice<T>,
    T: FromOtherTypeSlice<U>,
{
    test_properties(unsigneds::<T>, |&x| {
        let xs = U::vec_from_other_type(x);
        assert_eq!(U::vec_from_other_type_slice(&[x]), xs);
        assert_eq!(T::from_other_type_slice(&xs), x);
    });
}

#[test]
fn vec_from_other_type_properties() {
    vec_from_other_type_properties_helper::<u8, u8>();
    vec_from_other_type_properties_helper::<u8, u16>();
    vec_from_other_type_properties_helper::<u8, u32>();
    vec_from_other_type_properties_helper::<u8, u64>();
    vec_from_other_type_properties_helper::<u8, u128>();
    vec_from_other_type_properties_helper::<u8, usize>();
    vec_from_other_type_properties_helper::<u16, u8>();
    vec_from_other_type_properties_helper::<u16, u16>();
    vec_from_other_type_properties_helper::<u16, u32>();
    vec_from_other_type_properties_helper::<u16, u64>();
    vec_from_other_type_properties_helper::<u16, u128>();
    vec_from_other_type_properties_helper::<u16, usize>();
    vec_from_other_type_properties_helper::<u32, u8>();
    vec_from_other_type_properties_helper::<u32, u16>();
    vec_from_other_type_properties_helper::<u32, u32>();
    vec_from_other_type_properties_helper::<u32, u64>();
    vec_from_other_type_properties_helper::<u32, u128>();
    vec_from_other_type_properties_helper::<u32, usize>();
    vec_from_other_type_properties_helper::<u64, u8>();
    vec_from_other_type_properties_helper::<u64, u16>();
    vec_from_other_type_properties_helper::<u64, u32>();
    vec_from_other_type_properties_helper::<u64, u64>();
    vec_from_other_type_properties_helper::<u64, u128>();
    vec_from_other_type_properties_helper::<u64, usize>();
    vec_from_other_type_properties_helper::<usize, u8>();
    vec_from_other_type_properties_helper::<usize, u16>();
    vec_from_other_type_properties_helper::<usize, u32>();
    vec_from_other_type_properties_helper::<usize, u64>();
    vec_from_other_type_properties_helper::<usize, u128>();
    vec_from_other_type_properties_helper::<usize, usize>();
}
