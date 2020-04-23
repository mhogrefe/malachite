use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{triples_of_signeds, triples_of_unsigneds};

fn saturating_sub_mul_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        let result = x.saturating_sub_mul(y, z);

        let mut x_alt = x;
        x_alt.saturating_sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.saturating_sub_mul(z, y), result);
        assert!(result <= x);
    });
}

fn saturating_sub_mul_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(triples_of_signeds::<T>, |&(x, y, z)| {
        let result = x.saturating_sub_mul(y, z);

        let mut x_alt = x;
        x_alt.saturating_sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.saturating_sub_mul(z, y), result);
    });
}

#[test]
fn saturating_sub_mul_properties() {
    saturating_sub_mul_properties_unsigned_helper::<u8>();
    saturating_sub_mul_properties_unsigned_helper::<u16>();
    saturating_sub_mul_properties_unsigned_helper::<u32>();
    saturating_sub_mul_properties_unsigned_helper::<u64>();
    saturating_sub_mul_properties_unsigned_helper::<usize>();

    saturating_sub_mul_properties_signed_helper::<i8>();
    saturating_sub_mul_properties_signed_helper::<i16>();
    saturating_sub_mul_properties_signed_helper::<i32>();
    saturating_sub_mul_properties_signed_helper::<i64>();
    saturating_sub_mul_properties_signed_helper::<isize>();
}
