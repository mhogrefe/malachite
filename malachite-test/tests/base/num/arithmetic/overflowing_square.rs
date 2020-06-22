use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn unsigned_overflowing_square_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&x| {
        let mut square = x;
        let overflow = square.overflowing_square_assign();
        assert_eq!((square, overflow), x.overflowing_square());
        assert_eq!((square, overflow), x.overflowing_pow(2));
        assert_eq!(x.wrapping_square(), square);
        assert_eq!(x.checked_square().is_none(), overflow);
        if !overflow {
            assert_eq!(square, x.square());
        }
    });
}

fn signed_overflowing_square_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&x| {
        let mut square = x;
        let overflow = square.overflowing_square_assign();
        assert_eq!((square, overflow), x.overflowing_square());
        assert_eq!((square, overflow), x.overflowing_pow(2));
        assert_eq!(x.wrapping_square(), square);
        assert_eq!(x.checked_square().is_none(), overflow);
        if !overflow {
            assert_eq!(square, x.square());
        }
    });
}

#[test]
fn overflowing_square_properties() {
    unsigned_overflowing_square_properties_helper::<u8>();
    unsigned_overflowing_square_properties_helper::<u16>();
    unsigned_overflowing_square_properties_helper::<u32>();
    unsigned_overflowing_square_properties_helper::<u64>();
    unsigned_overflowing_square_properties_helper::<usize>();

    signed_overflowing_square_properties_helper::<i8>();
    signed_overflowing_square_properties_helper::<i16>();
    signed_overflowing_square_properties_helper::<i32>();
    signed_overflowing_square_properties_helper::<i64>();
    signed_overflowing_square_properties_helper::<isize>();
}
