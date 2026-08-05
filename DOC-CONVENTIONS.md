# Documentation conventions

How to document Malachite functions: complexity blocks, scratch requirements, and the prose
around them. Distilled from the 2026-08 documentation audit (full log and findings in
`audit/CAMPAIGN-LOG.md`); the machine-checkable parts are enforced by
`complexity-doc-check.py`, which runs as part of `additional-lints.sh`.

Every documented function is audited along five dimensions: **prose** (accurate, complete,
correct names), **formulas** (the LaTeX math describing the function), **panics** (every panic
listed, including indirect ones), **examples** (exercising the exact impl being documented),
and **complexity** (a well-formed block whose claims are true and tight).

## Complexity blocks

### Format

A block is one of five headers followed by either the constant form, a delegation, or a
$T$/$M$/where triple:

```text
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
```

- Headers: `# Worst-case complexity`, `# Worst-case complexity (amortized)`,
  `# Worst-case complexity per iteration`, `# Expected complexity`,
  `# Expected complexity per iteration`. No other variants — the checker only knows these.
- The constant form is the single line `Constant time and additional memory.`
- A delegation form (`Same as ...`) is allowed when a function is a trivial wrapper.
- The where-line must define **every** variable used in the formulas, and must define $T$ as
  time and $M$ as additional memory. Callee-relative symbols ($T^\prime$, $T_S$, ...) must be
  explained there too.
- When the bound is non-obvious, append a short derivation after the where-line, set off by a
  colon:

```text
/// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`: setting a two's
/// complement bit subtracts a power of 2 from the stored magnitude, whose borrow can run
/// through the entire slice.
```

Accuracy arguments (why the exponent is what it is) belong on **leaf algorithms**; delegators
are verified against their callees and don't repeat the argument.

### Name every driver

The single most common historical error was costing a function in one parameter while another
parameter also drives the work.

- A precision-parameterized function is **never** costed in `prec` alone. Its input's size is
  a driver too: truncation with a correct `Ordering` scans discarded bits, argument-reduction
  subtractions read the operand, rational helpers do rational arithmetic. Use the additive
  two-variable form, with the m-term matched to what the implementation actually does:

```text
/// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
///
/// $M(n, m) = O(n \log n + m \log m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
/// `x.significant_bits()`.
```

  m-term classes: **linear** ($+ m$) for scans, sticky-bit checks, and `from_rational`
  squeezes; **mul-class** ($+ m \log m \log\log m$) for exact-root/exact-power detection;
  **gcd-class** ($+ m (\log m)^2 \log\log m$) when the helper performs `Rational` arithmetic.
  Prefer the additive form over folding into `max(prec, ...)` when the dependences have
  different exponents; a fold is acceptable when the kernel genuinely runs at the max width.
- `_round`-style functions with no `prec` parameter are costed in "the precision of the
  input" (or `self.significant_bits()`).
- Functions whose only inputs are machine words (e.g. `sqrt_unsigned_prec_round`) may be
  costed in `prec` alone — the input is bounded.

### Primitive integers: the width-stability rule

Primitive claims are analyzed as if generic over the word width $W$. "Constant time" is
claimed only when the operation count is independent of both $W$ and the input values.
Otherwise name the driver ($n$ = `significant_bits()`, or $W$ itself). Consequences:

- A word Euclid/binary gcd is $O(n)$ constant-cost word operations with no allocation — not
  the $O(n^2)$ bit-complexity of the bignum literature.
- A macro that documents many widths uniformly must be checked at the **widest** type: a
  `u128` impl often delegates to an $O(n)$ algorithm even when `u64` is a single instruction.

### Memory conventions

- $M$ is additional memory and **includes returned allocations**: a function returning a
  fresh `Vec` or cloning a `Natural` is $M(n) = O(n)$, even if it takes `&self`. Getters that
  clone are not constant.
- Full multiplication, division, and square root at $n$ bits use $M(n) = O(n \log n)$ (FFT
  scratch); this is the house convention, and anything that calls them at full size inherits
  it. Bounded working sets (AGM iterations, Ziv loops) peak at one multiplication's memory.
- A single `Vec` push is treated as amortized $O(1)$; wholesale `extend`/`to_vec`/`resize`
  growth is $O(\text{growth})$ and belongs in $M$.
- In-place functions with two mirrored variants (`_left`/`_right`): the growth term follows
  the **receiver**. Swapping the receiver swaps the direction of the size difference.

### Cost cheat-sheet (bits, house-standard bounds)

| operation | $T$ | $M$ |
|---|---|---|
| word ops, comparisons vs words | constant | constant |
| limb add/sub/logic/shift/scan | $O(n)$ | $O(1)$ in place, $O(n)$ allocating |
| mul, div, sqrt, root | $O(n \log n \log\log n)$ | $O(n \log n)$ |
| gcd, xgcd, jacobi/kronecker | $O(n (\log n)^2 \log\log n)$ | $O(n \log n)$ |
| `Rational` arithmetic | gcd-class in operand bits | $O(n \log n)$ |
| exact-root / exact-power detection | mul-class | $O(n \log n)$ |
| product tree, total size $N$ | $O(N (\log N)^2 \log\log N)$ | $O(N \log N)$ |
| base conversion (to/from string) | $O(n (\log n)^2 \log\log n)$ | $O(n \log n)$ |
| float exp-family `prec` part | $O(n^{3/2} \log n \log\log n)$ | $O(n \log n)$ |
| float log/pow Ziv `prec` part | $O(n (\log n)^2 \log\log n)$ | $O(n \log n)$ |

A product tree costs a $\log$ factor beyond one multiplication at the total size **unless**
the recursion is geometric (factorial's prime-swing is; primorial's flat tree is not). When a
result has $\Theta(n \log n)$ bits (like `product_of_first_n_primes`), substitute that size
into the tree bound — the exponents go up.

## Iterators and generators

Generator functions carry a **per-iteration** block. The standard shape is callee-relative:

```text
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell + T^\prime(i))$
///
/// $M(i) = O(\ell + M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `xs`, and $\ell$ is the number of elements
/// in the $i$th output.
```

- $\ell$ is `len`/`k` for fixed-length generators, or defined as the $i$th output's size.
- Random and hash-based generators use `# Expected complexity per iteration`; b-tree
  variants keep worst-case and pay $\ell \log \ell$.
- Mean parameters are quoted as the quotient: $n$ is
  `mean_length_numerator / mean_length_denominator` (or $O(n/m + 1)$ with both named) — not
  $O(n + m)$.
- An iterator that **clones and returns** its current value costs $\Theta(\ell)$ per
  iteration, where $\ell$ is the output's size. It is never "amortized constant" and never
  $O(i)$; only the increment's carry chain is amortized.
- Unique/distinct-drawing generators note their retry behavior and the hang condition when
  the source can't produce enough distinct values.

## Scratch buffers

- Every `*_scratch_len` formula carries a derivation comment. Related kernels keep one
  master derivation plus pointers.
- The mul/div heartland (toom, fft, mul_low/high/mod, div_exact, div_mod, half_gcd, square,
  gcd_reduced) is guarded by **sentinel canary tests**: fill the scratch with
  `SCRATCH_SENTINEL`, run threshold-straddling input sweeps, measure the high-water mark, and
  check outputs against a value oracle. Harness: `malachite-nz/src/test_util/scratch.rs`;
  examples in `malachite-nz/tests/natural/arithmetic/{mul,div_mod,square,gcd}.rs`. New
  scratch-taking kernels in that family get a canary; elsewhere a written derivation is the
  bar.

## Prose, panics, and examples

- Examples must exercise the **exact impl** being documented: the right operator (an `^=`
  example must not use `|=`, even if the values happen to agree) and the right by-value /
  by-reference form (`x ^= &y` for the `&Self` impl).
- "taking the [`X`] by reference" must name the type actually taken.
- Panics sections list every panic, including those reached through callees.
- `limbs_*` functions are internal: doc-hidden, minimal comments plus GMP/FLINT/MPFR
  provenance lines, no rustdoc examples.
- Public docs use a neutral voice regarding other libraries.
- 100-column limit; reflow comments with `bash ../superfmt.sh` from the crate directory, run
  doctests with `bash ../rundoc.sh`. Unbreakable long lines (links, `//!` docs, macro bodies)
  are exempted in `dylint.toml`'s `long_lines_exceptions`, which self-reports stale entries.

## Pitfalls checklist

Before shipping a new doc block, check it against the classes the audit actually found:

1. **Carry chains**: by-value add/sub worst case runs through the *longer* operand; bit
   operations on negative representations can carry through the whole number no matter how
   small the bit index is. (Comparisons and Natural-side bit sets genuinely are min/index
   bounded — know which side you're on.)
2. **Word algorithms** are $O(n)$ word ops; don't paste bignum bit-complexity onto them.
3. **Macro-uniform claims**: verify the widest type.
4. **Formatting** costs include field width and padding.
5. **Don't copy $T$'s shape into $M$** — memory usually loses a log (or more).
6. **Mirrored variants**: re-derive, don't copy — growth directions and receivers swap.
7. **Precision functions name their input sizes** — never `prec` alone.
8. **Iterator output clones cost $\Theta(\ell)$** every single iteration.
9. **Product trees** cost an extra $\log$ unless the recursion is geometric.
10. **Returned allocations count**: `&self` + clone ≠ constant.
11. Sums/products of many operands: signs enable adversarial carry oscillation
    (`Integer::Sum` is a tight $O(n^2)$; monotone `Natural` sums amortize).
12. If you write a new header variant, wrapping style, or symbol, teach
    `complexity-doc-check.py` about it in the same commit — blocks it can't parse are
    invisible to every future sweep.

## Tooling

- `python3 complexity-doc-check.py` (repo root; also in `additional-lints.sh`): every block
  well-formed, every variable defined and used, no orphaned where-lines, no TODO placeholders.
- `python3 doc-audit-inventory.py`: regenerates the `audit/checklist-*.md` files (one
  checkbox per documented item, tagged with its audit dimensions) if a re-audit is ever
  needed; checked boxes survive regeneration.
- `audit/CAMPAIGN-LOG.md`: the 2026-08 audit's plan, chunk-by-chunk findings log, and
  retrospective — the evidence behind the rules above.
