use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    nonuples_of_unsigneds, sextuples_of_unsigneds, triples_of_unsigneds,
};

fn xxx_add_yyy_is_zzz_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(
        sextuples_of_unsigneds::<T>,
        |&(x_2, x_1, x_0, y_2, y_1, y_0)| {
            let (z_2, z_1, z_0) = T::xxx_add_yyy_is_zzz(x_2, x_1, x_0, y_2, y_1, y_0);

            assert_eq!(
                T::xxx_sub_yyy_is_zzz(z_2, z_1, z_0, y_2, y_1, y_0),
                (x_2, x_1, x_0)
            );
            assert_eq!(
                T::xxx_sub_yyy_is_zzz(z_2, z_1, z_0, x_2, x_1, x_0),
                (y_2, y_1, y_0)
            );
            assert_eq!(
                T::xxx_add_yyy_is_zzz(y_2, y_1, y_0, x_2, x_1, x_0),
                (z_2, z_1, z_0)
            );

            let (neg_y_2, neg_y_1, neg_y_0) =
                T::xxx_sub_yyy_is_zzz(T::ZERO, T::ZERO, T::ZERO, y_2, y_1, y_0);
            assert_eq!(
                T::xxx_sub_yyy_is_zzz(x_2, x_1, x_0, neg_y_2, neg_y_1, neg_y_0),
                (z_2, z_1, z_0)
            );
        },
    );

    test_properties(triples_of_unsigneds::<T>, |&(x_2, x_1, x_0)| {
        assert_eq!(
            T::xxx_add_yyy_is_zzz(x_2, x_1, x_0, T::ZERO, T::ZERO, T::ZERO),
            (x_2, x_1, x_0)
        );
        assert_eq!(
            T::xxx_add_yyy_is_zzz(T::ZERO, T::ZERO, T::ZERO, x_2, x_1, x_0),
            (x_2, x_1, x_0)
        );

        let (neg_x_2, neg_x_1, neg_x_0) =
            T::xxx_sub_yyy_is_zzz(T::ZERO, T::ZERO, T::ZERO, x_2, x_1, x_0);
        assert_eq!(
            T::xxx_add_yyy_is_zzz(x_2, x_1, x_0, neg_x_2, neg_x_1, neg_x_0),
            (T::ZERO, T::ZERO, T::ZERO)
        );
    });

    test_properties(
        nonuples_of_unsigneds::<T>,
        |&(x_2, x_1, x_0, y_2, y_1, y_0, z_2, z_1, z_0)| {
            let (sum_1_2, sum_1_1, sum_1_0) = T::xxx_add_yyy_is_zzz(x_2, x_1, x_0, y_2, y_1, y_0);
            let (sum_2_2, sum_2_1, sum_2_0) = T::xxx_add_yyy_is_zzz(y_2, y_1, y_0, z_2, z_1, z_0);
            assert_eq!(
                T::xxx_add_yyy_is_zzz(sum_1_2, sum_1_1, sum_1_0, z_2, z_1, z_0),
                T::xxx_add_yyy_is_zzz(x_2, x_1, x_0, sum_2_2, sum_2_1, sum_2_0)
            );
        },
    );
}

#[test]
fn xxx_add_yyy_is_zzz_properties() {
    xxx_add_yyy_is_zzz_properties_helper::<u8>();
    xxx_add_yyy_is_zzz_properties_helper::<u16>();
    xxx_add_yyy_is_zzz_properties_helper::<u32>();
    xxx_add_yyy_is_zzz_properties_helper::<u64>();
    xxx_add_yyy_is_zzz_properties_helper::<usize>();
}
