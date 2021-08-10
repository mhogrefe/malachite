use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};

#[test]
fn test_wrapping_sub() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: T) {
        assert_eq!(x.wrapping_sub(y), out);

        let mut x = x;
        x.wrapping_sub_assign(y);
        assert_eq!(x, out);
    }
    test::<u16>(456, 123, 333);
    test::<u8>(123, 200, 179);
    test::<i16>(123, -456, 579);
    test::<i8>(123, -45, -88);
    test::<i8>(-123, 45, 88);
}

fn wrapping_sub_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let mut diff = x;
        diff.wrapping_sub_assign(y);
        assert_eq!(diff, x.wrapping_sub(y));
        assert_eq!(diff, x.wrapping_add(y.wrapping_neg()));
        assert_eq!(diff.wrapping_add(y), x);
        assert_eq!(y.wrapping_sub(x), diff.wrapping_neg());
    });
}

fn wrapping_sub_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let mut diff = x;
        diff.wrapping_sub_assign(y);
        assert_eq!(diff, x.wrapping_sub(y));
        assert_eq!(diff, x.wrapping_add(y.wrapping_neg()));
        assert_eq!(diff.wrapping_add(y), x);
        assert_eq!(y.wrapping_sub(x), diff.wrapping_neg());
    });
}

#[test]
fn wrapping_sub_properties() {
    apply_fn_to_unsigneds!(wrapping_sub_properties_helper_unsigned);
    apply_fn_to_signeds!(wrapping_sub_properties_helper_signed);
}
