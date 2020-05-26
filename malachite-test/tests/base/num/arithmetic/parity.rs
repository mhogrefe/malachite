use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn unsigned_even_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&x| {
        let even = x.even();
        assert_eq!(x.divisible_by(T::TWO), even);
        assert_eq!(!x.odd(), even);
        if x != T::MAX {
            assert_eq!((x + T::ONE).odd(), even);
        }
        if x != T::ZERO {
            assert_eq!((x - T::ONE).odd(), even);
        }
    });
}

fn signed_even_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&x| {
        let even = x.even();
        assert_eq!(x.divisible_by(T::TWO), even);
        assert_eq!(!x.odd(), even);
        if x != T::MAX {
            assert_eq!((x + T::ONE).odd(), even);
        }
        if x != T::MIN {
            assert_eq!((x - T::ONE).odd(), even);
            assert_eq!((-x).even(), even);
        }
    });
}

#[test]
fn even_properties() {
    unsigned_even_properties_helper::<u8>();
    unsigned_even_properties_helper::<u16>();
    unsigned_even_properties_helper::<u32>();
    unsigned_even_properties_helper::<u64>();
    unsigned_even_properties_helper::<usize>();

    signed_even_properties_helper::<i8>();
    signed_even_properties_helper::<i16>();
    signed_even_properties_helper::<i32>();
    signed_even_properties_helper::<i64>();
    signed_even_properties_helper::<isize>();
}

fn unsigned_odd_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&x| {
        let odd = x.odd();
        assert_ne!(x.divisible_by(T::TWO), odd);
        assert_eq!(!x.even(), odd);
        if x != T::MAX {
            assert_eq!((x + T::ONE).even(), odd);
        }
        if x != T::ZERO {
            assert_eq!((x - T::ONE).even(), odd);
        }
    });
}

fn signed_odd_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&x| {
        let odd = x.odd();
        assert_ne!(x.divisible_by(T::TWO), odd);
        assert_eq!(!x.even(), odd);
        if x != T::MAX {
            assert_eq!((x + T::ONE).even(), odd);
        }
        if x != T::MIN {
            assert_eq!((x - T::ONE).even(), odd);
            assert_eq!((-x).odd(), odd);
        }
    });
}

#[test]
fn odd_properties() {
    unsigned_odd_properties_helper::<u8>();
    unsigned_odd_properties_helper::<u16>();
    unsigned_odd_properties_helper::<u32>();
    unsigned_odd_properties_helper::<u64>();
    unsigned_odd_properties_helper::<usize>();

    signed_odd_properties_helper::<i8>();
    signed_odd_properties_helper::<i16>();
    signed_odd_properties_helper::<i32>();
    signed_odd_properties_helper::<i64>();
    signed_odd_properties_helper::<isize>();
}
