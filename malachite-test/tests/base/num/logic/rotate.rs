use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::Rotate;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_signed_and_unsigned, pairs_of_unsigned_and_unsigned};

macro_rules! rotate_left_properties_helper_unsigned {
    ($t:ident) => {
        test_properties(pairs_of_unsigned_and_unsigned::<$t, u64>, |&(n, index)| {
            assert_eq!(
                Rotate::rotate_left(n, index),
                n.rotate_left(u32::wrapping_from(index))
            );
        });

        test_properties(pairs_of_unsigned_and_unsigned::<$t, u32>, |&(n, index)| {
            assert_eq!(
                Rotate::rotate_left(n, u64::from(index)),
                n.rotate_left(index)
            );
        });
    };
}

macro_rules! rotate_left_properties_helper_signed {
    ($t:ident) => {
        test_properties(pairs_of_signed_and_unsigned::<$t, u64>, |&(n, index)| {
            assert_eq!(
                Rotate::rotate_left(n, index),
                n.rotate_left(u32::wrapping_from(index))
            );
        });

        test_properties(pairs_of_signed_and_unsigned::<$t, u32>, |&(n, index)| {
            assert_eq!(
                Rotate::rotate_left(n, u64::from(index)),
                n.rotate_left(index)
            );
        });
    };
}

#[test]
fn rotate_left_properties() {
    rotate_left_properties_helper_unsigned!(u8);
    rotate_left_properties_helper_unsigned!(u16);
    rotate_left_properties_helper_unsigned!(u32);
    rotate_left_properties_helper_unsigned!(u64);
    rotate_left_properties_helper_unsigned!(usize);
    rotate_left_properties_helper_signed!(i8);
    rotate_left_properties_helper_signed!(i16);
    rotate_left_properties_helper_signed!(i32);
    rotate_left_properties_helper_signed!(i64);
    rotate_left_properties_helper_signed!(isize);
}

macro_rules! rotate_right_properties_helper_unsigned {
    ($t:ident) => {
        test_properties(pairs_of_unsigned_and_unsigned::<$t, u64>, |&(n, index)| {
            assert_eq!(
                Rotate::rotate_right(n, index),
                n.rotate_right(u32::wrapping_from(index))
            );
        });

        test_properties(pairs_of_unsigned_and_unsigned::<$t, u32>, |&(n, index)| {
            assert_eq!(
                Rotate::rotate_right(n, u64::from(index)),
                n.rotate_right(index)
            );
        });
    };
}

macro_rules! rotate_right_properties_helper_signed {
    ($t:ident) => {
        test_properties(pairs_of_signed_and_unsigned::<$t, u64>, |&(n, index)| {
            assert_eq!(
                Rotate::rotate_right(n, index),
                n.rotate_right(u32::wrapping_from(index))
            );
        });

        test_properties(pairs_of_signed_and_unsigned::<$t, u32>, |&(n, index)| {
            assert_eq!(
                Rotate::rotate_right(n, u64::from(index)),
                n.rotate_right(index)
            );
        });
    };
}

#[test]
fn rotate_right_properties() {
    rotate_right_properties_helper_unsigned!(u8);
    rotate_right_properties_helper_unsigned!(u16);
    rotate_right_properties_helper_unsigned!(u32);
    rotate_right_properties_helper_unsigned!(u64);
    rotate_right_properties_helper_unsigned!(usize);
    rotate_right_properties_helper_signed!(i8);
    rotate_right_properties_helper_signed!(i16);
    rotate_right_properties_helper_signed!(i32);
    rotate_right_properties_helper_signed!(i64);
    rotate_right_properties_helper_signed!(isize);
}
