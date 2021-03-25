use malachite_base::num::arithmetic::traits::{ModShr, ModShrAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

#[test]
fn test_mod_shr() {
    fn test<
        T: ModShr<U, T, Output = T> + ModShrAssign<U, T> + PrimitiveUnsigned,
        U: PrimitiveInt,
    >(
        t: T,
        i: U,
        m: T,
        out: T,
    ) {
        assert_eq!(t.mod_shr(i, m), out);

        let mut t = t;
        t.mod_shr_assign(i, m);
        assert_eq!(t, out);
    }
    test::<u64, i8>(0, 0, 1, 0);
    test::<u64, i8>(0, 0, 5, 0);
    test::<u32, i16>(8, -2, 10, 2);
    test::<u16, i32>(10, -100, 17, 7);

    test::<u8, i64>(10, 2, 15, 2);
    test::<u8, i64>(10, 100, 19, 0);
    test::<u128, i8>(10, 100, 19, 0);
}
