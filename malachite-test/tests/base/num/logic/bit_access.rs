use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_signed_and_u64_width_range,
    pairs_of_signed_and_u64_width_range_var_1, pairs_of_signed_and_u64_width_range_var_2,
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_and_u64_width_range,
    triples_of_signed_unsigned_width_range_and_bool_var_1,
    triples_of_unsigned_unsigned_width_range_and_bool_var_1, unsigneds,
};

fn get_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        pairs_of_unsigned_and_small_unsigned::<T, u64>,
        |&(n, index)| {
            let bit = n.get_bit(index);
            if index >= T::WIDTH {
                assert!(!bit);
            } else {
                assert_eq!(bit, !(!n).get_bit(index));
            }
        },
    );

    test_properties(unsigneds, |&n: &T| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if n != T::ZERO {
            assert!(n.get_bit(significant_bits - 1));
        }
    });
}

fn get_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        pairs_of_signed_and_small_unsigned,
        |&(n, index): &(T, u64)| {
            let bit = n.get_bit(index);
            if index >= T::WIDTH {
                assert_eq!(bit, n < T::ZERO);
            } else {
                assert_eq!(bit, !(!n).get_bit(index));
            }
        },
    );
}

#[test]
fn get_bit_properties() {
    get_bit_properties_helper_unsigned::<u8>();
    get_bit_properties_helper_unsigned::<u16>();
    get_bit_properties_helper_unsigned::<u32>();
    get_bit_properties_helper_unsigned::<u64>();
    get_bit_properties_helper_unsigned::<usize>();
    get_bit_properties_helper_signed::<i8>();
    get_bit_properties_helper_signed::<i16>();
    get_bit_properties_helper_signed::<i32>();
    get_bit_properties_helper_signed::<i64>();
    get_bit_properties_helper_signed::<isize>();
}

fn set_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigned_and_u64_width_range, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.set_bit(index);

        let mut mut_n_2 = n;
        mut_n_2.assign_bit(index, true);
        assert_eq!(mut_n_2, mut_n);

        assert_ne!(mut_n, T::ZERO);
        assert!(mut_n >= n);
        if n.get_bit(index) {
            assert_eq!(mut_n, n);
        } else {
            assert_ne!(mut_n, n);
            mut_n.clear_bit(index);
            assert_eq!(mut_n, n);
        }
    });
}

fn set_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_u64_width_range_var_1, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.set_bit(index);

        let mut mut_n_2 = n;
        mut_n_2.assign_bit(index, true);
        assert_eq!(mut_n_2, mut_n);

        assert_ne!(mut_n, T::ZERO);
        if n >= T::ZERO && index == T::WIDTH - 1 {
            assert!(mut_n < T::ZERO);
        } else {
            assert!(mut_n >= n);
        }
        if n.get_bit(index) {
            assert_eq!(mut_n, n);
        } else {
            assert_ne!(mut_n, n);
            mut_n.clear_bit(index);
            assert_eq!(mut_n, n);
        }

        let mut m = !n;
        m.clear_bit(index);
        m.not_assign();
        let mut mut_n = n;
        mut_n.set_bit(index);
        assert_eq!(m, mut_n);
    });
}

#[test]
fn set_bit_properties() {
    set_bit_properties_helper_unsigned::<u8>();
    set_bit_properties_helper_unsigned::<u16>();
    set_bit_properties_helper_unsigned::<u32>();
    set_bit_properties_helper_unsigned::<u64>();
    set_bit_properties_helper_unsigned::<usize>();
    set_bit_properties_helper_signed::<i8>();
    set_bit_properties_helper_signed::<i16>();
    set_bit_properties_helper_signed::<i32>();
    set_bit_properties_helper_signed::<i64>();
    set_bit_properties_helper_signed::<isize>();
}

fn clear_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigned_and_small_unsigned, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.clear_bit(index);

        let mut mut_n_2 = n;
        mut_n_2.assign_bit(index, false);
        assert_eq!(mut_n_2, mut_n);

        assert!(mut_n <= n);
        if n.get_bit(index) {
            assert_ne!(mut_n, n);
            mut_n.set_bit(index);
            assert_eq!(mut_n, n);
        } else {
            assert_eq!(mut_n, n);
        }
    });
}

fn clear_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_u64_width_range_var_2, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.clear_bit(index);

        let mut mut_n_2 = n;
        mut_n_2.assign_bit(index, false);
        assert_eq!(mut_n_2, mut_n);

        if n < T::ZERO && index == T::WIDTH - 1 {
            assert!(mut_n >= T::ZERO);
        } else {
            assert!(mut_n <= n);
        }
        if n.get_bit(index) {
            assert_ne!(mut_n, n);
            mut_n.set_bit(index);
            assert_eq!(mut_n, n);
        } else {
            assert_eq!(mut_n, n);
        }

        let mut m = !n;
        m.set_bit(index);
        m.not_assign();
        let mut mut_n = n;
        mut_n.clear_bit(index);
        assert_eq!(m, mut_n);
    });
}

#[test]
fn clear_bit_properties() {
    clear_bit_properties_helper_unsigned::<u8>();
    clear_bit_properties_helper_unsigned::<u16>();
    clear_bit_properties_helper_unsigned::<u32>();
    clear_bit_properties_helper_unsigned::<u64>();
    clear_bit_properties_helper_unsigned::<usize>();
    clear_bit_properties_helper_signed::<i8>();
    clear_bit_properties_helper_signed::<i16>();
    clear_bit_properties_helper_signed::<i32>();
    clear_bit_properties_helper_signed::<i64>();
    clear_bit_properties_helper_signed::<isize>();
}

fn assign_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        triples_of_unsigned_unsigned_width_range_and_bool_var_1,
        |&(n, index, bit)| {
            let mut mut_n: T = n;
            mut_n.assign_bit(index, bit);
        },
    );
}

fn assign_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_signed_unsigned_width_range_and_bool_var_1,
        |&(n, index, bit)| {
            let mut mut_n: T = n;
            mut_n.assign_bit(index, bit);
        },
    );
}

#[test]
fn assign_bit_properties() {
    assign_bit_properties_helper_unsigned::<u8>();
    assign_bit_properties_helper_unsigned::<u16>();
    assign_bit_properties_helper_unsigned::<u32>();
    assign_bit_properties_helper_unsigned::<u64>();
    assign_bit_properties_helper_unsigned::<usize>();
    assign_bit_properties_helper_signed::<i8>();
    assign_bit_properties_helper_signed::<i16>();
    assign_bit_properties_helper_signed::<i32>();
    assign_bit_properties_helper_signed::<i64>();
    assign_bit_properties_helper_signed::<isize>();
}

fn flip_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigned_and_u64_width_range, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.flip_bit(index);
        assert_ne!(mut_n, n);

        mut_n.flip_bit(index);
        assert_eq!(mut_n, n);
    });
}

fn flip_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_u64_width_range, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.flip_bit(index);
        assert_ne!(mut_n, n);

        mut_n.flip_bit(index);
        assert_eq!(mut_n, n);
    });
}

#[test]
fn flip_bit_properties() {
    flip_bit_properties_helper_unsigned::<u8>();
    flip_bit_properties_helper_unsigned::<u16>();
    flip_bit_properties_helper_unsigned::<u32>();
    flip_bit_properties_helper_unsigned::<u64>();
    flip_bit_properties_helper_unsigned::<usize>();
    flip_bit_properties_helper_signed::<i8>();
    flip_bit_properties_helper_signed::<i16>();
    flip_bit_properties_helper_signed::<i32>();
    flip_bit_properties_helper_signed::<i64>();
    flip_bit_properties_helper_signed::<isize>();
}
