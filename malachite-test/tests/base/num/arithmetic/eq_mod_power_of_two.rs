use malachite_base::num::arithmetic::traits::ModPowerOfTwo;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_signeds, pairs_of_unsigned_and_small_unsigned,
    pairs_of_unsigneds, quadruples_of_three_signeds_and_small_unsigned,
    quadruples_of_three_unsigneds_and_small_unsigned, triples_of_signed_signed_and_small_unsigned,
    triples_of_unsigned_unsigned_and_small_unsigned,
};

fn unsigned_eq_mod_power_of_two_power_of_two_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        triples_of_unsigned_unsigned_and_small_unsigned::<T, u64>,
        |&(x, y, pow)| {
            let eq_mod_power_of_two = x.eq_mod_power_of_two(y, pow);
            assert_eq!(y.eq_mod_power_of_two(x, pow), eq_mod_power_of_two);
            assert_eq!(
                x.mod_power_of_two(pow) == y.mod_power_of_two(pow),
                eq_mod_power_of_two
            );
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<T, u64>,
        |&(n, pow)| {
            assert!(n.eq_mod_power_of_two(n, pow));
            assert_eq!(
                n.eq_mod_power_of_two(T::ZERO, pow),
                n.divisible_by_power_of_two(pow)
            );
            assert_eq!(
                T::ZERO.eq_mod_power_of_two(n, pow),
                n.divisible_by_power_of_two(pow)
            );
        },
    );

    test_properties(
        quadruples_of_three_unsigneds_and_small_unsigned::<T, u64>,
        |&(x, y, z, pow)| {
            if x.eq_mod_power_of_two(y, pow) && y.eq_mod_power_of_two(z, pow) {
                assert!(x.eq_mod_power_of_two(z, pow));
            }
        },
    );

    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        assert!(x.eq_mod_power_of_two(y, 0));
    });
}

fn signed_eq_mod_power_of_two_power_of_two_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand + WrappingFrom<T>,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_signed_signed_and_small_unsigned::<T, u64>,
        |&(x, y, pow)| {
            let eq_mod_power_of_two = x.eq_mod_power_of_two(y, pow);
            assert_eq!(y.eq_mod_power_of_two(x, pow), eq_mod_power_of_two);
            assert_eq!(
                T::UnsignedOfEqualWidth::wrapping_from(x).mod_power_of_two(pow)
                    == T::UnsignedOfEqualWidth::wrapping_from(y).mod_power_of_two(pow),
                eq_mod_power_of_two,
            );
        },
    );

    test_properties(pairs_of_signed_and_small_unsigned::<T, u64>, |&(n, pow)| {
        assert!(n.eq_mod_power_of_two(n, pow));
        assert_eq!(
            n.eq_mod_power_of_two(T::ZERO, pow),
            n.divisible_by_power_of_two(pow)
        );
        assert_eq!(
            T::ZERO.eq_mod_power_of_two(n, pow),
            n.divisible_by_power_of_two(pow)
        );
    });

    test_properties(
        quadruples_of_three_signeds_and_small_unsigned::<T, u64>,
        |&(x, y, z, pow)| {
            if x.eq_mod_power_of_two(y, pow) && y.eq_mod_power_of_two(z, pow) {
                assert!(x.eq_mod_power_of_two(z, pow));
            }
        },
    );

    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        assert!(x.eq_mod_power_of_two(y, 0));
    });
}

#[test]
fn eq_mod_power_of_two_power_of_two_properties() {
    unsigned_eq_mod_power_of_two_power_of_two_properties_helper::<u8>();
    unsigned_eq_mod_power_of_two_power_of_two_properties_helper::<u16>();
    unsigned_eq_mod_power_of_two_power_of_two_properties_helper::<u32>();
    unsigned_eq_mod_power_of_two_power_of_two_properties_helper::<u64>();
    unsigned_eq_mod_power_of_two_power_of_two_properties_helper::<usize>();

    signed_eq_mod_power_of_two_power_of_two_properties_helper::<i8>();
    signed_eq_mod_power_of_two_power_of_two_properties_helper::<i16>();
    signed_eq_mod_power_of_two_power_of_two_properties_helper::<i32>();
    signed_eq_mod_power_of_two_power_of_two_properties_helper::<i64>();
    signed_eq_mod_power_of_two_power_of_two_properties_helper::<isize>();
}
