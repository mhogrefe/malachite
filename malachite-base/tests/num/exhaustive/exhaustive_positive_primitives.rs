use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::exhaustive::exhaustive_positive_primitives;

fn exhaustive_positive_primitives_helper<T: PrimitiveInteger>()
where
    u8: ExactFrom<T>,
{
    assert_eq!(
        exhaustive_positive_primitives::<T>()
            .map(u8::exact_from)
            .take(20)
            .collect::<Vec<u8>>(),
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
    )
}

fn exhaustive_positive_primitives_long_helper<T: PrimitiveInteger>(last_20: &[T]) {
    let expected_len = if T::MIN == T::ZERO {
        usize::power_of_two(T::WIDTH) - 1
    } else {
        usize::power_of_two(T::WIDTH - 1) - 1
    };
    let xs = exhaustive_positive_primitives::<T>();
    assert_eq!(xs.clone().count(), expected_len);
    assert_eq!(xs.skip(expected_len - 20).collect::<Vec<T>>(), last_20)
}

#[test]
fn test_exhaustive_positive_primitives() {
    apply_fn_to_primitive_ints!(exhaustive_positive_primitives_helper);

    exhaustive_positive_primitives_long_helper::<u8>(&[
        236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253,
        254, 255,
    ]);
    exhaustive_positive_primitives_long_helper::<i8>(&[
        108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
        126, 127,
    ]);
    exhaustive_positive_primitives_long_helper::<u16>(&[
        65_516,
        65_517,
        65_518,
        65_519,
        0xfff0,
        0xfff1,
        0xfff2,
        0xfff3,
        0xfff4,
        0xfff5,
        0xfff6,
        0xfff7,
        0xfff8,
        0xfff9,
        0xfffa,
        0xfffb,
        0xfffc,
        0xfffd,
        u16::MAX - 1,
        u16::MAX,
    ]);
    exhaustive_positive_primitives_long_helper::<i16>(&[
        32_748,
        32_749,
        32_750,
        32_751,
        0x7ff0,
        0x7ff1,
        0x7ff2,
        0x7ff3,
        0x7ff4,
        0x7ff5,
        0x7ff6,
        0x7ff7,
        0x7ff8,
        0x7ff9,
        0x7ffa,
        0x7ffb,
        0x7ffc,
        0x7ffd,
        i16::MAX - 1,
        i16::MAX,
    ]);
}
