use malachite_base::num::arithmetic::xx_sub_yy_is_zz::_explicit_xx_sub_yy_is_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_unsigneds, quadruples_of_unsigneds};

fn xx_sub_yy_is_zz_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(quadruples_of_unsigneds::<T>, |&(x_1, x_0, y_1, y_0)| {
        let (z_1, z_0) = T::xx_sub_yy_is_zz(x_1, x_0, y_1, y_0);
        assert_eq!(_explicit_xx_sub_yy_is_zz(x_1, x_0, y_1, y_0), (z_1, z_0));

        assert_eq!(T::xx_add_yy_is_zz(z_1, z_0, y_1, y_0), (x_1, x_0));
        assert_eq!(
            T::xx_sub_yy_is_zz(z_1, z_0, x_1, x_0),
            T::xx_sub_yy_is_zz(T::ZERO, T::ZERO, y_1, y_0)
        );
        assert_eq!(
            T::xx_sub_yy_is_zz(y_1, y_0, x_1, x_0),
            T::xx_sub_yy_is_zz(T::ZERO, T::ZERO, z_1, z_0)
        );

        let (neg_y_1, neg_y_0) = T::xx_sub_yy_is_zz(T::ZERO, T::ZERO, y_1, y_0);
        assert_eq!(T::xx_add_yy_is_zz(x_1, x_0, neg_y_1, neg_y_0), (z_1, z_0));
    });

    test_properties(pairs_of_unsigneds::<T>, |&(x_1, x_0)| {
        assert_eq!(T::xx_sub_yy_is_zz(x_1, x_0, T::ZERO, T::ZERO), (x_1, x_0));
        assert_eq!(T::xx_sub_yy_is_zz(x_1, x_0, x_1, x_0), (T::ZERO, T::ZERO));
    });
}

#[test]
fn xx_sub_yy_is_zz_properties() {
    xx_sub_yy_is_zz_properties_helper::<u8>();
    xx_sub_yy_is_zz_properties_helper::<u16>();
    xx_sub_yy_is_zz_properties_helper::<u32>();
    xx_sub_yy_is_zz_properties_helper::<u64>();
    xx_sub_yy_is_zz_properties_helper::<usize>();
}
