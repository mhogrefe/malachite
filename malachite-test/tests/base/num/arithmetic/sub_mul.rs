use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{triples_of_signeds_var_3, triples_of_unsigneds_var_4};

fn sub_mul_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds_var_4::<T>, |&(x, y, z)| {
        let result = x.sub_mul(y, z);

        let mut x_alt = x;
        x_alt.sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.sub_mul(z, y), result);
        assert_eq!(result.add_mul(y, z), x);
        assert_eq!(x.checked_sub_mul(y, z), Some(result));
        assert_eq!(x.saturating_sub_mul(y, z), result);
        assert_eq!(x.wrapping_sub_mul(y, z), result);
        assert_eq!(x.overflowing_sub_mul(y, z), (result, false));
    });
}

fn sub_mul_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(triples_of_signeds_var_3::<T>, |&(x, y, z)| {
        let result = x.sub_mul(y, z);

        let mut x_alt = x;
        x_alt.sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.sub_mul(z, y), result);
        assert_eq!(result.add_mul(y, z), x);
        assert_eq!(x.checked_sub_mul(y, z), Some(result));
        assert_eq!(x.saturating_sub_mul(y, z), result);
        assert_eq!(x.wrapping_sub_mul(y, z), result);
        assert_eq!(x.overflowing_sub_mul(y, z), (result, false));
    });
}

#[test]
fn sub_mul_properties() {
    sub_mul_properties_unsigned_helper::<u8>();
    sub_mul_properties_unsigned_helper::<u16>();
    sub_mul_properties_unsigned_helper::<u32>();
    sub_mul_properties_unsigned_helper::<u64>();
    sub_mul_properties_unsigned_helper::<usize>();

    sub_mul_properties_signed_helper::<i8>();
    sub_mul_properties_signed_helper::<i16>();
    sub_mul_properties_signed_helper::<i32>();
    sub_mul_properties_signed_helper::<i64>();
    sub_mul_properties_signed_helper::<isize>();
}
