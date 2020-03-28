use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_signeds, pairs_of_unsigneds};

fn unsigned_overflowing_add_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        let mut sum = x;
        let overflow = sum.overflowing_add_assign(y);
        assert_eq!((sum, overflow), x.overflowing_add(y));
        assert_eq!(x.wrapping_add(y), sum);
        if !overflow {
            assert_eq!(sum, x + y);
        }
    });
}

fn signed_overflowing_add_assign_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        let mut sum = x;
        let overflow = sum.overflowing_add_assign(y);
        assert_eq!((sum, overflow), x.overflowing_add(y));
        assert_eq!(x.wrapping_add(y), sum);
        if !overflow {
            assert_eq!(sum, x + y);
        }
    });
}

#[test]
fn overflowing_add_assign_properties() {
    unsigned_overflowing_add_assign_properties_helper::<u8>();
    unsigned_overflowing_add_assign_properties_helper::<u16>();
    unsigned_overflowing_add_assign_properties_helper::<u32>();
    unsigned_overflowing_add_assign_properties_helper::<u64>();
    unsigned_overflowing_add_assign_properties_helper::<usize>();

    signed_overflowing_add_assign_properties_helper::<i8>();
    signed_overflowing_add_assign_properties_helper::<i16>();
    signed_overflowing_add_assign_properties_helper::<i32>();
    signed_overflowing_add_assign_properties_helper::<i64>();
    signed_overflowing_add_assign_properties_helper::<isize>();
}
