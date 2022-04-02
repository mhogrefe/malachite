use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};

#[test]
fn test_wrapping_mul() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.wrapping_mul(y), out);

        let mut x = x;
        x.wrapping_mul_assign(y);
        assert_eq!(x, out);
    }
    test::<u16>(123, 456, 56088);
    test::<u8>(123, 200, 24);
    test::<i16>(123, -45, -5535);
    test::<i8>(123, 45, -97);
    test::<i8>(-123, 45, 97);
}

fn wrapping_mul_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let mut product = x;
        product.wrapping_mul_assign(y);
        assert_eq!(product, x.wrapping_mul(y));
        assert_eq!(y.wrapping_mul(x), product);
    });
}

fn wrapping_mul_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let mut product = x;
        product.wrapping_mul_assign(y);
        assert_eq!(product, x.wrapping_mul(y));
        assert_eq!(y.wrapping_mul(x), product);
    });
}

#[test]
fn wrapping_mul_properties() {
    apply_fn_to_unsigneds!(wrapping_mul_properties_helper_unsigned);
    apply_fn_to_signeds!(wrapping_mul_properties_helper_signed);
}
