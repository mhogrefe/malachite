# Porting C functions to Malachite

This guide describes the workflow for porting functions from GMP, FLINT, or MPFR into Malachite,
along with the house conventions, tooling, and known traps. It exists so that collaborators —
human or AI — don't have to rediscover any of this the hard way.

## Licensing

Malachite is LGPL, so deriving code from GMP, FLINT, and MPFR (all LGPL) is fine. Every file
records its provenance in its header: files containing adopted code carry a "Uses code adopted
from the GNU MP Library" (or MPFR/FLINT) block with the FSF copyright. Files without such a block
are original. **Keep this accurate** — the headers are the project's provenance map, and they
determine which code can ever be relicensed or ported to permissively-licensed projects (only
sole-copyright original code can; never the adopted parts).

Citations name the version current at port time, e.g. "This is equivalent to `mpfr_log1p` from
log1p.c, MPFR 4.3.0." Existing functions keep their original citation unless an API change or
major improvement warrants re-porting.

## The workflow

1. **Translate the C code to Rust, nearly 1-to-1, in safe Rust.** Don't optimize yet. For each C
   function call, search Malachite for an existing translation — look for the "This is equivalent
   to" comments. If a callee hasn't been translated, translate it first (bottom-up).
   - Pointer manipulation becomes slice manipulation: `split_at`, `split_at_mut`, and the
     `split_into_chunks!`/`split_into_chunks_mut!` macros for many equal-length pieces. For
     complex pointer arithmetic, use one big slice with `usize` offsets.
   - `goto`s: first translate with the gotos and labels left as comments. Forward gotos become
     `if` statements (a flag named like `goto_foo` if needed); backward gotos become loops.
2. **Write demos** (see `bin_util/demo_and_bench`). Input generators must match the function's
   valid-input preconditions. Demos reveal typical behavior, wrong or slow output, and infinite
   loops, and they validate the generator choice for reuse in property tests. Run all three
   generation modes, and for `Float` functions always include and stress the `extreme` variants —
   they find exponent-range bugs that nothing else finds.
3. **Write tests**: unit tests with many specific values, plus property tests. See the tooling
   section for how to fill in expected values mechanically.
4. **Manual coverage testing.** Create a debug string (e.g.
   `let debug_s = format!("{:#x} {:#x}", ComparableFloat(x), ComparableFloat(y));`) and insert a
   panic with it at every interesting branch; run the property tests; add each discovered case to
   the unit tests with a `// - <branch description>` comment; remove that panic; repeat. (Batch
   variant: use distinct `eprintln!("COV <branch>: {debug_s}")` markers instead of panics, run
   the property suites once with `--nocapture`, `sort | uniq -c` the markers, and take the first
   exemplar per branch. Estimate the output volume first and pad the estimate 5-10x.) For
   branches that remain unhit, choose among: more iterations, steering the generators, proving
   unreachability, leaving `fail_on_untested_path` (panics under `test_build`, no-op otherwise)
   with a comment explaining why it's unreachable in practice, or dropping the marker for
   unimportant branches.
5. **Debug bad behavior** by comparing the C and Rust sources side by side. Before adding debug
   printing inside bignum loops, estimate the output volume — it is easy to generate tens of
   gigabytes and crash the machine.
6. **Format and lint.** Run `bash ../superfmt.sh` from the crate directory, then clippy with the
   relevant features (`test_build`, `bin_build`, `--tests`) — several house lints (`use_self`,
   `if_not_else`, `missing_const_for_fn`) are stricter than the defaults — and
   `bash additional-lints.sh` for the Dylint house lints. Fix everything mechanical here so the
   manual refactor can focus on naming and structure.
7. **Refactor to idiomatic Rust**, including renaming variables. The tests make this safe.
8. **Document**: prose, LaTeX definition ($f(x,p,m) = \ldots + \varepsilon$ with epsilon bounds),
   special cases, overflow/underflow behavior, worst-case complexity, panic conditions, and
   doctests with *verified* values (run `cargo test -p <crate> --doc <name>`; never guess).
   Follow **DOC-CONVENTIONS.md** (repo root) for the complexity-block format, the rules for
   naming every cost driver (input sizes *and* precision), the house cost cheat-sheet, and the
   pitfalls checklist; `complexity-doc-check.py` enforces the mechanical parts.

## House conventions

- **Float function families**: each operation gets variants for by-value vs by-reference, and for
  explicit `prec`, `rm`, both, or neither, plus `_assign` forms — mirror an existing family like
  `ln`. Many Float functions also take Rational arguments (Rationals can't be losslessly converted
  to Float), but skip those when the function reduces trivially to an existing Rational entry
  point.
- **Lint-enforced conventions** (run `bash additional-lints.sh`; one-time setup
  `cargo install cargo-dylint dylint-link`): the mechanical house conventions are enforced by the
  Dylint lints in `malachite-lints`, with self-explanatory messages; the full list, with each
  rule's rationale and exemption mechanics, lives in `malachite-lints/README.md`. Highlights: use
  the `*_prec*` shorthands rather than explicit `Nearest`; compare bignums with primitives
  directly; shift rather than multiplying or dividing by `power_of_2`, and compare exponents
  (shift-amount literals are `u64` — `x << 1u64`, not `1u32` — matching the library-wide
  convention that bit counts are `u64`; exceptions only where a generic bound or an
  amount-type-specific test fixes another type)
  rather than comparing with `power_of_2`; use the named constants and convert other literals at
  compile time; use the in-place `*_assign*` variants and the by-reference variants rather than
  cloning; use `square()`, `even()`/`odd()`, and `reciprocal()` over their spelled-out
  equivalents; use `split_in_half()` rather than taking `upper_half()` and `lower_half()`
  separately; and keep lines within 100 columns.
- **Conversions that cannot fail**: when the surrounding argument already establishes that a value
  fits its target type, convert with `wrapping_from` (or `as` in a `const` context, where the
  trait methods are unavailable), not `exact_from`. `exact_from` is for conversions whose success
  is a genuine precondition worth asserting; where the fit is already proven — a residue known to
  be smaller than a single-limb modulus, the limb count of an allocated value into `usize` — the
  check is dead weight, and the spelling misleads the reader into thinking the bound is in doubt.
  Record the reason it fits in a comment instead. (This is not lint-enforced: whether a bound is
  known is a fact about the algorithm, not the syntax.)
- **Visibility macros**: `private_test_fn!`/`crate_test_fn!` make internals `pub` under `test_build` so
  tests, demos, and tuning code can call them. For tuner- or test-only entry points that don't fit
  the macros, add explicit `#[cfg(feature = "test_build")] pub fn ..._for_tuning` wrappers.
- **Generators** live in `test_util/generators/` in exhaustive/random/special_random triples with
  a shared validity predicate (e.g. `ln_round_valid`), wrapped in `mod.rs`, with `_rm` variants
  when rug comparison is wanted. Numbers (`..._gen_var_NN`) are assigned by taking the next free
  number — grep `mod.rs` for the current maximum, and beware collisions if multiple branches add
  generators concurrently. An `Exact` rounding mode is valid only for inputs whose result is
  exactly representable; encode that in the predicate.
- **rug as oracle**: every Float function gets `rug_*` comparison helpers in
  `test_util/arithmetic/`, and the property tests cross-check results *and* ternary values against
  MPFR via rug. This is the strongest correctness signal in the whole process; don't skip it.
- **Coverage comments** (`// - !(...) first time`) mark which unit-test case exercises which
  branch. Keep adding them for new coverage cases.
- **`Exact` panics; it does not return `None`.** Every `Float` function taking a `RoundingMode`
  asserts when `Exact` is requested but the result is not exactly representable
  (`from_rational_prec_round`, `get_str`, `to_sci_string`, the whole `pow` family). Keep this even
  in functions that already return `Option` for some other reason, or "not exactly representable"
  and "malformed input" collapse into the same answer. Note the inconsistency: malachite-base's
  `FromSciString` for primitive integers returns `None` in this case instead — changing it to
  panic is a possible future cleanup.
- **An options struct owns its rounding mode.** `ToSciOptions` and `FromSciStringOptions` each
  carry one, so a function taking options must not also take an `rm` parameter; there is no
  `..._with_options_prec_round`. (`Rational` documents that it ignores the `FromSciStringOptions`
  rounding mode, because nothing it produces needs rounding. `Float` does not have that luxury and
  must honor it.)

## String conversion

Both directions have the same three-layer shape: one MPFR-ported numeric engine, with two
front-ends over it — one reproducing MPFR's grammar, one reproducing Malachite's — because the two
grammars genuinely conflict while the arithmetic underneath does not.

    Float -> String                            String -> Float
    get_str.rs      (mpfr_get_str)             set_str.rs       (parsed_string_to_mpfr)
     |- format_float.rs   (printf grammar)      |- strtofr.rs         (parse_string + entry points)
     \- to_sci.rs         (ToSciOptions)        \- from_sci_string.rs (FromSciStringOptions)
         \- to_string.rs  (Display, {:x}, ...)      \- from_string.rs  (FromStr, FromStringBase)

- **The grammars conflict, but over digits never silently.** Malachite's `preprocess_sci_string`
  treats `e`/`E` as an exponent marker in every base, requiring an explicit `+`/`-` after it when
  the base is 15 or greater (to disambiguate from the digit `e`). MPFR accepts `e`/`E` only when
  the base is 10 or less, uses `@` for every base, adds `p`/`P` binary exponents in bases 2 and 16,
  and accepts `0x`/`0b` prefixes and the names `nan`/`inf`/`infinity`. Checked over every base from
  2 to 36, there is **no** digit string that both grammars accept and read as different values:
  every such divergence is one side accepting and the other rejecting. That is what makes two
  front-ends safe. The most visible case is Malachite's own hex output, `0x1.0E+25#1` — MPFR cannot
  parse it at all, since `E` is a hex digit there.
- **The one silent disagreement is over the names of the special values.** MPFR reads
  `nan`/`inf`/`infinity` case-insensitively only up to base 16 — the last base in which `i`, worth
  18, is not a digit — precisely so that a name can never also be a digit string; above that only
  the delimited `@nan@` and `@inf@` are read. Malachite reads `NaN` and `Infinity` in every base,
  because that is what `Display` writes and a `Float` has to be readable from its own output. So
  from base 24 up (`n` is 23) `NaN` is a special to Malachite and a digit string to MPFR, and
  likewise from base 35 up (`y` is 34) for `Infinity`; `-Infinity` in base 35 is the counterexample
  that found this. Those two, in those bases, are the whole of the overlap — the lowercase and
  `@`-delimited spellings are always rejected by Malachite, so they cannot disagree. The
  cross-check in `from_sci_string_properties` excludes exactly this class, and nothing more.
- **The semantics are shared, so a naive reference implementation exists.** MPFR's Ziv loop
  computes exactly "the exact value of the string, rounded once to the target precision". Verified
  against rug over 24000 cases (bases 2/3/8/10/16/36 × precisions 1/2/10/53/100 × all five rounding
  modes) plus the overflow and underflow boundaries: zero mismatches in value, sign of zero, *and*
  ternary. So `Float::from_rational_prec_round(Rational::from_sci_string(s), prec, rm)` is a
  correct oracle for property tests — but not a correct implementation. It is O(exponent): parsing
  `1e100000000` that way takes 297 ms and builds a 40 MB integer, against 10 µs for MPFR, which is
  O(log exponent) because `mpfr_mpn_exp` truncates b^e to the working precision. Beware of timing
  this with sparse values — a big all-zero allocation is nearly free until it is touched (1 GB
  allocates in 9 µs, faults in over 268 ms), so use a dense case like `10^e`, which is ~35% ones.
- **The engine's parts are already ported.** `parsed_string_to_mpfr` needs `mpn_set_str`
  (`limbs_from_digits_small_base`), `mpfr_mpn_exp` (`limbs_float_exp`), `mpfr_round_p`
  (`round_helper_2`), and `mpfr_round_raw` (`round_helper_raw`) — all in
  `malachite-nz/src/natural/arithmetic/float/` (`exp.rs` and `round.rs`). Watch the two live FIXMEs in `strtofr.c`:
  bits dropped by `mpn_rshift` are not counted in the error analysis, and `MPFR_SADD_OVERFLOW` is
  called with bounds the macro does not support. Probe both deliberately in step 4.
- **Precision comes from the string, and cannot be inferred from the digit count alone.**
  `0xff.0#8` is three hex digits (12 bits) at precision 8, and `0x1.0#1` is precision 1, so the
  `#p` suffix that `ComparableFloat` emits is load-bearing; it is what makes the round trip exact,
  in every base. Where a bare string has no `#p`, infer `ceil(n log2(b))` bits from the n
  significant digits — that is `ceil_mul(n, b, 0)`, the same `MPFR_L2B` table `get_str_ndigits`
  uses in the other direction — and then shrink to the minimal precision if the value happens to
  be exactly representable in fewer bits. The shrink step makes bare literals agree with
  `Float::from`: `"1.5"` gives precision 2 and `"255"` gives 8, matching `Float::from(1.5)` and
  `Float::from(255)`. The cap is what keeps `"1e100000000"` at 4 bits instead of its exact 232
  million. This rule never under-estimates (verified for precisions 1 to 2000 in six bases;
  overshoot is 0 bits in base 2, at most 7 in base 10), but it is coarse for short inputs —
  `"0.1"` yields 4 bits, i.e. 0.1015625. Document that surprise explicitly, the way
  `Rational::from_sci_string_simplest` documents its own.
- **API shape.** Parsing rounds, so it follows the usual pair: `_prec_round` takes the mode and
  `_prec` assumes `Nearest`, both returning the ternary — `from_sci_string_prec_round(s, prec, rm)`,
  `from_sci_string_prec(s, prec)`, and `from_sci_string_with_options_prec(s, options, prec)`, each
  `-> Option<(Float, Ordering)>`, with `None` meaning unparseable only. The MPFR-side entry points
  stay 1-to-1 with C — `strtofr(s, base, prec, rm) -> (Float, Ordering, usize)` reporting the bytes
  consumed, and `set_str(s, base, prec, rm) -> Option<(Float, Ordering)>` requiring the whole
  string — with no `Nearest` shorthand, since `mpfr_strtofr` always takes an explicit `mpfr_rnd_t`.
  `FromStr` and `FromStringBase` are `Nearest`-only, their signatures carrying neither a mode nor a
  ternary; add an inherent `from_string_base_round(base, s, rm)` if the ternary is wanted there
  (no `_prec`, since that form takes its precision from the string).
- **Oracles, in two tiers.** `rug::Float::parse_radix(s, base).complete_round(prec, rm)` is the
  simple one and covers the bulk. It is not raw MPFR, though: rug's own validator rejects the
  `0x`/`0b` prefixes, `p`/`P` exponents, and `nan`/`inf` in bases 11-16, and panics outside bases
  2-36, all of which `mpfr_strtofr` accepts; conversely it accepts interior whitespace and `_`
  separators, which `mpfr_strtofr` does not (it skips leading whitespace only). For those corners
  use an `unsafe extern "C" { fn mpfr_strtofr(...) }` declaration in `tests/`, the same trick
  `tests/conversion/string/format_float.rs` uses for `mpfr_snprintf`.

## Tooling

- `bash ../superfmt.sh` (run from a crate directory): cargo fmt plus automatic comment
  re-wrapping. Gotcha: consecutive `// foo` / `// bar` lines are merged into one paragraph; keep
  lines separate with an empty `//` line between them or by using `// -` bullets.
- `~/rust/test-fixer <crate-dir> <test-filter>`: loops `cargo test --features test_build` (debug
  profile — right for unit tests, fast recompiles), parses each `assert_eq` failure, and replaces
  the expected value with the actual one in the failing file. Limitations: the expected string
  must occur exactly once in the file (non-unique placeholders make it choke), it can't handle
  multi-line expected strings, and it doesn't fix `Ordering` expectations. On a choke, fix that
  assertion manually and restart.
- **Bulk regeneration** (for mirroring a whole sibling test file): temporarily convert each unit
  test closure to *print* its corrected `test(...)` row instead of asserting (keep the rug
  cross-check asserts intact so printed values are MPFR-verified), run once with
  `--nocapture --test-threads 1`, splice the rows back in order, restore the closures. One run
  replaces dozens of fixer cycles. Trap: libtest prints its `test foo ...` status line without a
  newline, so the first printed row of each test glues to it — handle the first row separately.
- `~/rust/format-long-string`: formats very long expected strings for multi-line embedding.
- Demos: `cargo run --release --features bin_build -p <crate> -- -l <limit> -m <mode> -d <demo>`
  with modes `exhaustive`, `random`, `special_random`.

## Testing notes

- Run test suites with `--release` and a module filter; debug-profile property tests over many or
  large inputs are unusably slow (unit tests are fine in debug). The full suite takes hours.
- **Always run extreme-input tests with `--release`** — this includes unit-test rows, not just
  property tests. Extreme Rationals (e.g. `Rational::power_of_2(i64::from(Float::MAX_EXPONENT))`)
  carry multi-megabyte numerators/denominators, and extreme Floats trigger high working
  precisions; in debug mode these rows can stall a test run indefinitely.
- Cap builds at `-j 4` (full parallelism has OOM'd the machine). Concurrent benchmarks contaminate
  each other's timings, so give the user a heads-up before benchmarking.

## Verification checklist

Run through this for every new or changed function in the `Float` transcendental/log/exp families
(and use judgment elsewhere). These items encode the failure modes that most reliably slip past
general-case tests — every one of them corresponds to a real bug class found in the 2026 audits.
(The documentation-side counterpart is the pitfalls checklist in DOC-CONVENTIONS.md.)

- **Handle exact results before Ziv loops.** Enumerate the inputs whose results are exactly
  representable (integer arguments, powers of the base, commensurable base/argument pairs) and
  dispatch them before the loop: a Ziv loop can never certify an exact result and will hang on
  one. Property tests should assert that an `Equal` ternary implies rounding-mode invariance.
- **Sweep the exponent-range boundaries.** Test inputs *and* results near `MIN_EXPONENT` and
  `MAX_EXPONENT` (dyadic sweeps work well), cross-checked against rug — rug's default exponent
  range equals `Float`'s, so it is a faithful overflow/underflow oracle. Where rug is impractical
  (it computes some deep regimes at full precision, taking hours), use the rational
  rounding-certificate pattern instead: bracket the exact value between rationals via an
  independent computation path and check the defining inequalities of correct rounding for each
  rounding mode.
- **Treat precisions up to `MAX_EXPONENT` as reachable.** A prec of ~2^30 is testable by value
  (the result is a ~128 MB `Float`); include such a case whenever the algorithm compares `prec`
  against exponent magnitudes. Also test prec = 1: `working_prec - k` margins paired with
  too-small initial slack wrap silently in release builds.
- **Approximations never decide.** An `f64` estimate (`approx_log` and friends) may only choose
  what to try first; every definite branch — overflow, underflow, exactness — needs an exact
  bound check.
- **Extreme generators are load-bearing.** If an extreme-variant demo or property test OOMs or
  hangs, fix the algorithm or the test harness; never delete the coverage. The extreme cases are
  where the real bugs live.
- **Audit `Float` shifts near the range edges.** Plain `<<`/`>>` ignore the rounding mode at the
  exponent-range boundary; a shift whose result can reach the boundary in a rounding path needs
  `shl_prec_round`/`shl_prec_round_assign_helper`. But intermediate directed rounding (e.g.
  `Floor` bounds inside a Ziv loop) is often deliberate — understand each case rather than
  blanket-rewriting.
- **Compose extreme-regime machinery rather than reimplementing it.** When a function's hard
  regime reduces to a sibling's (e.g. $e^x - 1 = 2^{x/\ln 2} - 1$), bracket the transformed
  input between dyadic `Float`s and squeeze through the sibling's correctly-rounded function;
  its deep-regime handling comes along for free.
- **Don't trust green against observation.** If a clean test, lint, or sweep result contradicts
  something directly visible in the code, treat the checker as broken and debug *it* first;
  stale caches, vacuous matching, and over-broad exemptions all produce false cleans.
- **Never weaken a test to make a failure go away** — no deleted extreme rows, no loosened
  asserts. If the test itself is wrong, fix it and say so explicitly.
- **Run `bash additional-lints.sh` before handing work off**; the lints enforce the mechanical
  conventions so review can focus on the algorithmic ones.
- **Add a `CHANGELOG.md` entry when an arc lands.** Every user-visible addition, behavioral
  change, or notable performance win gets a line in the unreleased section at the top of the
  repo-root `CHANGELOG.md`, under its crate; breaking or behavior-changing items also go in the
  release's "Breaking and behavioral changes" list. Writing the line is part of finishing the
  work (step 7 for ports), not something to reconstruct from git at release time.

## Known traps

### MPFR-specific

- **Extended exponent range**: MPFR computes intermediates with widened emin/emax
  (`MPFR_SAVE_EXPO_MARK` ... `mpfr_check_range`), so its intermediate sums, products, and logs
  cannot overflow or underflow. Malachite Floats have no such mechanism. Every port must
  explicitly handle intermediate `±Infinity` and `0` — e.g. in `ln_1_plus_x`, `1 + x` can
  overflow to `+Infinity` (use `ln(x)` instead) or in principle underflow to zero (exact
  `Rational` fallback). Extreme-float demos and property tests find these within a few thousand
  iterations.
- **`MPFR_RNDF` (faithful rounding)** has no Malachite equivalent; use `Nearest`, which is
  strictly more accurate, leaving the error analyses valid.
- **`MPFR_FAST_COMPUTE_IF_SMALL_INPUT`** is a wrapper around `mpfr_round_near_x`, ported as
  `float_round_near_x` in `malachite-float/src/arithmetic/round_near_x.rs`. Its argument order is
  `(y, v, err1, err2, dir, rnd, extra)` with `err = err1 + err2`, gated on `err1 > 0` and
  `err > prec + 1`.
- **Ziv loops** (`MPFR_ZIV_INIT`/`NEXT`) follow the house pattern instead of MPFR's exact growth
  schedule:
  `increment = Limb::WIDTH; ...; working_prec += increment; increment = working_prec >> 1`, with
  `float_can_round` as the exit test.
- **Order asserts to match C control flow**: e.g. an `assert_ne!(rm, Exact)` belongs *after* the
  special-value and domain checks, since `Exact` is valid for inputs with exactly representable
  results (specials, domain-boundary values).

### Malachite-Float semantics

- **`Float::increment`/`decrement` do not preserve precision** when crossing a power of 2; they
  are not substitutes for `mpfr_nexttoinf`/`mpfr_nexttozero`. Fixed-precision neighbor-stepping
  must account for the spacing halving below a power of 2, ulp-underflow at `MIN_EXPONENT`
  (result: signed zero), and overflow at `MAX_EXPONENT` (result: infinity). A robust formulation:
  multiply by $1 \pm 2^{-(p+1)}$ and round directionally at precision $p$ (see
  `step_away_from_zero`/`step_toward_zero` in `round_near_x.rs`).
- **`Float::ulp()` returns `None`** when the ulp itself falls outside the exponent range, even
  though neighboring values exist; don't unwrap it near the range edges.
- **Ternary values are part of the contract**: `Equal` means the result is exactly the
  mathematical value. A coincidentally-exact intermediate rounding must not produce `Equal` for a
  transcendental result; the property tests check this against rug.
- **`Float`'s `Display` renders every value**, including extreme magnitudes (it is built on
  `get_str`, which is why `4.6e301029995` prints fine). It shows the round-trip digit count for
  the value's precision — `1 + ceil(p log10(2))` significant digits, trailing zeros included — so
  the same value at different precisions prints differently, and a printed string alone does not
  pin down a `Float`.
- **Label a `Float` with both forms in tests**: the decimal `Display` string and, alongside it, the
  `{:#x}` `ComparableFloat` form, which is exact and carries the precision (`0x1.8#2`). By
  convention the hex argument is named after the decimal one plus `_hex` (`out` / `out_hex`), and
  the hex is the source of truth: `parse_hex_string` reads it, and a decimal expectation can always
  be recomputed from it. Debug output should likewise print the hex form.
