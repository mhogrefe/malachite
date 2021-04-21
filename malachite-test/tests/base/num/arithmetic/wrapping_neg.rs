use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn unsigned_wrapping_neg_assign_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&n| {
        let mut neg = n;
        neg.wrapping_neg_assign();
        assert_eq!(neg, n.wrapping_neg());
        assert_eq!(neg.wrapping_neg(), n);
        assert_eq!(neg == n, n == T::ZERO || n == T::power_of_2(T::WIDTH - 1));
        assert_eq!(n.wrapping_add(neg), T::ZERO);
    });
}

fn signed_wrapping_neg_assign_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&n| {
        let mut neg = n;
        neg.wrapping_neg_assign();
        assert_eq!(neg, n.wrapping_neg());
        assert_eq!(neg.wrapping_neg(), n);
        assert_eq!(neg == n, n == T::ZERO || n == T::MIN);
        assert_eq!(n.wrapping_add(neg), T::ZERO);
    });
}

#[test]
fn wrapping_neg_assign_properties() {
    unsigned_wrapping_neg_assign_properties_helper::<u8>();
    unsigned_wrapping_neg_assign_properties_helper::<u16>();
    unsigned_wrapping_neg_assign_properties_helper::<u32>();
    unsigned_wrapping_neg_assign_properties_helper::<u64>();
    unsigned_wrapping_neg_assign_properties_helper::<usize>();

    signed_wrapping_neg_assign_properties_helper::<i8>();
    signed_wrapping_neg_assign_properties_helper::<i16>();
    signed_wrapping_neg_assign_properties_helper::<i32>();
    signed_wrapping_neg_assign_properties_helper::<i64>();
    signed_wrapping_neg_assign_properties_helper::<isize>();
}
