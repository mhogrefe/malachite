use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::CheckedFrom;

fn abs_helper<T: PrimitiveSigned>() {
    let test = |n: T, out| {
        assert_eq!(n.abs(), out);

        let mut n = n;
        n.abs_assign();
        assert_eq!(n, out);
    };
    test(T::ZERO, T::ZERO);
    test(T::ONE, T::ONE);
    test(T::exact_from(100), T::exact_from(100));
    test(T::NEGATIVE_ONE, T::ONE);
    test(T::exact_from(-100), T::exact_from(100));
}

#[test]
fn test_abs() {
    abs_helper::<i8>();
    abs_helper::<i16>();
    abs_helper::<i32>();
    abs_helper::<i64>();
    abs_helper::<i128>();
    abs_helper::<isize>();
}

fn test<T: PrimitiveSigned, U: PrimitiveUnsigned>(n: T, out: U)
where
    T: UnsignedAbs<Output = U>,
{
    assert_eq!(n.unsigned_abs(), out);
}

fn unsigned_abs_helper<T: PrimitiveSigned, U: PrimitiveUnsigned>()
where
    T: UnsignedAbs<Output = U>,
    U: CheckedFrom<T>,
{
    test(T::ZERO, U::ZERO);
    test(T::ONE, U::ONE);
    test(T::exact_from(100), U::exact_from(100));
    test(T::NEGATIVE_ONE, U::ONE);
    test(T::exact_from(-100), U::exact_from(100));
    test(T::MIN, U::exact_from(T::MAX) + U::ONE);
}

#[test]
fn test_unsigned_abs() {
    unsigned_abs_helper::<i8, u8>();
    unsigned_abs_helper::<i16, u16>();
    unsigned_abs_helper::<i32, u32>();
    unsigned_abs_helper::<i64, u64>();
    unsigned_abs_helper::<i128, u128>();
    unsigned_abs_helper::<isize, usize>();
}
