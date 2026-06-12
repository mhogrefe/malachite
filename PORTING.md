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
6. **Refactor to idiomatic Rust**, including renaming variables. The tests make this safe. Run
   clippy with the relevant features (`test_build`, `bin_build`, `--tests`) — several house lints
   (`use_self`, `if_not_else`, `missing_const_for_fn`) are stricter than the defaults.
7. **Document**: prose, LaTeX definition ($f(x,p,m) = \ldots + \varepsilon$ with epsilon bounds),
   special cases, overflow/underflow behavior, worst-case complexity, panic conditions, and
   doctests with *verified* values (run `cargo test -p <crate> --doc <name>`; never guess).

## House conventions

- **Float function families**: each operation gets variants for by-value vs by-reference, and for
  explicit `prec`, `rm`, both, or neither, plus `_assign` forms — mirror an existing family like
  `ln`. Many Float functions also take Rational arguments (Rationals can't be losslessly converted
  to Float), but skip those when the function reduces trivially to an existing Rational entry
  point.
- **Use the shorthand variants**: in algorithm bodies, `x.ln_prec_ref(p)` rather than
  `x.ln_prec_round_ref(p, Nearest)` — `additional-lints.py` enforces this. Three contexts keep the
  explicit form: the shorthand's own defining delegation, operator/assign trait impls (`fn add`,
  `fn ln_assign`, ...), and doc examples demonstrating the `_prec_round*` function itself.
- **Visibility macros**: `pub_test!`/`pub_crate_test!` make internals `pub` under `test_build` so
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
- Heavy builds and all benchmarks should respect the machine-wide bench lock (see CLAUDE.md /
  `perf/bench-lock.sh` where present); concurrent timing runs contaminate each other.

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
  schedule: `increment = Limb::WIDTH; ...; working_prec += increment; increment = working_prec >> 1`,
  with `float_can_round` as the exit test.
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
- Float `to_string` currently renders extreme-magnitude values as `too_big`/`too_small`; use the
  `{:#x}` `ComparableFloat` debug format in tests and debug strings.
