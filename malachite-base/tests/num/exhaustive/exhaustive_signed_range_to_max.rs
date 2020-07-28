use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::exhaustive::exhaustive_signed_range_to_max;

fn expected_range_len<T: PrimitiveSigned>(a: T) -> usize
where
    usize: WrappingFrom<T>,
{
    usize::wrapping_from(T::MAX).wrapping_sub(usize::wrapping_from(a)) + 1
}

fn exhaustive_signed_range_to_max_helper_helper<T: PrimitiveSigned>(a: T, values: &[i32])
where
    i32: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    let xs = exhaustive_signed_range_to_max::<T>(a)
        .map(i32::exact_from)
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(xs, values);
    if T::WIDTH <= u16::WIDTH {
        assert_eq!(
            exhaustive_signed_range_to_max(a).count(),
            expected_range_len(a)
        );
    }
}

fn exhaustive_signed_range_to_max_rev_helper<T: PrimitiveSigned>(a: T, rev_values: &[T])
where
    usize: WrappingFrom<T>,
{
    let len = expected_range_len(a);
    assert_eq!(exhaustive_signed_range_to_max(a).count(), len);
    let mut tail = exhaustive_signed_range_to_max::<T>(a)
        .skip(len.saturating_sub(20))
        .collect::<Vec<_>>();
    tail.reverse();
    assert_eq!(tail, rev_values);
}

fn exhaustive_signed_range_to_max_helper<T: PrimitiveSigned>()
where
    i32: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    exhaustive_signed_range_to_max_helper_helper(
        T::ZERO,
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
    );
    exhaustive_signed_range_to_max_helper_helper(
        T::exact_from(20),
        &[
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
        ],
    );
    exhaustive_signed_range_to_max_helper_helper(
        T::MIN,
        &[
            0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10,
        ],
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_exhaustive_signed_range_to_max() {
    apply_fn_to_signeds!(exhaustive_signed_range_to_max_helper);

    exhaustive_signed_range_to_max_helper_helper(
        i8::MAX - 10,
        &[117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127],
    );
    exhaustive_signed_range_to_max_helper_helper(
        i16::MAX - 10,
        &[
            32757, 32758, 32759, 32760, 32761, 32762, 32763, 32764, 32765, 32766, 32767,
        ],
    );

    exhaustive_signed_range_to_max_rev_helper::<i8>(
        0,
        &[
            127, 126, 125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114, 113, 112, 111,
            110, 109, 108,
        ],
    );
    exhaustive_signed_range_to_max_rev_helper::<i8>(
        20,
        &[
            127, 126, 125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114, 113, 112, 111,
            110, 109, 108,
        ],
    );
    exhaustive_signed_range_to_max_rev_helper::<i16>(
        0,
        &[
            32767, 32766, 32765, 32764, 32763, 32762, 32761, 32760, 32759, 32758, 32757, 32756,
            32755, 32754, 32753, 32752, 32751, 32750, 32749, 32748,
        ],
    );
    exhaustive_signed_range_to_max_rev_helper::<i16>(
        20,
        &[
            32767, 32766, 32765, 32764, 32763, 32762, 32761, 32760, 32759, 32758, 32757, 32756,
            32755, 32754, 32753, 32752, 32751, 32750, 32749, 32748,
        ],
    );
}
