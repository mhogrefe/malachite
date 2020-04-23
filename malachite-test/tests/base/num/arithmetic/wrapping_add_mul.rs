use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{triples_of_signeds, triples_of_unsigneds};

fn wrapping_add_mul_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        let result = x.wrapping_add_mul(y, z);

        let mut x_alt = x;
        x_alt.wrapping_add_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.wrapping_add_mul(z, y), result);
        assert_eq!(result.wrapping_sub_mul(y, z), x);
        assert_eq!(x.overflowing_add_mul(y, z).0, result);
    });
}

fn wrapping_add_mul_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(triples_of_signeds::<T>, |&(x, y, z)| {
        let result = x.wrapping_add_mul(y, z);

        let mut x_alt = x;
        x_alt.wrapping_add_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.wrapping_add_mul(z, y), result);
        assert_eq!(result.wrapping_sub_mul(y, z), x);
        assert_eq!(x.overflowing_add_mul(y, z).0, result);
    });
}

#[test]
fn wrapping_add_mul_properties() {
    wrapping_add_mul_properties_unsigned_helper::<u8>();
    wrapping_add_mul_properties_unsigned_helper::<u16>();
    wrapping_add_mul_properties_unsigned_helper::<u32>();
    wrapping_add_mul_properties_unsigned_helper::<u64>();
    wrapping_add_mul_properties_unsigned_helper::<usize>();

    wrapping_add_mul_properties_signed_helper::<i8>();
    wrapping_add_mul_properties_signed_helper::<i16>();
    wrapping_add_mul_properties_signed_helper::<i32>();
    wrapping_add_mul_properties_signed_helper::<i64>();
    wrapping_add_mul_properties_signed_helper::<isize>();
}
