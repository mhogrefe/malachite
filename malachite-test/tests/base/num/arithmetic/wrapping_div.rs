use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_nonzero_signed, pairs_of_unsigned_and_positive_unsigned,
};

fn unsigned_wrapping_div_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<T, T>,
        |&(x, y)| {
            let mut quotient = x;
            quotient.wrapping_div_assign(y);
            assert_eq!(quotient, x.wrapping_div(y));
            assert_eq!(x / y, quotient);
        },
    );
}

fn signed_wrapping_div_assign_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_nonzero_signed::<T, T>, |&(x, y)| {
        let mut quotient = x;
        quotient.wrapping_div_assign(y);
        assert_eq!(quotient, x.wrapping_div(y));
        if x != T::MIN || y != T::NEGATIVE_ONE {
            assert_eq!(x / y, quotient);
        }
    });
}

#[test]
fn wrapping_div_assign_properties() {
    unsigned_wrapping_div_assign_properties_helper::<u8>();
    unsigned_wrapping_div_assign_properties_helper::<u16>();
    unsigned_wrapping_div_assign_properties_helper::<u32>();
    unsigned_wrapping_div_assign_properties_helper::<u64>();
    unsigned_wrapping_div_assign_properties_helper::<usize>();

    signed_wrapping_div_assign_properties_helper::<i8>();
    signed_wrapping_div_assign_properties_helper::<i16>();
    signed_wrapping_div_assign_properties_helper::<i32>();
    signed_wrapping_div_assign_properties_helper::<i64>();
    signed_wrapping_div_assign_properties_helper::<isize>();
}
