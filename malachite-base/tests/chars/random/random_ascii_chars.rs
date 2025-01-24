// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::chars::random::random_ascii_chars;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

#[test]
fn test_random_ascii_chars() {
    let xs = random_ascii_chars(EXAMPLE_SEED);
    let values = xs.clone().take(200).collect::<String>();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_str(), common_values.as_slice(), median),
        (
            "q^\u{17}bF\\4T!/\u{1}q6\n/\u{11}Y\\wB\\r\\^\u{15}3\'.\'r\u{7}$\u{17}S:\rr@I(\u{10}\
            \u{11}}\u{b}\u{7}0z5.n1\u{10}At<9.w\\?b\u{15}(\\hJ\u{10}cO\\^5Edc\u{1f}kq{t=z./\u{5}x\
            \u{1}dZrJ%\u{5}`=VU_\u{7f}b;\u{13}\u{6}U.k\r\u{6}PBk]$p\u{1a}+FOH.\r,a\u{1}=DZZ\u{16}\
            \u{18}cY\t\u{1e}\u{19}&<,\u{13}%\u{c}{Z!$Z,\u{17}\u{8}?\u{3}\u{4}]\u{1}H\u{c}(K*|l\
            \u{15}8^:\u{e}\u{7f}D(P\u{1}XEk!$\u{14}/];E9d\u{e})|v\u{e}W*).\u{19}\u{11}5\u{7f}b8\
            \u{18}:",
            &[
                ('\u{2}', 8077),
                ('y', 8039),
                ('0', 8015),
                ('q', 7966),
                ('\u{8}', 7937),
                ('M', 7933),
                ('2', 7928),
                ('[', 7927),
                ('R', 7925),
                ('f', 7924)
            ][..],
            ('?', None)
        )
    );
}
