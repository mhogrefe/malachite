use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{small_u64s_var_2, small_u64s_var_3};

fn unsigned_power_of_two_properties_helper<T: PrimitiveUnsigned>() {
    test_properties_no_special(small_u64s_var_2::<T>, |&pow| {
        let mut n = T::power_of_two(pow);
        assert_eq!(n.checked_log_two(), Some(pow));
        n.clear_bit(pow);
        assert_eq!(n, T::ZERO);
    });
}

fn signed_power_of_two_properties_helper<T: PrimitiveSigned>()
where
    T::UnsignedOfEqualWidth: ExactFrom<T>,
{
    test_properties_no_special(small_u64s_var_3::<T>, |&pow| {
        let mut n = T::power_of_two(pow);
        assert_eq!(
            T::UnsignedOfEqualWidth::exact_from(n),
            T::UnsignedOfEqualWidth::power_of_two(pow)
        );
        n.clear_bit(pow);
        assert_eq!(n, T::ZERO);
    });
}

#[test]
fn power_of_two_properties() {
    unsigned_power_of_two_properties_helper::<u8>();
    unsigned_power_of_two_properties_helper::<u16>();
    unsigned_power_of_two_properties_helper::<u32>();
    unsigned_power_of_two_properties_helper::<u64>();
    unsigned_power_of_two_properties_helper::<u128>();
    unsigned_power_of_two_properties_helper::<usize>();

    signed_power_of_two_properties_helper::<i8>();
    signed_power_of_two_properties_helper::<i16>();
    signed_power_of_two_properties_helper::<i32>();
    signed_power_of_two_properties_helper::<i64>();
    signed_power_of_two_properties_helper::<i128>();
    signed_power_of_two_properties_helper::<isize>();
}
