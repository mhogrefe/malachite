use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_signeds, pairs_of_unsigneds, signeds, unsigneds};

fn properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        let cmp = x.cmp_abs(&y);
        assert_eq!(x.cmp(&y), cmp);
        assert_eq!(x.partial_cmp_abs(&y), Some(cmp));
        assert_eq!(x.lt_abs(&y), cmp == Ordering::Less);
        assert_eq!(x.gt_abs(&y), cmp == Ordering::Greater);
        assert_eq!(x.le_abs(&y), cmp != Ordering::Greater);
        assert_eq!(x.ge_abs(&y), cmp != Ordering::Less);
        assert_eq!(y.cmp_abs(&x), cmp.reverse());
    });

    test_properties(unsigneds::<T>, |x| {
        assert_eq!(x.cmp_abs(&x), Ordering::Equal);
        assert!(x.le_abs(&x));
        assert!(x.ge_abs(&x));
        assert!(!x.lt_abs(&x));
        assert!(!x.gt_abs(&x));
        assert!(x.le_abs(&T::MAX));
        assert!(x.ge_abs(&T::ZERO));
    });
}

fn properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as UnsignedAbs>::Output: Ord,
{
    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        let cmp = x.cmp_abs(&y);
        if x != T::MIN {
            if y != T::MIN {
                assert_eq!(x.unsigned_abs().cmp(&y.unsigned_abs()), cmp);
            }
            assert_eq!((-x).cmp_abs(&y), cmp);
        }
        if y != T::MIN {
            assert_eq!(x.cmp_abs(&-y), cmp);
        }
        assert_eq!(x.partial_cmp_abs(&y), Some(cmp));
        assert_eq!(x.lt_abs(&y), cmp == Ordering::Less);
        assert_eq!(x.gt_abs(&y), cmp == Ordering::Greater);
        assert_eq!(x.le_abs(&y), cmp != Ordering::Greater);
        assert_eq!(x.ge_abs(&y), cmp != Ordering::Less);
        assert_eq!(y.cmp_abs(&x), cmp.reverse());
    });

    test_properties(signeds::<T>, |x| {
        assert_eq!(x.cmp_abs(&x), Ordering::Equal);
        assert!(x.le_abs(&x));
        assert!(x.ge_abs(&x));
        assert!(!x.lt_abs(&x));
        assert!(!x.gt_abs(&x));
        assert!(x.le_abs(&T::MIN));
        assert!(x.ge_abs(&T::ZERO));
    });
}

#[test]
fn ord_abs_partial_ord_abs_and_comparators_properties() {
    properties_helper_unsigned::<u8>();
    properties_helper_unsigned::<u16>();
    properties_helper_unsigned::<u32>();
    properties_helper_unsigned::<u64>();
    properties_helper_unsigned::<usize>();
    properties_helper_signed::<i8>();
    properties_helper_signed::<i16>();
    properties_helper_signed::<i32>();
    properties_helper_signed::<i64>();
    properties_helper_signed::<isize>();
}
