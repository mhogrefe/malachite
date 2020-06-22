use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::exhaustive::{exhaustive_natural_signeds, exhaustive_negative_signeds};

fn exhaustive_negative_signeds_helper<T: PrimitiveSigned>()
where
    i8: ExactFrom<T>,
{
    let xs = exhaustive_negative_signeds::<T>()
        .map(i8::exact_from)
        .take(20)
        .collect::<Vec<i8>>();
    assert_eq!(
        xs,
        &[
            -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19,
            -20
        ]
    );
    assert!(itertools::equal(
        xs,
        exhaustive_natural_signeds::<T>()
            .map(|x| !i8::exact_from(x))
            .take(20)
    ));
}

fn exhaustive_negative_signeds_long_helper<T: PrimitiveSigned>(last_20: &[T]) {
    let expected_len = usize::power_of_two(T::WIDTH - 1);
    let xs = exhaustive_negative_signeds::<T>();
    assert_eq!(xs.clone().count(), expected_len);
    assert_eq!(xs.skip(expected_len - 20).collect::<Vec<T>>(), last_20)
}

#[test]
fn test_exhaustive_negative_signeds() {
    exhaustive_negative_signeds_helper::<i8>();
    exhaustive_negative_signeds_helper::<i16>();
    exhaustive_negative_signeds_helper::<i32>();
    exhaustive_negative_signeds_helper::<i64>();
    exhaustive_negative_signeds_helper::<i128>();
    exhaustive_negative_signeds_helper::<isize>();

    exhaustive_negative_signeds_long_helper::<i8>(&[
        -109, -110, -111, -112, -113, -114, -115, -116, -117, -118, -119, -120, -121, -122, -123,
        -124, -125, -126, -127, -128,
    ]);
    exhaustive_negative_signeds_long_helper::<i16>(&[
        -32_749,
        -32_750,
        -32_751,
        -0x7ff0,
        -0x7ff1,
        -0x7ff2,
        -0x7ff3,
        -0x7ff4,
        -0x7ff5,
        -0x7ff6,
        -0x7ff7,
        -0x7ff8,
        -0x7ff9,
        -0x7ffa,
        -0x7ffb,
        -0x7ffc,
        -0x7ffd,
        -0x7ffe,
        i16::MIN + 1,
        i16::MIN,
    ]);
}
