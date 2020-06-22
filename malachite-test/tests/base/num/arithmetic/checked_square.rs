use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn unsigned_checked_square_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&x| {
        let square = x.checked_square();
        assert_eq!(square, x.checked_pow(2));
        if let Some(square) = square {
            assert_eq!(x.square(), square);
        }
    });
}

fn signed_checked_square_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&x| {
        let square = x.checked_square();
        assert_eq!(square, x.checked_pow(2));
        if let Some(square) = square {
            assert_eq!(x.square(), square);
        }
    });
}

#[test]
fn checked_square_properties() {
    unsigned_checked_square_properties_helper::<u8>();
    unsigned_checked_square_properties_helper::<u16>();
    unsigned_checked_square_properties_helper::<u32>();
    unsigned_checked_square_properties_helper::<u64>();
    unsigned_checked_square_properties_helper::<usize>();

    signed_checked_square_properties_helper::<i8>();
    signed_checked_square_properties_helper::<i16>();
    signed_checked_square_properties_helper::<i32>();
    signed_checked_square_properties_helper::<i64>();
    signed_checked_square_properties_helper::<isize>();
}
