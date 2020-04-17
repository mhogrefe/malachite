use malachite_base::num::arithmetic::xx_div_mod_y_is_qr::_explicit_xx_div_mod_y_is_qr;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{positive_unsigneds, triples_of_unsigneds_var_2};

fn xx_div_mod_y_is_qr_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(triples_of_unsigneds_var_2::<T>, |&(x_1, x_0, y)| {
        let (q, r) = T::xx_div_mod_y_is_qr(x_1, x_0, y);
        assert_eq!(_explicit_xx_div_mod_y_is_qr(x_1, x_0, y), (q, r));

        assert!(r < y);
        let (product_1, product_0) = T::x_mul_y_is_zz(q, y);
        assert_eq!(
            T::xx_add_yy_is_zz(product_1, product_0, T::ZERO, r),
            (x_1, x_0)
        );
    });

    test_properties(positive_unsigneds::<T>, |&a| {
        assert_eq!(
            T::xx_div_mod_y_is_qr(T::ZERO, T::ZERO, a),
            (T::ZERO, T::ZERO)
        );
        assert_eq!(T::xx_div_mod_y_is_qr(T::ZERO, a, a), (T::ONE, T::ZERO));
    });
}

#[test]
fn xx_div_mod_y_is_qr_properties() {
    xx_div_mod_y_is_qr_properties_helper::<u8>();
    xx_div_mod_y_is_qr_properties_helper::<u16>();
    xx_div_mod_y_is_qr_properties_helper::<u32>();
    xx_div_mod_y_is_qr_properties_helper::<u64>();
    xx_div_mod_y_is_qr_properties_helper::<usize>();
}
