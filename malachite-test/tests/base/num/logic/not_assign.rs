use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn not_assign_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&u| {
        let mut x = u;
        x.not_assign();
        assert_eq!(x, !u);
        x.not_assign();
        assert_eq!(x, u);
    });
}

fn not_assign_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&i| {
        let mut x = i;
        x.not_assign();
        assert_eq!(x, !i);
        x.not_assign();
        assert_eq!(x, i);
    });
}

#[test]
fn not_assign_properties() {
    not_assign_properties_helper_unsigned::<u8>();
    not_assign_properties_helper_unsigned::<u16>();
    not_assign_properties_helper_unsigned::<u32>();
    not_assign_properties_helper_unsigned::<u64>();
    not_assign_properties_helper_unsigned::<usize>();
    not_assign_properties_helper_signed::<i8>();
    not_assign_properties_helper_signed::<i16>();
    not_assign_properties_helper_signed::<i32>();
    not_assign_properties_helper_signed::<i64>();
    not_assign_properties_helper_signed::<isize>();
}
