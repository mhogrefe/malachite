use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds;

fn saturating_neg_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&n| {
        let mut neg = n;
        neg.saturating_neg_assign();
        assert_eq!(neg, n.saturating_neg());
        if n != T::MIN {
            assert_eq!(neg.saturating_neg(), n);
        }
        assert_eq!(neg == n, n == T::ZERO);
    });
}

#[test]
fn saturating_neg_properties() {
    saturating_neg_properties_helper::<i8>();
    saturating_neg_properties_helper::<i16>();
    saturating_neg_properties_helper::<i32>();
    saturating_neg_properties_helper::<i64>();
    saturating_neg_properties_helper::<isize>();
}
