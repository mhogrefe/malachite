use malachite_base::comparison::Max;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::integers::_get_bits_naive;
use malachite_base::num::logic::traits::BitBlockAccess;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_and_small_unsigned,
    pairs_of_unsigneds_var_3, small_unsigneds,
    triples_of_signed_small_unsigned_and_small_unsigned_var_1,
    triples_of_unsigned_small_unsigned_and_small_unsigned_var_1,
};

fn get_bits_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>()
where
    T: ExactFrom<<T as BitBlockAccess>::Output>,
{
    let width = u64::from(T::WIDTH);

    test_properties(
        triples_of_unsigned_small_unsigned_and_small_unsigned_var_1::<T, u64>,
        |&(n, start, end)| {
            let bits = T::exact_from(n.get_bits(start, end));
            assert!(bits <= n);
            assert_eq!(_get_bits_naive::<T, T>(n, start, end), bits);
            assert_eq!(
                T::exact_from(n.get_bits(start + width, end + width)),
                T::ZERO
            );
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<T, u64>,
        |&(n, start)| {
            assert_eq!(T::exact_from(n.get_bits(start, start)), T::ZERO);
        },
    );

    test_properties(pairs_of_unsigneds_var_3, |&(start, end)| {
        assert_eq!(T::exact_from(T::ZERO.get_bits(start, end)), T::ZERO);
    });
}

fn get_bits_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand + ExactFrom<<T as BitBlockAccess>::Output>,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    let width = u64::from(T::WIDTH);

    test_properties(
        triples_of_signed_small_unsigned_and_small_unsigned_var_1::<T, u64>,
        |&(n, start, end)| {
            let bits = T::UnsignedOfEqualWidth::exact_from(n.get_bits(start, end));
            assert_eq!(
                _get_bits_naive::<T, T::UnsignedOfEqualWidth>(n, start, end),
                bits
            );
        },
    );

    test_properties(
        pairs_of_signed_and_small_unsigned::<T, u64>,
        |&(n, start)| {
            assert_eq!(
                T::UnsignedOfEqualWidth::exact_from(n.get_bits(start, start)),
                T::UnsignedOfEqualWidth::ZERO
            );
            assert_eq!(
                T::UnsignedOfEqualWidth::exact_from(
                    n.get_bits(start + width, start + (width << 1))
                ),
                if n >= T::ZERO {
                    T::UnsignedOfEqualWidth::ZERO
                } else {
                    T::UnsignedOfEqualWidth::MAX
                }
            );
        },
    );

    test_properties(pairs_of_unsigneds_var_3, |&(start, end)| {
        assert_eq!(
            T::UnsignedOfEqualWidth::exact_from(T::ZERO.get_bits(start, end)),
            T::UnsignedOfEqualWidth::ZERO
        );
    });

    test_properties_no_special(small_unsigneds, |&start| {
        assert_eq!(
            T::UnsignedOfEqualWidth::exact_from(T::NEGATIVE_ONE.get_bits(start, start + width)),
            T::UnsignedOfEqualWidth::MAX
        );
    });
}

#[test]
fn get_bits_properties() {
    get_bits_properties_helper_unsigned::<u8>();
    get_bits_properties_helper_unsigned::<u16>();
    get_bits_properties_helper_unsigned::<u32>();
    get_bits_properties_helper_unsigned::<u64>();
    get_bits_properties_helper_unsigned::<usize>();
    get_bits_properties_helper_signed::<i8>();
    get_bits_properties_helper_signed::<i16>();
    get_bits_properties_helper_signed::<i32>();
    get_bits_properties_helper_signed::<i64>();
    get_bits_properties_helper_signed::<isize>();
}
