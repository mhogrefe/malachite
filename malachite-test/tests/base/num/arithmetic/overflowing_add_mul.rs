use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signeds, pairs_of_unsigneds, triples_of_signeds, triples_of_unsigneds,
};

fn overflowing_add_mul_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        let (result, overflow) = x.overflowing_add_mul(y, z);

        let mut x_alt = x;
        assert_eq!(x_alt.overflowing_add_mul_assign(y, z), overflow);
        assert_eq!(x_alt, result);

        assert_eq!(x.overflowing_add_mul(z, y), (result, overflow));
        assert_eq!(result.overflowing_sub_mul(y, z), (x, overflow));
        assert_eq!(x.wrapping_add_mul(y, z), result);
        assert_eq!(x.checked_add_mul(y, z).is_none(), overflow);
    });

    test_properties(pairs_of_unsigneds::<T>, |&(a, b)| {
        assert_eq!(a.overflowing_add_mul(T::ZERO, b), (a, false));
        assert_eq!(a.overflowing_add_mul(T::ONE, b), a.overflowing_add(b));
        assert_eq!(T::ZERO.overflowing_add_mul(a, b), a.overflowing_mul(b));
        assert_eq!(a.overflowing_add_mul(b, T::ZERO), (a, false));
        assert_eq!(a.overflowing_add_mul(b, T::ONE), a.overflowing_add(b));
    });
}

fn overflowing_add_mul_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(triples_of_signeds::<T>, |&(x, y, z)| {
        let (result, overflow) = x.overflowing_add_mul(y, z);

        let mut x_alt = x;
        assert_eq!(x_alt.overflowing_add_mul_assign(y, z), overflow);
        assert_eq!(x_alt, result);

        assert_eq!(x.overflowing_add_mul(z, y), (result, overflow));
        assert_eq!(result.overflowing_sub_mul(y, z), (x, overflow));
        assert_eq!(x.wrapping_add_mul(y, z), result);
        assert_eq!(x.checked_add_mul(y, z).is_none(), overflow);
    });

    test_properties(pairs_of_signeds::<T>, |&(a, b)| {
        assert_eq!(a.overflowing_add_mul(T::ZERO, b), (a, false));
        assert_eq!(a.overflowing_add_mul(T::ONE, b), a.overflowing_add(b));
        assert_eq!(T::ZERO.overflowing_add_mul(a, b), a.overflowing_mul(b));
        assert_eq!(a.overflowing_add_mul(b, T::ZERO), (a, false));
        assert_eq!(a.overflowing_add_mul(b, T::ONE), a.overflowing_add(b));
    });
}

#[test]
fn overflowing_add_mul_properties() {
    overflowing_add_mul_properties_unsigned_helper::<u8>();
    overflowing_add_mul_properties_unsigned_helper::<u16>();
    overflowing_add_mul_properties_unsigned_helper::<u32>();
    overflowing_add_mul_properties_unsigned_helper::<u64>();
    overflowing_add_mul_properties_unsigned_helper::<usize>();

    overflowing_add_mul_properties_signed_helper::<i8>();
    overflowing_add_mul_properties_signed_helper::<i16>();
    overflowing_add_mul_properties_signed_helper::<i32>();
    overflowing_add_mul_properties_signed_helper::<i64>();
    overflowing_add_mul_properties_signed_helper::<isize>();
}
