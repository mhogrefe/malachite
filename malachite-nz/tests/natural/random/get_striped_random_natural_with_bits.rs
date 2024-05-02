// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::random::striped::StripedBitSource;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::random::get_striped_random_natural_with_bits;

fn get_striped_random_natural_with_bits_helper(
    m_numerator: u64,
    m_denominator: u64,
    bits: u64,
    out: &str,
) {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, m_numerator, m_denominator);
    assert_eq!(
        get_striped_random_natural_with_bits(&mut bit_source, bits).to_string(),
        out
    );
}

#[test]
fn test_get_striped_random_natural_with_bits() {
    get_striped_random_natural_with_bits_helper(2, 1, 0, "0");
    get_striped_random_natural_with_bits_helper(2, 1, 1, "1");
    get_striped_random_natural_with_bits_helper(2, 1, 10, "716");
    get_striped_random_natural_with_bits_helper(2, 1, 100, "756308944479610176770360563916");
    get_striped_random_natural_with_bits_helper(
        2,
        1,
        1000,
        "106603640182258658206083103566270645700512417336938536408951275387366895485449865320133052\
        9188610070039476426288535963559977582497470497937854727434805905648395790056513373361685117\
        0447014047433756921281989047837597391841613716389932684381637088728893046979209497320344797\
        581420099450165301088751016140",
    );

    get_striped_random_natural_with_bits_helper(10, 1, 0, "0");
    get_striped_random_natural_with_bits_helper(10, 1, 1, "1");
    get_striped_random_natural_with_bits_helper(10, 1, 10, "1016");
    get_striped_random_natural_with_bits_helper(10, 1, 100, "950737912392312175425017102328");
    get_striped_random_natural_with_bits_helper(
        10,
        1,
        1000,
        "535816573811028114746842915666558214499888135362831834387264695681575119799994689757287827\
        8363096783827886911589242856640635533315179867213030979090927223140806482585564866175368333\
        5984663366032716099765341041735279416091316510062421684707527426682687794404521538192109445\
        28718553525957103310205894648",
    );

    get_striped_random_natural_with_bits_helper(11, 10, 0, "0");
    get_striped_random_natural_with_bits_helper(11, 10, 1, "1");
    get_striped_random_natural_with_bits_helper(11, 10, 10, "682");
    get_striped_random_natural_with_bits_helper(11, 10, 100, "1063803140432100403291953916586");
    get_striped_random_natural_with_bits_helper(
        11,
        10,
        1000,
        "892923796723010326289072282443350944961207977741857279306817374354286093126437953655146711\
        1265297962015834728928882271330897607870697615306329567845176199860105934578611877105699420\
        3611125967040868655526771507387733287135058353581608090744020753841394314031678704513672424\
        30551069260362892051555117738",
    );
}
