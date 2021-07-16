use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{
    signed_unsigned_pair_gen_var_15, unsigned_pair_gen_var_29,
};

#[test]
fn test_pow() {
    fn test<T: PrimitiveInt>(x: T, y: u64, out: T) {
        assert_eq!(x.pow(y), out);

        let mut x = x;
        x.pow_assign(y);
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
    test::<i32>(-10, 8, 100000000);
}

fn pow_assign_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_29::<T>().test_properties(|(x, y)| {
        let mut power = x;
        power.pow_assign(y);
        assert_eq!(power, x.pow(y));
        if x > T::ONE {
            assert_eq!(power.checked_log_base(x), Some(y));
        }
    });
}

fn pow_assign_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_15::<T>().test_properties(|(x, y)| {
        let mut power = x;
        power.pow_assign(y);
        assert_eq!(power, x.pow(y));
    });
}

#[test]
fn pow_assign_properties() {
    apply_fn_to_unsigneds!(pow_assign_properties_helper_unsigned);
    apply_fn_to_signeds!(pow_assign_properties_helper_signed);
}
