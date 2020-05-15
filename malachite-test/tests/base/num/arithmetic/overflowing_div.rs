use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_nonzero_signed, pairs_of_unsigned_and_positive_unsigned,
};

fn unsigned_overflowing_div_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<T, T>,
        |&(x, y)| {
            let mut quotient = x;
            let overflow = quotient.overflowing_div_assign(y);
            assert_eq!((quotient, overflow), x.overflowing_div(y));
            assert_eq!(x.wrapping_div(y), quotient);
            assert!(!overflow);
            assert_eq!(quotient, x / y);
        },
    );
}

fn signed_overflowing_div_assign_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_nonzero_signed::<T, T>, |&(x, y)| {
        let mut quotient = x;
        let overflow = quotient.overflowing_div_assign(y);
        assert_eq!((quotient, overflow), x.overflowing_div(y));
        assert_eq!(x.wrapping_div(y), quotient);
        if !overflow {
            assert_eq!(quotient, x / y);
        }
    });
}

#[test]
fn overflowing_div_assign_properties() {
    unsigned_overflowing_div_assign_properties_helper::<u8>();
    unsigned_overflowing_div_assign_properties_helper::<u16>();
    unsigned_overflowing_div_assign_properties_helper::<u32>();
    unsigned_overflowing_div_assign_properties_helper::<u64>();
    unsigned_overflowing_div_assign_properties_helper::<usize>();

    signed_overflowing_div_assign_properties_helper::<i8>();
    signed_overflowing_div_assign_properties_helper::<i16>();
    signed_overflowing_div_assign_properties_helper::<i32>();
    signed_overflowing_div_assign_properties_helper::<i64>();
    signed_overflowing_div_assign_properties_helper::<isize>();
}
