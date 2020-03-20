use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, signeds_no_min};

fn abs_assign_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>
        + ExactFrom<<T as UnsignedAbs>::Output>,
{
    test_properties(signeds_no_min::<T>, |&n| {
        let mut abs = n;
        abs.abs_assign();
        assert_eq!(abs, n.abs());
        assert_eq!(abs.abs(), abs);
        assert_eq!(abs == n, n >= T::ZERO);
        assert_eq!(T::exact_from(n.unsigned_abs()), abs)
    });
}

#[test]
fn abs_assign_properties() {
    abs_assign_properties_helper::<i8>();
    abs_assign_properties_helper::<i16>();
    abs_assign_properties_helper::<i32>();
    abs_assign_properties_helper::<i64>();
    abs_assign_properties_helper::<isize>();
}

fn unsigned_abs_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&n| {
        n.unsigned_abs();
    });
}

#[test]
fn unsigned_abs_properties() {
    unsigned_abs_properties_helper::<i8>();
    unsigned_abs_properties_helper::<i16>();
    unsigned_abs_properties_helper::<i32>();
    unsigned_abs_properties_helper::<i64>();
    unsigned_abs_properties_helper::<isize>();
}
