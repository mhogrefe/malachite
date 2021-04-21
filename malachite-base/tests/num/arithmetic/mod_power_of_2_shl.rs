use malachite_base::num::arithmetic::traits::{ModPowerOf2Shl, ModPowerOf2ShlAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_mod_power_of_2_shl() {
    fn test<
        T: ModPowerOf2Shl<U, Output = T> + ModPowerOf2ShlAssign<U> + PrimitiveUnsigned,
        U: PrimitiveInt,
    >(
        t: T,
        u: U,
        pow: u64,
        out: T,
    ) {
        assert_eq!(t.mod_power_of_2_shl(u, pow), out);

        let mut t = t;
        t.mod_power_of_2_shl_assign(u, pow);
        assert_eq!(t, out);
    }
    test::<u64, u8>(0, 0, 0, 0);
    test::<u64, u8>(0, 0, 5, 0);
    test::<u32, i16>(12, 2, 5, 16);
    test::<u16, u32>(10, 100, 4, 0);
    test::<u8, i64>(10, -2, 4, 2);
    test::<u8, i64>(10, -100, 4, 0);
    test::<u128, i8>(10, -100, 4, 0);
}
