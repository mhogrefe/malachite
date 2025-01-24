// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::vecs::exhaustive_vec_permutations;

fn exhaustive_vec_permutations_helper(cs: &str, out: &[&str]) {
    let cs = cs.chars().collect_vec();
    let css: Vec<String> = exhaustive_vec_permutations(cs)
        .map(|ds| ds.into_iter().collect())
        .collect();
    assert_eq!(css.iter().map(String::as_str).collect_vec().as_slice(), out);
}

#[test]
fn test_exhaustive_vec_permutations() {
    exhaustive_vec_permutations_helper("", &[""]);
    exhaustive_vec_permutations_helper("1", &["1"]);
    exhaustive_vec_permutations_helper("12", &["12", "21"]);
    exhaustive_vec_permutations_helper("123", &["123", "132", "213", "231", "312", "321"]);
    exhaustive_vec_permutations_helper(
        "1234",
        &[
            "1234", "1243", "1324", "1342", "1423", "1432", "2134", "2143", "2314", "2341", "2413",
            "2431", "3124", "3142", "3214", "3241", "3412", "3421", "4123", "4132", "4213", "4231",
            "4312", "4321",
        ],
    );
    exhaustive_vec_permutations_helper(
        "12345",
        &[
            "12345", "12354", "12435", "12453", "12534", "12543", "13245", "13254", "13425",
            "13452", "13524", "13542", "14235", "14253", "14325", "14352", "14523", "14532",
            "15234", "15243", "15324", "15342", "15423", "15432", "21345", "21354", "21435",
            "21453", "21534", "21543", "23145", "23154", "23415", "23451", "23514", "23541",
            "24135", "24153", "24315", "24351", "24513", "24531", "25134", "25143", "25314",
            "25341", "25413", "25431", "31245", "31254", "31425", "31452", "31524", "31542",
            "32145", "32154", "32415", "32451", "32514", "32541", "34125", "34152", "34215",
            "34251", "34512", "34521", "35124", "35142", "35214", "35241", "35412", "35421",
            "41235", "41253", "41325", "41352", "41523", "41532", "42135", "42153", "42315",
            "42351", "42513", "42531", "43125", "43152", "43215", "43251", "43512", "43521",
            "45123", "45132", "45213", "45231", "45312", "45321", "51234", "51243", "51324",
            "51342", "51423", "51432", "52134", "52143", "52314", "52341", "52413", "52431",
            "53124", "53142", "53214", "53241", "53412", "53421", "54123", "54132", "54213",
            "54231", "54312", "54321",
        ],
    );
    exhaustive_vec_permutations_helper(
        "abcd",
        &[
            "abcd", "abdc", "acbd", "acdb", "adbc", "adcb", "bacd", "badc", "bcad", "bcda", "bdac",
            "bdca", "cabd", "cadb", "cbad", "cbda", "cdab", "cdba", "dabc", "dacb", "dbac", "dbca",
            "dcab", "dcba",
        ],
    );
}
