use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::exhaustive::primitive_integer_increasing_range_to_max;

fn expected_range_len<T: PrimitiveInteger>(a: T) -> usize
where
    usize: WrappingFrom<T>,
{
    usize::wrapping_from(T::MAX).wrapping_sub(usize::wrapping_from(a)) + 1
}

fn primitive_integer_increasing_range_to_max_helper_helper<T: PrimitiveInteger>(
    a: T,
    values: &[i32],
) where
    i32: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    let xs = primitive_integer_increasing_range_to_max::<T>(a)
        .map(i32::exact_from)
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(xs, values);
    if T::WIDTH <= u16::WIDTH {
        let len = expected_range_len(a);
        assert_eq!(primitive_integer_increasing_range_to_max(a).count(), len);
        let mut init = primitive_integer_increasing_range_to_max::<T>(a)
            .rev()
            .skip(len.saturating_sub(20))
            .map(i32::exact_from)
            .collect::<Vec<_>>();
        init.reverse();
        assert_eq!(xs, init);
    }
}

fn primitive_integer_increasing_range_to_max_rev_helper<T: PrimitiveInteger>(a: T, rev_values: &[T])
where
    usize: WrappingFrom<T>,
{
    let xs = primitive_integer_increasing_range_to_max::<T>(a)
        .rev()
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(xs, rev_values);
    if T::WIDTH <= u16::WIDTH {
        let len = expected_range_len(a);
        assert_eq!(
            primitive_integer_increasing_range_to_max(a).rev().count(),
            len
        );
        let mut tail = primitive_integer_increasing_range_to_max::<T>(a)
            .skip(len.saturating_sub(20))
            .collect::<Vec<_>>();
        tail.reverse();
        assert_eq!(xs, tail);
    }
}

fn primitive_integer_increasing_range_to_max_helper<T: PrimitiveInteger>()
where
    i32: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    primitive_integer_increasing_range_to_max_helper_helper(
        T::ZERO,
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
    );
    primitive_integer_increasing_range_to_max_helper_helper(
        T::exact_from(20),
        &[
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
        ],
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_primitive_integer_increasing_range_to_max() {
    apply_fn_to_primitive_ints!(primitive_integer_increasing_range_to_max_helper);

    primitive_integer_increasing_range_to_max_helper_helper(
        u8::MAX - 10,
        &[245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255],
    );
    primitive_integer_increasing_range_to_max_helper_helper(
        u16::MAX - 10,
        &[
            65525, 65526, 65527, 65528, 65529, 65530, 65531, 65532, 65533, 65534, 65535,
        ],
    );
    primitive_integer_increasing_range_to_max_helper_helper(
        i8::MAX - 10,
        &[117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127],
    );
    primitive_integer_increasing_range_to_max_helper_helper(
        i16::MAX - 10,
        &[
            32757, 32758, 32759, 32760, 32761, 32762, 32763, 32764, 32765, 32766, 32767,
        ],
    );

    primitive_integer_increasing_range_to_max_rev_helper::<u8>(
        0,
        &[
            255, 254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240, 239,
            238, 237, 236,
        ],
    );
    primitive_integer_increasing_range_to_max_rev_helper::<u8>(
        20,
        &[
            255, 254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240, 239,
            238, 237, 236,
        ],
    );
    primitive_integer_increasing_range_to_max_rev_helper::<u16>(
        0,
        &[
            65535, 65534, 65533, 65532, 65531, 65530, 65529, 65528, 65527, 65526, 65525, 65524,
            65523, 65522, 65521, 65520, 65519, 65518, 65517, 65516,
        ],
    );
    primitive_integer_increasing_range_to_max_rev_helper::<u16>(
        20,
        &[
            65535, 65534, 65533, 65532, 65531, 65530, 65529, 65528, 65527, 65526, 65525, 65524,
            65523, 65522, 65521, 65520, 65519, 65518, 65517, 65516,
        ],
    );

    primitive_integer_increasing_range_to_max_helper_helper::<i8>(
        i8::MIN,
        &[
            -128, -127, -126, -125, -124, -123, -122, -121, -120, -119, -118, -117, -116, -115,
            -114, -113, -112, -111, -110, -109,
        ],
    );
    primitive_integer_increasing_range_to_max_rev_helper::<i8>(
        0,
        &[
            127, 126, 125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114, 113, 112, 111,
            110, 109, 108,
        ],
    );
    primitive_integer_increasing_range_to_max_rev_helper::<i8>(
        20,
        &[
            127, 126, 125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114, 113, 112, 111,
            110, 109, 108,
        ],
    );
    primitive_integer_increasing_range_to_max_rev_helper::<i16>(
        0,
        &[
            32767, 32766, 32765, 32764, 32763, 32762, 32761, 32760, 32759, 32758, 32757, 32756,
            32755, 32754, 32753, 32752, 32751, 32750, 32749, 32748,
        ],
    );
    primitive_integer_increasing_range_to_max_rev_helper::<i16>(
        20,
        &[
            32767, 32766, 32765, 32764, 32763, 32762, 32761, 32760, 32759, 32758, 32757, 32756,
            32755, 32754, 32753, 32752, 32751, 32750, 32749, 32748,
        ],
    );
}
