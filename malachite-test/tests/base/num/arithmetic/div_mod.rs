use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::rounding_mode::RoundingMode;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    nonzero_signeds, pairs_of_signeds_var_2, pairs_of_unsigned_and_positive_unsigned,
    positive_unsigneds, signeds, unsigneds,
};

fn div_mod_and_div_rem_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<T, T>,
        |&(x, y)| {
            let mut mut_x = x;
            let r = mut_x.div_assign_mod(y);
            let q = mut_x;

            assert_eq!(x.div_mod(y), (q, r));

            let mut mut_x = x;
            let r_alt = mut_x.div_assign_rem(y);
            let q_alt = mut_x;
            assert_eq!((q_alt, r_alt), (q, r));

            assert_eq!(x.div_rem(y), (q, r));

            assert_eq!((x / y, x % y), (q, r));
            assert!(r < y);
            assert_eq!(q * y + r, x);
        },
    );

    test_properties(unsigneds::<T>, |&x| {
        assert_eq!(x.div_mod(T::ONE), (x, T::ZERO));
    });

    test_properties(positive_unsigneds::<T>, |&x| {
        assert_eq!(x.div_mod(x), (T::ONE, T::ZERO));
        assert_eq!(T::ZERO.div_mod(x), (T::ZERO, T::ZERO));
        if x > T::ONE {
            assert_eq!(T::ONE.div_mod(x), (T::ZERO, T::ONE));
        }
    });
}

fn div_mod_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signeds_var_2::<T>, |&(x, y)| {
        let mut mut_x = x;
        let r = mut_x.div_assign_mod(y);
        let q = mut_x;

        assert_eq!(x.div_mod(y), (q, r));

        let (q_alt, r_alt) = (x.div_round(y, RoundingMode::Floor), x.mod_op(y));
        assert_eq!(q_alt, q);
        assert_eq!(r_alt, r);

        assert!(r.lt_abs(&y));
        assert!(r == T::ZERO || (r > T::ZERO) == (y > T::ZERO));
        if let Some(product) = q.checked_mul(y) {
            assert_eq!(product + r, x);
        } else if q > T::ZERO {
            assert_eq!((q - T::ONE) * y + r + y, x);
        } else {
            assert_eq!((q + T::ONE) * y + r - y, x);
        }

        if x != T::MIN {
            let (neg_q, neg_r) = (-x).div_mod(y);
            assert_eq!(x.ceiling_div_mod(y), (-neg_q, -neg_r));
        }

        if y != T::MIN {
            let (neg_q, r) = x.div_mod(-y);
            assert_eq!(x.ceiling_div_mod(y), (-neg_q, r));
        }
    });

    test_properties(signeds::<T>, |&x| {
        let (q, r) = x.div_mod(T::ONE);
        assert_eq!(q, x);
        assert_eq!(r, T::ZERO);

        if x != T::MIN {
            let (q, r) = x.div_mod(T::NEGATIVE_ONE);
            assert_eq!(q, -x);
            assert_eq!(r, T::ZERO);
        }
    });

    test_properties(nonzero_signeds::<T>, |&x| {
        assert_eq!(x.div_mod(T::ONE), (x, T::ZERO));
        assert_eq!(x.div_mod(x), (T::ONE, T::ZERO));
        assert_eq!(T::ZERO.div_mod(x), (T::ZERO, T::ZERO));
        if x != T::MIN {
            assert_eq!(x.div_mod(T::NEGATIVE_ONE), (-x, T::ZERO));
            assert_eq!(x.div_mod(-x), (T::NEGATIVE_ONE, T::ZERO));
        }
        if x > T::ONE {
            assert_eq!(T::ONE.div_mod(x), (T::ZERO, T::ONE));
            assert_eq!(T::NEGATIVE_ONE.div_mod(x), (T::NEGATIVE_ONE, x - T::ONE));
        }
    });
}

#[test]
fn div_mod_properties() {
    div_mod_and_div_rem_properties_unsigned_helper::<u8>();
    div_mod_and_div_rem_properties_unsigned_helper::<u16>();
    div_mod_and_div_rem_properties_unsigned_helper::<u32>();
    div_mod_and_div_rem_properties_unsigned_helper::<u64>();
    div_mod_and_div_rem_properties_unsigned_helper::<usize>();
    div_mod_properties_signed_helper::<i8>();
    div_mod_properties_signed_helper::<i16>();
    div_mod_properties_signed_helper::<i32>();
    div_mod_properties_signed_helper::<i64>();
    div_mod_properties_signed_helper::<isize>();
}

fn div_rem_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signeds_var_2::<T>, |&(x, y)| {
        let mut mut_x = x;
        let r = mut_x.div_assign_rem(y);
        let q = mut_x;

        assert_eq!(x.div_rem(y), (q, r));

        assert_eq!((x / y, x % y), (q, r));

        assert!(r.lt_abs(&y));
        assert!(r == T::ZERO || (r > T::ZERO) == (x > T::ZERO));
        assert_eq!(q * y + r, x);

        if x != T::MIN {
            assert_eq!((-x).div_rem(y), (-q, -r));
        }
        if y != T::MIN {
            assert_eq!(x.div_rem(-y), (-q, r));
        }
    });

    test_properties(signeds::<T>, |&x| {
        let (q, r) = x.div_rem(T::ONE);
        assert_eq!(q, x);
        assert_eq!(r, T::ZERO);

        if x != T::MIN {
            let (q, r) = x.div_rem(T::NEGATIVE_ONE);
            assert_eq!(q, -x);
            assert_eq!(r, T::ZERO);
        }
    });

    test_properties(nonzero_signeds::<T>, |&x| {
        assert_eq!(x.div_rem(T::ONE), (x, T::ZERO));
        assert_eq!(x.div_rem(x), (T::ONE, T::ZERO));
        assert_eq!(T::ZERO.div_rem(x), (T::ZERO, T::ZERO));
        if x != T::MIN {
            assert_eq!(x.div_rem(T::NEGATIVE_ONE), (-x, T::ZERO));
            assert_eq!(x.div_rem(-x), (T::NEGATIVE_ONE, T::ZERO));
        }
        if x > T::ONE {
            assert_eq!(T::ONE.div_rem(x), (T::ZERO, T::ONE));
            assert_eq!(T::NEGATIVE_ONE.div_rem(x), (T::ZERO, T::NEGATIVE_ONE));
        }
    });
}

#[test]
fn div_rem_properties() {
    div_rem_properties_signed_helper::<i8>();
    div_rem_properties_signed_helper::<i16>();
    div_rem_properties_signed_helper::<i32>();
    div_rem_properties_signed_helper::<i64>();
    div_rem_properties_signed_helper::<isize>();
}

fn ceiling_div_neg_mod_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<T, T>,
        |&(x, y)| {
            let mut mut_x = x;
            let r = mut_x.ceiling_div_assign_neg_mod(y);
            let q = mut_x;

            assert_eq!(x.ceiling_div_neg_mod(y), (q, r));

            let (q_alt, r_alt) = (x.div_round(y, RoundingMode::Ceiling), x.neg_mod(y));
            assert_eq!(q_alt, q);
            assert_eq!(r_alt, r);

            assert!(r < y);
            if let Some(product) = q.checked_mul(y) {
                assert_eq!(product - r, x);
            } else {
                assert_eq!((q - T::ONE) * y - r + y, x);
            }
        },
    );

    test_properties(unsigneds::<T>, |&x| {
        assert_eq!(x.ceiling_div_neg_mod(T::ONE), (x, T::ZERO));
    });

    test_properties(positive_unsigneds::<T>, |&x| {
        assert_eq!(x.ceiling_div_neg_mod(x), (T::ONE, T::ZERO));
        assert_eq!(T::ZERO.ceiling_div_neg_mod(x), (T::ZERO, T::ZERO));
        if x > T::ONE {
            assert_eq!(T::ONE.ceiling_div_neg_mod(x), (T::ONE, x - T::ONE));
        }
    });
}

#[test]
fn ceiling_div_neg_mod_properties() {
    ceiling_div_neg_mod_properties_helper::<u8>();
    ceiling_div_neg_mod_properties_helper::<u16>();
    ceiling_div_neg_mod_properties_helper::<u32>();
    ceiling_div_neg_mod_properties_helper::<u64>();
    ceiling_div_neg_mod_properties_helper::<usize>();
}

fn ceiling_div_mod_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signeds_var_2::<T>, |&(x, y)| {
        let mut mut_x = x;
        let r = mut_x.ceiling_div_assign_mod(y);
        let q = mut_x;

        assert_eq!(x.ceiling_div_mod(y), (q, r));

        let (q_alt, r_alt) = (x.div_round(y, RoundingMode::Ceiling), x.ceiling_mod(y));
        assert_eq!(q_alt, q);
        assert_eq!(r_alt, r);

        assert!(r.lt_abs(&y));
        assert!(r == T::ZERO || (r > T::ZERO) != (y > T::ZERO));
        if let Some(product) = q.checked_mul(y) {
            assert_eq!(product + r, x);
        } else if q > T::ZERO {
            assert_eq!((q - T::ONE) * y + r + y, x);
        } else {
            assert_eq!((q + T::ONE) * y + r - y, x);
        }

        if x != T::MIN {
            let (neg_q, neg_r) = (-x).ceiling_div_mod(y);
            assert_eq!(x.div_mod(y), (-neg_q, -neg_r));
        }
        if y != T::MIN {
            let (neg_q, r) = x.ceiling_div_mod(-y);
            assert_eq!(x.div_mod(y), (-neg_q, r));
        }
    });

    test_properties(signeds::<T>, |&x| {
        let (q, r) = x.ceiling_div_mod(T::ONE);
        assert_eq!(q, x);
        assert_eq!(r, T::ZERO);

        if x != T::MIN {
            let (q, r) = x.ceiling_div_mod(T::NEGATIVE_ONE);
            assert_eq!(q, -x);
            assert_eq!(r, T::ZERO);
        }
    });

    test_properties(nonzero_signeds::<T>, |&x| {
        assert_eq!(x.ceiling_div_mod(T::ONE), (x.clone(), T::ZERO));
        if x != T::MIN {
            assert_eq!(x.ceiling_div_mod(T::NEGATIVE_ONE), (-x, T::ZERO));
        }
        assert_eq!(x.ceiling_div_mod(x), (T::ONE, T::ZERO));
        if x != T::MIN {
            assert_eq!(x.ceiling_div_mod(-x), (T::NEGATIVE_ONE, T::ZERO));
        }
        assert_eq!(T::ZERO.ceiling_div_mod(x), (T::ZERO, T::ZERO));
    });
}

#[test]
fn ceiling_div_mod_properties() {
    ceiling_div_mod_properties_helper::<i8>();
    ceiling_div_mod_properties_helper::<i16>();
    ceiling_div_mod_properties_helper::<i32>();
    ceiling_div_mod_properties_helper::<i64>();
    ceiling_div_mod_properties_helper::<isize>();
}
