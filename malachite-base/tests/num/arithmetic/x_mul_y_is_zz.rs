use malachite_base::num::arithmetic::x_mul_y_is_zz::_explicit_x_mul_y_is_zz;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{
    unsigned_gen, unsigned_pair_gen_var_27, unsigned_triple_gen_var_19,
};

#[test]
fn test_x_mul_y_is_zz() {
    fn test<T: PrimitiveUnsigned>(x: T, y: T, z_1: T, z_0: T) {
        assert_eq!(T::x_mul_y_is_zz(x, y), (z_1, z_0));
        assert_eq!(_explicit_x_mul_y_is_zz(x, y), (z_1, z_0));
    }
    test::<u32>(0, 0, 0, 0);
    test::<u64>(15, 3, 0, 45);
    test::<u8>(0x78, 0x9a, 0x48, 0x30);
    test::<u8>(u8::MAX, 0, 0, 0);
    test::<u8>(u8::MAX, 1, 0, u8::MAX);
    test(u16::MAX, u16::MAX, u16::MAX - 1, 1);
}

fn x_mul_y_is_zz_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let (z_1, z_0) = T::x_mul_y_is_zz(x, y);
        assert_eq!(_explicit_x_mul_y_is_zz(x, y), (z_1, z_0));
        assert_eq!(T::x_mul_y_is_zz(y, x), (z_1, z_0));
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(T::x_mul_y_is_zz(x, T::ZERO), (T::ZERO, T::ZERO));
        assert_eq!(T::x_mul_y_is_zz(T::ZERO, x), (T::ZERO, T::ZERO));
        assert_eq!(T::x_mul_y_is_zz(x, T::ONE), (T::ZERO, x));
        assert_eq!(T::x_mul_y_is_zz(T::ONE, x), (T::ZERO, x));
    });

    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, z)| {
        let (_, product_1) = T::x_mul_y_is_zz(x, y);
        let (_, product_2) = T::x_mul_y_is_zz(y, z);
        assert_eq!(product_1.wrapping_mul(z), x.wrapping_mul(product_2));
    });
}

#[test]
fn x_mul_y_is_zz_properties() {
    apply_fn_to_unsigneds!(x_mul_y_is_zz_properties_helper);
}
