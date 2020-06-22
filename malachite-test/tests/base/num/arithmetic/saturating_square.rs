use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn unsigned_saturating_square_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&x| {
        let mut square = x;
        square.saturating_square_assign();
        assert_eq!(square, x.saturating_square());
        assert_eq!(square, x.saturating_pow(2));
        assert!(square >= x);
        if square < T::MAX {
            assert_eq!(square, x.square());
        }
    });
}

fn signed_saturating_square_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&x| {
        let mut square = x;
        square.saturating_square_assign();
        assert_eq!(square, x.saturating_square());
        assert_eq!(square, x.saturating_pow(2));
        if square > T::MIN && square < T::MAX {
            assert_eq!(square, x.square());
        }
    });
}

#[test]
fn saturating_square_properties() {
    unsigned_saturating_square_properties_helper::<u8>();
    unsigned_saturating_square_properties_helper::<u16>();
    unsigned_saturating_square_properties_helper::<u32>();
    unsigned_saturating_square_properties_helper::<u64>();
    unsigned_saturating_square_properties_helper::<usize>();

    signed_saturating_square_properties_helper::<i8>();
    signed_saturating_square_properties_helper::<i16>();
    signed_saturating_square_properties_helper::<i32>();
    signed_saturating_square_properties_helper::<i64>();
    signed_saturating_square_properties_helper::<isize>();
}
