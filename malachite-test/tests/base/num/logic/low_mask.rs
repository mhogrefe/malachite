use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::small_u64s_var_4;

fn unsigned_low_mask_properties_helper<T: PrimitiveUnsigned>() {
    test_properties_no_special(small_u64s_var_4::<T>, |&bits| {
        let n = T::low_mask(bits);
        assert_eq!(n.count_ones(), bits);
        assert_eq!(n.index_of_next_false_bit(0), Some(bits));
    });
}

fn signed_low_mask_properties_helper<T: PrimitiveSigned>() {
    test_properties_no_special(small_u64s_var_4::<T>, |&bits| {
        let n = T::low_mask(bits);
        assert_eq!(n.count_ones(), bits);
        assert_eq!(
            n.index_of_next_false_bit(0),
            if bits == T::WIDTH { None } else { Some(bits) }
        );
    });
}

#[test]
fn low_mask_assign_properties() {
    unsigned_low_mask_properties_helper::<u8>();
    unsigned_low_mask_properties_helper::<u16>();
    unsigned_low_mask_properties_helper::<u32>();
    unsigned_low_mask_properties_helper::<u64>();
    unsigned_low_mask_properties_helper::<u128>();
    unsigned_low_mask_properties_helper::<usize>();

    signed_low_mask_properties_helper::<i8>();
    signed_low_mask_properties_helper::<i16>();
    signed_low_mask_properties_helper::<i32>();
    signed_low_mask_properties_helper::<i64>();
    signed_low_mask_properties_helper::<i128>();
    signed_low_mask_properties_helper::<isize>();
}
