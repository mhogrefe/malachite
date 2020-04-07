use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_signeds, pairs_of_unsigneds};

fn unsigned_saturating_mul_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        let mut product = x;
        product.saturating_mul_assign(y);
        assert_eq!(product, x.saturating_mul(y));
        assert_eq!(y.saturating_mul(x), product);
        assert!(product == T::ZERO || product >= x);
        assert!(product == T::ZERO || product >= y);
        if product < T::MAX {
            assert_eq!(product, x * y);
        }
    });
}

fn signed_saturating_mul_assign_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        let mut product = x;
        product.saturating_mul_assign(y);
        assert_eq!(product, x.saturating_mul(y));
        assert_eq!(y.saturating_mul(x), product);
        if product > T::MIN && product < T::MAX {
            assert_eq!(product, x * y);
        }
    });
}

#[test]
fn saturating_mul_assign_properties() {
    unsigned_saturating_mul_assign_properties_helper::<u8>();
    unsigned_saturating_mul_assign_properties_helper::<u16>();
    unsigned_saturating_mul_assign_properties_helper::<u32>();
    unsigned_saturating_mul_assign_properties_helper::<u64>();
    unsigned_saturating_mul_assign_properties_helper::<usize>();

    signed_saturating_mul_assign_properties_helper::<i8>();
    signed_saturating_mul_assign_properties_helper::<i16>();
    signed_saturating_mul_assign_properties_helper::<i32>();
    signed_saturating_mul_assign_properties_helper::<i64>();
    signed_saturating_mul_assign_properties_helper::<isize>();
}
