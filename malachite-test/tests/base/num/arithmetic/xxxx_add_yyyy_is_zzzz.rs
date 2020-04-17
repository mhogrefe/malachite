use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    duodecuples_of_unsigneds, octuples_of_unsigneds, quadruples_of_unsigneds,
};

fn _xxxx_sub_yyyy_is_zzzz<T: PrimitiveUnsigned>(
    x_3: T,
    x_2: T,
    x_1: T,
    x_0: T,
    y_3: T,
    y_2: T,
    y_1: T,
    y_0: T,
) -> (T, T, T, T) {
    let (z_0, borrow_1) = x_0.overflowing_sub(y_0);
    let (mut z_1, mut borrow_2) = x_1.overflowing_sub(y_1);
    if borrow_1 {
        borrow_2 |= z_1.overflowing_sub_assign(T::ONE);
    }
    let (mut z_2, mut borrow_3) = x_2.overflowing_sub(y_2);
    if borrow_2 {
        borrow_3 |= z_2.overflowing_sub_assign(T::ONE);
    }
    let mut z_3 = x_3.wrapping_sub(y_3);
    if borrow_3 {
        z_3.wrapping_sub_assign(T::ONE);
    }
    (z_3, z_2, z_1, z_0)
}

fn xxxx_add_yyyy_is_zzzz_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(
        octuples_of_unsigneds::<T>,
        |&(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0)| {
            let (z_3, z_2, z_1, z_0) =
                T::xxxx_add_yyyy_is_zzzz(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0);

            assert_eq!(
                _xxxx_sub_yyyy_is_zzzz(z_3, z_2, z_1, z_0, y_3, y_2, y_1, y_0),
                (x_3, x_2, x_1, x_0)
            );
            assert_eq!(
                _xxxx_sub_yyyy_is_zzzz(z_3, z_2, z_1, z_0, x_3, x_2, x_1, x_0),
                (y_3, y_2, y_1, y_0)
            );
            assert_eq!(
                T::xxxx_add_yyyy_is_zzzz(y_3, y_2, y_1, y_0, x_3, x_2, x_1, x_0),
                (z_3, z_2, z_1, z_0)
            );

            let (neg_y_3, neg_y_2, neg_y_1, neg_y_0) =
                _xxxx_sub_yyyy_is_zzzz(T::ZERO, T::ZERO, T::ZERO, T::ZERO, y_3, y_2, y_1, y_0);
            assert_eq!(
                _xxxx_sub_yyyy_is_zzzz(x_3, x_2, x_1, x_0, neg_y_3, neg_y_2, neg_y_1, neg_y_0),
                (z_3, z_2, z_1, z_0)
            );
        },
    );

    test_properties(quadruples_of_unsigneds::<T>, |&(x_3, x_2, x_1, x_0)| {
        assert_eq!(
            T::xxxx_add_yyyy_is_zzzz(x_3, x_2, x_1, x_0, T::ZERO, T::ZERO, T::ZERO, T::ZERO),
            (x_3, x_2, x_1, x_0)
        );
        assert_eq!(
            T::xxxx_add_yyyy_is_zzzz(T::ZERO, T::ZERO, T::ZERO, T::ZERO, x_3, x_2, x_1, x_0),
            (x_3, x_2, x_1, x_0)
        );

        let (neg_x_3, neg_x_2, neg_x_1, neg_x_0) =
            _xxxx_sub_yyyy_is_zzzz(T::ZERO, T::ZERO, T::ZERO, T::ZERO, x_3, x_2, x_1, x_0);
        assert_eq!(
            T::xxxx_add_yyyy_is_zzzz(x_3, x_2, x_1, x_0, neg_x_3, neg_x_2, neg_x_1, neg_x_0),
            (T::ZERO, T::ZERO, T::ZERO, T::ZERO)
        );
    });

    test_properties(
        duodecuples_of_unsigneds::<T>,
        |&(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0, z_3, z_2, z_1, z_0)| {
            let (sum_1_3, sum_1_2, sum_1_1, sum_1_0) =
                T::xxxx_add_yyyy_is_zzzz(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0);
            let (sum_2_3, sum_2_2, sum_2_1, sum_2_0) =
                T::xxxx_add_yyyy_is_zzzz(y_3, y_2, y_1, y_0, z_3, z_2, z_1, z_0);
            assert_eq!(
                T::xxxx_add_yyyy_is_zzzz(sum_1_3, sum_1_2, sum_1_1, sum_1_0, z_3, z_2, z_1, z_0),
                T::xxxx_add_yyyy_is_zzzz(x_3, x_2, x_1, x_0, sum_2_3, sum_2_2, sum_2_1, sum_2_0)
            );
        },
    );
}

#[test]
fn xxxx_add_yyyy_is_zzzz_properties() {
    xxxx_add_yyyy_is_zzzz_properties_helper::<u8>();
    xxxx_add_yyyy_is_zzzz_properties_helper::<u16>();
    xxxx_add_yyyy_is_zzzz_properties_helper::<u32>();
    xxxx_add_yyyy_is_zzzz_properties_helper::<u64>();
    xxxx_add_yyyy_is_zzzz_properties_helper::<usize>();
}
