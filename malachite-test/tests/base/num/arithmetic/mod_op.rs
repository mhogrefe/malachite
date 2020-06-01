use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    nonzero_signeds, pairs_of_signed_and_nonzero_signed, pairs_of_signeds_var_2,
    pairs_of_unsigned_and_positive_unsigned, positive_unsigneds, signeds, unsigneds,
};

fn mod_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<T, T>,
        |&(x, y)| {
            let mut mut_x = x;
            mut_x.mod_assign(y);
            let r = mut_x;

            assert_eq!(x.mod_op(y), r);

            let mut mut_x = x;
            mut_x %= y;
            assert_eq!(mut_x, r);
            assert_eq!(x % y, r);
            assert_eq!(x.div_mod(y).1, r);
            assert_eq!(x.div_rem(y).1, r);
            assert!(r < y);
        },
    );

    test_properties(unsigneds::<T>, |&x| {
        assert_eq!(x.mod_op(T::ONE), T::ZERO);
    });

    test_properties(positive_unsigneds::<T>, |&x| {
        assert_eq!(x.mod_op(x), T::ZERO);
        assert_eq!(T::ZERO.mod_op(x), T::ZERO);
        if x > T::ONE {
            assert_eq!(T::ONE.mod_op(x), T::ONE);
        }
    });
}

fn mod_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_nonzero_signed::<T, T>, |&(x, y)| {
        let mut mut_x = x;
        mut_x.mod_assign(y);
        let r = mut_x;

        assert_eq!(x.mod_op(y), r);
        assert!(r.lt_abs(&y));
        assert!(r == T::ZERO || (r > T::ZERO) == (y > T::ZERO));
        if x != T::MIN {
            assert_eq!(x.ceiling_mod(y), -(-x).mod_op(y));
        }
        if y != T::MIN {
            assert_eq!(x.ceiling_mod(y), x.mod_op(-y));
        }
    });

    test_properties(pairs_of_signeds_var_2::<T>, |&(x, y)| {
        assert_eq!(x.mod_op(y), x.div_mod(y).1);
    });

    test_properties(signeds::<T>, |&x| {
        assert_eq!(x.mod_op(T::ONE), T::ZERO);
        assert_eq!(x.mod_op(T::NEGATIVE_ONE), T::ZERO);
    });

    test_properties(nonzero_signeds::<T>, |&x| {
        assert_eq!(x.mod_op(T::ONE), T::ZERO);
        assert_eq!(x.mod_op(x), T::ZERO);
        assert_eq!(T::ZERO.mod_op(x), T::ZERO);
        assert_eq!(x.mod_op(T::NEGATIVE_ONE), T::ZERO);
        if x != T::MIN {
            assert_eq!(x.mod_op(-x), T::ZERO);
        }
        if x > T::ONE {
            assert_eq!(T::ONE.mod_op(x), T::ONE);
            assert_eq!(T::NEGATIVE_ONE.mod_op(x), x - T::ONE);
        }
    });
}

#[test]
fn mod_properties() {
    mod_properties_unsigned_helper::<u8>();
    mod_properties_unsigned_helper::<u16>();
    mod_properties_unsigned_helper::<u32>();
    mod_properties_unsigned_helper::<u64>();
    mod_properties_unsigned_helper::<usize>();
    mod_properties_signed_helper::<i8>();
    mod_properties_signed_helper::<i16>();
    mod_properties_signed_helper::<i32>();
    mod_properties_signed_helper::<i64>();
    mod_properties_signed_helper::<isize>();
}

fn neg_mod_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<T, T>,
        |&(x, y)| {
            let mut mut_x = x;
            mut_x.neg_mod_assign(y);
            let r = mut_x;
            assert_eq!(x.neg_mod(y), r);
            assert_eq!(x.ceiling_div_neg_mod(y).1, r);
            assert!(r < y);
        },
    );

    test_properties(unsigneds::<T>, |&x| {
        assert_eq!(x.neg_mod(T::ONE), T::ZERO);
    });

    test_properties(positive_unsigneds::<T>, |&x| {
        assert_eq!(x.neg_mod(x), T::ZERO);
        assert_eq!(T::ZERO.neg_mod(x), T::ZERO);
        if x > T::ONE {
            assert_eq!(T::ONE.neg_mod(x), x - T::ONE);
        }
    });
}

#[test]
fn neg_mod_properties() {
    neg_mod_properties_helper::<u8>();
    neg_mod_properties_helper::<u16>();
    neg_mod_properties_helper::<u32>();
    neg_mod_properties_helper::<u64>();
    neg_mod_properties_helper::<usize>();
}

fn ceiling_mod_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_nonzero_signed::<T, T>, |&(x, y)| {
        let mut mut_x = x;
        mut_x.ceiling_mod_assign(y);
        let r = mut_x;
        assert_eq!(x.ceiling_mod(y), r);
        assert!(r.lt_abs(&y));
        assert!(r == T::ZERO || (r > T::ZERO) != (y > T::ZERO));
        if x != T::MIN {
            assert_eq!(x.mod_op(y), -(-x).ceiling_mod(y));
        }
        if y != T::MIN {
            assert_eq!(x.mod_op(y), x.ceiling_mod(-y));
        }
    });

    test_properties(pairs_of_signeds_var_2::<T>, |&(x, y)| {
        assert_eq!(x.ceiling_mod(y), x.ceiling_div_mod(y).1);
    });

    test_properties(signeds::<T>, |&x| {
        assert_eq!(x.ceiling_mod(T::ONE), T::ZERO);
        assert_eq!(x.ceiling_mod(T::NEGATIVE_ONE), T::ZERO);
    });

    test_properties(nonzero_signeds::<T>, |&x| {
        assert_eq!(x.ceiling_mod(T::ONE), T::ZERO);
        assert_eq!(x.ceiling_mod(T::NEGATIVE_ONE), T::ZERO);
        assert_eq!(x.ceiling_mod(x), T::ZERO);
        if x != T::MIN {
            assert_eq!(x.ceiling_mod(-x), T::ZERO);
        }
        assert_eq!(T::ZERO.ceiling_mod(x), T::ZERO);
    });
}

#[test]
fn ceiling_mod_properties() {
    ceiling_mod_properties_helper::<i8>();
    ceiling_mod_properties_helper::<i16>();
    ceiling_mod_properties_helper::<i32>();
    ceiling_mod_properties_helper::<i64>();
    ceiling_mod_properties_helper::<isize>();
}
