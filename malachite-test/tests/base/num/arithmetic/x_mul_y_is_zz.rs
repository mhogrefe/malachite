use malachite_base::num::arithmetic::x_mul_y_is_zz::_explicit_x_mul_y_is_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigneds, triples_of_unsigneds, unsigneds};

fn x_mul_y_is_zz_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        let (z_1, z_0) = T::x_mul_y_is_zz(x, y);
        assert_eq!(_explicit_x_mul_y_is_zz(x, y), (z_1, z_0));

        assert_eq!(T::x_mul_y_is_zz(y, x), (z_1, z_0));
    });

    test_properties(unsigneds::<T>, |&x| {
        assert_eq!(T::x_mul_y_is_zz(x, T::ZERO), (T::ZERO, T::ZERO));
        assert_eq!(T::x_mul_y_is_zz(T::ZERO, x), (T::ZERO, T::ZERO));
        assert_eq!(T::x_mul_y_is_zz(x, T::ONE), (T::ZERO, x));
        assert_eq!(T::x_mul_y_is_zz(T::ONE, x), (T::ZERO, x));
    });

    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        let (_, product_1) = T::x_mul_y_is_zz(x, y);
        let (_, product_2) = T::x_mul_y_is_zz(y, z);
        assert_eq!(product_1.wrapping_mul(z), x.wrapping_mul(product_2));
    });
}

#[test]
fn x_mul_y_is_zz_properties() {
    x_mul_y_is_zz_properties_helper::<u8>();
    x_mul_y_is_zz_properties_helper::<u16>();
    x_mul_y_is_zz_properties_helper::<u32>();
    x_mul_y_is_zz_properties_helper::<u64>();
    x_mul_y_is_zz_properties_helper::<usize>();
}
