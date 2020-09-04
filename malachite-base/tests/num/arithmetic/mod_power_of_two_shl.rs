use malachite_base::num::arithmetic::traits::{ModPowerOfTwoShl, ModPowerOfTwoShlAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_mod_power_of_two_shl() {
    fn test<T: PrimitiveUnsigned, U: PrimitiveInt>(t: T, u: U, pow: u64, out: T)
    where
        T: ModPowerOfTwoShl<U, Output = T> + ModPowerOfTwoShlAssign<U>,
    {
        assert_eq!(t.mod_power_of_two_shl(u, pow), out);

        let mut t = t;
        t.mod_power_of_two_shl_assign(u, pow);
        assert_eq!(t, out);
    };
    test::<u64, u8>(0, 0, 0, 0);
    test::<u64, u8>(0, 0, 5, 0);
    test::<u32, i16>(12, 2, 5, 16);
    test::<u16, u32>(10, 100, 4, 0);
    test::<u8, i64>(10, -2, 4, 2);
    test::<u8, i64>(10, -100, 4, 0);
    test::<u128, i8>(10, -100, 4, 0);
}
