use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signeds, pairs_of_unsigneds, triples_of_signeds, triples_of_unsigneds,
};

fn sub_mul_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        let result = x.checked_sub_mul(y, z);
        assert_eq!(x.checked_sub_mul(z, y), result);
        assert_eq!(result.is_none(), x.overflowing_sub_mul(y, z).1);
    });

    test_properties(pairs_of_unsigneds::<T>, |&(a, b)| {
        assert_eq!(a.checked_sub_mul(T::ZERO, b), Some(a));
        assert_eq!(a.checked_sub_mul(T::ONE, b), a.checked_sub(b));
        assert_eq!(a.checked_sub_mul(b, T::ZERO), Some(a));
        assert_eq!(a.checked_sub_mul(b, T::ONE), a.checked_sub(b));
    });
}

fn sub_mul_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(triples_of_signeds::<T>, |&(x, y, z)| {
        let result = x.checked_sub_mul(y, z);
        assert_eq!(x.checked_sub_mul(z, y), result);
        assert_eq!(result.is_none(), x.overflowing_sub_mul(y, z).1);
    });

    test_properties(pairs_of_signeds::<T>, |&(a, b)| {
        assert_eq!(a.checked_sub_mul(T::ZERO, b), Some(a));
        assert_eq!(a.checked_sub_mul(T::ONE, b), a.checked_sub(b));
        assert_eq!(a.checked_sub_mul(b, T::ZERO), Some(a));
        assert_eq!(a.checked_sub_mul(b, T::ONE), a.checked_sub(b));
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
