use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::LowMask;

fn low_mask_primitive_helper<T: PrimitiveInteger>() {
    let test = |bits, out| {
        assert_eq!(T::low_mask(bits), out);
    };
    test(0, T::ZERO);
    test(1, T::ONE);
    test(2, T::exact_from(3));
    test(3, T::exact_from(7));
}

fn low_mask_unsigned_helper<T: PrimitiveUnsigned>() {
    let test = |bits, out| {
        assert_eq!(T::low_mask(bits), out);
    };
    test(T::WIDTH - 1, (T::ONE << (T::WIDTH - 1)) - T::ONE);
    test(T::WIDTH, T::MAX);
}

fn low_mask_signed_helper<T: PrimitiveSigned>() {
    let test = |bits, out| {
        assert_eq!(T::low_mask(bits), out);
    };
    test(T::WIDTH - 1, T::MAX);
    test(T::WIDTH, T::NEGATIVE_ONE);
}

#[test]
fn test_low_mask() {
    apply_fn_to_primitive_ints!(low_mask_primitive_helper);
    apply_fn_to_unsigneds!(low_mask_unsigned_helper);
    apply_fn_to_signeds!(low_mask_signed_helper);
}

macro_rules! low_mask_fail {
    ($t:ident, $low_mask_fail:ident) => {
        #[test]
        #[should_panic]
        fn $low_mask_fail() {
            $t::low_mask($t::WIDTH + 1);
        }
    };
}

low_mask_fail!(u8, low_mask_u8_fail);
low_mask_fail!(u16, low_mask_u16_fail);
low_mask_fail!(u32, low_mask_u32_fail);
low_mask_fail!(u64, low_mask_u64_fail);
low_mask_fail!(u128, low_mask_u128_fail);
low_mask_fail!(usize, low_mask_usize_fail);
low_mask_fail!(i8, low_mask_i8_fail);
low_mask_fail!(i16, low_mask_i16_fail);
low_mask_fail!(i32, low_mask_i32_fail);
low_mask_fail!(i64, low_mask_i64_fail);
low_mask_fail!(i128, low_mask_i128_fail);
low_mask_fail!(isize, low_mask_isize_fail);
