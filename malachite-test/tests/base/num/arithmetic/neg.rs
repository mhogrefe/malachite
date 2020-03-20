use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds_no_min;

fn neg_assign_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds_no_min::<T>, |&n| {
        let mut neg = n;
        neg.neg_assign();
        assert_eq!(neg, -n);
        assert_eq!(-neg, n);
        assert_eq!(neg == n, n == T::ZERO);
        assert_eq!(n + neg, T::ZERO);
    });
}

#[test]
fn neg_assign_properties() {
    neg_assign_properties_helper::<i8>();
    neg_assign_properties_helper::<i16>();
    neg_assign_properties_helper::<i32>();
    neg_assign_properties_helper::<i64>();
    neg_assign_properties_helper::<isize>();
}
