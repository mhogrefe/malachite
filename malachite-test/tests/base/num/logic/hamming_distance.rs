use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signeds, pairs_of_unsigneds, signeds, triples_of_signeds, triples_of_unsigneds,
    unsigneds,
};

fn hamming_distance_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        let distance = x.hamming_distance(y);
        assert_eq!(y.hamming_distance(x), distance);
        assert_eq!(distance == 0, x == y);
        assert_eq!(u64::from((x ^ y).count_ones()), distance);
        assert_eq!((!x).hamming_distance(!y), distance);
    });

    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        assert!(x.hamming_distance(z) <= x.hamming_distance(y) + y.hamming_distance(z));
    });

    test_properties(unsigneds::<T>, |&n| {
        assert_eq!(n.hamming_distance(n), 0);
        assert_eq!(n.hamming_distance(!n), u64::from(T::WIDTH));
        assert_eq!(n.hamming_distance(T::ZERO), u64::from(n.count_ones()));
        assert_eq!(T::ZERO.hamming_distance(n), u64::from(n.count_ones()));
    });
}

fn hamming_distance_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        let distance = x.hamming_distance(y);
        assert_eq!(y.hamming_distance(x), distance);
        assert_eq!(distance == 0, x == y);
        assert_eq!(u64::from((x ^ y).count_ones()), distance);
        assert_eq!((!x).hamming_distance(!y), distance);
    });

    test_properties(triples_of_signeds::<T>, |&(x, y, z)| {
        assert!(x.hamming_distance(z) <= x.hamming_distance(y) + y.hamming_distance(z));
    });

    test_properties(signeds::<T>, |&n| {
        assert_eq!(n.hamming_distance(n), 0);
        assert_eq!(n.hamming_distance(!n), u64::from(T::WIDTH));
        assert_eq!(n.hamming_distance(T::ZERO), u64::from(n.count_ones()));
        assert_eq!(T::ZERO.hamming_distance(n), u64::from(n.count_ones()));
        assert_eq!(
            n.hamming_distance(T::NEGATIVE_ONE),
            u64::from(n.count_zeros())
        );
        assert_eq!(
            T::NEGATIVE_ONE.hamming_distance(n),
            u64::from(n.count_zeros())
        );
    });
}

#[test]
fn hamming_distance_properties() {
    hamming_distance_properties_helper_unsigned::<u8>();
    hamming_distance_properties_helper_unsigned::<u16>();
    hamming_distance_properties_helper_unsigned::<u32>();
    hamming_distance_properties_helper_unsigned::<u64>();
    hamming_distance_properties_helper_unsigned::<usize>();
    hamming_distance_properties_helper_signed::<i8>();
    hamming_distance_properties_helper_signed::<i16>();
    hamming_distance_properties_helper_signed::<i32>();
    hamming_distance_properties_helper_signed::<i64>();
    hamming_distance_properties_helper_signed::<isize>();
}
