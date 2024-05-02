// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::random::striped::StripedBitSource;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::random::get_striped_random_natural_with_up_to_bits;

fn get_striped_random_natural_with_up_to_bits_helper(
    m_numerator: u64,
    m_denominator: u64,
    bits: u64,
    out: &str,
) {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, m_numerator, m_denominator);
    assert_eq!(
        get_striped_random_natural_with_up_to_bits(&mut bit_source, bits).to_string(),
        out
    );
}

#[test]
fn test_get_random_natural_with_up_to_bits() {
    get_striped_random_natural_with_up_to_bits_helper(2, 1, 0, "0");
    get_striped_random_natural_with_up_to_bits_helper(2, 1, 1, "0");
    get_striped_random_natural_with_up_to_bits_helper(2, 1, 10, "204");
    get_striped_random_natural_with_up_to_bits_helper(2, 1, 100, "756308944479610176770360563916");
    get_striped_random_natural_with_up_to_bits_helper(
        2,
        1,
        1000,
        "530282098229452921586618511132705551724421767516618560367637559688493429292030591954731339\
        7807621409756790898297593901473840098546243409160554985560709769081990616677721618124140633\
        1444828618628179441909125246001056008709800170061531016096640501974357613809706540492609673\
        66504273137971882485916981452",
    );

    get_striped_random_natural_with_up_to_bits_helper(10, 1, 0, "0");
    get_striped_random_natural_with_up_to_bits_helper(10, 1, 1, "0");
    get_striped_random_natural_with_up_to_bits_helper(10, 1, 10, "1016");
    get_striped_random_natural_with_up_to_bits_helper(10, 1, 100, "316912612278197474676665499640");
    get_striped_random_natural_with_up_to_bits_helper(
        10,
        1,
        1000,
        "622702178944542726303911365573092191857295100650306653895014963995942375266285106886384284\
        6174931899135470014771225146998068867182969950386903035779357388391986981527506826577962959\
        3515103233263288545758093603615063849795162246258569877970413681149384221331054812711431380\
        2727213763684707371859960",
    );

    get_striped_random_natural_with_up_to_bits_helper(11, 10, 0, "0");
    get_striped_random_natural_with_up_to_bits_helper(11, 10, 1, "0");
    get_striped_random_natural_with_up_to_bits_helper(11, 10, 10, "682");
    get_striped_random_natural_with_up_to_bits_helper(
        11,
        10,
        100,
        "1063803140432100403291953916586",
    );
    get_striped_random_natural_with_up_to_bits_helper(
        11,
        10,
        1000,
        "357169493129876665814859757913350039680505571889090475584942180169110567563969892408547521\
        7186818671377861364341116537204961881442236045088337279057826912458138650691199761612988883\
        0585814111331478884616006275012815377428721359743812263024290368526821458049290271802834122\
        15635242948169473448721083050",
    );
}
