// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

fn rle_encode<T: Clone + Eq>(xs: &[T]) -> Vec<(T, usize)> {
    let mut out = Vec::new();
    let mut previous: Option<T> = None;
    let mut count = 0;
    for x in xs {
        if let Some(p) = previous.as_ref() {
            if x == p {
                count += 1;
            } else {
                out.push((p.clone(), count));
                previous = Some(x.clone());
                count = 1;
            }
        } else {
            count = 1;
            previous = Some(x.clone());
        }
    }
    if let Some(p) = previous {
        out.push((p, count));
    }
    out
}

pub(crate) fn generate_rle_encoding() {
    // Example xs
    let xs = &[1, 1, 1, 0, 0, 0, 0, 0, 0, 2, 2];
    println!("{:?}", rle_encode(xs));
}
