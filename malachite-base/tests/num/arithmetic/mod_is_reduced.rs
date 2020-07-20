use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_is_reduced_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, m, out| {
        assert_eq!(n.mod_is_reduced(&m), out);
    };

    test(T::ZERO, T::exact_from(5), true);
    test(T::exact_from(100), T::exact_from(100), false);
    test(T::exact_from(100), T::exact_from(101), true);
    test(T::MAX - T::ONE, T::MAX - T::ONE, false);
    test(T::MAX - T::ONE, T::MAX, true);
    test(T::MAX, T::MAX, false);
}

#[test]
fn test_mod_is_reduced() {
    apply_fn_to_unsigneds!(mod_is_reduced_helper);
}

macro_rules! mod_is_reduced_fail {
    ($t:ident, $mod_is_reduced_fail:ident) => {
        #[test]
        #[should_panic]
        fn $mod_is_reduced_fail() {
            $t::ZERO.mod_is_reduced(&$t::ZERO);
        }
    };
}

mod_is_reduced_fail!(u8, mod_is_reduced_u8_fail);
mod_is_reduced_fail!(u16, mod_is_reduced_u16_fail);
mod_is_reduced_fail!(u32, mod_is_reduced_u32_fail);
mod_is_reduced_fail!(u64, mod_is_reduced_u64_fail);
mod_is_reduced_fail!(u128, mod_is_reduced_u128_fail);
mod_is_reduced_fail!(usize, mod_is_reduced_usize_fail);
