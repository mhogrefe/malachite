use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{triples_of_signeds, triples_of_unsigneds};

fn add_mul_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        let result = x.checked_add_mul(y, z);
        assert_eq!(x.checked_add_mul(z, y), result);
        assert_eq!(result.is_none(), x.overflowing_add_mul(y, z).1);
    });
}

fn add_mul_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(triples_of_signeds::<T>, |&(x, y, z)| {
        let result = x.checked_add_mul(y, z);
        assert_eq!(x.checked_add_mul(z, y), result);
        assert_eq!(result.is_none(), x.overflowing_add_mul(y, z).1);
    });
}

#[test]
fn add_mul_properties() {
    add_mul_properties_unsigned_helper::<u8>();
    add_mul_properties_unsigned_helper::<u16>();
    add_mul_properties_unsigned_helper::<u32>();
    add_mul_properties_unsigned_helper::<u64>();
    add_mul_properties_unsigned_helper::<usize>();

    add_mul_properties_signed_helper::<i8>();
    add_mul_properties_signed_helper::<i16>();
    add_mul_properties_signed_helper::<i32>();
    add_mul_properties_signed_helper::<i64>();
    add_mul_properties_signed_helper::<isize>();
}
