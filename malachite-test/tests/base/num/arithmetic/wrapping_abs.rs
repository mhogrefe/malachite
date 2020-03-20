use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds;

fn wrapping_abs_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&n| {
        let mut abs = n;
        abs.wrapping_abs_assign();
        assert_eq!(abs, n.wrapping_abs());
        assert_eq!(abs.wrapping_abs(), abs);
        if n != T::MIN {
            assert_eq!(n.abs(), abs);
        }
        assert_eq!(abs == n, n >= T::ZERO || n == T::MIN);
    });
}

#[test]
fn wrapping_abs_properties() {
    wrapping_abs_properties_helper::<i8>();
    wrapping_abs_properties_helper::<i16>();
    wrapping_abs_properties_helper::<i32>();
    wrapping_abs_properties_helper::<i64>();
    wrapping_abs_properties_helper::<isize>();
}
