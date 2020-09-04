use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_arithmetic_checked_shl() {
    fn test<T: PrimitiveInt, U: PrimitiveInt>(t: T, u: U, out: Option<T>)
    where
        T: ArithmeticCheckedShl<U, Output = T>,
    {
        assert_eq!(t.arithmetic_checked_shl(u), out);
    };
    test::<u16, u8>(0, 0, Some(0));
    test::<u8, u16>(3, 6, Some(192));
    test::<u8, u32>(3, 7, None);
    test::<u64, u64>(3, 100, None);
    test::<u64, u128>(0, 100, Some(0));

    test::<u32, i8>(100, -3, Some(12));
    test::<u32, i16>(100, -100, Some(0));

    test::<i8, u8>(3, 5, Some(96));
    test::<i8, u16>(3, 6, None);
    test::<i8, u32>(-3, 5, Some(-96));
    test::<i8, u64>(-3, 6, None);
    test::<i16, u128>(3, 100, None);
    test::<i16, usize>(-3, 100, None);
    test::<i64, u8>(0, 100, Some(0));

    test::<i8, i8>(3, 5, Some(96));
    test::<i8, i16>(3, 6, None);
    test::<i8, i32>(-3, 5, Some(-96));
    test::<i8, i64>(-3, 6, None);
    test::<i16, i128>(3, 100, None);
    test::<i16, isize>(-3, 100, None);
    test::<i32, i8>(0, 100, Some(0));
    test::<i32, i16>(100, -3, Some(12));
    test::<i32, i32>(-100, -3, Some(-13));
    test::<i64, i64>(100, -100, Some(0));
    test::<i64, i128>(-100, -100, Some(-1));
}
