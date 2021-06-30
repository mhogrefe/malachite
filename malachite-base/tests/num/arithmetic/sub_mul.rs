use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{
    signed_pair_gen, signed_triple_gen_var_2, unsigned_pair_gen_var_27, unsigned_triple_gen_var_2,
};

#[test]
fn test_sub_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, out: T) {
        assert_eq!(x.sub_mul(y, z), out);

        let mut x = x;
        x.sub_mul_assign(y, z);
        assert_eq!(x, out);
    }
    test::<u8>(100, 3, 7, 79);
    test::<u32>(60, 5, 10, 10);
    test::<u64>(1000000, 456, 789, 640216);
    test::<i32>(123, -456, 789, 359907);
    test::<i128>(-123, 456, 789, -359907);
    test::<i8>(127, 2, 100, -73);
    test::<i8>(-127, -2, 100, 73);
    test::<i8>(-128, 1, 0, -128);
}

fn sub_mul_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_2::<T>().test_properties(|(x, y, z)| {
        let result = x.sub_mul(y, z);

        let mut x_alt = x;
        x_alt.sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.sub_mul(z, y), result);
        assert_eq!(result.add_mul(y, z), x);
        assert_eq!(x.checked_sub_mul(y, z), Some(result));
        assert_eq!(x.saturating_sub_mul(y, z), result);
        assert_eq!(x.wrapping_sub_mul(y, z), result);
        assert_eq!(x.overflowing_sub_mul(y, z), (result, false));
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(a, b)| {
        assert_eq!(a.sub_mul(T::ZERO, b), a);
        assert_eq!(a.sub_mul(b, T::ZERO), a);
    });
}

fn sub_mul_properties_helper_signed<T: PrimitiveSigned>() {
    signed_triple_gen_var_2::<T>().test_properties(|(x, y, z)| {
        let result = x.sub_mul(y, z);

        let mut x_alt = x;
        x_alt.sub_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.sub_mul(z, y), result);
        assert_eq!(result.add_mul(y, z), x);
        assert_eq!(x.checked_sub_mul(y, z), Some(result));
        assert_eq!(x.saturating_sub_mul(y, z), result);
        assert_eq!(x.wrapping_sub_mul(y, z), result);
        assert_eq!(x.overflowing_sub_mul(y, z), (result, false));
    });

    signed_pair_gen::<T>().test_properties(|(a, b)| {
        assert_eq!(a.sub_mul(T::ZERO, b), a);
        assert_eq!(a.sub_mul(b, T::ZERO), a);
    });
}

#[test]
fn sub_mul_properties() {
    apply_fn_to_unsigneds!(sub_mul_properties_helper_unsigned);
    apply_fn_to_signeds!(sub_mul_properties_helper_signed);
}
