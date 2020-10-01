use malachite_base::num::arithmetic::traits::{ModShl, ModShlAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_mod_shl() {
    fn test<
        T: ModShl<U, T, Output = T> + ModShlAssign<U, T> + PrimitiveUnsigned,
        U: PrimitiveInt,
    >(
        t: T,
        u: U,
        m: T,
        out: T,
    ) {
        assert_eq!(t.mod_shl(u, m), out);

        let mut t = t;
        t.mod_shl_assign(u, m);
        assert_eq!(t, out);
    };
    test::<u64, u8>(0, 0, 1, 0);
    test::<u64, u8>(0, 0, 5, 0);
    test::<u32, i16>(8, 2, 10, 2);
    test::<u16, u32>(10, 100, 17, 7);

    test::<u8, i64>(10, -2, 15, 2);
    test::<u8, i64>(10, -100, 19, 0);
    test::<u128, i8>(10, -100, 19, 0);
}
