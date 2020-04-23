use std::cmp::min;

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::bit_block_access::{_assign_bits_naive, _get_bits_naive};
use malachite_base::num::logic::traits::BitBlockAccess;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_and_small_unsigned,
    pairs_of_unsigneds_var_3, quadruples_of_signed_small_u64_small_u64_and_unsigned_var_1,
    quadruples_of_unsigned_small_u64_small_u64_and_unsigned_var_1, small_unsigneds,
    triples_of_signed_small_unsigned_and_small_unsigned_var_1,
    triples_of_signed_small_unsigned_and_unsigned,
    triples_of_unsigned_small_unsigned_and_small_unsigned_var_1,
    triples_of_unsigned_small_unsigned_and_unsigned,
};

fn get_bits_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>()
where
    T: ExactFrom<<T as BitBlockAccess>::Bits>,
    <T as BitBlockAccess>::Bits: ExactFrom<T>,
{
    let width = T::WIDTH;
    test_properties(
        triples_of_unsigned_small_unsigned_and_small_unsigned_var_1::<T, u64>,
        |&(n, start, end)| {
            let bits = T::exact_from(n.get_bits(start, end));
            assert_eq!(_get_bits_naive::<T, T>(&n, start, end), bits);
            assert!(bits <= n);
            assert_eq!(
                T::exact_from(n.get_bits(start + width, end + width)),
                T::ZERO
            );
            let mut n_alt = n;
            n_alt.assign_bits(start, end, &<T as BitBlockAccess>::Bits::exact_from(bits));
            assert_eq!(n_alt, n);
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

fn get_bits_properties_helper_signed<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned>()
where
    T: BitBlockAccess<Bits = U>,
    <T as PrimitiveSigned>::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    let width = T::WIDTH;
    test_properties(
        triples_of_signed_small_unsigned_and_small_unsigned_var_1::<T, u64>,
        |&(n, start, end)| {
            let bits = n.get_bits(start, end);
            assert_eq!(_get_bits_naive::<T, U>(&n, start, end), bits);
            let mut n_alt = n;
            n_alt.assign_bits(start, end, &bits);
            assert_eq!(n_alt, n);
        },
    );

    test_properties(
        pairs_of_signed_and_small_unsigned::<T, u64>,
        |&(n, start)| {
            assert_eq!(n.get_bits(start, start), U::ZERO);
            assert_eq!(
                n.get_bits(start + width, start + (width << 1)),
                if n >= T::ZERO { U::ZERO } else { U::MAX }
            );
        },
    );

    test_properties(pairs_of_unsigneds_var_3, |&(start, end)| {
        assert_eq!(T::ZERO.get_bits(start, end), U::ZERO);
    });

    test_properties_no_special(small_unsigneds, |&start| {
        assert_eq!(T::NEGATIVE_ONE.get_bits(start, start + width), U::MAX);
    });
}

#[test]
fn get_bits_properties() {
    get_bits_properties_helper_unsigned::<u8>();
    get_bits_properties_helper_unsigned::<u16>();
    get_bits_properties_helper_unsigned::<u32>();
    get_bits_properties_helper_unsigned::<u64>();
    get_bits_properties_helper_unsigned::<usize>();
    get_bits_properties_helper_signed::<i8, u8>();
    get_bits_properties_helper_signed::<i16, u16>();
    get_bits_properties_helper_signed::<i32, u32>();
    get_bits_properties_helper_signed::<i64, u64>();
    get_bits_properties_helper_signed::<isize, usize>();
}

fn assign_bits_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>()
where
    T: BitBlockAccess<Bits = T>,
{
    let width = T::WIDTH;
    test_properties(
        quadruples_of_unsigned_small_u64_small_u64_and_unsigned_var_1::<T, T>,
        |&(n, start, end, bits)| {
            let mut mut_n = n;
            mut_n.assign_bits(start, end, &bits);
            let mut mut_n_alt = mut_n;
            mut_n_alt.assign_bits(start, end, &bits);
            assert_eq!(mut_n_alt, mut_n);
            let mut mut_n_alt = n;
            _assign_bits_naive::<T, T>(&mut mut_n_alt, start, end, &bits);
            assert_eq!(mut_n_alt, mut_n);
            assert_eq!(
                mut_n.get_bits(start, end),
                bits.mod_power_of_two(end - start)
            );
        },
    );

    test_properties(
        triples_of_unsigned_small_unsigned_and_unsigned::<T, u64>,
        |&(n, start, bits)| {
            let mut mut_n = n;
            mut_n.assign_bits(start, start, &bits);
            assert_eq!(mut_n, n);
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<T, u64>,
        |&(n, start)| {
            let mut mut_n = n;
            mut_n.assign_bits(start + width, start + (width << 1), &T::ZERO);
            assert_eq!(mut_n, n);
        },
    );

    test_properties(pairs_of_unsigneds_var_3, |&(start, end)| {
        let mut n = T::ZERO;
        n.assign_bits(start, end, &T::ZERO);
        assert_eq!(n, T::ZERO);
    });
}

fn assign_bits_properties_helper_signed<T: PrimitiveSigned + Rand, U: PrimitiveUnsigned + Rand>()
where
    T: BitBlockAccess<Bits = U>,
    <T as PrimitiveSigned>::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as UnsignedAbs>::Output: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
{
    let width = T::WIDTH;
    test_properties(
        quadruples_of_signed_small_u64_small_u64_and_unsigned_var_1::<T, U>,
        |&(n, start, end, bits)| {
            let mut mut_n = n;
            mut_n.assign_bits(start, end, &bits);
            let mut mut_n_alt = mut_n;
            mut_n_alt.assign_bits(start, end, &bits);
            assert_eq!(mut_n_alt, mut_n);
            let mut mut_n_alt = n;
            _assign_bits_naive::<T, U>(&mut mut_n_alt, start, end, &bits);
            assert_eq!(mut_n_alt, mut_n);
            assert_eq!(
                mut_n.get_bits(start, end),
                bits.mod_power_of_two(end - start)
            );
            assert_eq!(mut_n >= T::ZERO, n >= T::ZERO);
        },
    );

    test_properties(
        triples_of_signed_small_unsigned_and_unsigned::<T, u64, U>,
        |&(n, start, bits)| {
            let mut mut_n = n;
            mut_n.assign_bits(start, start, &bits);
            assert_eq!(mut_n, n);
        },
    );

    test_properties(
        pairs_of_signed_and_small_unsigned::<T, u64>,
        |&(n, start)| {
            let mut mut_n = n;
            mut_n.assign_bits(
                start + width - 1,
                start + (width << 1) - 1,
                &(if n >= T::ZERO { U::ZERO } else { U::MAX }),
            );
            assert_eq!(mut_n, n);
        },
    );

    test_properties(pairs_of_unsigneds_var_3, |&(start, end)| {
        let mut n = T::ZERO;
        n.assign_bits(start, end, &U::ZERO);
        assert_eq!(n, T::ZERO);

        let mut n = T::NEGATIVE_ONE;
        n.assign_bits(start, min(end, start.saturating_add(width)), &U::MAX);
        assert_eq!(n, T::NEGATIVE_ONE);
    });
}

#[test]
fn assign_bits_properties() {
    assign_bits_properties_helper_unsigned::<u8>();
    assign_bits_properties_helper_unsigned::<u16>();
    assign_bits_properties_helper_unsigned::<u32>();
    assign_bits_properties_helper_unsigned::<u64>();
    assign_bits_properties_helper_unsigned::<usize>();
    assign_bits_properties_helper_signed::<i8, u8>();
    assign_bits_properties_helper_signed::<i16, u16>();
    assign_bits_properties_helper_signed::<i32, u32>();
    assign_bits_properties_helper_signed::<i64, u64>();
    assign_bits_properties_helper_signed::<isize, usize>();
}
