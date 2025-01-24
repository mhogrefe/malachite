// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::thue_morse_sequence;

#[test]
pub fn test_thue_morse_sequence() {
    let s: String = thue_morse_sequence()
        .take(1000)
        .map(|b| if b { '1' } else { '0' })
        .collect();
    assert_eq!(
        s,
        "01101001100101101001011001101001100101100110100101101001100101101001011001101001011010011\
        001011001101001100101101001011001101001100101100110100101101001100101100110100110010110100\
        101100110100101101001100101101001011001101001100101100110100101101001100101101001011001101\
        001011010011001011001101001100101101001011001101001011010011001011010010110011010011001011\
        001101001011010011001011001101001100101101001011001101001100101100110100101101001100101101\
        001011001101001011010011001011001101001100101101001011001101001100101100110100101101001100\
        101100110100110010110100101100110100101101001100101101001011001101001100101100110100101101\
        001100101100110100110010110100101100110100110010110011010010110100110010110100101100110100\
        101101001100101100110100110010110100101100110100101101001100101101001011001101001100101100\
        110100101101001100101101001011001101001011010011001011001101001100101101001011001101001100\
        101100110100101101001100101100110100110010110100101100110100101101001100101101001011001101\
        00110010110"
    );
}
