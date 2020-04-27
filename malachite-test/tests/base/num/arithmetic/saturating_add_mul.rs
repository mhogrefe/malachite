use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signeds, pairs_of_unsigneds, triples_of_signeds, triples_of_unsigneds,
};

fn saturating_add_mul_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        let result = x.saturating_add_mul(y, z);

        let mut x_alt = x;
        x_alt.saturating_add_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.saturating_add_mul(z, y), result);
        assert!(result >= x);
    });

    test_properties(pairs_of_unsigneds::<T>, |&(a, b)| {
        assert_eq!(a.saturating_add_mul(T::ZERO, b), a);
        assert_eq!(a.saturating_add_mul(T::ONE, b), a.saturating_add(b));
        assert_eq!(T::ZERO.saturating_add_mul(a, b), a.saturating_mul(b));
        assert_eq!(a.saturating_add_mul(b, T::ZERO), a);
        assert_eq!(a.saturating_add_mul(b, T::ONE), a.saturating_add(b));
    });
}

fn saturating_add_mul_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(triples_of_signeds::<T>, |&(x, y, z)| {
        let result = x.saturating_add_mul(y, z);

        let mut x_alt = x;
        x_alt.saturating_add_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.saturating_add_mul(z, y), result);
    });

    test_properties(pairs_of_signeds::<T>, |&(a, b)| {
        assert_eq!(a.saturating_add_mul(T::ZERO, b), a);
        assert_eq!(a.saturating_add_mul(T::ONE, b), a.saturating_add(b));
        assert_eq!(T::ZERO.saturating_add_mul(a, b), a.saturating_mul(b));
        assert_eq!(a.saturating_add_mul(b, T::ZERO), a);
        assert_eq!(a.saturating_add_mul(b, T::ONE), a.saturating_add(b));
    });
}

#[test]
fn saturating_add_mul_properties() {
    saturating_add_mul_properties_unsigned_helper::<u8>();
    saturating_add_mul_properties_unsigned_helper::<u16>();
    saturating_add_mul_properties_unsigned_helper::<u32>();
    saturating_add_mul_properties_unsigned_helper::<u64>();
    saturating_add_mul_properties_unsigned_helper::<usize>();

    saturating_add_mul_properties_signed_helper::<i8>();
    saturating_add_mul_properties_signed_helper::<i16>();
    saturating_add_mul_properties_signed_helper::<i32>();
    saturating_add_mul_properties_signed_helper::<i64>();
    saturating_add_mul_properties_signed_helper::<isize>();
}
