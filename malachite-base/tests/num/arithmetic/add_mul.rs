use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{
    signed_pair_gen, signed_triple_gen_var_1, unsigned_pair_gen, unsigned_triple_gen_var_1,
};

#[test]
fn test_add_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, out: T) {
        assert_eq!(x.add_mul(y, z), out);

        let mut x = x;
        x.add_mul_assign(y, z);
        assert_eq!(x, out);
    };
    test::<u8>(2, 3, 7, 23);
    test::<u32>(7, 5, 10, 57);
    test::<u64>(123, 456, 789, 359907);
    test::<i32>(123, -456, 789, -359661);
    test::<i128>(-123, 456, 789, 359661);
    test::<i8>(127, -2, 100, -73);
    test::<i8>(-127, 2, 100, 73);
    test::<i8>(-128, 1, 0, -128);
}

fn add_mul_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_1::<T>().test_properties(|(x, y, z)| {
        let result = x.add_mul(y, z);

        let mut x_alt = x;
        x_alt.add_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.add_mul(z, y), result);
        assert_eq!(result.sub_mul(y, z), x);
        assert_eq!(x.checked_add_mul(y, z), Some(result));
        assert_eq!(x.saturating_add_mul(y, z), result);
        assert_eq!(x.wrapping_add_mul(y, z), result);
        assert_eq!(x.overflowing_add_mul(y, z), (result, false));
    });

    unsigned_pair_gen::<T>().test_properties(|(a, b)| {
        assert_eq!(a.add_mul(T::ZERO, b), a);
        assert_eq!(a.add_mul(b, T::ZERO), a);
    });
}

fn add_mul_properties_helper_signed<T: PrimitiveSigned>() {
    signed_triple_gen_var_1::<T>().test_properties(|(x, y, z)| {
        let result = x.add_mul(y, z);

        let mut x_alt = x;
        x_alt.add_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.add_mul(z, y), result);
        assert_eq!(result.sub_mul(y, z), x);
        assert_eq!(x.checked_add_mul(y, z), Some(result));
        assert_eq!(x.saturating_add_mul(y, z), result);
        assert_eq!(x.wrapping_add_mul(y, z), result);
        assert_eq!(x.overflowing_add_mul(y, z), (result, false));
    });

    signed_pair_gen::<T>().test_properties(|(a, b)| {
        assert_eq!(a.add_mul(T::ZERO, b), a);
        assert_eq!(a.add_mul(b, T::ZERO), a);
    });
}

#[test]
fn add_mul_properties() {
    apply_fn_to_unsigneds!(add_mul_properties_helper_unsigned);
    apply_fn_to_signeds!(add_mul_properties_helper_signed);
}
