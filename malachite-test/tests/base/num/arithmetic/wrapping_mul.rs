use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_signeds, pairs_of_unsigneds};

fn unsigned_wrapping_mul_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        let mut product = x;
        product.wrapping_mul_assign(y);
        assert_eq!(product, x.wrapping_mul(y));
        assert_eq!(y.wrapping_mul(x), product);
    });
}

fn signed_wrapping_mul_assign_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        let mut product = x;
        product.wrapping_mul_assign(y);
        assert_eq!(product, x.wrapping_mul(y));
        assert_eq!(y.wrapping_mul(x), product);
    });
}

#[test]
fn wrapping_mul_assign_properties() {
    unsigned_wrapping_mul_assign_properties_helper::<u8>();
    unsigned_wrapping_mul_assign_properties_helper::<u16>();
    unsigned_wrapping_mul_assign_properties_helper::<u32>();
    unsigned_wrapping_mul_assign_properties_helper::<u64>();
    unsigned_wrapping_mul_assign_properties_helper::<usize>();

    signed_wrapping_mul_assign_properties_helper::<i8>();
    signed_wrapping_mul_assign_properties_helper::<i16>();
    signed_wrapping_mul_assign_properties_helper::<i32>();
    signed_wrapping_mul_assign_properties_helper::<i64>();
    signed_wrapping_mul_assign_properties_helper::<isize>();
}
