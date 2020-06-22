use malachite_base::num::arithmetic::traits::NegModPowerOfTwo;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
use malachite_base::slices::slice_test_zero;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

fn vec_from_other_type_slice_properties_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned>()
where
    U: VecFromOtherTypeSlice<T>,
    T: VecFromOtherTypeSlice<U>,
{
    test_properties(vecs_of_unsigned, |xs| {
        let xs: &[T] = &xs;
        let ys = U::vec_from_other_type_slice(xs);
        let xs_alt = T::vec_from_other_type_slice(&ys);
        let xs_alt: &[T] = &xs_alt;
        if T::LOG_WIDTH >= U::LOG_WIDTH {
            assert_eq!(xs_alt, xs);
        } else {
            let number_of_extra_zeros = xs.len().neg_mod_power_of_two(U::LOG_WIDTH - T::LOG_WIDTH);
            let (xs_alt_lo, xs_alt_hi) = xs_alt.split_at(xs.len());
            assert_eq!(xs_alt_hi.len(), number_of_extra_zeros);
            assert_eq!(xs_alt_lo, xs);
            assert!(slice_test_zero(xs_alt_hi));
        }
    });
}

#[test]
fn vec_from_other_type_slice_properties() {
    vec_from_other_type_slice_properties_helper::<u8, u8>();
    vec_from_other_type_slice_properties_helper::<u8, u16>();
    vec_from_other_type_slice_properties_helper::<u8, u32>();
    vec_from_other_type_slice_properties_helper::<u8, u64>();
    vec_from_other_type_slice_properties_helper::<u8, u128>();
    vec_from_other_type_slice_properties_helper::<u8, usize>();
    vec_from_other_type_slice_properties_helper::<u16, u8>();
    vec_from_other_type_slice_properties_helper::<u16, u16>();
    vec_from_other_type_slice_properties_helper::<u16, u32>();
    vec_from_other_type_slice_properties_helper::<u16, u64>();
    vec_from_other_type_slice_properties_helper::<u16, u128>();
    vec_from_other_type_slice_properties_helper::<u16, usize>();
    vec_from_other_type_slice_properties_helper::<u32, u8>();
    vec_from_other_type_slice_properties_helper::<u32, u16>();
    vec_from_other_type_slice_properties_helper::<u32, u32>();
    vec_from_other_type_slice_properties_helper::<u32, u64>();
    vec_from_other_type_slice_properties_helper::<u32, u128>();
    vec_from_other_type_slice_properties_helper::<u32, usize>();
    vec_from_other_type_slice_properties_helper::<u64, u8>();
    vec_from_other_type_slice_properties_helper::<u64, u16>();
    vec_from_other_type_slice_properties_helper::<u64, u32>();
    vec_from_other_type_slice_properties_helper::<u64, u64>();
    vec_from_other_type_slice_properties_helper::<u64, u128>();
    vec_from_other_type_slice_properties_helper::<u64, usize>();
    vec_from_other_type_slice_properties_helper::<usize, u8>();
    vec_from_other_type_slice_properties_helper::<usize, u16>();
    vec_from_other_type_slice_properties_helper::<usize, u32>();
    vec_from_other_type_slice_properties_helper::<usize, u64>();
    vec_from_other_type_slice_properties_helper::<usize, u128>();
    vec_from_other_type_slice_properties_helper::<usize, usize>();
}
