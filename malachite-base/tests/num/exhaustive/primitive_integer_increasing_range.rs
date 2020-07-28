use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::exhaustive::primitive_integer_increasing_range;

fn expected_range_len<T: PrimitiveInteger>(a: T, b: T) -> usize
where
    usize: WrappingFrom<T>,
{
    usize::wrapping_from(b).wrapping_sub(usize::wrapping_from(a))
}

fn primitive_integer_increasing_range_helper_helper<T: PrimitiveInteger>(a: T, b: T, values: &[i8])
where
    i8: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    let xs = primitive_integer_increasing_range::<T>(a, b)
        .map(i8::exact_from)
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(xs, values);
    if T::WIDTH <= u16::WIDTH {
        let len = expected_range_len(a, b);
        assert_eq!(primitive_integer_increasing_range(a, b).count(), len);
        let mut init = primitive_integer_increasing_range::<T>(a, b)
            .rev()
            .skip(len.saturating_sub(20))
            .map(i8::exact_from)
            .collect::<Vec<_>>();
        init.reverse();
        assert_eq!(xs, init);
    }
}

fn primitive_integer_increasing_range_rev_helper<T: PrimitiveInteger>(a: T, b: T, rev_values: &[T])
where
    usize: WrappingFrom<T>,
{
    let xs = primitive_integer_increasing_range::<T>(a, b)
        .rev()
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(xs, rev_values);
    if T::WIDTH <= u16::WIDTH {
        let len = expected_range_len(a, b);
        assert_eq!(primitive_integer_increasing_range(a, b).rev().count(), len);
        let mut tail = primitive_integer_increasing_range::<T>(a, b)
            .skip(len.saturating_sub(20))
            .collect::<Vec<_>>();
        tail.reverse();
        assert_eq!(xs, tail);
    }
}

fn primitive_integer_increasing_range_helper<T: PrimitiveInteger>()
where
    i8: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    primitive_integer_increasing_range_helper_helper(T::ZERO, T::ZERO, &[]);
    primitive_integer_increasing_range_helper_helper(T::ZERO, T::ONE, &[0]);
    primitive_integer_increasing_range_helper_helper(
        T::ZERO,
        T::exact_from(10),
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    );
    primitive_integer_increasing_range_helper_helper(
        T::exact_from(10),
        T::exact_from(20),
        &[10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    );
    primitive_integer_increasing_range_helper_helper(
        T::ZERO,
        T::MAX,
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
    );
    primitive_integer_increasing_range_helper_helper(
        T::ZERO,
        T::MAX - T::ONE,
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
    );
}

fn primitive_integer_increasing_range_helper_signed<T: PrimitiveSigned>()
where
    i8: ExactFrom<T>,
    usize: WrappingFrom<T>,
{
    primitive_integer_increasing_range_helper_helper(
        T::exact_from(-20),
        T::exact_from(-10),
        &[-20, -19, -18, -17, -16, -15, -14, -13, -12, -11],
    );
    primitive_integer_increasing_range_helper_helper(
        T::exact_from(-100),
        T::exact_from(100),
        &[
            -100, -99, -98, -97, -96, -95, -94, -93, -92, -91, -90, -89, -88, -87, -86, -85, -84,
            -83, -82, -81,
        ],
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_primitive_integer_increasing_range() {
    apply_fn_to_primitive_ints!(primitive_integer_increasing_range_helper);
    apply_fn_to_signeds!(primitive_integer_increasing_range_helper_signed);

    primitive_integer_increasing_range_rev_helper::<u8>(
        0,
        u8::MAX,
        &[
            254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240, 239, 238,
            237, 236, 235,
        ],
    );
    primitive_integer_increasing_range_rev_helper::<u8>(
        0,
        u8::MAX - 1,
        &[
            253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240, 239, 238, 237,
            236, 235, 234,
        ],
    );
    primitive_integer_increasing_range_rev_helper::<u16>(
        0,
        u16::MAX,
        &[
            65534, 65533, 65532, 65531, 65530, 65529, 65528, 65527, 65526, 65525, 65524, 65523,
            65522, 65521, 65520, 65519, 65518, 65517, 65516, 65515,
        ],
    );
    primitive_integer_increasing_range_rev_helper::<u16>(
        0,
        u16::MAX - 1,
        &[
            65533, 65532, 65531, 65530, 65529, 65528, 65527, 65526, 65525, 65524, 65523, 65522,
            65521, 65520, 65519, 65518, 65517, 65516, 65515, 65514,
        ],
    );

    primitive_integer_increasing_range_helper_helper::<i8>(
        i8::MIN,
        i8::MAX,
        &[
            -128, -127, -126, -125, -124, -123, -122, -121, -120, -119, -118, -117, -116, -115,
            -114, -113, -112, -111, -110, -109,
        ],
    );
    primitive_integer_increasing_range_helper_helper::<i8>(
        i8::MIN + 1,
        i8::MAX - 1,
        &[
            -127, -126, -125, -124, -123, -122, -121, -120, -119, -118, -117, -116, -115, -114,
            -113, -112, -111, -110, -109, -108,
        ],
    );

    primitive_integer_increasing_range_rev_helper::<i8>(
        i8::MIN,
        i8::MAX,
        &[
            126, 125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114, 113, 112, 111, 110,
            109, 108, 107,
        ],
    );
    primitive_integer_increasing_range_rev_helper::<i8>(
        i8::MIN + 1,
        i8::MAX - 1,
        &[
            125, 124, 123, 122, 121, 120, 119, 118, 117, 116, 115, 114, 113, 112, 111, 110, 109,
            108, 107, 106,
        ],
    );
    primitive_integer_increasing_range_rev_helper::<i16>(
        i16::MIN,
        i16::MAX,
        &[
            32766, 32765, 32764, 32763, 32762, 32761, 32760, 32759, 32758, 32757, 32756, 32755,
            32754, 32753, 32752, 32751, 32750, 32749, 32748, 32747,
        ],
    );
    primitive_integer_increasing_range_rev_helper::<i16>(
        i16::MIN + 1,
        i16::MAX - 1,
        &[
            32765, 32764, 32763, 32762, 32761, 32760, 32759, 32758, 32757, 32756, 32755, 32754,
            32753, 32752, 32751, 32750, 32749, 32748, 32747, 32746,
        ],
    );
}

macro_rules! primitive_integer_increasing_range_fail {
    (
        $t:ident,
        $primitive_integer_increasing_range_fail:ident
    ) => {
        #[test]
        #[should_panic]
        fn $primitive_integer_increasing_range_fail() {
            primitive_integer_increasing_range::<$t>(1, 0);
        }
    };
}
primitive_integer_increasing_range_fail!(u8, primitive_integer_increasing_range_u8_fail);
primitive_integer_increasing_range_fail!(u16, primitive_integer_increasing_range_u16_fail);
primitive_integer_increasing_range_fail!(u32, primitive_integer_increasing_range_u32_fail);
primitive_integer_increasing_range_fail!(u64, primitive_integer_increasing_range_u64_fail);
primitive_integer_increasing_range_fail!(u128, primitive_integer_increasing_range_u128_fail);
primitive_integer_increasing_range_fail!(usize, primitive_integer_increasing_range_usize_fail);
primitive_integer_increasing_range_fail!(i8, primitive_integer_increasing_range_i8_fail);
primitive_integer_increasing_range_fail!(i16, primitive_integer_increasing_range_i16_fail);
primitive_integer_increasing_range_fail!(i32, primitive_integer_increasing_range_i32_fail);
primitive_integer_increasing_range_fail!(i64, primitive_integer_increasing_range_i64_fail);
primitive_integer_increasing_range_fail!(i128, primitive_integer_increasing_range_i128_fail);
primitive_integer_increasing_range_fail!(isize, primitive_integer_increasing_range_isize_fail);
