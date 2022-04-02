use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShr, ModPowerOf2Shl, ModPowerOf2Shr, ModPowerOf2ShrAssign,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_12, unsigned_pair_gen_var_17,
    unsigned_signed_unsigned_triple_gen_var_1,
};

#[test]
fn test_mod_power_of_2_shr() {
    fn test<
        T: ModPowerOf2Shr<U, Output = T> + ModPowerOf2ShrAssign<U> + PrimitiveUnsigned,
        U: PrimitiveSigned,
    >(
        t: T,
        u: U,
        pow: u64,
        out: T,
    ) {
        assert_eq!(t.mod_power_of_2_shr(u, pow), out);

        let mut t = t;
        t.mod_power_of_2_shr_assign(u, pow);
        assert_eq!(t, out);
    }
    test::<u64, i8>(0, 0, 0, 0);
    test::<u64, i8>(0, 0, 5, 0);
    test::<u32, i16>(12, -2, 5, 16);
    test::<u16, i32>(10, -100, 4, 0);
    test::<u8, i64>(10, 2, 4, 2);
    test::<u8, i64>(10, 100, 4, 0);
    test::<u128, i8>(10, 100, 4, 0);
}

fn mod_power_of_2_shl_properties_helper<
    T: ArithmeticCheckedShr<U, Output = T>
        + ModPowerOf2Shl<U, Output = T>
        + ModPowerOf2Shr<U, Output = T>
        + ModPowerOf2ShrAssign<U>
        + PrimitiveUnsigned,
    U: PrimitiveSigned,
>() {
    unsigned_signed_unsigned_triple_gen_var_1::<T, U>().test_properties(|(n, i, pow)| {
        assert!(n.mod_power_of_2_is_reduced(pow));
        let shifted = n.mod_power_of_2_shr(i, pow);
        assert!(shifted.mod_power_of_2_is_reduced(pow));

        let mut shifted_alt = n;
        shifted_alt.mod_power_of_2_shr_assign(i, pow);
        assert_eq!(shifted_alt, shifted);

        if let Some(shifted_alt) = n.arithmetic_checked_shr(i) {
            assert_eq!(shifted_alt.mod_power_of_2(pow), shifted);
        }

        if i != U::MIN {
            assert_eq!(n.mod_power_of_2_shl(-i, pow), shifted);
        }
    });

    unsigned_pair_gen_var_17::<T>().test_properties(|(n, pow)| {
        assert_eq!(n.mod_power_of_2_shr(U::ZERO, pow), n);
    });

    signed_unsigned_pair_gen_var_12::<U, T>().test_properties(|(i, pow)| {
        assert_eq!(T::ZERO.mod_power_of_2_shr(i, pow), T::ZERO);
    });
}

#[test]
fn mod_power_of_2_shl_properties() {
    apply_fn_to_unsigneds_and_signeds!(mod_power_of_2_shl_properties_helper);
}
