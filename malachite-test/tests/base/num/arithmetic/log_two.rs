use std::{u16, u32, u64, u8};

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::positive_unsigneds;

fn floor_log_two_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(positive_unsigneds, |&n: &T| {
        let floor_log_two = n.floor_log_two();
        assert_eq!(floor_log_two, n.significant_bits() - 1);
        assert!(floor_log_two < u64::from(T::WIDTH));
        assert_eq!(floor_log_two == 0, n == T::ONE);
    });
}

#[test]
fn floor_log_two_properties() {
    floor_log_two_properties_helper::<u8>();
    floor_log_two_properties_helper::<u16>();
    floor_log_two_properties_helper::<u32>();
    floor_log_two_properties_helper::<u64>();
}

fn ceiling_log_two_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(positive_unsigneds, |&n: &T| {
        let ceiling_log_two = n.ceiling_log_two();
        assert!(ceiling_log_two <= u64::from(T::WIDTH));
        assert_eq!(ceiling_log_two == 0, n == T::ONE);
    });
}

#[test]
fn ceiling_log_two_properties() {
    ceiling_log_two_properties_helper::<u8>();
    ceiling_log_two_properties_helper::<u16>();
    ceiling_log_two_properties_helper::<u32>();
    ceiling_log_two_properties_helper::<u64>();
}
