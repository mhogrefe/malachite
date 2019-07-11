use common::test_properties;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{Assign, WrappingFrom};
use malachite_test::inputs::base::{
    pairs_of_signed_and_signed, pairs_of_signed_and_unsigned, pairs_of_unsigned_and_unsigned,
};
use rand::Rand;

fn assign_properties_helper_unsigned_unsigned<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>()
where
    T: Assign<U> + From<U>,
{
    test_properties(pairs_of_unsigned_and_unsigned::<T, U>, |&(mut t, u)| {
        t.assign(u);
        assert_eq!(t, T::from(u));
    });
}

fn assign_properties_helper_signed_signed<T: PrimitiveSigned + Rand, U: PrimitiveSigned + Rand>()
where
    T: Assign<U> + From<U>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_signed::<T, U>, |&(mut t, u)| {
        t.assign(u);
        assert_eq!(t, T::from(u));
    });
}

fn assign_properties_helper_signed_unsigned<
    T: PrimitiveSigned + Rand,
    U: PrimitiveUnsigned + Rand,
>()
where
    T: Assign<U> + From<U>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_unsigned::<T, U>, |&(mut t, u)| {
        t.assign(u);
        assert_eq!(t, T::from(u));
    });
}

#[test]
fn assign_properties() {
    assign_properties_helper_unsigned_unsigned::<u8, u8>();
    assign_properties_helper_unsigned_unsigned::<u16, u8>();
    assign_properties_helper_unsigned_unsigned::<u16, u16>();
    assign_properties_helper_unsigned_unsigned::<u32, u8>();
    assign_properties_helper_unsigned_unsigned::<u32, u16>();
    assign_properties_helper_unsigned_unsigned::<u32, u32>();
    assign_properties_helper_unsigned_unsigned::<u64, u8>();
    assign_properties_helper_unsigned_unsigned::<u64, u16>();
    assign_properties_helper_unsigned_unsigned::<u64, u32>();
    assign_properties_helper_unsigned_unsigned::<u64, u64>();
    assign_properties_helper_unsigned_unsigned::<usize, u8>();
    assign_properties_helper_unsigned_unsigned::<usize, u16>();

    assign_properties_helper_signed_signed::<i8, i8>();
    assign_properties_helper_signed_signed::<i16, i8>();
    assign_properties_helper_signed_signed::<i16, i16>();
    assign_properties_helper_signed_signed::<i32, i8>();
    assign_properties_helper_signed_signed::<i32, i16>();
    assign_properties_helper_signed_signed::<i32, i32>();
    assign_properties_helper_signed_signed::<i64, i8>();
    assign_properties_helper_signed_signed::<i64, i16>();
    assign_properties_helper_signed_signed::<i64, i32>();
    assign_properties_helper_signed_signed::<i64, i64>();
    assign_properties_helper_signed_signed::<isize, i8>();
    assign_properties_helper_signed_signed::<isize, i16>();

    assign_properties_helper_signed_unsigned::<i16, u8>();
    assign_properties_helper_signed_unsigned::<i32, u8>();
    assign_properties_helper_signed_unsigned::<i32, u16>();
    assign_properties_helper_signed_unsigned::<i64, u8>();
    assign_properties_helper_signed_unsigned::<i64, u16>();
    assign_properties_helper_signed_unsigned::<i64, u32>();
    assign_properties_helper_signed_unsigned::<isize, u8>();
}
