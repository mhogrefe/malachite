/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's inverse trigonometric functions against FLINT's arb ball arithmetic.
    The mode `arf_atrig` accepts lines of the forms

    - `X.f_prec_round(p, rm) = (Z, O)`, `X.f_pi_prec_round(p, rm) = (Z, O)`, and
      `X.f_with_period_prec_round(u, p, rm) = (Z, O)`;
    - `BigExponentFloat::f_rational_prec_round(Q, p, rm) = (Z, O)`, and likewise for the `_pi`
      and `_with_period` (with `u`) forms;
    - `Y.atan2_prec_round(X, p, rm) = (Z, O)` (and the `_pi` and `_with_period` forms), and
      `BigExponentFloat::atan2_rational_prec_round(P, Q, p, rm) = (Z, O)` (and the same forms);

    and their `_prec` forms (without the rounding mode), for f one of asin, acos, atan, acot, asec,
    and acsc. X, Y, and Z are BigExponentFloats in Malachite's exact hexadecimal format, and P and
    Q are Rationals written as `a/b` or `a`.

    The special values follow Malachite's documentation: NaN outside the domain and at NaN; the
    values at infinities, zeros, and ±1 that are rational multiples of π (or, with a period, of
    u), with the unsigned zero taken as +0 (acot(0) = π/2, atan2(0, x) = π for x < 0, and atan2(0,
    0) = 0, as in arb_atan2); and, with a period, the rational values at Niven's points (±1/2, ±2,
    and, for atan2, the diagonals), which are rounded exactly. A period of 0 gives 0.

    Otherwise the value is computed with arb_asin, arb_acos, arb_atan, and arb_atan2 (acot, asec, and
    acsc from the reciprocal of the argument), times u/(2π) with a period, and rounded by
    bracketing it between consecutive numbers with one more bit than the target precision, with the
    working precision doubling while a comparison is undecided. For a tiny or huge argument (or, for
    atan2, a tiny or huge ratio of the arguments), whose value is within a tiny amount of a simple
    dyadic number A (the tiny argument s itself, with no period, or k u/4 with one), the value is
    written as A plus a ball for the rest, enclosing the next term of the series, so that it can be
    separated from A even when A is a breakpoint.

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

#define MAX_ARGS 6
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

/* r = a / u */
static void
fmpq_div_ulong(fmpq_t r, const fmpq_t a, ulong u)
{
    fmpz_t d;
    fmpz_init_set_ui(d, u);
    fmpq_div_fmpz(r, a, d);
    fmpz_clear(d);
}

/* The sign of (main - w) + small, or 2 if undecided. */
static int
sign_of_difference(const arb_t main, const arb_t small, const arf_t w, slong wp)
{
    arb_t d;
    arb_init(d);
    if (arb_is_exact(main))
    {
        arf_t e;
        arf_init(e);
        arf_sub(e, arb_midref(main), w, ARF_PREC_EXACT, ARF_RND_DOWN);
        arb_add_arf(d, small, e, wp);
        arf_clear(e);
    }
    else
    {
        arb_sub_arf(d, main, w, wp);
        arb_add(d, d, small, wp);
    }
    int s = arb_is_positive(d) ? 1 : arb_is_negative(d) ? -1 : arb_is_zero(d) ? 0 : 2;
    arb_clear(d);
    return s;
}

typedef enum { F_ASIN, F_ACOS, F_ATAN, F_ACOT, F_ASEC, F_ACSC, F_ATAN2 } fn_t;

static const char * FN_NAMES[] = {"asin", "acos", "atan", "acot", "asec", "acsc", "atan2"};

/* An argument: an arf_t (exact), or a Rational that is not dyadic (`rational`). A dyadic Rational
   is converted to an arf_t, so that tiny arguments get the exact leading terms below. */
typedef struct
{
    int rational;
    arf_t a;
    fmpq_t q;
} arg_t;

static void
arg_init(arg_t * v)
{
    v->rational = 0;
    arf_init(v->a);
    fmpq_init(v->q);
}

static void
arg_clear(arg_t * v)
{
    arf_clear(v->a);
    fmpq_clear(v->q);
}

/* Sets v from the Rational q: an arf_t when q is dyadic. */
static void
arg_set_fmpq(arg_t * v, const fmpq_t q)
{
    if ((slong) fmpz_val2(fmpq_denref(q)) == (slong) fmpz_bits(fmpq_denref(q)) - 1)
    {
        v->rational = 0;
        arf_set_fmpq(v->a, q, fmpz_bits(fmpq_numref(q)) + 2, ARF_RND_DOWN);
    }
    else
    {
        v->rational = 1;
        fmpq_set(v->q, q);
    }
}

static void
arg_ball(arb_t r, const arg_t * v, slong wp)
{
    if (v->rational)
        arb_set_fmpq(r, v->q, wp);
    else
        arb_set_arf(r, v->a);
}

static int
arg_sgn(const arg_t * v)
{
    return v->rational ? fmpq_sgn(v->q) : arf_sgn(v->a);
}

/* Compares |v| with 1. */
static int
arg_cmpabs_one(const arg_t * v)
{
    if (v->rational)
        return fmpz_cmpabs(fmpq_numref(v->q), fmpq_denref(v->q));
    arf_t one;
    arf_init(one);
    arf_one(one);
    int c = arf_cmpabs(v->a, one);
    arf_clear(one);
    return c;
}

/* The exact value of v as a Fraction, if it has a moderate size; returns 0 on success. */
static int
arg_moderate_fmpq(fmpq_t r, const arg_t * v)
{
    if (v->rational)
    {
        fmpq_set(r, v->q);
        return 0;
    }
    if (!arf_is_finite(v->a) || (!arf_is_zero(v->a) && arf_cmpabs_2exp_si(v->a, 4096) > 0)
        || (!arf_is_zero(v->a) && arf_cmpabs_2exp_si(v->a, -4096) < 0))
        return -1;
    arf_get_fmpq(r, v->a);
    return 0;
}

/* f(x)/π at the points where it is rational (the special values and Niven's points), as a
   Fraction; returns 1 if rational, 0 if irrational, 2 for NaN. The zero of acot is taken as +0. */
static int
exact_angle(fmpq_t r, fn_t f, const arg_t * x)
{
    if (!x->rational && arf_is_nan(x->a))
        return 2;
    if (!x->rational && arf_is_inf(x->a))
    {
        int s = arf_sgn(x->a);
        switch (f)
        {
            case F_ASIN: case F_ACOS: return 2;
            case F_ATAN: fmpq_set_si(r, s, 2); return 1;
            case F_ASEC: fmpq_set_si(r, 1, 2); return 1;
            default: fmpq_zero(r); return 1;
        }
    }
    int c = arg_cmpabs_one(x);
    if ((f == F_ASIN || f == F_ACOS) && c > 0)
        return 2;
    if ((f == F_ASEC || f == F_ACSC) && c < 0)
        return 2;
    fmpq_t v;
    fmpq_init(v);
    if (arg_moderate_fmpq(v, x) != 0)
    {
        fmpq_clear(v);
        return 0;
    }
    /* the table, for v >= 0, with odd/even extension */
    int neg = fmpq_sgn(v) < 0;
    fmpq_abs(v, v);
    slong n = -1;
    if (fmpz_is_one(fmpq_denref(v)) && fmpz_cmp_ui(fmpq_numref(v), 2) <= 0)
        n = fmpz_get_si(fmpq_numref(v));
    else if (fmpz_equal_si(fmpq_denref(v), 2) && fmpz_is_one(fmpq_numref(v)))
        n = 3; /* code for 1/2 */
    fmpq_clear(v);
    if (n < 0)
        return 0;
    /* angle/π numerator and denominator for |x| = 0, 1, 2, 1/2 (code 3) */
    slong an = -1, ad = 1;
    int odd = 1;
    switch (f)
    {
        case F_ASIN: if (n == 0) { an = 0; } else if (n == 1) { an = 1; ad = 2; } else if (n == 3) { an = 1; ad = 6; } break;
        case F_ATAN: if (n == 0) { an = 0; } else if (n == 1) { an = 1; ad = 4; } break;
        case F_ACSC: if (n == 1) { an = 1; ad = 2; } else if (n == 2) { an = 1; ad = 6; } break;
        case F_ACOT: if (n == 0) { an = 1; ad = 2; } else if (n == 1) { an = 1; ad = 4; } break;
        case F_ACOS:
            odd = 0;
            if (n == 0) { an = 1; ad = 2; } else if (n == 1) { an = neg ? 1 : 0; } else if (n == 3) { an = neg ? 2 : 1; ad = 3; }
            break;
        case F_ASEC:
            odd = 0;
            if (n == 1) { an = neg ? 1 : 0; } else if (n == 2) { an = neg ? 2 : 1; ad = 3; }
            break;
        default: break;
    }
    if (an < 0)
        return 0;
    fmpq_set_si(r, an, ad);
    if (odd && neg && !(f == F_ACOT && n == 0))
        fmpq_neg(r, r);
    return 1;
}

/* The value being rounded, main + small. */
typedef struct
{
    fn_t f;
    const arg_t * x;
    const arg_t * y; /* atan2: the first argument; x is the second */
    int has_u;
    ulong u;         /* the result is the angle times u/(2π) if has_u */
} value_t;

/* r = θ in radians, a ball. */
static void
angle_ball(arb_t r, const value_t * v, slong wp)
{
    arb_t x, t;
    arb_init(x);
    arb_init(t);
    arg_ball(x, v->x, wp);
    switch (v->f)
    {
        case F_ASIN: arb_asin(r, x, wp); break;
        case F_ACOS: arb_acos(r, x, wp); break;
        case F_ATAN: arb_atan(r, x, wp); break;
        case F_ACOT: arb_inv(t, x, wp); arb_atan(r, t, wp); break;
        case F_ASEC: arb_inv(t, x, wp); arb_acos(r, t, wp); break;
        case F_ACSC: arb_inv(t, x, wp); arb_asin(r, t, wp); break;
        case F_ATAN2: arg_ball(t, v->y, wp); arb_atan2(r, t, x, wp); break;
    }
    arb_clear(x);
    arb_clear(t);
}

/* Multiplies a radian ball by u/(2π) if there is a period. */
static void
to_units(arb_t r, const value_t * v, slong wp)
{
    if (!v->has_u)
        return;
    arb_t p;
    arb_init(p);
    arb_const_pi(p, wp);
    arb_mul_2exp_si(p, p, 1);
    arb_mul_ui(r, r, v->u, wp);
    arb_div(r, r, p, wp);
    arb_clear(p);
}

/* For a value close to a simple dyadic number A, sets main = A (exact) and small = the rest, and
   returns 1; returns 0 if the value is not of that kind (then a ball suffices). The cases are
   those of the decomposition θ = kπ/2 + σ g(s) with s tiny (|s| < 2^(-wp/2 - 10)): with no period
   and k = 0, A = s (when s is exact; otherwise a ball for s, which arb_atan does not enclose as
   tightly for an s this small), and small = g(s) - s, enclosed by its leading term (s^3/6 for
   asin, -s^3/3 for atan) with twice the next term as the radius; with a period and k != 0, A = k
   u/4 and small = σ g(s) u/(2π), a ball with a small relative radius. */
static int
near_simple(arb_t main, arb_t small, const value_t * v, slong wp)
{
    /* find k, σ, g (asin or atan), and s */
    arf_t s;
    arf_init(s);
    int have_exact_s = 0;
    arb_t sb;
    arb_init(sb);
    slong k = 0;
    int sigma = 1, asin = 0;
    const arg_t * x = v->x;
    int sx = arg_sgn(x);
    slong lim = -(wp / 2) - 10;
    int found = 0;
    if (v->f == F_ATAN2)
    {
        const arg_t * y = v->y;
        if (y->rational || x->rational || !arf_is_finite(y->a) || !arf_is_finite(x->a)
            || arf_is_zero(y->a) || arf_is_zero(x->a))
            goto done;
        int sy = arf_sgn(y->a);
        fmpz_t gap;
        fmpz_init(gap);
        fmpz_sub(gap, ARF_EXPREF(y->a), ARF_EXPREF(x->a));
        if (fmpz_cmp_si(gap, lim) < 0)
        {
            /* |y| << |x|: atan(y/x), or ±π + atan(y/x) */
            k = sx < 0 ? 2 * sy : 0;
            sigma = 1;
            arb_set_arf(sb, y->a);
            arb_div_arf(sb, sb, x->a, wp);
            /* exact when |x| is a power of 2 */
            if (arf_bits(x->a) == 1)
            {
                arf_div(s, y->a, x->a, ARF_PREC_EXACT, ARF_RND_DOWN);
                have_exact_s = 1;
            }
            found = 1;
        }
        else if (fmpz_cmp_si(gap, -lim) > 0)
        {
            /* |y| >> |x|: ±π/2 - atan(x/y) */
            k = sy;
            sigma = -1;
            arb_set_arf(sb, x->a);
            arb_div_arf(sb, sb, y->a, wp);
            found = 1;
        }
        fmpz_clear(gap);
    }
    else if (!x->rational && arf_is_finite(x->a) && !arf_is_zero(x->a))
    {
        int tiny = arf_cmpabs_2exp_si(x->a, lim) < 0;
        int huge = arf_cmpabs_2exp_si(x->a, -lim) > 0;
        if (tiny || huge)
        {
            int recip = huge;
            switch (v->f)
            {
                case F_ASIN: k = 0; sigma = 1; asin = 1; break;
                case F_ACOS: k = 1; sigma = -1; asin = 1; break;
                case F_ATAN: k = tiny ? 0 : sx; sigma = tiny ? 1 : -1; break;
                case F_ACOT: k = tiny ? sx : 0; sigma = tiny ? -1 : 1; break;
                case F_ASEC: k = 1; sigma = -1; asin = 1; break;
                case F_ACSC: k = 0; sigma = 1; asin = 1; break;
                default: break;
            }
            arb_set_arf(sb, x->a);
            if (recip)
            {
                arb_inv(sb, sb, wp);
                if (arf_bits(x->a) == 1)
                {
                    arf_ui_div(s, 1, x->a, ARF_PREC_EXACT, ARF_RND_DOWN);
                    have_exact_s = 1;
                }
            }
            else
            {
                arf_set(s, x->a);
                have_exact_s = 1;
            }
            found = 1;
        }
    }
    if (!found)
        goto done;
    if (!v->has_u)
    {
        if (k != 0)
        {
            found = 0;
            goto done;
        }
        /* main = s (exact, or a ball with a small relative radius), small = g(s) - s */
        if (have_exact_s)
            arb_set_arf(main, s);
        else
            arb_set(main, sb);
        arb_t t, s2;
        mag_t rad;
        arb_init(t);
        arb_init(s2);
        mag_init(rad);
        arb_set(t, main);
        arb_sqr(s2, t, wp);
        arb_mul(small, s2, t, wp);
        arb_div_ui(small, small, asin ? 6 : 3, wp);
        if (!asin)
            arb_neg(small, small);
        arb_mul(t, small, s2, wp);
        arb_get_mag(rad, t);
        mag_mul_2exp_si(rad, rad, 1);
        arb_add_error_mag(small, rad);
        arb_clear(t);
        arb_clear(s2);
        mag_clear(rad);
    }
    else
    {
        if (k == 0)
        {
            found = 0;
            goto done;
        }
        /* main = k u/4, small = σ g(s) u/(2π) */
        arf_set_si(arb_midref(main), k);
        arf_mul_ui(arb_midref(main), arb_midref(main), v->u, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_mul_2exp_si(arb_midref(main), arb_midref(main), -2);
        mag_zero(arb_radref(main));
        if (asin)
            arb_asin(small, sb, wp);
        else
            arb_atan(small, sb, wp);
        if (sigma < 0)
            arb_neg(small, small);
        to_units(small, v, wp);
    }
done:
    arf_clear(s);
    arb_clear(sb);
    return found;
}

static void
eval(arb_t main, arb_t small, const value_t * v, slong wp)
{
    arb_zero(small);
    if (near_simple(main, small, v, wp))
        return;
    angle_ball(main, v, wp);
    to_units(main, v, wp);
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

/* Checks one value `value` with ordering `ordering` against the expected (z, o). */
static int
compare(int line_number, const char * value, const char * ordering, const arf_t z, int o, int exact)
{
    arf_t got;
    arf_init(got);
    int res = 0;
    if (o == 2)
        res = report(line_number, "working precision limit reached");
    else if (exact && o != 0)
        res = report(line_number, "Exact rounding of an inexact result");
    else if (ordering == NULL || arf_parse_base(value, 4, got) != 0)
        res = report(line_number, "unreadable result");
    else if (!arf_equal(got, z))
        res = report(line_number, "wrong value");
    else if (strcmp(ordering, ordering_name(o)) != 0)
        res = report(line_number, "wrong ordering");
    arf_clear(got);
    return res;
}

/* Rounds the value main + small to `prec` bits by bracketing; returns the ordering, or 2. */
static int
round_value(arf_t z, const value_t * v, slong prec, arf_rnd_t rnd)
{
    slong q = prec + 1;
    arb_t main, small, sum;
    arf_t a, b, c;
    arb_init(main);
    arb_init(small);
    arb_init(sum);
    arf_init(a);
    arf_init(b);
    arf_init(c);
    int result = 2;
    for (slong wp = prec + 64; result == 2 && wp <= MAX_PREC; wp *= 2)
    {
        eval(main, small, v, wp);
        arb_add(sum, main, small, wp);
        if (!arb_is_finite(sum) || arf_is_zero(arb_midref(sum)))
            continue;
        arf_set_round(a, arb_midref(sum), q, ARF_RND_FLOOR);
        for (int step = 0; step < 8; step++)
        {
            if (arf_is_zero(a))
                break;
            int sa = sign_of_difference(main, small, a, wp);
            if (sa == 2 || sa == 0)
                break;
            if (sa < 0)
            {
                neighbor(a, a, q, 0);
                continue;
            }
            neighbor(b, a, q, 1);
            int sb = sign_of_difference(main, small, b, wp);
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
    arb_clear(main);
    arb_clear(small);
    arb_clear(sum);
    arf_clear(a);
    arf_clear(b);
    arf_clear(c);
    return result;
}

/* Rounds the Fraction r (the angle/π) times π (no period) or u/2 (a period u); returns the
   ordering, or 2 when irrational and undecided. */
static int
round_angle_over_pi(arf_t z, const fmpq_t r, const value_t * v, slong prec, arf_rnd_t rnd)
{
    if (fmpq_is_zero(r))
    {
        arf_zero(z);
        return 0;
    }
    if (v->has_u)
    {
        fmpq_t w;
        fmpq_init(w);
        fmpq_mul_ui(w, r, v->u);
        fmpq_div_ulong(w, w, 2);
        arf_set_fmpq(z, w, prec, rnd);
        fmpq_t t;
        fmpq_init(t);
        arf_get_fmpq(t, z);
        int c = fmpq_cmp(t, w);
        fmpq_clear(t);
        fmpq_clear(w);
        return c < 0 ? -1 : c > 0;
    }
    /* r π: bracket the ball r π */
    slong q = prec + 1;
    arb_t m, zero;
    arf_t a, b, c;
    arb_init(m);
    arb_init(zero);
    arf_init(a);
    arf_init(b);
    arf_init(c);
    int result = 2;
    for (slong wp = prec + 64; result == 2 && wp <= MAX_PREC; wp *= 2)
    {
        arb_const_pi(m, wp);
        arb_mul_fmpz(m, m, fmpq_numref(r), wp);
        arb_div_fmpz(m, m, fmpq_denref(r), wp);
        arf_set_round(a, arb_midref(m), q, ARF_RND_FLOOR);
        for (int step = 0; step < 8; step++)
        {
            int sa = sign_of_difference(m, zero, a, wp);
            if (sa == 2 || sa == 0)
                break;
            if (sa < 0)
            {
                neighbor(a, a, q, 0);
                continue;
            }
            neighbor(b, a, q, 1);
            int sb = sign_of_difference(m, zero, b, wp);
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
    arb_clear(m);
    arb_clear(zero);
    arf_clear(a);
    arf_clear(b);
    arf_clear(c);
    return result;
}

/* atan2(y, x)/π when rational (the special values and the diagonals); returns 1 if rational, 0 if
   irrational, 2 for NaN. The zero is taken as +0, as arb_atan2 takes it. */
static int
atan2_exact(fmpq_t r, const arg_t * y, const arg_t * x)
{
    int ynan = !y->rational && arf_is_nan(y->a), xnan = !x->rational && arf_is_nan(x->a);
    if (ynan || xnan)
        return 2;
    int yinf = !y->rational && arf_is_inf(y->a), xinf = !x->rational && arf_is_inf(x->a);
    int sy = arg_sgn(y), sx = arg_sgn(x);
    int ysign = sy < 0 ? -1 : 1;
    if (yinf && xinf)
    {
        fmpq_set_si(r, ysign * (sx > 0 ? 1 : 3), 4);
        return 1;
    }
    if (yinf)
    {
        fmpq_set_si(r, ysign, 2);
        return 1;
    }
    if (xinf || sy == 0)
    {
        fmpq_set_si(r, sx < 0 ? ysign : 0, 1);
        return 1;
    }
    if (sx == 0)
    {
        fmpq_set_si(r, ysign, 2);
        return 1;
    }
    /* the diagonals */
    int diag;
    if (!y->rational && !x->rational)
        diag = arf_cmpabs(y->a, x->a) == 0;
    else
    {
        fmpq_t a, b;
        fmpq_init(a);
        fmpq_init(b);
        int ok = arg_moderate_fmpq(a, y) == 0 && arg_moderate_fmpq(b, x) == 0;
        diag = ok && fmpz_cmpabs(fmpq_numref(a), fmpq_numref(b)) == 0 && fmpz_equal(fmpq_denref(a), fmpq_denref(b));
        fmpq_clear(a);
        fmpq_clear(b);
    }
    if (diag)
    {
        fmpq_set_si(r, ysign * (sx > 0 ? 1 : 3), 4);
        return 1;
    }
    return 0;
}

static int
expected(arf_t z, fn_t f, const arg_t * y, const arg_t * x, int has_u, ulong u, slong prec,
         arf_rnd_t rnd)
{
    fmpq_t r;
    fmpq_init(r);
    int e = f == F_ATAN2 ? atan2_exact(r, y, x) : exact_angle(r, f, x);
    int o;
    if (e == 2)
    {
        arf_nan(z);
        o = 0;
    }
    else if (has_u && u == 0)
    {
        arf_zero(z);
        o = 0;
    }
    else if (e == 1)
    {
        value_t v = {f, x, y, has_u, u};
        o = round_angle_over_pi(z, r, &v, prec, rnd);
    }
    else
    {
        value_t v = {f, x, y, has_u, u};
        o = round_value(z, &v, prec, rnd);
    }
    fmpq_clear(r);
    return o;
}

/* Parses a BigExponentFloat (hexadecimal) or a Rational argument. */
static int
parse_arg(arg_t * v, const char * s, int rational)
{
    if (!rational)
    {
        v->rational = 0;
        return arf_parse_base(s, 4, v->a);
    }
    fmpq_t q;
    fmpq_init(q);
    int res = parse_rational(q, s);
    if (res == 0)
        arg_set_fmpq(v, q);
    fmpq_clear(q);
    return res;
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
    fn_t f = F_ASIN;
    const char * fam = NULL;
    if (strncmp(name, "atan2", 5) == 0)
    {
        f = F_ATAN2;
        fam = name + 5;
    }
    else
        for (int i = 0; i < 6 && fam == NULL; i++)
            if (strncmp(name, FN_NAMES[i], 4) == 0)
            {
                f = (fn_t) i;
                fam = name + 4;
            }
    if (fam == NULL)
        return report(line_number, "unrecognized function");
    int rational = 0, period = 0, pi = 0;
    if (strcmp(fam, "") == 0)
        ;
    else if (strcmp(fam, "_rational") == 0)
        rational = 1;
    else if (strcmp(fam, "_with_period") == 0)
        period = 1;
    else if (strcmp(fam, "_with_period_rational") == 0)
        period = rational = 1;
    else if (strcmp(fam, "_pi") == 0)
        pi = 1;
    else if (strcmp(fam, "_pi_rational") == 0)
        pi = rational = 1;
    else
        return report(line_number, "unrecognized function");
    if ((xtext == NULL) != rational)
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
    int n_values = (f == F_ATAN2 ? 1 : 0) + (rational ? 1 : 0);
    if (n_items != n_values + period + 1 + with_rm)
        return report(line_number, "wrong number of arguments");
    /* the arguments: for atan2, y then x */
    const char * first = rational ? items[0] : xtext;
    const char * second = f == F_ATAN2 ? (rational ? items[1] : items[0]) : NULL;
    char * end = NULL;
    ulong u = 2;
    if (period)
    {
        u = strtoull(items[n_values], &end, 10);
        if (*end != '\0')
            return report(line_number, "unreadable period");
    }
    long long prec = strtoll(items[n_values + period], &end, 10);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    arg_t a1, a2;
    arg_init(&a1);
    arg_init(&a2);
    arf_t z;
    arf_init(z);
    int res = 0;
    if (parse_arg(&a1, first, rational) != 0 || (second != NULL && parse_arg(&a2, second, rational) != 0))
        res = report(line_number, "unreadable argument");
    else if (*end != '\0' || prec <= 0)
        res = report(line_number, "unreadable precision");
    else if (with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        int o = f == F_ATAN2 ? expected(z, f, &a1, &a2, period || pi, u, prec, rnd)
                             : expected(z, f, NULL, &a1, period || pi, u, prec, rnd);
        char * value = result;
        size_t rl = strlen(value);
        if (rl < 2 || value[0] != '(' || value[rl - 1] != ')')
            res = report(line_number, "unrecognized result");
        else
        {
            value[rl - 1] = '\0';
            value++;
            char * ordering = split(value, ", ");
            res = compare(line_number, value, ordering, z, o, exact);
        }
    }
    arg_clear(&a1);
    arg_clear(&a2);
    arf_clear(z);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_atrig(const char * arg)
{
    checked = 0;
    int result = arf_for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_atrig", checked);
}
