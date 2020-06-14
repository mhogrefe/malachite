use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signeds, pairs_of_unsigneds, triples_of_signeds, triples_of_signeds_var_4,
    triples_of_unsigneds, triples_of_unsigneds_var_5,
};

fn unsigned_eq_mod_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds::<T>, |&(x, y, m)| {
        let equal = x.eq_mod(y, m);
        assert_eq!(y.eq_mod(x, m), equal);
    });

    test_properties(triples_of_unsigneds_var_5::<T>, |&(x, y, m)| {
        assert!(!x.eq_mod(y, m));
        assert!(!y.eq_mod(x, m));
    });

    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        assert!(x.eq_mod(y, T::ONE));
        assert_eq!(x.eq_mod(T::ZERO, y), x.divisible_by(y));
        assert!(x.eq_mod(x, y));
        assert_eq!(x.eq_mod(y, T::ZERO), x == y);
    });
}

fn signed_eq_mod_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(triples_of_signeds::<T>, |&(x, y, m)| {
        let equal = x.eq_mod(y, m);
        assert_eq!(y.eq_mod(x, m), equal);

        if x != T::MIN && y != T::MIN {
            assert_eq!((-x).eq_mod(-y, m), equal);
        }
        if m != T::MIN {
            assert_eq!(x.eq_mod(y, -m), equal);
        }
    });

    test_properties(triples_of_signeds_var_4::<T>, |&(x, y, m)| {
        assert!(!x.eq_mod(y, m));
        assert!(!y.eq_mod(x, m));
    });

    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        assert!(x.eq_mod(y, T::ONE));
        assert_eq!(x.eq_mod(y, T::ZERO), x == y);
        assert_eq!(x.eq_mod(T::ZERO, y), x.divisible_by(y));
        assert!(x.eq_mod(x, y));
    });
}

#[test]
fn eq_mod_properties() {
    unsigned_eq_mod_properties_helper::<u8>();
    unsigned_eq_mod_properties_helper::<u16>();
    unsigned_eq_mod_properties_helper::<u32>();
    unsigned_eq_mod_properties_helper::<u64>();
    unsigned_eq_mod_properties_helper::<usize>();

    signed_eq_mod_properties_helper::<i8>();
    signed_eq_mod_properties_helper::<i16>();
    signed_eq_mod_properties_helper::<i32>();
    signed_eq_mod_properties_helper::<i64>();
    signed_eq_mod_properties_helper::<isize>();
}
