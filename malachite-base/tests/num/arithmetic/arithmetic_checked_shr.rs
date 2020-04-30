use malachite_base::num::arithmetic::traits::ArithmeticCheckedShr;
use malachite_base::num::basic::integers::PrimitiveInteger;

#[test]
fn test_arithmetic_checked_shr() {
    fn test<T: PrimitiveInteger, U: PrimitiveInteger>(t: T, u: U, out: Option<T>)
    where
        T: ArithmeticCheckedShr<U, Output = T>,
    {
        assert_eq!(t.arithmetic_checked_shr(u), out);
    };
    test::<u32, i8>(100, 3, Some(12));
    test::<u32, i16>(100, 100, Some(0));

    test::<i8, i8>(3, -5, Some(96));
    test::<i8, i16>(3, -6, None);
    test::<i8, i32>(-3, -5, Some(-96));
    test::<i8, i64>(-3, -6, None);
    test::<i16, i128>(3, -100, None);
    test::<i16, isize>(-3, -100, None);
    test::<i32, i8>(0, -100, Some(0));
    test::<i32, i16>(100, 3, Some(12));
    test::<i32, i32>(-100, 3, Some(-13));
    test::<i64, i64>(100, 100, Some(0));
    test::<i64, i128>(-100, 100, Some(-1));
}
