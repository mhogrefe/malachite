use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn get_highest_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |u| {
        assert_eq!(u.get_highest_bit(), u >= &(T::power_of_two(T::WIDTH - 1)));
    });
}

fn get_highest_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |i| {
        assert_eq!(i.get_highest_bit(), i < &T::ZERO);
    });
}

#[test]
fn get_highest_bit_properties() {
    get_highest_bit_properties_helper_unsigned::<u8>();
    get_highest_bit_properties_helper_unsigned::<u16>();
    get_highest_bit_properties_helper_unsigned::<u32>();
    get_highest_bit_properties_helper_unsigned::<u64>();
    get_highest_bit_properties_helper_unsigned::<usize>();
    get_highest_bit_properties_helper_signed::<i8>();
    get_highest_bit_properties_helper_signed::<i16>();
    get_highest_bit_properties_helper_signed::<i32>();
    get_highest_bit_properties_helper_signed::<i64>();
    get_highest_bit_properties_helper_signed::<isize>();
}
