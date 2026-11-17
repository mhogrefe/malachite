/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's logarithms against FLINT's arb ball arithmetic. The mode
    `arf_logarithm` accepts lines of the forms

    - `X.op_prec_round([k, ]p, rm) = (Z, O)` and `X.op_prec([k, ]p) = (Z, O)`, for op one of ln,
      ln_1_plus_x, log_base_2, log_base_2_1_plus_x, log_base_10, log_base_10_1_plus_x,
      log_base_power_of_2, and log_base_power_of_2_1_plus_x (the last two with the power k);
    - `BigExponentFloat::op_prec_round(Q, [k, ]p, rm) = (Z, O)` and
      `BigExponentFloat::op_prec(Q, [k, ]p) = (Z, O)`, for op one of ln_rational,
      log_base_2_rational, log_base_10_rational, log_base_power_of_2_rational, and ln_unsigned;

    where X and Z are BigExponentFloats in Malachite's exact hexadecimal format and Q is a Rational
    written as `a/b` or `a` (a u64 for ln_unsigned).

    The value is f = L / ln(b) for b = e, 2, 10, or 2^k, where L is ln(x) or ln(1 + x). It is
    written as f = main + small, where small is tiny or zero and is enclosed in a ball with a small
    relative radius:
    - for ln(1 + t) and ln(t + 1) (with t = x - 1 when 1/2 <= x <= 2), with |t| < 2^-8: main = t /
      ln(b) and small = (ln(1 + t) - t) / ln(b), whose series is summed with an explicit bound;
    - for ln(1 + x) with x >= 2: main = log_b(x) and small = ln(1 + 1/x) / ln(b), computed the same
      way, so that log_2(1 + 2^n) = n + small is resolved without n bits of precision;
    - otherwise: main = log_b(x) and small = 0.
    main is exact when it is a rational with a finite binary expansion: n / k for log_b(2^n) with b
    = 2 or 2^k, and j for log_10(10^j); and t itself for base e. The result is rounded by
    bracketing f between consecutive numbers with one more bit than the target precision; each
    comparison of f with such a number w is the sign of (main - w) + small, with the working
    precision doubling while a comparison is undecided.

    That never decides when f itself is a number with at most prec + 1 bits. Such an f is exact
    (main exact and small = 0): log_2 and log_(2^k) of powers of 2, log_10 of powers of 10, the
    same for 1 + x, and 0 at 1; it is rounded directly.

    The special cases follow Malachite's documentation (no negative zero; for b = 2^k with k < 0
    the infinities change sign).

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

#define MAX_ARGS 5
#define MAX_PREC (1L << 26)

static long checked;

static int
report(int line_number, const char * what)
{
    flint_printf("line %d: %s\n", line_number, what);
    return 1;
}

static char *
split(char * s, const char * sep)
{
    char * p = strstr(s, sep);
    if (p == NULL)
        return NULL;
    *p = '\0';
    return p + strlen(sep);
}

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
        return -1;
    return 0;
}

static const char *
ordering_name(int c)
{
    return c < 0 ? "Less" : c > 0 ? "Greater" : "Equal";
}

static int
parse_rational(fmpq_t q, const char * s)
{
    char buf[4096];
    size_t n = strlen(s);
    if (n == 0 || n >= sizeof(buf))
        return -1;
    memcpy(buf, s, n + 1);
    char * slash = strchr(buf, '/');
    if (slash != NULL)
        *slash = '\0';
    if (fmpz_set_str(fmpq_numref(q), buf, 10) != 0)
        return -1;
    if (slash == NULL)
        fmpz_one(fmpq_denref(q));
    else if (fmpz_set_str(fmpq_denref(q), slash + 1, 10) != 0 || fmpz_sgn(fmpq_denref(q)) <= 0)
        return -1;
    fmpq_t c;
    fmpq_init(c);
    fmpq_set(c, q);
    fmpq_canonicalise(c);
    int ok = fmpq_equal(c, q);
    fmpq_clear(c);
    return ok ? 0 : -1;
}

/* base: 0 for e, 2, 10, or 1 for 2^k */
typedef struct
{
    const char * name;
    int base;
    int one_plus;
    int input; /* 0: BigExponentFloat, 1: Rational, 2: u64 */
} op_t;

static const op_t OPS[] = {
    {"ln", 0, 0, 0},
    {"ln_1_plus_x", 0, 1, 0},
    {"log_base_2", 2, 0, 0},
    {"log_base_2_1_plus_x", 2, 1, 0},
    {"log_base_10", 10, 0, 0},
    {"log_base_10_1_plus_x", 10, 1, 0},
    {"log_base_power_of_2", 1, 0, 0},
    {"log_base_power_of_2_1_plus_x", 1, 1, 0},
    {"ln_rational", 0, 0, 1},
    {"log_base_2_rational", 2, 0, 1},
    {"log_base_10_rational", 10, 0, 1},
    {"log_base_power_of_2_rational", 1, 0, 1},
    {"ln_unsigned", 0, 0, 2},
};

/* Sets r to a ball containing ln(b) (for b = 2^k, k ln(2)). */
static void
ln_base(arb_t r, const op_t * op, slong k, slong wp)
{
    if (op->base == 0)
        arb_one(r);
    else if (op->base == 10)
        arb_const_log10(r, wp);
    else
    {
        arb_const_log2(r, wp);
        if (op->base == 1)
            arb_mul_si(r, r, k, wp);
    }
}

/* For a ball t with |t| < 2^-8, sets r to a ball containing ln(1 + t) - t = -t^2/2 + t^3/3 - ...,
   summed until a term is below 2^-wp times the first; the omitted terms add up to less than the
   last term computed (each term is at most |t| times the one before), which is added as an error
   bound. */
static void
log1p_tail(arb_t r, const arb_t t, slong wp)
{
    arb_t power, term;
    arb_init(power);
    arb_init(term);
    arb_sqr(power, t, wp);
    arb_mul_2exp_si(r, power, -1);
    arb_neg(r, r);
    mag_t first, m;
    mag_init(first);
    mag_init(m);
    arb_get_mag(first, r);
    mag_mul_2exp_si(first, first, -wp);
    for (ulong k = 3;; k++)
    {
        arb_mul(power, power, t, wp);
        arb_div_ui(term, power, k, wp);
        if (k % 2 == 0)
            arb_sub(r, r, term, wp);
        else
            arb_add(r, r, term, wp);
        arb_get_mag(m, term);
        if (mag_cmp(m, first) < 0)
            break;
    }
    arb_add_error_mag(r, m);
    mag_clear(first);
    mag_clear(m);
    arb_clear(power);
    arb_clear(term);
}

static int
small_ball(const arb_t t)
{
    return arb_is_finite(t) && !arb_contains_zero(t) && arf_cmpabs_2exp_si(arb_midref(t), -9) < 0;
}

/* The decomposition f = main + small described at the top, at working precision wp. If
   `main_exact` is set, main is the exact arf `me` (and the ball `main` holds it too). */
typedef struct
{
    arb_t main;
    arb_t small;
    arf_t me;
    int main_exact;
} parts_t;

static void
parts_init(parts_t * p)
{
    arb_init(p->main);
    arb_init(p->small);
    arf_init(p->me);
    p->main_exact = 0;
}

static void
parts_clear(parts_t * p)
{
    arb_clear(p->main);
    arb_clear(p->small);
    arf_clear(p->me);
}

/* If the arf x is a power of 2, 2^n, sets n and returns 1. */
static int
power_of_2(fmpz_t n, const arf_t x)
{
    if (arf_sgn(x) <= 0 || arf_bits(x) != 1)
        return 0;
    fmpz_sub_ui(n, ARF_EXPREF(x), 1);
    return 1;
}

/* If the positive rational x is 10^j (j may be negative), sets j and returns 1. */
static int
power_of_10_rational(fmpz_t j, const fmpq_t x)
{
    const fmpz * num = fmpq_numref(x);
    const fmpz * den = fmpq_denref(x);
    int inverse = !fmpz_is_one(den);
    if (inverse && !fmpz_is_one(num))
        return 0;
    const fmpz * v = inverse ? den : num;
    if (fmpz_sgn(v) <= 0 || fmpz_bits(v) > (1 << 22))
        return 0;
    fmpz_t r, ten;
    fmpz_init(r);
    fmpz_init_set_ui(ten, 10);
    slong e = fmpz_remove(r, v, ten);
    int ok = fmpz_is_one(r);
    fmpz_clear(r);
    fmpz_clear(ten);
    if (ok)
        fmpz_set_si(j, inverse ? -e : e);
    return ok;
}

/* Whether the positive fmpz v is a power of 2; if so, sets *n to its exponent. */
static int
fmpz_power_of_2(slong * n, const fmpz_t v)
{
    if (fmpz_sgn(v) <= 0 || fmpz_val2(v) + 1 != fmpz_bits(v))
        return 0;
    *n = fmpz_val2(v);
    return 1;
}

/* Sets q to n / k (or n, for base 2), the exact log_b(2^n) for b = 2 or 2^k. */
static void
power_of_2_log(fmpq_t q, const op_t * op, slong k, const fmpz_t n)
{
    fmpz_set(fmpq_numref(q), n);
    fmpz_set_si(fmpq_denref(q), op->base == 1 ? k : 1);
    fmpq_canonicalise(q);
}

/* Exact log_b(v) for a positive rational v of moderate size, when it is rational: 0 at 1, n / k
   for v = 2^n with b = 2 or 2^k, and j for v = 10^j with b = 10. Sets q and returns 1 if so. */
static int
exact_log(fmpq_t q, const op_t * op, slong k, const fmpq_t v)
{
    if (fmpq_is_one(v))
    {
        fmpq_zero(q);
        return 1;
    }
    const fmpz * num = fmpq_numref(v);
    const fmpz * den = fmpq_denref(v);
    if (op->base == 2 || op->base == 1)
    {
        slong e;
        fmpz_t n;
        fmpz_init(n);
        int ok = 0;
        if (fmpz_is_one(den) && fmpz_power_of_2(&e, num))
            fmpz_set_si(n, e), ok = 1;
        else if (fmpz_is_one(num) && fmpz_power_of_2(&e, den))
            fmpz_set_si(n, -e), ok = 1;
        if (ok)
            power_of_2_log(q, op, k, n);
        fmpz_clear(n);
        return ok;
    }
    if (op->base == 10)
    {
        fmpz_t j;
        fmpz_init(j);
        int ok = power_of_10_rational(j, v);
        if (ok)
            fmpq_set_fmpz(q, j);
        fmpz_clear(j);
        return ok;
    }
    return 0;
}

/* Sets p to the decomposition of f for the input, at working precision wp. For a BigExponentFloat
   input, x is exact; a Rational input is given as the ball x (and the fmpq xq). */
static void
compute_parts(parts_t * p, const op_t * op, slong k, const arf_t xa, const fmpq_t xq, slong wp)
{
    arb_t x, t, lb, tmp;
    arb_init(x);
    arb_init(t);
    arb_init(lb);
    arb_init(tmp);
    if (xa != NULL)
        arb_set_arf(x, xa);
    else
        arb_set_fmpq(x, xq, wp);
    ln_base(lb, op, k, wp);
    p->main_exact = 0;
    arb_zero(p->small);
    /* t = x (for 1 + x) or x - 1 (for x near 1) */
    int use_t = 0;
    if (op->one_plus)
    {
        arb_set(t, x);
        use_t = 1;
    }
    else if (xa != NULL ? arf_cmp_2exp_si(xa, -1) >= 0 && arf_cmp_2exp_si(xa, 1) <= 0
                        : 0)
    {
        arb_sub_ui(t, x, 1, ARF_PREC_EXACT);
        use_t = 1;
    }
    else if (xa == NULL)
    {
        fmpq_t tq;
        fmpq_init(tq);
        fmpq_sub_si(tq, xq, 1);
        if (!fmpq_is_zero(tq))
        {
            arb_set_fmpq(t, tq, wp);
            use_t = small_ball(t);
        }
        fmpq_clear(tq);
    }
    if (use_t && small_ball(t))
    {
        /* main = t / ln(b), small = (ln(1 + t) - t) / ln(b) */
        log1p_tail(p->small, t, wp);
        if (op->base == 0)
        {
            arb_set(p->main, t);
            if (arb_is_exact(t))
            {
                arf_set(p->me, arb_midref(t));
                p->main_exact = 1;
            }
        }
        else
        {
            arb_div(p->main, t, lb, wp);
            arb_div(p->small, p->small, lb, wp);
        }
    }
    else if (op->one_plus && xa != NULL && arf_cmp_si(xa, 2) >= 0)
    {
        /* main = log_b(x), small = ln(1 + 1/x) / ln(b) */
        fmpz_t n;
        fmpz_init(n);
        if ((op->base == 2 || op->base == 1) && power_of_2(n, xa))
        {
            fmpq_t q;
            fmpq_init(q);
            fmpz_set(fmpq_numref(q), n);
            fmpz_set_si(fmpq_denref(q), op->base == 1 ? k : 1);
            fmpq_canonicalise(q);
            arb_set_fmpq(p->main, q, wp);
            if (arb_is_exact(p->main))
            {
                arf_set(p->me, arb_midref(p->main));
                p->main_exact = 1;
            }
            fmpq_clear(q);
        }
        else
        {
            arb_log(p->main, x, wp);
            arb_div(p->main, p->main, lb, wp);
        }
        fmpz_clear(n);
        arb_inv(tmp, x, wp);
        if (small_ball(tmp))
            log1p_tail(p->small, tmp, wp), arb_add(p->small, p->small, tmp, wp);
        else
            arb_log1p(p->small, tmp, wp);
        arb_div(p->small, p->small, lb, wp);
    }
    else
    {
        if (op->one_plus)
            arb_log1p(p->main, x, wp);
        else
            arb_log(p->main, x, wp);
        arb_div(p->main, p->main, lb, wp);
    }
    arb_clear(x);
    arb_clear(t);
    arb_clear(lb);
    arb_clear(tmp);
}

/* The sign of f - w, or 2 if undecided. */
static int
sign_of_difference(const parts_t * p, const arf_t w, slong wp)
{
    arb_t d;
    arb_init(d);
    if (p->main_exact)
    {
        arf_t e;
        arf_init(e);
        arf_sub(e, p->me, w, ARF_PREC_EXACT, ARF_RND_DOWN);
        arb_set_arf(d, e);
        arf_clear(e);
    }
    else
        arb_sub_arf(d, p->main, w, wp);
    arb_add(d, d, p->small, wp);
    int s = arb_is_positive(d) ? 1 : arb_is_negative(d) ? -1 : 2;
    arb_clear(d);
    return s;
}

static void
neighbor(arf_t r, const arf_t a, slong q, int up)
{
    fmpz_t e;
    fmpz_init(e);
    fmpz_set(e, ARF_EXPREF(a));
    fmpz_sub_ui(e, e, q + 10);
    arf_t tiny;
    arf_init(tiny);
    arf_one(tiny);
    arf_mul_2exp_fmpz(tiny, tiny, e);
    if (up)
    {
        arf_add(r, a, tiny, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_set_round(r, r, q, ARF_RND_CEIL);
    }
    else
    {
        arf_sub(r, a, tiny, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_set_round(r, r, q, ARF_RND_FLOOR);
    }
    arf_clear(tiny);
    fmpz_clear(e);
}

/* Rounds f by bracketing (see the top). Returns the ordering, or 2 if undecided. */
static int
round_by_bracketing(arf_t z, const op_t * op, slong k, const arf_t xa, const fmpq_t xq, slong prec,
                    arf_rnd_t rnd)
{
    slong q = prec + 1;
    parts_t p;
    parts_init(&p);
    arf_t a, b, c, y;
    arf_init(a);
    arf_init(b);
    arf_init(c);
    arf_init(y);
    arb_t sum;
    arb_init(sum);
    int result = 2;
    for (slong wp = prec + 64; result == 2 && wp <= MAX_PREC; wp *= 2)
    {
        compute_parts(&p, op, k, xa, xq, wp);
        arb_add(sum, p.main, p.small, wp);
        arf_set(y, arb_midref(sum));
        if (arf_is_zero(y))
            continue;
        arf_set_round(a, y, q, ARF_RND_FLOOR);
        for (int step = 0; step < 8; step++)
        {
            if (arf_is_zero(a))
                break;
            int sa = sign_of_difference(&p, a, wp);
            if (sa == 2 || sa == 0)
                break;
            if (sa < 0)
            {
                neighbor(a, a, q, 0);
                continue;
            }
            neighbor(b, a, q, 1);
            int sb = sign_of_difference(&p, b, wp);
            if (sb == 2 || sb == 0)
                break;
            if (sb > 0)
            {
                arf_set(a, b);
                continue;
            }
            arf_add(c, a, b, ARF_PREC_EXACT, ARF_RND_DOWN);
            arf_mul_2exp_si(c, c, -1);
            arf_set_round(z, c, prec, rnd);
            result = arf_cmp(z, c) < 0 ? -1 : 1;
            break;
        }
    }
    parts_clear(&p);
    arf_clear(a);
    arf_clear(b);
    arf_clear(c);
    arf_clear(y);
    arb_clear(sum);
    return result;
}

static int
round_fmpq(arf_t z, const fmpq_t v, slong prec, arf_rnd_t rnd)
{
    arf_fmpz_div_fmpz(z, fmpq_numref(v), fmpq_denref(v), prec, rnd);
    /* ordering: compare z with v exactly */
    fmpq_t zq;
    fmpq_init(zq);
    arf_get_fmpq(zq, z);
    int c = fmpq_cmp(zq, v);
    fmpq_clear(zq);
    return c;
}

/* The expected result. Returns the ordering, or 2 if undecided. */
static int
expected(arf_t z, const op_t * op, slong k, const arf_t xa, const fmpq_t xq, slong prec,
         arf_rnd_t rnd)
{
    int decreasing = op->base == 1 && k < 0;
    if (xa != NULL)
    {
        if (arf_is_nan(xa))
        {
            arf_nan(z);
            return 0;
        }
        if (arf_is_pos_inf(xa))
        {
            if (decreasing)
                arf_neg_inf(z);
            else
                arf_pos_inf(z);
            return 0;
        }
        if (arf_is_neg_inf(xa))
        {
            arf_nan(z);
            return 0;
        }
        if (op->one_plus)
        {
            if (arf_is_zero(xa))
            {
                arf_zero(z);
                return 0;
            }
            int c = arf_cmp_si(xa, -1);
            if (c < 0)
            {
                arf_nan(z);
                return 0;
            }
            if (c == 0)
            {
                if (decreasing)
                    arf_pos_inf(z);
                else
                    arf_neg_inf(z);
                return 0;
            }
        }
        else
        {
            if (arf_is_zero(xa))
            {
                if (decreasing)
                    arf_pos_inf(z);
                else
                    arf_neg_inf(z);
                return 0;
            }
            if (arf_sgn(xa) < 0)
            {
                arf_nan(z);
                return 0;
            }
        }
    }
    else
    {
        if (fmpq_is_zero(xq))
        {
            if (decreasing)
                arf_pos_inf(z);
            else
                arf_neg_inf(z);
            return 0;
        }
        if (fmpq_sgn(xq) < 0)
        {
            arf_nan(z);
            return 0;
        }
    }
    /* The exact cases. For a BigExponentFloat 2^n with b = 2 or 2^k, log_b(2^n) = n / k is found
       from n directly, whatever its size. Otherwise v is x itself (or 1 + x) as a rational, formed
       only when x's exponent is moderate: a larger x can only give an exact logarithm through
       1 + x = 2^n or 10^j with x an integer whose bits are all used, and those have more than
       2^22 bits only for inputs that large. */
    fmpq_t v, ex;
    fmpq_init(v);
    fmpq_init(ex);
    int found = 0;
    if (xa == NULL)
        found = exact_log(ex, op, k, xq);
    else
    {
        fmpz_t n;
        fmpz_init(n);
        if (!op->one_plus && power_of_2(n, xa))
        {
            if (op->base == 2 || op->base == 1)
            {
                power_of_2_log(ex, op, k, n);
                found = 1;
            }
            else if (fmpz_is_zero(n))
            {
                fmpq_zero(ex);
                found = 1;
            }
        }
        else if (fmpz_bits(ARF_EXPREF(xa)) <= 22 && arf_bits(xa) < (1 << 22))
        {
            arf_t w;
            arf_init(w);
            if (op->one_plus)
                arf_add_ui(w, xa, 1, ARF_PREC_EXACT, ARF_RND_DOWN);
            else
                arf_set(w, xa);
            if (arf_sgn(w) > 0)
            {
                arf_get_fmpq(v, w);
                found = exact_log(ex, op, k, v);
            }
            arf_clear(w);
        }
        fmpz_clear(n);
    }
    if (found)
    {
        int o = round_fmpq(z, ex, prec, rnd);
        fmpq_clear(v);
        fmpq_clear(ex);
        return o;
    }
    fmpq_clear(v);
    fmpq_clear(ex);
    return round_by_bracketing(z, op, k, xa, xq, prec, rnd);
}

static int
check_line(char * line, int line_number)
{
    size_t len = strlen(line);
    if (len != 0 && line[len - 1] == '\n')
        line[len - 1] = '\0';
    if (line[0] == '\0')
        return 0;
    char * result = split(line, " = ");
    if (result == NULL)
        return report(line_number, "unrecognized line");
    char * name;
    char * xtext = NULL;
    const char * prefix = "BigExponentFloat::";
    if (strncmp(line, prefix, strlen(prefix)) == 0)
        name = line + strlen(prefix);
    else
    {
        char * dot = NULL;
        for (char * p = strchr(line, '.'); p != NULL && dot == NULL; p = strchr(p + 1, '.'))
        {
            char * r = p + 1;
            while ((*r >= 'a' && *r <= 'z') || *r == '_' || (*r >= '0' && *r <= '9'))
                r++;
            if (r > p + 1 && *r == '(')
                dot = p;
        }
        if (dot == NULL)
            return report(line_number, "unrecognized line");
        *dot = '\0';
        xtext = line;
        name = dot + 1;
    }
    char * args = split(name, "(");
    size_t al = args == NULL ? 0 : strlen(args);
    if (args == NULL || al == 0 || args[al - 1] != ')')
        return report(line_number, "unrecognized line");
    args[al - 1] = '\0';
    char * suffix = strstr(name, "_prec");
    if (suffix == NULL || (strcmp(suffix, "_prec_round") != 0 && strcmp(suffix, "_prec") != 0))
        return report(line_number, "missing precision");
    int with_rm = strcmp(suffix, "_prec_round") == 0;
    *suffix = '\0';
    const op_t * op = NULL;
    for (size_t i = 0; i < sizeof(OPS) / sizeof(OPS[0]); i++)
        if (strcmp(OPS[i].name, name) == 0)
            op = &OPS[i];
    if (op == NULL || (xtext != NULL) != (op->input == 0))
        return report(line_number, "unrecognized function");
    char * items[MAX_ARGS];
    int n_items = 0;
    for (char * p = args; *p != '\0' && n_items < MAX_ARGS;)
    {
        items[n_items++] = p;
        char * next = split(p, ", ");
        if (next == NULL)
            break;
        p = next;
    }
    int has_k = op->base == 1;
    int first = op->input == 0 ? 0 : 1;
    if (n_items != first + has_k + 1 + with_rm)
        return report(line_number, "wrong number of arguments");
    if (op->input != 0)
        xtext = items[0];
    slong k = 0;
    char * end = NULL;
    if (has_k)
    {
        k = strtoll(items[first], &end, 10);
        if (*end != '\0' || k == 0)
            return report(line_number, "unreadable power");
    }
    long long prec = strtoll(items[first + has_k], &end, 10);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    arf_t x, z, got;
    fmpq_t q;
    arf_init(x);
    arf_init(z);
    arf_init(got);
    fmpq_init(q);
    int res = 0;
    if (op->input == 0 ? arf_parse_base(xtext, 4, x) != 0 : parse_rational(q, xtext) != 0)
        res = report(line_number, "unreadable argument");
    else if (*end != '\0' || prec <= 0)
        res = report(line_number, "unreadable precision");
    else if (with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        int o = expected(z, op, k, op->input == 0 ? x : NULL, q, prec, rnd);
        char * value = result;
        char * ordering = NULL;
        size_t rl = strlen(value);
        if (rl < 2 || value[0] != '(' || value[rl - 1] != ')')
            res = report(line_number, "unrecognized result");
        else
        {
            value[rl - 1] = '\0';
            value++;
            ordering = split(value, ", ");
        }
        if (res != 0)
            ;
        else if (o == 2)
            res = report(line_number, "working precision limit reached");
        else if (exact && o != 0)
            res = report(line_number, "Exact rounding of an inexact result");
        else if (ordering == NULL || arf_parse_base(value, 4, got) != 0)
            res = report(line_number, "unreadable result");
        else if (!arf_equal(got, z))
            res = report(line_number, "wrong value");
        else if (strcmp(ordering, ordering_name(o)) != 0)
            res = report(line_number, "wrong ordering");
    }
    arf_clear(x);
    arf_clear(z);
    arf_clear(got);
    fmpq_clear(q);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_logarithm(const char * arg)
{
    checked = 0;
    int result = arf_for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_logarithm", checked);
}
