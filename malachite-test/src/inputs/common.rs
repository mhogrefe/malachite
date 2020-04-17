pub(crate) fn reshape_2_1_to_3<A: 'static, B: 'static, C: 'static>(
    it: Box<dyn Iterator<Item = ((A, B), C)>>,
) -> Box<dyn Iterator<Item = (A, B, C)>> {
    Box::new(it.map(|((a, b), c)| (a, b, c)))
}

pub(crate) fn reshape_1_2_to_3<A: 'static, B: 'static, C: 'static>(
    it: Box<dyn Iterator<Item = (A, (B, C))>>,
) -> Box<dyn Iterator<Item = (A, B, C)>> {
    Box::new(it.map(|(a, (b, c))| (a, b, c)))
}

pub(crate) fn reshape_3_1_to_4<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<dyn Iterator<Item = ((A, B, C), D)>>,
) -> Box<dyn Iterator<Item = (A, B, C, D)>> {
    Box::new(it.map(|((a, b, c), d)| (a, b, c, d)))
}

pub(crate) fn reshape_2_2_to_4<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<dyn Iterator<Item = ((A, B), (C, D))>>,
) -> Box<dyn Iterator<Item = (A, B, C, D)>> {
    Box::new(it.map(|((a, b), (c, d))| (a, b, c, d)))
}

pub(crate) fn reshape_3_3_3_to_9<
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    H: 'static,
    I: 'static,
>(
    it: Box<dyn Iterator<Item = ((A, B, C), (D, E, F), (G, H, I))>>,
) -> Box<dyn Iterator<Item = (A, B, C, D, E, F, G, H, I)>> {
    Box::new(it.map(|((a, b, c), (d, e, f), (g, h, i))| (a, b, c, d, e, f, g, h, i)))
}

pub(crate) fn reshape_4_4_4_to_12<
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    H: 'static,
    I: 'static,
    J: 'static,
    K: 'static,
    L: 'static,
>(
    it: Box<dyn Iterator<Item = ((A, B, C, D), (E, F, G, H), (I, J, K, L))>>,
) -> Box<dyn Iterator<Item = (A, B, C, D, E, F, G, H, I, J, K, L)>> {
    Box::new(
        it.map(|((a, b, c, d), (e, f, g, h), (i, j, k, l))| (a, b, c, d, e, f, g, h, i, j, k, l)),
    )
}

pub(crate) fn permute_2_1<A: 'static, B: 'static>(
    it: Box<dyn Iterator<Item = (A, B)>>,
) -> Box<dyn Iterator<Item = (B, A)>> {
    Box::new(it.map(|(a, b)| (b, a)))
}

pub(crate) fn permute_1_3_2<A: 'static, B: 'static, C: 'static>(
    it: Box<dyn Iterator<Item = (A, B, C)>>,
) -> Box<dyn Iterator<Item = (A, C, B)>> {
    Box::new(it.map(|(a, b, c)| (a, c, b)))
}

pub(crate) fn permute_2_1_3<A: 'static, B: 'static, C: 'static>(
    it: Box<dyn Iterator<Item = (A, B, C)>>,
) -> Box<dyn Iterator<Item = (B, A, C)>> {
    Box::new(it.map(|(a, b, c)| (b, a, c)))
}

pub(crate) fn permute_1_2_4_3<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<dyn Iterator<Item = (A, B, C, D)>>,
) -> Box<dyn Iterator<Item = (A, B, D, C)>> {
    Box::new(it.map(|(a, b, c, d)| (a, b, d, c)))
}

pub(crate) fn permute_1_3_4_2<A: 'static, B: 'static, C: 'static, D: 'static>(
    it: Box<dyn Iterator<Item = (A, B, C, D)>>,
) -> Box<dyn Iterator<Item = (A, C, D, B)>> {
    Box::new(it.map(|(a, b, c, d)| (a, c, d, b)))
}
