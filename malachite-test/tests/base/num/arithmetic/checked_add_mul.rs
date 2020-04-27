use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signeds, pairs_of_unsigneds, triples_of_signeds, triples_of_unsigneds,
};

fn add_mul_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        let result = x.checked_add_mul(y, z);
        assert_eq!(x.checked_add_mul(z, y), result);
        assert_eq!(result.is_none(), x.overflowing_add_mul(y, z).1);
    });

    test_properties(pairs_of_unsigneds::<T>, |&(a, b)| {
        assert_eq!(a.checked_add_mul(T::ZERO, b), Some(a));
        assert_eq!(a.checked_add_mul(T::ONE, b), a.checked_add(b));
        assert_eq!(T::ZERO.checked_add_mul(a, b), a.checked_mul(b));
        assert_eq!(a.checked_add_mul(b, T::ZERO), Some(a));
        assert_eq!(a.checked_add_mul(b, T::ONE), a.checked_add(b));
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

    test_properties(pairs_of_signeds::<T>, |&(a, b)| {
        assert_eq!(a.checked_add_mul(T::ZERO, b), Some(a));
        assert_eq!(a.checked_add_mul(T::ONE, b), a.checked_add(b));
        assert_eq!(T::ZERO.checked_add_mul(a, b), a.checked_mul(b));
        assert_eq!(a.checked_add_mul(b, T::ZERO), Some(a));
        assert_eq!(a.checked_add_mul(b, T::ONE), a.checked_add(b));
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
