// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::iterators::iterator_to_bit_chunks;
use std::panic::{catch_unwind, RefUnwindSafe};

fn iterator_to_bit_chunks_helper<T: PrimitiveUnsigned, U: PrimitiveUnsigned + WrappingFrom<T>>(
    xs: &[T],
    in_chunk_size: u64,
    out_chunk_size: u64,
    out: &[U],
) {
    assert_eq!(
        iterator_to_bit_chunks::<_, T, U>(xs.iter().copied(), in_chunk_size, out_chunk_size)
            .map(Option::unwrap)
            .collect_vec()
            .as_slice(),
        out
    );
}

#[test]
fn test_iterator_to_bit_chunks() {
    iterator_to_bit_chunks_helper::<u16, u16>(&[123, 456], 10, 10, &[123, 456]);
    iterator_to_bit_chunks_helper::<u16, u32>(&[123, 456], 10, 10, &[123, 456]);
    iterator_to_bit_chunks_helper::<u32, u16>(&[123, 456], 10, 10, &[123, 456]);

    iterator_to_bit_chunks_helper::<u16, u16>(
        &[0b000111111, 0b110010010],
        9,
        3,
        &[0b111, 0b111, 0b000, 0b010, 0b010, 0b110],
    );
    iterator_to_bit_chunks_helper::<u16, u32>(
        &[0b000111111, 0b110010010],
        9,
        3,
        &[0b111, 0b111, 0b000, 0b010, 0b010, 0b110],
    );
    iterator_to_bit_chunks_helper::<u32, u16>(
        &[0b000111111, 0b110010010],
        9,
        3,
        &[0b111, 0b111, 0b000, 0b010, 0b010, 0b110],
    );

    iterator_to_bit_chunks_helper::<u16, u16>(
        &[0b111, 0b111, 0b000, 0b010, 0b010, 0b110],
        3,
        9,
        &[0b000111111, 0b110010010],
    );
    iterator_to_bit_chunks_helper::<u16, u32>(
        &[0b111, 0b111, 0b000, 0b010, 0b010, 0b110],
        3,
        9,
        &[0b000111111, 0b110010010],
    );
    iterator_to_bit_chunks_helper::<u32, u16>(
        &[0b111, 0b111, 0b000, 0b010, 0b010, 0b110],
        3,
        9,
        &[0b000111111, 0b110010010],
    );

    iterator_to_bit_chunks_helper::<u16, u16>(
        &[0b1010101, 0b1111101, 0b0100001, 0b110010],
        7,
        6,
        &[0b010101, 0b111011, 0b000111, 0b010010, 0b110],
    );
    iterator_to_bit_chunks_helper::<u16, u32>(
        &[0b1010101, 0b1111101, 0b0100001, 0b110010],
        7,
        6,
        &[0b010101, 0b111011, 0b000111, 0b010010, 0b110],
    );
    iterator_to_bit_chunks_helper::<u32, u16>(
        &[0b1010101, 0b1111101, 0b0100001, 0b110010],
        7,
        6,
        &[0b010101, 0b111011, 0b000111, 0b010010, 0b110],
    );

    iterator_to_bit_chunks_helper::<u16, u16>(
        &[0b010101, 0b111011, 0b000111, 0b010010, 0b110],
        6,
        7,
        &[0b1010101, 0b1111101, 0b0100001, 0b110010],
    );
    iterator_to_bit_chunks_helper::<u16, u32>(
        &[0b010101, 0b111011, 0b000111, 0b010010, 0b110],
        6,
        7,
        &[0b1010101, 0b1111101, 0b0100001, 0b110010],
    );
    iterator_to_bit_chunks_helper::<u32, u16>(
        &[0b010101, 0b111011, 0b000111, 0b010010, 0b110],
        6,
        7,
        &[0b1010101, 0b1111101, 0b0100001, 0b110010],
    );

    // The output may have trailing zero chunks.
    iterator_to_bit_chunks_helper::<u32, u16>(&[0b100], 32, 8, &[0b100, 0, 0, 0]);
}

fn test_iterator_to_bit_chunks_fail_helper<
    T: PrimitiveUnsigned + RefUnwindSafe,
    U: PrimitiveUnsigned + WrappingFrom<T>,
>() {
    let xs = [T::exact_from(12), T::exact_from(34)];
    assert_panic!({
        iterator_to_bit_chunks::<_, T, U>(xs.iter().copied(), 0, 4);
    });
    assert_panic!({
        iterator_to_bit_chunks::<_, T, U>(xs.iter().copied(), 4, 0);
    });
    assert_panic!({
        iterator_to_bit_chunks::<_, T, U>(xs.iter().copied(), T::WIDTH + 1, 4);
    });
    assert_panic!({
        iterator_to_bit_chunks::<_, T, U>(xs.iter().copied(), 4, U::WIDTH + 1);
    });
}

#[test]
fn iterator_to_bit_chunks_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(test_iterator_to_bit_chunks_fail_helper);
}
