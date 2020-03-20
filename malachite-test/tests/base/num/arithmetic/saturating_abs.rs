use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds;

fn saturating_abs_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&n| {
        let mut abs = n;
        abs.saturating_abs_assign();
        assert_eq!(abs, n.saturating_abs());
        assert_eq!(abs.saturating_abs(), abs);
        if n != T::MIN {
            assert_eq!(n.abs(), abs);
        }
        assert_eq!(abs == n, n >= T::ZERO);
    });
}

#[test]
fn saturating_abs_properties() {
    saturating_abs_properties_helper::<i8>();
    saturating_abs_properties_helper::<i16>();
    saturating_abs_properties_helper::<i32>();
    saturating_abs_properties_helper::<i64>();
    saturating_abs_properties_helper::<isize>();
}
