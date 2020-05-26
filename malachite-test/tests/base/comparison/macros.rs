use std::cmp::{max, min};

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signeds, pairs_of_unsigneds, signeds, triples_of_signeds, triples_of_unsigneds,
    unsigneds,
};

fn unsigned_max_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&x| {
        assert_eq!(max!(x), x);
    });

    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        assert_eq!(max!(x, y), max(x, y));
    });

    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        assert_eq!(max!(x, y, z), max(max(x, y), z));
        assert_eq!(max!(x, y, z), max(x, max(y, z)));
    });
}

fn signed_max_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&x| {
        assert_eq!(max!(x), x);
    });

    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        assert_eq!(max!(x, y), max(x, y));
    });

    test_properties(triples_of_signeds::<T>, |&(x, y, z)| {
        assert_eq!(max!(x, y, z), max(max(x, y), z));
        assert_eq!(max!(x, y, z), max(x, max(y, z)));
    });
}

#[test]
fn max_properties() {
    unsigned_max_properties_helper::<u8>();
    unsigned_max_properties_helper::<u16>();
    unsigned_max_properties_helper::<u32>();
    unsigned_max_properties_helper::<u64>();
    unsigned_max_properties_helper::<usize>();

    signed_max_properties_helper::<i8>();
    signed_max_properties_helper::<i16>();
    signed_max_properties_helper::<i32>();
    signed_max_properties_helper::<i64>();
    signed_max_properties_helper::<isize>();
}

fn unsigned_min_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&x| {
        assert_eq!(min!(x), x);
    });

    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        assert_eq!(min!(x, y), min(x, y));
    });

    test_properties(triples_of_unsigneds::<T>, |&(x, y, z)| {
        assert_eq!(min!(x, y, z), min(min(x, y), z));
        assert_eq!(min!(x, y, z), min(x, min(y, z)));
    });
}

fn signed_min_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&x| {
        assert_eq!(min!(x), x);
    });

    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        assert_eq!(min!(x, y), min(x, y));
    });

    test_properties(triples_of_signeds::<T>, |&(x, y, z)| {
        assert_eq!(min!(x, y, z), min(min(x, y), z));
        assert_eq!(min!(x, y, z), min(x, min(y, z)));
    });
}

#[test]
fn min_properties() {
    unsigned_min_properties_helper::<u8>();
    unsigned_min_properties_helper::<u16>();
    unsigned_min_properties_helper::<u32>();
    unsigned_min_properties_helper::<u64>();
    unsigned_min_properties_helper::<usize>();

    signed_min_properties_helper::<i8>();
    signed_min_properties_helper::<i16>();
    signed_min_properties_helper::<i32>();
    signed_min_properties_helper::<i64>();
    signed_min_properties_helper::<isize>();
}
