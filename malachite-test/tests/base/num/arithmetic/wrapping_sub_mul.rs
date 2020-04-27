use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signeds, pairs_of_unsigneds, triples_of_signeds, triples_of_unsigneds,
};

fn wrapping_sub_mul_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        let result = x.wrapping_sub_mul(y, z);

        let mut x_alt = x;
        x_alt.wrapping_sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.wrapping_sub_mul(z, y), result);
        assert_eq!(result.wrapping_add_mul(y, z), x);
        assert_eq!(x.overflowing_sub_mul(y, z).0, result);
    });

    test_properties(pairs_of_unsigneds::<T>, |&(a, b)| {
        assert_eq!(a.wrapping_sub_mul(T::ZERO, b), a);
        assert_eq!(a.wrapping_sub_mul(T::ONE, b), a.wrapping_sub(b));
        assert_eq!(
            T::ZERO.wrapping_sub_mul(a, b),
            a.wrapping_mul(b).wrapping_neg()
        );
        assert_eq!(a.wrapping_sub_mul(b, T::ZERO), a);
        assert_eq!(a.wrapping_sub_mul(b, T::ONE), a.wrapping_sub(b));
    });
}

fn wrapping_sub_mul_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(triples_of_signeds::<T>, |&(x, y, z)| {
        let result = x.wrapping_sub_mul(y, z);

        let mut x_alt = x;
        x_alt.wrapping_sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.wrapping_sub_mul(z, y), result);
        assert_eq!(result.wrapping_add_mul(y, z), x);
        assert_eq!(x.overflowing_sub_mul(y, z).0, result);
    });

    test_properties(pairs_of_signeds::<T>, |&(a, b)| {
        assert_eq!(a.wrapping_sub_mul(T::ZERO, b), a);
        assert_eq!(a.wrapping_sub_mul(T::ONE, b), a.wrapping_sub(b));
        assert_eq!(
            T::ZERO.wrapping_sub_mul(a, b),
            a.wrapping_mul(b).wrapping_neg()
        );
        assert_eq!(a.wrapping_sub_mul(b, T::ZERO), a);
        assert_eq!(a.wrapping_sub_mul(b, T::ONE), a.wrapping_sub(b));
    });
}

#[test]
fn wrapping_sub_mul_properties() {
    wrapping_sub_mul_properties_unsigned_helper::<u8>();
    wrapping_sub_mul_properties_unsigned_helper::<u16>();
    wrapping_sub_mul_properties_unsigned_helper::<u32>();
    wrapping_sub_mul_properties_unsigned_helper::<u64>();
    wrapping_sub_mul_properties_unsigned_helper::<usize>();

    wrapping_sub_mul_properties_signed_helper::<i8>();
    wrapping_sub_mul_properties_signed_helper::<i16>();
    wrapping_sub_mul_properties_signed_helper::<i32>();
    wrapping_sub_mul_properties_signed_helper::<i64>();
    wrapping_sub_mul_properties_signed_helper::<isize>();
}
