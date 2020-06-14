use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    nonzero_signeds, pairs_of_signeds, pairs_of_signeds_var_3, pairs_of_signeds_var_4,
    pairs_of_unsigned_and_positive_unsigned_var_1, pairs_of_unsigneds, pairs_of_unsigneds_var_7,
    positive_unsigneds, signeds, unsigneds,
};

fn unsigned_divisible_by_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(x == T::ZERO || y != T::ZERO && x % y == T::ZERO, divisible);
    });

    test_properties(pairs_of_unsigneds_var_7::<T>, |&(x, y)| {
        assert!(x.divisible_by(y));
        assert!(x == T::ZERO || y != T::ZERO && x % y == T::ZERO);
    });

    test_properties(
        pairs_of_unsigned_and_positive_unsigned_var_1::<T>,
        |&(x, y)| {
            assert!(!x.divisible_by(y));
            assert!(x != T::ZERO && (y == T::ZERO || x % y != T::ZERO));
        },
    );

    test_properties(unsigneds::<T>, |&n| {
        assert!(n.divisible_by(T::ONE));
    });

    test_properties(positive_unsigneds::<T>, |&n| {
        assert!(!n.divisible_by(T::ZERO));
        assert!(T::ZERO.divisible_by(n));
        if n > T::ONE {
            assert!(!T::ONE.divisible_by(n));
        }
        assert!(n.divisible_by(n));
    });
}

fn signed_divisible_by_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(
            x == T::ZERO || x == T::MIN && y == T::NEGATIVE_ONE || y != T::ZERO && x % y == T::ZERO,
            divisible
        );
        if x != T::MIN {
            assert_eq!((-x).divisible_by(y), divisible);
        }
        if y != T::MIN {
            assert_eq!(x.divisible_by(-y), divisible);
        }
    });

    test_properties(pairs_of_signeds_var_4::<T>, |&(x, y)| {
        assert!(x.divisible_by(y));
        assert!(
            x == T::ZERO || x == T::MIN && y == T::NEGATIVE_ONE || y != T::ZERO && x % y == T::ZERO
        );
    });

    test_properties(pairs_of_signeds_var_3::<T>, |&(x, y)| {
        assert!(!x.divisible_by(y));
        assert!(
            x != T::ZERO
                && (x != T::MIN || y != T::NEGATIVE_ONE)
                && (y == T::ZERO || x % y != T::ZERO)
        );
    });

    test_properties(signeds::<T>, |&n| {
        assert!(n.divisible_by(T::ONE));
        assert!(n.divisible_by(T::NEGATIVE_ONE));
    });

    test_properties(nonzero_signeds::<T>, |&n| {
        assert!(!n.divisible_by(T::ZERO));
        assert!(T::ZERO.divisible_by(n));
        if n > T::ONE {
            assert!(!T::ONE.divisible_by(n));
        }
        assert!(n.divisible_by(n));
    });
}

#[test]
fn divisible_by_properties() {
    unsigned_divisible_by_properties_helper::<u8>();
    unsigned_divisible_by_properties_helper::<u16>();
    unsigned_divisible_by_properties_helper::<u32>();
    unsigned_divisible_by_properties_helper::<u64>();
    unsigned_divisible_by_properties_helper::<usize>();

    signed_divisible_by_properties_helper::<i8>();
    signed_divisible_by_properties_helper::<i16>();
    signed_divisible_by_properties_helper::<i32>();
    signed_divisible_by_properties_helper::<i64>();
    signed_divisible_by_properties_helper::<isize>();
}
