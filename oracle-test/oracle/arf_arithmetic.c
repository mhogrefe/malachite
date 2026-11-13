/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the arithmetic of Malachite's BigExponentFloat against FLINT's arf_t. A BigExponentFloat
    is written in Malachite's exact hexadecimal format, and a slice as `[A, B, ...]`. The mode
    `arf_arithmetic` accepts these lines:

    - `-X = Z`, `X.abs() = Z`, and `X.sign() = O`, against arf_neg, arf_abs, and arf_sgn.
    - `X << K = Z` and `X >> K = Z`, for any integer K, against arf_mul_2exp_fmpz.
    - The exact operations `X + Y = Z`, `X - Y = Z`, `X * Y = Z`, `X.square() = Z`,
      `X.add_mul(Y, W) = Z`, `X.sub_mul(Y, W) = Z`, `X.mul_add_mul(Y, V, W) = Z`,
      `X.mul_sub_mul(Y, V, W) = Z`, `sum([...]) = Z`, `product([...]) = Z`, and
      `dot([...], [...]) = Z`.
    - The rounded operations `X.op_prec_round(args, p, rm) = (Z, O)` and
      `X.op_prec(args, p) = (Z, O)` for op among add, sub, mul, div, square, reciprocal, sqrt,
      add_mul, sub_mul, mul_add_mul, and mul_sub_mul, and `op_prec_round(slices, p, rm) = (Z, O)`
      and `op_prec(slices, p) = (Z, O)` for op among sum, product, and dot.
    - The rounded operations with a Rational Q, written `n/d` or `n`:
      `X.op_prec_round(Q, p, rm) = (Z, O)` and `X.op_prec(Q, p) = (Z, O)` for op among
      add_rational, sub_rational, mul_rational, and div_rational, and
      `BigExponentFloat::rational_div_big_exponent_float_prec_round(Q, X, p, rm) = (Z, O)` and
      `BigExponentFloat::rational_div_big_exponent_float_prec(Q, X, p) = (Z, O)` for Q / X.
    - The roots and powers `X.op_prec_round(p, rm) = (Z, O)` and `X.op_prec(p) = (Z, O)` for op
      among reciprocal_sqrt and cbrt; `X.op_prec_round(K, p, rm) = (Z, O)` and
      `X.op_prec(K, p) = (Z, O)` for op among root_u, root_s, pow_u, pow_s, and pow_integer, with an
      integer K; `X.hypot_prec_round(Y, p, rm) = (Z, O)` and `X.hypot_prec(Y, p) = (Z, O)`; and the
      exact power `X.pow(K) = Z`.

    The FLINT references are arf_add, arf_sub, arf_mul, arf_div, arf_ui_div with numerator 1,
    arf_sqrt, arf_addmul, arf_submul, and arf_sum. The fused operations on two products and the
    dot product form their products exactly with arf_mul and then round once with arf_add,
    arf_sub, or arf_sum; a product of a slice is formed exactly and rounded with arf_set_round.
    The exact operations use precision ARF_PREC_EXACT, and exact sums are chains of exact
    additions.

    FLINT has no arithmetic between arf_t and fmpq_t values, so the operations with Q = n/d are
    written as one division, rounded by arf_div, of values formed exactly: X * Q = (X * n) / d,
    X / Q = (X * d) / n, Q / X = n / (X * d), and X + Q = (X * d + n) / d. A dyadic Q is added with
    arf_add directly. When X and Q are more than 2^20 bits apart in magnitude, the exact sum
    X * d + n would be too long to form, so the smaller of the two is first replaced by a short
    value that rounds the same way (see add_rational); the conditions this relies on are checked,
    and the oracle aborts if they fail.

    FLINT reports only whether a rounded result is inexact, so the ordering O is derived here: an
    exact result gives `Equal`; `Floor` and `Ceiling` round in a known direction; `Down` and `Up`
    round toward or away from zero, so the direction follows from the result's sign; and a
    `Nearest` result is compared with the same operation rounded toward zero, whose direction is
    known. This needs no exact result, which, for a sum of values whose exponents are far apart,
    could be enormous. `Exact` requires the operation to be exact.

    The reciprocal square root is arf_rsqrt. FLINT's arf_root takes only positive orders and gives
    NaN for every negative X, so roots are computed as arf_root does it, but with MPFR's
    mpfr_rootn_ui or mpfr_rootn_si: with X = m * 2^e and e = q|K| + r for 0 <= r < |K|,
    X^(1/K) = (m * 2^r)^(1/K) * 2^(q / sign(K)). Powers are computed with mpfr_pow_si after X is
    scaled to exponent 0: X^K = (X / 2^e)^K * 2^(eK), and, for |K| >= 2^60, as the limit of
    arb_pow_fmpz balls (see apply_pow_huge). MPFR's exponent range is widened to its
    maximum, about 2^62, for these; the oracle aborts if an exponent passed to MPFR is out of range.
    The hypotenuse is the square root of the exact sum of squares, rounded by arf_sqrt; when the
    exponents of X and Y are more than 2^20 apart, the smaller value is first replaced by a short
    value that rounds the same way (see apply_hypot), with the conditions this relies on checked.
    An exact power is formed by repeated exact multiplication. The special cases of roots and powers
    of zero and infinity follow Malachite's documentation: without a negative zero, 0^(1/K) and 0^K
    are NaN for negative odd K, and infinity for negative even K.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

/* mpfr.h comes first, so that arf.h declares its MPFR conversions. */
#include <mpfr.h>

#include <flint/arb.h>
#include <flint/arf.h>
#include <flint/fmpq.h>
#include <flint/fmpz.h>

#include "oracle.h"

static long checked;

static int
report(int line_number, const char * what)
{
    flint_printf("error in arf_arithmetic test, line %d: %s\n", line_number, what);
    return 1;
}

/* Splits `line` at the first occurrence of `sep`, in place, returning the text after it, or NULL. */
static char *
split(char * line, const char * sep)
{
    char * p = strstr(line, sep);
    if (p == NULL)
        return NULL;
    *p = '\0';
    return p + strlen(sep);
}

/* Parses a rounding mode; `*exact` is set for `Exact`, which is checked with rounding down. */
static int
parse_rm(const char * s, arf_rnd_t * rnd, int * exact)
{
    *exact = 0;
    if (strcmp(s, "Floor") == 0)
        *rnd = ARF_RND_FLOOR;
    else if (strcmp(s, "Ceiling") == 0)
        *rnd = ARF_RND_CEIL;
    else if (strcmp(s, "Down") == 0)
        *rnd = ARF_RND_DOWN;
    else if (strcmp(s, "Up") == 0)
        *rnd = ARF_RND_UP;
    else if (strcmp(s, "Nearest") == 0)
        *rnd = ARF_RND_NEAR;
    else if (strcmp(s, "Exact") == 0)
    {
        *rnd = ARF_RND_DOWN;
        *exact = 1;
    }
    else
        return 1;
    return 0;
}

static const char *
ordering_name(int c)
{
    return c < 0 ? "Less" : c > 0 ? "Greater" : "Equal";
}

typedef enum
{
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    OP_SQUARE,
    OP_RECIPROCAL,
    OP_SQRT,
    OP_ADD_MUL,
    OP_SUB_MUL,
    OP_MUL_ADD_MUL,
    OP_MUL_SUB_MUL,
    OP_SUM,
    OP_PRODUCT,
    OP_DOT,
    OP_ADD_RATIONAL,
    OP_SUB_RATIONAL,
    OP_MUL_RATIONAL,
    OP_DIV_RATIONAL,
    OP_RATIONAL_DIV,
    OP_RECIPROCAL_SQRT,
    OP_CBRT,
    OP_ROOT_U,
    OP_ROOT_S,
    OP_POW_U,
    OP_POW_S,
    OP_POW_INTEGER,
    OP_HYPOT
} op_kind;

/* An operation and its operands: `xs` holds the values (the receiver first), or, for the slice
   operations, the first slice, and `ys` the second slice of a dot product; `q` holds the Rational
   operand of the operations that have one. */
typedef struct
{
    op_kind op;
    arf_ptr xs;
    slong n;
    arf_ptr ys;
    slong m;
    fmpq_t q;
} call_t;

static void
call_init(call_t * c)
{
    c->xs = NULL;
    c->ys = NULL;
    c->n = 0;
    c->m = 0;
    fmpq_init(c->q);
}

static void
call_clear(call_t * c)
{
    if (c->xs != NULL)
        _arf_vec_clear(c->xs, c->n);
    if (c->ys != NULL)
        _arf_vec_clear(c->ys, c->m);
    fmpq_clear(c->q);
}

/* Parses a Rational, `n/d` or `n`, in lowest terms with a positive denominator. */
static int
parse_rational(const char * s, fmpq_t q)
{
    const char * slash = strchr(s, '/');
    if (slash == NULL)
    {
        fmpz_one(fmpq_denref(q));
        return *s == '\0' || fmpz_set_str(fmpq_numref(q), s, 10);
    }
    size_t n = slash - s;
    char * num = flint_malloc(n + 1);
    memcpy(num, s, n);
    num[n] = '\0';
    int result = n == 0 || fmpz_set_str(fmpq_numref(q), num, 10)
        || fmpz_set_str(fmpq_denref(q), slash + 1, 10) || fmpz_sgn(fmpq_denref(q)) <= 0;
    flint_free(num);
    if (result == 0)
    {
        fmpq_t r;
        fmpq_init(r);
        fmpq_set(r, q);
        fmpq_canonicalise(r);
        result = !fmpq_equal(r, q);
        fmpq_clear(r);
    }
    return result;
}

/* Gaps between magnitudes, in bits, beyond which add_rational uses a stand-in. */
#define FAR (WORD(1) << 20)

/* x + n/d, for finite nonzero x, and n/d with d not a power of 2, rounded to `prec` bits
   with `rnd`; formed as (x * d + n) / d, unless x and n/d are far apart:

   - If |x| is more than 2^FAR times |n/d|, which is less than 2^k for k = bits(n) - bits(d) + 1,
     then, with g the smaller of x's lowest bit position and e - prec - 2 for x's exponent e, every
     value at which rounding x + n/d to `prec` bits could change is a multiple of 2^g, and so is x.
     If k < g, then x + n/d and x + s, for s = ±2^(k - 1) with the sign of n, both lie strictly
     between x and the next multiple of 2^g on that side, and round alike; so x + s is rounded.
   - If |n/d| is more than 2^FAR times |x|, then with j = bits(n) - bits(d) - prec - 3, which is
     at most floor(log_2(|n/d|)) - prec - 2, every value at which rounding n/d + x could change is a
     multiple of 2^j. Since the odd part of d divides neither n nor n * 2^-j, n/d is at least
     2^h away from every multiple of 2^j, for h = min(j, 0) - bits(d). If |x| < 2^h, then x + n/d
     and s + n/d, for s = ±2^(h - 1) with x's sign, lie strictly between n/d and the nearest
     multiple of 2^j on that side, and round alike; so (s * d + n) / d is rounded.

   Returns -1 if neither case applies. The oracle aborts if the condition k < g, or |x| < 2^h,
   fails; both hold whenever the magnitudes are that far apart and the operands have fewer than
   about 2^19 bits. */
static int
add_rational_far(arf_t z, const arf_t x, const fmpz_t n, const fmpz_t d, slong prec,
                 arf_rnd_t rnd)
{
    fmpz_t gap, k, g, low, t;
    fmpz_init(gap);
    fmpz_init(k);
    fmpz_init(g);
    fmpz_init(low);
    fmpz_init(t);
    int inexact = -1;
    slong n_bits = fmpz_bits(n);
    slong d_bits = fmpz_bits(d);
    /* gap = e - log_2(|n/d|), roughly: the exponent of x minus bits(n) - bits(d). */
    fmpz_sub_si(gap, ARF_EXPREF(x), n_bits - d_bits);
    if (fmpz_cmp_si(gap, FAR) > 0)
    {
        fmpz_set_si(k, n_bits - d_bits + 1);
        fmpz_sub_ui(low, ARF_EXPREF(x), arf_bits(x));
        fmpz_sub_si(g, ARF_EXPREF(x), prec + 2);
        if (fmpz_cmp(low, g) < 0)
            fmpz_set(g, low);
        if (fmpz_cmp(k, g) >= 0)
            flint_abort();
        arf_t s;
        arf_init(s);
        arf_one(s);
        fmpz_sub_ui(t, k, 1);
        arf_mul_2exp_fmpz(s, s, t);
        if (fmpz_sgn(n) < 0)
            arf_neg(s, s);
        inexact = arf_add(z, x, s, prec, rnd);
        arf_clear(s);
    }
    else if (fmpz_cmp_si(gap, -FAR) < 0)
    {
        slong j = n_bits - d_bits - prec - 3;
        slong h = (j < 0 ? j : 0) - d_bits;
        if (fmpz_cmp_si(ARF_EXPREF(x), h) > 0)
            flint_abort();
        arf_t s, dd, nn;
        arf_init(s);
        arf_init(dd);
        arf_init(nn);
        arf_one(s);
        arf_mul_2exp_si(s, s, h - 1);
        if (arf_sgn(x) < 0)
            arf_neg(s, s);
        arf_set_fmpz(dd, d);
        arf_set_fmpz(nn, n);
        arf_mul(s, s, dd, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_add(s, s, nn, ARF_PREC_EXACT, ARF_RND_DOWN);
        inexact = arf_div(z, s, dd, prec, rnd);
        arf_clear(s);
        arf_clear(dd);
        arf_clear(nn);
    }
    fmpz_clear(gap);
    fmpz_clear(k);
    fmpz_clear(g);
    fmpz_clear(low);
    fmpz_clear(t);
    return inexact;
}

/* The operations with a Rational operand n/d, as described above. */
static int
apply_rational(const call_t * c, arf_t z, slong prec, arf_rnd_t rnd)
{
    arf_srcptr x = c->xs;
    fmpz_t n_neg;
    fmpz_init(n_neg);
    const fmpz * n = fmpq_numref(c->q);
    const fmpz * d = fmpq_denref(c->q);
    if (c->op == OP_SUB_RATIONAL)
    {
        fmpz_neg(n_neg, n);
        n = n_neg;
    }
    arf_t nn, dd, t;
    arf_init(nn);
    arf_init(dd);
    arf_init(t);
    arf_set_fmpz(nn, n);
    arf_set_fmpz(dd, d);
    int inexact = -1;
    switch (c->op)
    {
        case OP_MUL_RATIONAL:
            arf_mul(t, x, nn, ARF_PREC_EXACT, ARF_RND_DOWN);
            inexact = arf_div(z, t, dd, prec, rnd);
            break;
        case OP_DIV_RATIONAL:
            arf_mul(t, x, dd, ARF_PREC_EXACT, ARF_RND_DOWN);
            inexact = arf_div(z, t, nn, prec, rnd);
            break;
        case OP_RATIONAL_DIV:
            arf_mul(t, x, dd, ARF_PREC_EXACT, ARF_RND_DOWN);
            inexact = arf_div(z, nn, t, prec, rnd);
            break;
        default:
            if (fmpz_val2(d) + 1 == fmpz_bits(d))
            {
                /* d is a power of 2, so n/d is an arf_t. */
                arf_div(t, nn, dd, ARF_PREC_EXACT, ARF_RND_DOWN);
                inexact = arf_add(z, x, t, prec, rnd);
            }
            else if (!arf_is_finite(x) || arf_is_zero(x)
                     || (inexact = add_rational_far(z, x, n, d, prec, rnd)) < 0)
            {
                arf_mul(t, x, dd, ARF_PREC_EXACT, ARF_RND_DOWN);
                arf_add(t, t, nn, ARF_PREC_EXACT, ARF_RND_DOWN);
                inexact = arf_div(z, t, dd, prec, rnd);
            }
    }
    arf_clear(nn);
    arf_clear(dd);
    arf_clear(t);
    fmpz_clear(n_neg);
    return inexact;
}

static mpfr_rnd_t
mpfr_rnd(arf_rnd_t rnd)
{
    switch (rnd)
    {
        case ARF_RND_DOWN: return MPFR_RNDZ;
        case ARF_RND_UP: return MPFR_RNDA;
        case ARF_RND_FLOOR: return MPFR_RNDD;
        case ARF_RND_CEIL: return MPFR_RNDU;
        default: return MPFR_RNDN;
    }
}

/* The finite nonzero `x` divided by 2^s, as an MPFR value of the same precision. */
static void
arf_to_mpfr_scaled(mpfr_t f, const arf_t x, const fmpz_t s)
{
    arf_t t;
    fmpz_t ns;
    arf_init(t);
    fmpz_init(ns);
    fmpz_neg(ns, s);
    arf_mul_2exp_fmpz(t, x, ns);
    if (fmpz_cmp_si(ARF_EXPREF(t), mpfr_get_emax()) > 0
        || fmpz_cmp_si(ARF_EXPREF(t), mpfr_get_emin()) < 0)
        flint_abort();
    mpfr_init2(f, FLINT_MAX(arf_bits(t), 1));
    arf_get_mpfr(f, t, MPFR_RNDN);
    arf_clear(t);
    fmpz_clear(ns);
}

/* The MPFR value `f`, multiplied by 2^s. */
static void
mpfr_to_arf_scaled(arf_t z, const mpfr_t f, const fmpz_t s)
{
    arf_set_mpfr(z, f);
    arf_mul_2exp_fmpz(z, z, s);
}

/* x^(1/k), as described above. */
static int
apply_root(arf_srcptr x, const fmpz_t k, arf_t z, slong prec, arf_rnd_t rnd)
{
    int odd = fmpz_is_odd(k);
    int negative_k = fmpz_sgn(k) < 0;
    if (fmpz_is_zero(k) || arf_is_nan(x))
        arf_nan(z);
    else if (arf_is_inf(x))
    {
        if (arf_sgn(x) < 0 && !odd)
            arf_nan(z);
        else if (negative_k)
            arf_zero(z);
        else
            arf_set(z, x);
    }
    else if (arf_is_zero(x))
    {
        if (!negative_k)
            arf_zero(z);
        else if (odd)
            arf_nan(z);
        else
            arf_pos_inf(z);
    }
    else if (arf_sgn(x) < 0 && !odd)
        arf_nan(z);
    else
    {
        fmpz_t k_abs, q, r, s;
        fmpz_init(k_abs);
        fmpz_init(q);
        fmpz_init(r);
        fmpz_init(s);
        fmpz_abs(k_abs, k);
        fmpz_fdiv_qr(q, r, ARF_EXPREF(x), k_abs);
        if (fmpz_cmp_si(r, mpfr_get_emax()) > 0)
        {
            fmpz_sub(r, r, k_abs);
            fmpz_add_ui(q, q, 1);
        }
        fmpz_mul(s, q, k_abs);
        mpfr_t xf, zf;
        arf_to_mpfr_scaled(xf, x, s);
        mpfr_init2(zf, prec);
        int inexact = negative_k ? mpfr_rootn_si(zf, xf, fmpz_get_si(k), mpfr_rnd(rnd))
                                 : mpfr_rootn_ui(zf, xf, fmpz_get_ui(k), mpfr_rnd(rnd));
        if (negative_k)
            fmpz_neg(q, q);
        mpfr_to_arf_scaled(z, zf, q);
        mpfr_clear(xf);
        mpfr_clear(zf);
        fmpz_clear(k_abs);
        fmpz_clear(q);
        fmpz_clear(r);
        fmpz_clear(s);
        return inexact != 0;
    }
    return 0;
}

/* x^n for finite nonzero x and |n| >= 2^60, beyond what MPFR's exponent range allows: an arb_t
   ball containing x^n is computed with arb_pow_fmpz at increasing precisions, until both of its
   endpoints round to the same value. The power is then never exactly representable, unless x is
   plus or minus a power of 2: for positive n it is a dyadic rational with more than 2^60 bits, and
   for negative n it is not a dyadic rational. So no breakpoint can stop the loop, and a power of 2
   gives an exact ball. */
static int
apply_pow_huge(arf_srcptr x, const fmpz_t n, arf_t z, slong prec, arf_rnd_t rnd)
{
    arb_t a;
    arf_t r, lo, hi;
    arb_init(a);
    arf_init(r);
    arf_init(lo);
    arf_init(hi);
    int inexact;
    for (slong wp = prec + fmpz_bits(n) + 64;; wp *= 2)
    {
        arb_set_arf(a, x);
        arb_pow_fmpz(a, a, n, wp);
        arf_set_mag(r, arb_radref(a));
        arf_sub(lo, arb_midref(a), r, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_add(hi, arb_midref(a), r, ARF_PREC_EXACT, ARF_RND_DOWN);
        inexact = arf_set_round(lo, lo, prec, rnd);
        arf_set_round(hi, hi, prec, rnd);
        if (arf_equal(lo, hi))
        {
            if (!arf_is_zero(r))
                inexact = 1;
            arf_set(z, lo);
            break;
        }
    }
    arb_clear(a);
    arf_clear(r);
    arf_clear(lo);
    arf_clear(hi);
    return inexact;
}

/* x^n, as described above. */
static int
apply_pow(arf_srcptr x, const fmpz_t n, arf_t z, slong prec, arf_rnd_t rnd)
{
    int odd = fmpz_is_odd(n);
    int negative_n = fmpz_sgn(n) < 0;
    if (fmpz_is_zero(n))
        arf_one(z);
    else if (arf_is_nan(x))
        arf_nan(z);
    else if (arf_is_inf(x))
    {
        if (negative_n)
            arf_zero(z);
        else if (arf_sgn(x) < 0 && odd)
            arf_neg_inf(z);
        else
            arf_pos_inf(z);
    }
    else if (arf_is_zero(x))
    {
        if (!negative_n)
            arf_zero(z);
        else if (odd)
            arf_nan(z);
        else
            arf_pos_inf(z);
    }
    else
    {
        /* (x / 2^e)^n has an exponent between -|n| and |n| + 1. */
        if (fmpz_bits(n) > 60)
            return apply_pow_huge(x, n, z, prec, rnd);
        fmpz_t s;
        fmpz_init(s);
        mpfr_t xf, zf;
        arf_to_mpfr_scaled(xf, x, ARF_EXPREF(x));
        mpfr_init2(zf, prec);
        int inexact = mpfr_pow_si(zf, xf, fmpz_get_si(n), mpfr_rnd(rnd));
        fmpz_mul(s, ARF_EXPREF(x), n);
        mpfr_to_arf_scaled(z, zf, s);
        mpfr_clear(xf);
        mpfr_clear(zf);
        fmpz_clear(s);
        return inexact != 0;
    }
    return 0;
}

/* sqrt(x^2 + y^2). Infinities give infinity, even with a NaN; then NaN gives NaN; and a zero gives
   the other value's absolute value, rounded. Otherwise the exact sum of squares is rounded by
   arf_sqrt, unless the exponents of x and y are more than 2^20 apart:

   Let |x| be the larger, with exponent e, and let f be y's exponent. The hypotenuse is |x| + d with
   0 < d = y^2 / (sqrt(x^2 + y^2) + |x|) < y^2 / (2|x|) < 2^(2f - e). With p = max(bits(x),
   prec + 1), every value at which rounding to `prec` bits could change between 2^(e - 1) and 2^e
   is a multiple of 2^(e - p), and so is |x|. If 2f <= 2e - p, then d < 2^(e - p), and |x| + d lies
   strictly between |x| and the next such multiple; so does the hypotenuse of x and 2^(g - 1), for
   g = floor((2e - p) / 2), and the two round alike. The oracle aborts if 2f > 2e - p, which holds
   whenever the exponents are that far apart and the operands have fewer than about 2^20 bits. */
static int
apply_hypot(arf_srcptr x, arf_srcptr y, arf_t z, slong prec, arf_rnd_t rnd)
{
    if (arf_is_inf(x) || arf_is_inf(y))
    {
        arf_pos_inf(z);
        return 0;
    }
    if (arf_is_nan(x) || arf_is_nan(y))
    {
        arf_nan(z);
        return 0;
    }
    arf_t t, u;
    arf_init(t);
    arf_init(u);
    int inexact;
    if (arf_is_zero(x) || arf_is_zero(y))
    {
        arf_abs(t, arf_is_zero(x) ? y : x);
        inexact = arf_set_round(z, t, prec, rnd);
    }
    else
    {
        if (fmpz_cmp(ARF_EXPREF(x), ARF_EXPREF(y)) < 0)
        {
            arf_srcptr w = x;
            x = y;
            y = w;
        }
        fmpz_t gap, g;
        fmpz_init(gap);
        fmpz_init(g);
        fmpz_sub(gap, ARF_EXPREF(x), ARF_EXPREF(y));
        arf_set(u, y);
        if (fmpz_cmp_si(gap, FAR) > 0)
        {
            slong p = FLINT_MAX(arf_bits(x), prec + 1);
            fmpz_mul_2exp(g, ARF_EXPREF(x), 1);
            fmpz_sub_si(g, g, p);
            fmpz_fdiv_q_2exp(g, g, 1);
            if (fmpz_cmp(ARF_EXPREF(y), g) > 0)
                flint_abort();
            fmpz_sub_ui(g, g, 1);
            arf_one(u);
            arf_mul_2exp_fmpz(u, u, g);
        }
        arf_mul(t, x, x, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_mul(u, u, u, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_add(t, t, u, ARF_PREC_EXACT, ARF_RND_DOWN);
        inexact = arf_sqrt(z, t, prec, rnd);
        fmpz_clear(gap);
        fmpz_clear(g);
    }
    arf_clear(t);
    arf_clear(u);
    return inexact;
}

/* The exact sum of `len` terms. */
static void
exact_sum(arf_t z, arf_srcptr terms, slong len)
{
    arf_zero(z);
    for (slong i = 0; i < len; i++)
        arf_add(z, z, terms + i, ARF_PREC_EXACT, ARF_RND_DOWN);
}

/* Applies the operation, returning FLINT's inexact flag. */
static int
apply(const call_t * c, arf_t z, slong prec, arf_rnd_t rnd)
{
    arf_srcptr x = c->xs;
    int inexact = 0;
    switch (c->op)
    {
        case OP_ADD: return arf_add(z, x, x + 1, prec, rnd);
        case OP_SUB: return arf_sub(z, x, x + 1, prec, rnd);
        case OP_MUL: return arf_mul(z, x, x + 1, prec, rnd);
        case OP_DIV: return arf_div(z, x, x + 1, prec, rnd);
        case OP_SQUARE: return arf_mul(z, x, x, prec, rnd);
        case OP_RECIPROCAL: return arf_ui_div(z, 1, x, prec, rnd);
        case OP_SQRT: return arf_sqrt(z, x, prec, rnd);
        case OP_ADD_MUL:
            arf_set(z, x);
            return arf_addmul(z, x + 1, x + 2, prec, rnd);
        case OP_SUB_MUL:
            arf_set(z, x);
            return arf_submul(z, x + 1, x + 2, prec, rnd);
        case OP_MUL_ADD_MUL:
        case OP_MUL_SUB_MUL:
        {
            arf_t p, q;
            arf_init(p);
            arf_init(q);
            arf_mul(p, x, x + 1, ARF_PREC_EXACT, ARF_RND_DOWN);
            arf_mul(q, x + 2, x + 3, ARF_PREC_EXACT, ARF_RND_DOWN);
            inexact = c->op == OP_MUL_ADD_MUL ? arf_add(z, p, q, prec, rnd)
                                              : arf_sub(z, p, q, prec, rnd);
            arf_clear(p);
            arf_clear(q);
            return inexact;
        }
        case OP_SUM:
            if (prec == ARF_PREC_EXACT)
            {
                exact_sum(z, x, c->n);
                return 0;
            }
            return arf_sum(z, x, c->n, prec, rnd);
        case OP_PRODUCT:
        {
            arf_t p;
            arf_init(p);
            arf_one(p);
            for (slong i = 0; i < c->n; i++)
                arf_mul(p, p, x + i, ARF_PREC_EXACT, ARF_RND_DOWN);
            inexact = arf_set_round(z, p, prec, rnd);
            arf_clear(p);
            return inexact;
        }
        case OP_ADD_RATIONAL:
        case OP_SUB_RATIONAL:
        case OP_MUL_RATIONAL:
        case OP_DIV_RATIONAL:
        case OP_RATIONAL_DIV:
            return apply_rational(c, z, prec, rnd);
        case OP_RECIPROCAL_SQRT: return arf_rsqrt(z, x, prec, rnd);
        case OP_CBRT:
        {
            fmpz_t three;
            fmpz_init_set_ui(three, 3);
            inexact = apply_root(x, three, z, prec, rnd);
            fmpz_clear(three);
            return inexact;
        }
        case OP_ROOT_U:
        case OP_ROOT_S:
            return apply_root(x, fmpq_numref(c->q), z, prec, rnd);
        case OP_POW_U:
        case OP_POW_S:
        case OP_POW_INTEGER:
            return apply_pow(x, fmpq_numref(c->q), z, prec, rnd);
        case OP_HYPOT: return apply_hypot(x, x + 1, z, prec, rnd);
        default:
        {
            arf_ptr products = _arf_vec_init(c->n);
            for (slong i = 0; i < c->n; i++)
                arf_mul(products + i, x + i, c->ys + i, ARF_PREC_EXACT, ARF_RND_DOWN);
            if (prec == ARF_PREC_EXACT)
                exact_sum(z, products, c->n);
            else
                inexact = arf_sum(z, products, c->n, prec, rnd);
            _arf_vec_clear(products, c->n);
            return inexact;
        }
    }
}

/* The ordering of the rounded result `z` relative to the exact result, as described above. */
static int
rounding_direction(const call_t * c, const arf_t z, int inexact, slong prec, arf_rnd_t rnd)
{
    if (!inexact)
        return 0;
    switch (rnd)
    {
        case ARF_RND_FLOOR: return -1;
        case ARF_RND_CEIL: return 1;
        case ARF_RND_DOWN: return -arf_sgn(z);
        case ARF_RND_UP: return arf_sgn(z);
        default:
        {
            arf_t down;
            arf_init(down);
            apply(c, down, prec, ARF_RND_DOWN);
            int toward_zero = arf_equal(down, z);
            arf_clear(down);
            return toward_zero ? -arf_sgn(z) : arf_sgn(z);
        }
    }
}

static const struct
{
    const char * name;
    op_kind op;
    int arity; /* the number of values, including the receiver, or 0 for a slice operation */
    int rational; /* 1 if a Rational follows the receiver; 2 if a Rational comes first, with no
                     receiver */
} methods[] = {
    {"add", OP_ADD, 2, 0},
    {"sub", OP_SUB, 2, 0},
    {"mul", OP_MUL, 2, 0},
    {"div", OP_DIV, 2, 0},
    {"square", OP_SQUARE, 1, 0},
    {"reciprocal", OP_RECIPROCAL, 1, 0},
    {"sqrt", OP_SQRT, 1, 0},
    {"add_mul", OP_ADD_MUL, 3, 0},
    {"sub_mul", OP_SUB_MUL, 3, 0},
    {"mul_add_mul", OP_MUL_ADD_MUL, 4, 0},
    {"mul_sub_mul", OP_MUL_SUB_MUL, 4, 0},
    {"sum", OP_SUM, 0, 0},
    {"product", OP_PRODUCT, 0, 0},
    {"dot", OP_DOT, 0, 0},
    {"add_rational", OP_ADD_RATIONAL, 1, 1},
    {"sub_rational", OP_SUB_RATIONAL, 1, 1},
    {"mul_rational", OP_MUL_RATIONAL, 1, 1},
    {"div_rational", OP_DIV_RATIONAL, 1, 1},
    {"rational_div_big_exponent_float", OP_RATIONAL_DIV, 1, 2},
    {"reciprocal_sqrt", OP_RECIPROCAL_SQRT, 1, 0},
    {"cbrt", OP_CBRT, 1, 0},
    {"root_u", OP_ROOT_U, 1, 1},
    {"root_s", OP_ROOT_S, 1, 1},
    {"pow_u", OP_POW_U, 1, 1},
    {"pow_s", OP_POW_S, 1, 1},
    {"pow_integer", OP_POW_INTEGER, 1, 1},
    {"hypot", OP_HYPOT, 2, 0},
};

static int
find_op(const char * name)
{
    for (size_t i = 0; i < sizeof(methods) / sizeof(methods[0]); i++)
        if (strcmp(name, methods[i].name) == 0)
            return (int) i;
    return -1;
}

/* Parses a slice `[A, B, ...]` at the start of `s`, returning the text after the `]`, or NULL. */
static char *
parse_slice(char * s, arf_ptr * out, slong * len)
{
    if (*s != '[')
        return NULL;
    char * close = strchr(s, ']');
    if (close == NULL)
        return NULL;
    *close = '\0';
    char * body = s + 1;
    slong n = 0;
    if (*body != '\0')
    {
        n = 1;
        for (char * p = body; (p = strstr(p, ", ")) != NULL; p += 2)
            n++;
    }
    *out = _arf_vec_init(n);
    *len = n;
    char * item = body;
    for (slong i = 0; i < n; i++)
    {
        char * next = split(item, ", ");
        if (arf_parse_base(item, 4, *out + i))
            return NULL;
        item = next;
    }
    return close + 1;
}

/* Fills `c` with the operation `op` and its operands: `receiver`, if not NULL, then the
   comma-separated values or slices of `args`, which must be followed by exactly `tail_count`
   more comma-separated fields. Returns the start of those fields, or NULL. */
static char *
parse_operands(call_t * c, int m, const char * receiver, char * args, int tail_count)
{
    c->op = methods[m].op;
    if (methods[m].rational != 0)
    {
        /* `Q, p[, rm]` after a receiver X, or `Q, X, p[, rm]` */
        char * rest = split(args, ", ");
        if (rest == NULL || parse_rational(args, c->q))
            return NULL;
        if (c->op >= OP_ROOT_U && c->op <= OP_POW_INTEGER && !fmpz_is_one(fmpq_denref(c->q)))
            return NULL;
        c->xs = _arf_vec_init(1);
        c->n = 1;
        if (methods[m].rational == 1)
            return receiver != NULL && arf_parse_base(receiver, 4, c->xs) == 0 ? rest : NULL;
        char * tail = split(rest, ", ");
        return receiver == NULL && tail != NULL && arf_parse_base(rest, 4, c->xs) == 0 ? tail
                                                                                       : NULL;
    }
    if (methods[m].arity == 0)
    {
        char * rest = parse_slice(args, &c->xs, &c->n);
        if (rest == NULL)
            return NULL;
        if (c->op == OP_DOT)
        {
            if (strncmp(rest, ", ", 2) != 0)
                return NULL;
            rest = parse_slice(rest + 2, &c->ys, &c->m);
            if (rest == NULL || c->m != c->n)
                return NULL;
        }
        if (tail_count == 0)
            return *rest == '\0' ? rest : NULL;
        return strncmp(rest, ", ", 2) == 0 ? rest + 2 : NULL;
    }
    slong arity = methods[m].arity;
    c->xs = _arf_vec_init(arity);
    c->n = arity;
    if (receiver == NULL || arf_parse_base(receiver, 4, c->xs))
        return NULL;
    char * item = args;
    for (slong i = 1; i < arity; i++)
    {
        char * next = split(item, ", ");
        if (next == NULL && (i + 1 < arity || tail_count > 0))
            return NULL;
        if (arf_parse_base(item, 4, c->xs + i))
            return NULL;
        item = next;
    }
    if (tail_count == 0)
        return (arity == 1 && *args != '\0') ? NULL : args;
    return arity == 1 ? args : item;
}

/* `[X.]op_prec_round(...) = (Z, O)` or `[X.]op_prec(...) = (Z, O)`, with `receiver` NULL for the
   slice operations; `method` starts at the operation's name. */
static int
check_rounded(const char * receiver, char * method, int line_number)
{
    char * args = split(method, "(");
    char * result = args == NULL ? NULL : split(args, ") = (");
    if (result == NULL)
        return report(line_number, "unrecognized line");
    size_t n = strlen(result);
    if (n == 0 || result[n - 1] != ')')
        return report(line_number, "unrecognized result");
    result[n - 1] = '\0';
    char * o_str = strrchr(result, ',');
    if (o_str == NULL || o_str[1] != ' ')
        return report(line_number, "unrecognized result");
    *o_str = '\0';
    o_str += 2;
    char * z_str = result;
    char * suffix = strstr(method, "_prec");
    if (suffix == NULL)
        return report(line_number, "unrecognized method");
    *suffix = '\0';
    suffix += 5;
    int with_rm;
    if (strcmp(suffix, "_round") == 0)
        with_rm = 1;
    else if (*suffix == '\0')
        with_rm = 0;
    else
        return report(line_number, "unrecognized method");
    int m = find_op(method);
    if (m < 0 || (methods[m].arity == 0 || methods[m].rational == 2) != (receiver == NULL))
        return report(line_number, "unrecognized method");
    call_t c;
    call_init(&c);
    arf_t z, expected;
    arf_init(z);
    arf_init(expected);
    int res;
    char * tail = parse_operands(&c, m, receiver, args, with_rm ? 2 : 1);
    char * rm_str = tail == NULL ? NULL : (with_rm ? split(tail, ", ") : NULL);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    char * end = NULL;
    unsigned long long prec = tail == NULL ? 0 : strtoull(tail, &end, 10);
    if (tail == NULL || *end != '\0' || prec == 0 || prec > (unsigned long long) WORD_MAX
        || (with_rm && (rm_str == NULL || parse_rm(rm_str, &rnd, &exact))))
        res = report(line_number, "unreadable arguments");
    else if (arf_parse_base(z_str, 4, expected))
        res = report(line_number, "unreadable value");
    else
    {
        int inexact = apply(&c, z, (slong) prec, rnd);
        int dir = rounding_direction(&c, z, inexact, (slong) prec, rnd);
        if (exact && inexact)
            res = report(line_number, "Exact rounding of an inexact result");
        else if (!arf_equal(z, expected))
            res = report(line_number, "wrong value");
        else if (strcmp(ordering_name(dir), o_str) != 0)
            res = report(line_number, "wrong ordering");
        else
            res = 0;
    }
    call_clear(&c);
    arf_clear(z);
    arf_clear(expected);
    return res;
}

/* `[X.]op(args) = Z` for the exact fused and slice operations. */
static int
check_exact_call(const char * receiver, char * method, char * z_str, int line_number)
{
    char * args = split(method, "(");
    size_t n = args == NULL ? 0 : strlen(args);
    if (args == NULL || n == 0 || args[n - 1] != ')')
        return report(line_number, "unrecognized line");
    args[n - 1] = '\0';
    int m = find_op(method);
    if (m < 0 || (methods[m].arity == 0) != (receiver == NULL) || methods[m].arity == 2
        || methods[m].rational != 0)
        return report(line_number, "unrecognized method");
    call_t c;
    call_init(&c);
    arf_t z, expected;
    arf_init(z);
    arf_init(expected);
    int res;
    if (parse_operands(&c, m, receiver, args, 0) == NULL)
        res = report(line_number, "unreadable arguments");
    else if (arf_parse_base(z_str, 4, expected))
        res = report(line_number, "unreadable value");
    else
    {
        apply(&c, z, ARF_PREC_EXACT, ARF_RND_DOWN);
        res = arf_equal(z, expected) ? 0 : report(line_number, "wrong value");
    }
    call_clear(&c);
    arf_clear(z);
    arf_clear(expected);
    return res;
}

/* `X.pow(K) = Z`, the exact power for K >= 0, by repeated exact multiplication. */
static int
check_exact_pow(const char * receiver, char * method, const char * z_str, int line_number)
{
    char * args = split(method, "(");
    size_t n = args == NULL ? 0 : strlen(args);
    if (args == NULL || n == 0 || args[n - 1] != ')' || strcmp(method, "pow") != 0)
        return report(line_number, "unrecognized line");
    args[n - 1] = '\0';
    char * end = NULL;
    unsigned long long k = strtoull(args, &end, 10);
    arf_t x, z, expected;
    arf_init(x);
    arf_init(z);
    arf_init(expected);
    int res;
    if (*args == '\0' || *end != '\0')
        res = report(line_number, "unreadable exponent");
    else if (arf_parse_base(receiver, 4, x) || arf_parse_base(z_str, 4, expected))
        res = report(line_number, "unreadable value");
    else
    {
        arf_one(z);
        for (unsigned long long i = 0; i < k; i++)
            arf_mul(z, z, x, ARF_PREC_EXACT, ARF_RND_DOWN);
        res = arf_equal(z, expected) ? 0 : report(line_number, "wrong value");
    }
    arf_clear(x);
    arf_clear(z);
    arf_clear(expected);
    return res;
}

/* The `.` that begins a method call in `line`: one followed by a lowercase identifier and `(`.
   A hexadecimal value's point is followed by digits, which never reach a `(`. */
static char *
find_method(char * line)
{
    for (char * p = strchr(line, '.'); p != NULL; p = strchr(p + 1, '.'))
    {
        char * q = p + 1;
        while ((*q >= 'a' && *q <= 'z') || *q == '_')
            q++;
        if (q > p + 1 && *q == '(')
            return p;
    }
    return NULL;
}

/* The exact operations of one or two values: `-X = Z`, `X op Y = Z`, `X.abs() = Z`,
   `X.square() = Z`, and `X.sign() = O`. */
static int
check_simple(char * line, char * dot, char * z_str, int line_number)
{
    arf_t x, y, z;
    arf_init(x);
    arf_init(y);
    arf_init(z);
    int result = 0;
    const char * operators[] = {" + ", " - ", " * ", " << ", " >> "};
    int operator = -1;
    char * rhs = NULL;
    for (int i = 0; i < 5 && dot == NULL; i++)
    {
        rhs = strstr(line, operators[i]);
        if (rhs != NULL)
        {
            *rhs = '\0';
            rhs += strlen(operators[i]);
            operator = i;
            break;
        }
    }
    if (dot != NULL)
    {
        *dot = '\0';
        const char * method = dot + 1;
        if (arf_parse_base(line, 4, x))
            result = report(line_number, "unreadable value");
        else if (strcmp(method, "sign()") == 0)
        {
            if (arf_is_nan(x))
                result = report(line_number, "sign of NaN");
            else if (strcmp(ordering_name(arf_sgn(x)), z_str) != 0)
                result = report(line_number, "wrong sign");
        }
        else if (arf_parse_base(z_str, 4, y))
            result = report(line_number, "unreadable value");
        else
        {
            if (strcmp(method, "abs()") == 0)
                arf_abs(z, x);
            else if (strcmp(method, "square()") == 0)
                arf_mul(z, x, x, ARF_PREC_EXACT, ARF_RND_DOWN);
            else
                result = report(line_number, "unrecognized method");
            if (result == 0 && !arf_equal(z, y))
                result = report(line_number, "wrong value");
        }
    }
    else if (operator < 0)
    {
        /* `-X = Z` */
        if (line[0] != '-' || arf_parse_base(line + 1, 4, x) || arf_parse_base(z_str, 4, y))
            result = report(line_number, "unrecognized line");
        else
        {
            arf_neg(z, x);
            if (!arf_equal(z, y))
                result = report(line_number, "wrong value");
        }
    }
    else if (arf_parse_base(line, 4, x) || arf_parse_base(z_str, 4, y))
        result = report(line_number, "unreadable value");
    else if (operator >= 3)
    {
        fmpz_t k;
        fmpz_init(k);
        if (fmpz_set_str(k, rhs, 10))
            result = report(line_number, "unreadable shift");
        else
        {
            if (operator == 4)
                fmpz_neg(k, k);
            arf_mul_2exp_fmpz(z, x, k);
            if (!arf_equal(z, y))
                result = report(line_number, "wrong value");
        }
        fmpz_clear(k);
    }
    else
    {
        arf_t w;
        arf_init(w);
        if (arf_parse_base(rhs, 4, w))
            result = report(line_number, "unreadable value");
        else
        {
            if (operator == 0)
                arf_add(z, x, w, ARF_PREC_EXACT, ARF_RND_DOWN);
            else if (operator == 1)
                arf_sub(z, x, w, ARF_PREC_EXACT, ARF_RND_DOWN);
            else
                arf_mul(z, x, w, ARF_PREC_EXACT, ARF_RND_DOWN);
            if (!arf_equal(z, y))
                result = report(line_number, "wrong value");
        }
        arf_clear(w);
    }
    arf_clear(x);
    arf_clear(y);
    arf_clear(z);
    return result;
}

static int
starts_with_slice_op(const char * line)
{
    return strncmp(line, "sum", 3) == 0 || strncmp(line, "product", 7) == 0
        || strncmp(line, "dot", 3) == 0;
}

static int
check_line(char * line, int line_number)
{
    size_t n = strlen(line);
    if (n != 0 && line[n - 1] == '\n')
        line[n - 1] = '\0';
    if (line[0] == '\0')
        return 0;
    int result;
    const char * prefix = "BigExponentFloat::";
    if (strncmp(line, prefix, strlen(prefix)) == 0)
        result = check_rounded(NULL, line + strlen(prefix), line_number);
    else if (starts_with_slice_op(line))
    {
        if (strstr(line, "_prec") != NULL && strstr(line, "_prec") < strchr(line, '('))
            result = check_rounded(NULL, line, line_number);
        else
        {
            char * z_str = split(line, ") = ");
            if (z_str == NULL)
                return report(line_number, "unrecognized line");
            /* restore the `)` that split removed, for check_exact_call */
            size_t k = strlen(line);
            char * call = flint_malloc(k + 2);
            memcpy(call, line, k);
            call[k] = ')';
            call[k + 1] = '\0';
            result = check_exact_call(NULL, call, z_str, line_number);
            flint_free(call);
        }
    }
    else
    {
        char * dot = find_method(line);
        if (dot != NULL && strstr(dot, "_prec") != NULL)
        {
            *dot = '\0';
            result = check_rounded(line, dot + 1, line_number);
        }
        else
        {
            char * z_str = split(line, " = ");
            if (z_str == NULL)
                return report(line_number, "unrecognized line");
            char * method = dot == NULL ? NULL : dot + 1;
            if (method != NULL && strncmp(method, "pow(", 4) == 0)
            {
                *dot = '\0';
                result = check_exact_pow(line, method, z_str, line_number);
            }
            else if (method != NULL && strncmp(method, "abs(", 4) != 0
                && strncmp(method, "square(", 7) != 0 && strncmp(method, "sign(", 5) != 0)
            {
                *dot = '\0';
                result = check_exact_call(line, method, z_str, line_number);
            }
            else
                result = check_simple(line, dot, z_str, line_number);
        }
    }
    if (result == 0)
        checked++;
    return result;
}

int
run_arf_arithmetic(const char * arg)
{
    checked = 0;
    mpfr_set_emin(mpfr_get_emin_min());
    mpfr_set_emax(mpfr_get_emax_max());
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_arithmetic", checked);
}
