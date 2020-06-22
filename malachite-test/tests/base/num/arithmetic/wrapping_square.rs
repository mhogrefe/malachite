use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn unsigned_wrapping_square_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&x| {
        let mut square = x;
        square.wrapping_square_assign();
        assert_eq!(square, x.wrapping_square());
        assert_eq!(square, x.wrapping_pow(2));
    });
}

fn signed_wrapping_square_assign_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&x| {
        let mut square = x;
        square.wrapping_square_assign();
        assert_eq!(square, x.wrapping_square());
        assert_eq!(square, x.wrapping_pow(2));
        if x != T::MIN {
            assert_eq!((-x).wrapping_square(), square);
        }
    });
}

#[test]
fn wrapping_square_assign_properties() {
    unsigned_wrapping_square_assign_properties_helper::<u8>();
    unsigned_wrapping_square_assign_properties_helper::<u16>();
    unsigned_wrapping_square_assign_properties_helper::<u32>();
    unsigned_wrapping_square_assign_properties_helper::<u64>();
    unsigned_wrapping_square_assign_properties_helper::<usize>();

    signed_wrapping_square_assign_properties_helper::<i8>();
    signed_wrapping_square_assign_properties_helper::<i16>();
    signed_wrapping_square_assign_properties_helper::<i32>();
    signed_wrapping_square_assign_properties_helper::<i64>();
    signed_wrapping_square_assign_properties_helper::<isize>();
}
