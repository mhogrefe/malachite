use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{
    signed_unsigned_pair_gen_var_14, unsigned_pair_gen_var_28,
};

#[test]
fn test_saturating_pow() {
    fn test<T: PrimitiveInt>(x: T, y: u64, out: T) {
        assert_eq!(x.saturating_pow(y), out);

        let mut x = x;
        x.saturating_pow_assign(y);
        assert_eq!(x, out);
    }
    test::<u8>(0, 0, 1);
    test::<u64>(123, 0, 1);
    test::<u64>(123, 1, 123);
    test::<u16>(0, 123, 0);
    test::<u16>(1, 123, 1);
    test::<i16>(-1, 123, -1);
    test::<i16>(-1, 124, 1);
    test::<u8>(3, 3, 27);
    test::<i32>(-10, 9, -1000000000);
    test::<i32>(-10, 10, i32::MAX);
    test::<i16>(-10, 9, i16::MIN);
    test::<i16>(10, 9, i16::MAX);
    test::<i64>(123, 456, i64::MAX);
}

fn saturating_pow_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_28::<T, u64>().test_properties(|(x, y)| {
        let mut power = x;
        power.saturating_pow_assign(y);
        assert_eq!(power, x.saturating_pow(y));
        if y != 0 {
            assert!(power >= x);
        }
        if power < T::MAX {
            assert_eq!(power, x.pow(y));
        }
    });
}

fn saturating_pow_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_14::<T, u64>().test_properties(|(x, y)| {
        let mut power = x;
        power.saturating_pow_assign(y);
        assert_eq!(power, x.saturating_pow(y));
        if power > T::MIN && power < T::MAX {
            assert_eq!(power, x.pow(y));
        }
    });
}

#[test]
fn saturating_pow_properties() {
    apply_fn_to_unsigneds!(saturating_pow_properties_helper_unsigned);
    apply_fn_to_signeds!(saturating_pow_properties_helper_signed);
}
