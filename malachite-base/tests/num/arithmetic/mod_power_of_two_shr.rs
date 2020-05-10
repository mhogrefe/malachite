use malachite_base::num::arithmetic::traits::{ModPowerOfTwoShr, ModPowerOfTwoShrAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_mod_power_of_two_shr() {
    fn test<T: PrimitiveUnsigned, U: PrimitiveSigned>(t: T, u: U, pow: u64, out: T)
    where
        T: ModPowerOfTwoShr<U, Output = T> + ModPowerOfTwoShrAssign<U>,
    {
        assert_eq!(t.mod_power_of_two_shr(u, pow), out);

        let mut t = t;
        t.mod_power_of_two_shr_assign(u, pow);
        assert_eq!(t, out);
    };
    test::<u64, i8>(0, 0, 0, 0);
    test::<u64, i8>(0, 0, 5, 0);
    test::<u32, i16>(12, -2, 5, 16);
    test::<u16, i32>(10, -100, 4, 0);
    test::<u8, i64>(10, 2, 4, 2);
    test::<u8, i64>(10, 100, 4, 0);
    test::<u128, i8>(10, 100, 4, 0);
}
