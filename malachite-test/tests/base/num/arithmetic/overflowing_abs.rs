use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds;

fn overflowing_abs_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&n| {
        let mut abs = n;
        let overflow = abs.overflowing_abs_assign();
        assert_eq!((abs, overflow), n.overflowing_abs());
        assert_eq!(abs, n.wrapping_abs());
        if n != T::MIN {
            assert_eq!(n.abs(), abs);
        }
        assert_eq!(abs == n, n >= T::ZERO || n == T::MIN);
        assert_eq!(n == T::MIN, overflow);
    });
}

#[test]
fn overflowing_abs_properties() {
    overflowing_abs_properties_helper::<i8>();
    overflowing_abs_properties_helper::<i16>();
    overflowing_abs_properties_helper::<i32>();
    overflowing_abs_properties_helper::<i64>();
    overflowing_abs_properties_helper::<isize>();
}
