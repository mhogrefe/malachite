fn main() {
    let quadruples = [((1u8, 2u8), (3u8, 4u8))];
    let pairs = [(1u8, 2u8)];

    // Both halves are destructured only to be rebuilt: flagged twice, once per half.
    let _ = quadruples
        .iter()
        .filter(|&&((n_2, n_1), (d_1, d_0))| (n_2, n_1) < (d_1, d_0))
        .count();
    // The same shape from a `let`: flagged.
    let (a, b) = (1u8, 2u8);
    let _ = (a, b);
    // A three-element pattern rebuilt whole: flagged.
    let (p, q, r) = (1u8, 2u8, 3u8);
    let _ = (p, q, r);

    // One of the names is also used on its own, so the pattern earns its keep: fine.
    let _ = pairs
        .iter()
        .map(|&(x, y)| if x > 0 { (x, y) } else { (y, x) })
        .count();
    // Rebuilt in the wrong order, which is a different tuple: fine.
    let (s, t) = (1u8, 2u8);
    let _ = (t, s);
    // Only part of the pattern is rebuilt: fine.
    let (u, v, w) = (1u8, 2u8, 3u8);
    let _ = ((u, v), w);
    // Names from different patterns: fine.
    let (e, _f) = (1u8, 2u8);
    let (_g, h) = (3u8, 4u8);
    let _ = (e, h);
    // Not bindings at all: fine.
    let _ = (1u8, 2u8);
}
