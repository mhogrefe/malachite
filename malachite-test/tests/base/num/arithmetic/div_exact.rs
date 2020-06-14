use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::rounding_mode::RoundingMode;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    nonzero_signeds, pairs_of_signeds_var_4, pairs_of_unsigneds_var_7, positive_unsigneds, signeds,
    unsigneds,
};

fn unsigned_div_exact_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigneds_var_7::<T>, |&(x, y)| {
        let mut mut_x = x.clone();
        mut_x.div_exact_assign(y);
        let q = mut_x;

        assert_eq!(x.div_exact(y), q);
        assert_eq!(x.div_round(y, RoundingMode::Exact), q);
        assert_eq!(q * y, x);
    });

    test_properties(unsigneds::<T>, |&n| {
        assert_eq!(n.div_exact(T::ONE), n);
    });

    test_properties(positive_unsigneds::<T>, |&n| {
        assert_eq!(T::ZERO.div_exact(n), T::ZERO);
        assert_eq!(n.div_exact(n), T::ONE);
    });
}

fn signed_div_exact_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signeds_var_4::<T>, |&(x, y)| {
        let mut mut_x = x.clone();
        mut_x.div_exact_assign(y);
        let q = mut_x;

        assert_eq!(x.div_exact(y), q);
        assert_eq!(x.div_round(y, RoundingMode::Exact), q);
        assert_eq!(q * y, x);

        if x != T::MIN {
            assert_eq!((-x).div_exact(y), -q);
        }
        if y != T::MIN && q != T::MIN {
            assert_eq!(x.div_exact(-y), -q);
        }
    });

    test_properties(signeds::<T>, |&n| {
        assert_eq!(n.div_exact(T::ONE), n);
    });

    test_properties(nonzero_signeds::<T>, |&n| {
        assert_eq!(T::ZERO.div_exact(n), T::ZERO);
        assert_eq!(n.div_exact(n), T::ONE);
    });
}

#[test]
fn div_exact_properties() {
    unsigned_div_exact_properties_helper::<u8>();
    unsigned_div_exact_properties_helper::<u16>();
    unsigned_div_exact_properties_helper::<u32>();
    unsigned_div_exact_properties_helper::<u64>();
    unsigned_div_exact_properties_helper::<usize>();

    signed_div_exact_properties_helper::<i8>();
    signed_div_exact_properties_helper::<i16>();
    signed_div_exact_properties_helper::<i32>();
    signed_div_exact_properties_helper::<i64>();
    signed_div_exact_properties_helper::<isize>();
}
