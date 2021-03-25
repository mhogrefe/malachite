use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{
    signed_pair_gen, signed_triple_gen, unsigned_pair_gen, unsigned_triple_gen,
};

#[test]
fn test_checked_add_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, z: T, out: Option<T>) {
        assert_eq!(x.checked_add_mul(y, z), out);
    }
    test::<u8>(2, 3, 7, Some(23));
    test::<u32>(7, 5, 10, Some(57));
    test::<u64>(123, 456, 789, Some(359907));
    test::<i32>(123, -456, 789, Some(-359661));
    test::<i128>(-123, 456, 789, Some(359661));
    test::<i8>(127, -2, 100, Some(-73));
    test::<i8>(-127, 2, 100, Some(73));
    test::<i8>(-128, 1, 0, Some(-128));

    test::<u8>(2, 20, 20, None);
    test::<i8>(-127, -2, 100, None);
    test::<i8>(127, 1, 100, None);
    test::<i8>(-127, -1, 100, None);
    test::<i8>(-127, -10, 100, None);
}

fn checked_add_mul_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_triple_gen::<T>().test_properties(|(x, y, z)| {
        let result = x.checked_add_mul(y, z);
        assert_eq!(x.checked_add_mul(z, y), result);
        assert_eq!(result.is_none(), x.overflowing_add_mul(y, z).1);
    });

    unsigned_pair_gen::<T>().test_properties(|(a, b)| {
        assert_eq!(a.checked_add_mul(T::ZERO, b), Some(a));
        assert_eq!(a.checked_add_mul(T::ONE, b), a.checked_add(b));
        assert_eq!(T::ZERO.checked_add_mul(a, b), a.checked_mul(b));
        assert_eq!(a.checked_add_mul(b, T::ZERO), Some(a));
        assert_eq!(a.checked_add_mul(b, T::ONE), a.checked_add(b));
    });
}

fn checked_add_mul_properties_helper_signed<T: PrimitiveSigned>() {
    signed_triple_gen::<T>().test_properties(|(x, y, z)| {
        let result = x.checked_add_mul(y, z);
        assert_eq!(x.checked_add_mul(z, y), result);
        assert_eq!(result.is_none(), x.overflowing_add_mul(y, z).1);
    });

    signed_pair_gen::<T>().test_properties(|(a, b)| {
        assert_eq!(a.checked_add_mul(T::ZERO, b), Some(a));
        assert_eq!(a.checked_add_mul(T::ONE, b), a.checked_add(b));
        assert_eq!(T::ZERO.checked_add_mul(a, b), a.checked_mul(b));
        assert_eq!(a.checked_add_mul(b, T::ZERO), Some(a));
        assert_eq!(a.checked_add_mul(b, T::ONE), a.checked_add(b));
    });
}

#[test]
fn checked_add_mul_properties() {
    apply_fn_to_unsigneds!(checked_add_mul_properties_helper_unsigned);
    apply_fn_to_signeds!(checked_add_mul_properties_helper_signed);
}
