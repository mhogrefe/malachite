/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's general-base logarithms and compound function against FLINT's arb
    ball arithmetic. The mode `arf_logarithm_general` accepts lines of the forms

    - `X.op_prec_round(B, p, rm) = (Z, O)` and `X.op_prec(B, p) = (Z, O)`, for op one of log_base
      (B a u64), log_base_1_plus_x (B a u64), log_base_big_exponent_float_base and its
      `_1_plus_x` form (B a BigExponentFloat), log_base_rational_base and its `_1_plus_x` form (B
      a Rational), and compound (B the i64 exponent n);
    - `BigExponentFloat::op_prec_round(Q, B, p, rm) = (Z, O)` and
      `BigExponentFloat::op_prec(Q, B, p) = (Z, O)`, for op one of log_base_rational (B a u64),
      log_base_rational_big_exponent_float_base (B a BigExponentFloat), and
      log_base_rational_rational_base (B a Rational);

    where X, Z, and a BigExponentFloat B are in Malachite's exact hexadecimal format and Q and a
    Rational B are written as `a/b` or `a`.

    A logarithm is f = L / ln(b), where L is ln(x) or ln(1 + x), with the special values of arf_div
    (so a base of 1, where ln(b) = 0, gives NaN). As in the `arf_logarithm` mode, f is written as main +
    small, with small tiny or zero and enclosed in a ball with a small relative radius, and main
    exact when it is a dyadic rational: for x (or 1 + x) near 1, main = t / ln(b) and small = (ln(1
    + t) - t) / ln(b); for ln(1 + x) with x >= 2, main = log_b(x) and small = ln(1 + 1/x) / ln(b).
    log_b(v) is rational when v and b are powers of a common rational g: found here from the powers
    of 2 alone when both are powers of 2 (at any size), and otherwise, for v and b of moderate size,
    from b's primitive root (with FLINT's fmpz_is_perfect_power) and an exact check of g^a = v.

    The compound function is f = (1 + x)^n: computed exactly with fmpq arithmetic when 1 + x and its
    power are of moderate size; otherwise main = 1 and small = e^(n ln(1 + x)) - 1 for a tiny x,
    main = x^n (exact when it has few bits) and small = x^n ((1 + 1/x)^n - 1) for a large x, and main
    = (1 + x)^n with small = 0 between.

    The result is rounded by bracketing f between consecutive numbers with one more bit than the
    target precision; each comparison with such a number w is the sign of (main - w) + small, with
    the working precision doubling while a comparison is undecided. An exact f with at most prec + 1
    bits is rounded directly.

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
#define MODERATE_BITS (1L << 22)

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

/* A value given exactly: an arf (possibly with a huge exponent) or an fmpq. */
typedef struct
{
    int is_arf;
    arf_t a;
    fmpq_t q;
} value_t;

static void
value_init(value_t * v)
{
    arf_init(v->a);
    fmpq_init(v->q);
    v->is_arf = 0;
}

static void
value_clear(value_t * v)
{
    arf_clear(v->a);
    fmpq_clear(v->q);
}

static int
value_sgn(const value_t * v)
{
    return v->is_arf ? arf_sgn(v->a) : fmpq_sgn(v->q);
}

/* Compares v with the small integer c. */
static int
value_cmp_si(const value_t * v, slong c)
{
    if (v->is_arf)
        return arf_cmp_si(v->a, c);
    fmpq_t t;
    fmpq_init(t);
    fmpq_set_si(t, c, 1);
    int r = fmpq_cmp(v->q, t);
    fmpq_clear(t);
    return r;
}

/* Whether the arf a has a moderate size (exponent and bits below 2^22), and if so sets q. */
static int
arf_moderate_fmpq(fmpq_t q, const arf_t a)
{
    if (arf_is_zero(a))
    {
        fmpq_zero(q);
        return 1;
    }
    if (!arf_is_finite(a) || fmpz_bits(ARF_EXPREF(a)) > 22 || arf_bits(a) > MODERATE_BITS)
        return 0;
    arf_get_fmpq(q, a);
    return 1;
}

/* The value as an fmpq, if moderate. */
static int
value_moderate_fmpq(fmpq_t q, const value_t * v)
{
    if (!v->is_arf)
    {
        fmpq_set(q, v->q);
        return 1;
    }
    return arf_moderate_fmpq(q, v->a);
}

/* If the positive value is a power of 2, sets n (2^n) and returns 1. */
static int
value_power_of_2(fmpz_t n, const value_t * v)
{
    if (v->is_arf)
    {
        if (arf_sgn(v->a) <= 0 || arf_bits(v->a) != 1)
            return 0;
        fmpz_sub_ui(n, ARF_EXPREF(v->a), 1);
        return 1;
    }
    const fmpz * num = fmpq_numref(v->q);
    const fmpz * den = fmpq_denref(v->q);
    if (fmpz_sgn(num) <= 0)
        return 0;
    if (fmpz_is_one(den) && fmpz_val2(num) + 1 == fmpz_bits(num))
    {
        fmpz_set_ui(n, fmpz_val2(num));
        return 1;
    }
    if (fmpz_is_one(num) && fmpz_val2(den) + 1 == fmpz_bits(den))
    {
        fmpz_set_si(n, -(slong) fmpz_val2(den));
        return 1;
    }
    return 0;
}

/* Sets r to a ball containing the value, with precision wp. */
static void
value_ball(arb_t r, const value_t * v, slong wp)
{
    if (v->is_arf)
        arb_set_arf(r, v->a);
    else
        arb_set_fmpq(r, v->q, wp);
}

static ulong
gcd_ui(ulong a, ulong b)
{
    while (b != 0)
    {
        ulong t = a % b;
        a = b;
        b = t;
    }
    return a;
}

/* The perfect-power exponent of the positive fmpz n: 0 for n = 1 (any exponent), otherwise the
   largest k with n = r^k (1 if n is not a perfect power), and r. */
static ulong
perfect_power(fmpz_t r, const fmpz_t n)
{
    if (fmpz_is_one(n))
    {
        fmpz_one(r);
        return 0;
    }
    int k = fmpz_is_perfect_power(r, n);
    if (k == 0)
    {
        fmpz_set(r, n);
        return 1;
    }
    return (ulong) k;
}

/* log_b(v) for positive rationals v and b (b != 1, v != 1) of moderate size, if rational: b =
   g^e with g = b^(1/e) for the largest e, and then v = g^a for an integer a. */
static int
rational_log(fmpq_t q, const fmpq_t v, const fmpq_t b)
{
    fmpz_t rn, rd;
    fmpz_init(rn);
    fmpz_init(rd);
    ulong kn = perfect_power(rn, fmpq_numref(b));
    ulong kd = perfect_power(rd, fmpq_denref(b));
    ulong e = kn == 0 ? kd : kd == 0 ? kn : gcd_ui(kn, kd);
    fmpq_t g, t;
    fmpq_init(g);
    fmpq_init(t);
    fmpz_root(fmpq_numref(g), fmpq_numref(b), e);
    fmpz_root(fmpq_denref(g), fmpq_denref(b), e);
    int found = 0;
    /* a ~ log|v| / log|g| */
    arb_t lv, lg;
    arb_init(lv);
    arb_init(lg);
    arb_set_fmpq(lv, v, 128);
    arb_log(lv, lv, 128);
    arb_set_fmpq(lg, g, 128);
    arb_log(lg, lg, 128);
    arb_div(lv, lv, lg, 128);
    double a0 = arf_get_d(arb_midref(lv), ARF_RND_NEAR);
    if (a0 > -1e7 && a0 < 1e7)
    {
        slong a = (slong) (a0 < 0 ? a0 - 0.5 : a0 + 0.5);
        for (slong d = -1; d <= 1 && !found; d++)
        {
            if (a + d == 0)
                continue;
            fmpq_pow_si(t, g, a + d);
            if (fmpq_equal(t, v))
            {
                fmpq_set_si(q, a + d, e);
                found = 1;
            }
        }
    }
    arb_clear(lv);
    arb_clear(lg);
    fmpq_clear(g);
    fmpq_clear(t);
    fmpz_clear(rn);
    fmpz_clear(rd);
    return found;
}

/* The base: kind 0 = e, 3 = u64, 4 = BigExponentFloat, 5 = Rational. */
typedef struct
{
    int kind;
    value_t v;
} base_t;

/* log_b(v) for a positive value v != 1, if rational. */
static int
exact_log(fmpq_t q, const base_t * b, const value_t * v)
{
    if (b->kind == 0)
        return 0;
    fmpz_t n, s;
    fmpz_init(n);
    fmpz_init(s);
    int found = 0;
    if (value_power_of_2(n, v) && value_power_of_2(s, &b->v))
    {
        if (!fmpz_is_zero(s))
        {
            fmpz_set(fmpq_numref(q), n);
            fmpz_set(fmpq_denref(q), s);
            if (fmpz_sgn(s) < 0)
            {
                fmpz_neg(fmpq_numref(q), fmpq_numref(q));
                fmpz_neg(fmpq_denref(q), fmpq_denref(q));
            }
            fmpq_canonicalise(q);
            found = 1;
        }
    }
    else
    {
        fmpq_t vq, bq;
        fmpq_init(vq);
        fmpq_init(bq);
        if (value_moderate_fmpq(vq, v) && value_moderate_fmpq(bq, &b->v) && !fmpq_is_one(bq)
            && !fmpq_is_one(vq))
        {
            /* For a base below 1, log_b(v) = -log_{1/b}(v). */
            int flip = fmpq_cmp_ui(bq, 1) < 0;
            if (flip)
                fmpq_inv(bq, bq);
            found = rational_log(q, vq, bq);
            if (found && flip)
                fmpq_neg(q, q);
        }
        fmpq_clear(vq);
        fmpq_clear(bq);
    }
    fmpz_clear(n);
    fmpz_clear(s);
    return found;
}

/* Sets r to a ball containing ln(b). Near 1, it is computed as ln(1 + (b - 1)). */
static void
base_ln(arb_t r, const base_t * b, slong wp)
{
    if (b->kind == 0)
    {
        arb_one(r);
        return;
    }
    arb_t t;
    arb_init(t);
    value_ball(t, &b->v, wp + 64);
    int near = value_cmp_si(&b->v, 2) <= 0 && (b->v.is_arf ? arf_cmp_2exp_si(b->v.a, -1) >= 0
                                                           : 1);
    if (!b->v.is_arf)
    {
        fmpq_t h;
        fmpq_init(h);
        fmpq_set_si(h, 1, 2);
        near = near && fmpq_cmp(b->v.q, h) >= 0;
        fmpq_clear(h);
    }
    if (near)
    {
        arb_sub_ui(t, t, 1, b->v.is_arf ? ARF_PREC_EXACT : wp + 64);
        if (!b->v.is_arf)
        {
            fmpq_t d;
            fmpq_init(d);
            fmpq_sub_si(d, b->v.q, 1);
            arb_set_fmpq(t, d, wp + 64);
            fmpq_clear(d);
        }
        arb_log1p(r, t, wp);
    }
    else
        arb_log(r, t, wp);
    arb_clear(t);
}

/* For a ball t with |t| < 2^-8, sets r to a ball containing ln(1 + t) - t. */
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

typedef struct
{
    arb_t main;
    arb_t small;
    arf_t me;
    int main_exact;
} parts_t;

static int
fmpq_is_dyadic(const fmpq_t q)
{
    return fmpz_val2(fmpq_denref(q)) + 1 == fmpz_bits(fmpq_denref(q));
}

/* The decomposition of log_b(x) or log_b(1 + x) (see the top) at working precision wp. */
static void
log_parts(parts_t * p, const base_t * b, const value_t * x, int one_plus, slong wp)
{
    arb_t xb, t, lb, tmp;
    arb_init(xb);
    arb_init(t);
    arb_init(lb);
    arb_init(tmp);
    value_ball(xb, x, wp + 64);
    base_ln(lb, b, wp);
    p->main_exact = 0;
    arb_zero(p->small);
    int use_t = 0;
    if (one_plus)
    {
        arb_set(t, xb);
        use_t = 1;
    }
    else if (x->is_arf ? arf_cmp_2exp_si(x->a, -1) >= 0 && arf_cmp_2exp_si(x->a, 1) <= 0 : 0)
    {
        arb_sub_ui(t, xb, 1, ARF_PREC_EXACT);
        use_t = 1;
    }
    else if (!x->is_arf)
    {
        fmpq_t tq;
        fmpq_init(tq);
        fmpq_sub_si(tq, x->q, 1);
        if (!fmpq_is_zero(tq))
        {
            arb_set_fmpq(t, tq, wp + 64);
            use_t = small_ball(t);
        }
        fmpq_clear(tq);
    }
    if (use_t && small_ball(t))
    {
        log1p_tail(p->small, t, wp);
        arb_div(p->small, p->small, lb, wp);
        if (b->kind == 0 && arb_is_exact(t))
        {
            arb_set(p->main, t);
            arf_set(p->me, arb_midref(t));
            p->main_exact = 1;
        }
        else
            arb_div(p->main, t, lb, wp);
    }
    else if (one_plus && value_cmp_si(x, 2) >= 0)
    {
        fmpq_t q;
        fmpq_init(q);
        if (exact_log(q, b, x) && fmpq_is_dyadic(q))
        {
            arb_set_fmpq(p->main, q, wp);
            arf_set(p->me, arb_midref(p->main));
            p->main_exact = arb_is_exact(p->main);
        }
        if (!p->main_exact)
        {
            arb_log(p->main, xb, wp);
            arb_div(p->main, p->main, lb, wp);
        }
        fmpq_clear(q);
        arb_inv(tmp, xb, wp);
        if (small_ball(tmp))
            log1p_tail(p->small, tmp, wp), arb_add(p->small, p->small, tmp, wp);
        else
            arb_log1p(p->small, tmp, wp);
        arb_div(p->small, p->small, lb, wp);
    }
    else
    {
        if (one_plus)
            arb_log1p(p->main, xb, wp);
        else
            arb_log(p->main, xb, wp);
        arb_div(p->main, p->main, lb, wp);
    }
    arb_clear(xb);
    arb_clear(t);
    arb_clear(lb);
    arb_clear(tmp);
}

/* The decomposition of (1 + x)^n (see the top) at working precision wp. */
static void
compound_parts(parts_t * p, const value_t * x, slong n, slong wp)
{
    arb_t xb, t, u;
    arb_init(xb);
    arb_init(t);
    arb_init(u);
    value_ball(xb, x, wp + 64);
    p->main_exact = 0;
    arb_zero(p->small);
    if (small_ball(xb))
    {
        /* main = 1, small = e^(n ln(1 + x)) - 1 */
        arb_one(p->main);
        arf_one(p->me);
        p->main_exact = 1;
        arb_log1p(t, xb, wp);
        arb_mul_si(t, t, n, wp);
        arb_expm1(p->small, t, wp);
    }
    else if (x->is_arf && arf_cmp_2exp_si(x->a, 8) >= 0)
    {
        /* main = x^n, small = x^n ((1 + 1/x)^n - 1) */
        fmpz_t nn;
        fmpz_init_set_si(nn, n);
        arb_pow_fmpz(p->main, xb, nn, wp);
        if (arb_is_exact(p->main))
        {
            arf_set(p->me, arb_midref(p->main));
            p->main_exact = 1;
        }
        arb_inv(t, xb, wp);
        arb_log1p(t, t, wp);
        arb_mul_si(t, t, n, wp);
        arb_expm1(u, t, wp);
        arb_mul(p->small, p->main, u, wp);
        fmpz_clear(nn);
    }
    else
    {
        fmpz_t nn;
        fmpz_init_set_si(nn, n);
        arb_add_ui(t, xb, 1, x->is_arf ? ARF_PREC_EXACT : wp + 64);
        arb_pow_fmpz(p->main, t, nn, wp);
        fmpz_clear(nn);
    }
    arb_clear(xb);
    arb_clear(t);
    arb_clear(u);
}

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

/* What to round: a logarithm (with base b, possibly of 1 + x) or (1 + x)^n. */
typedef struct
{
    int compound;
    int one_plus;
    const base_t * b;
    slong n;
} fn_t;

static int
round_by_bracketing(arf_t z, const fn_t * f, const value_t * x, slong prec, arf_rnd_t rnd)
{
    slong q = prec + 1;
    parts_t p;
    arb_init(p.main);
    arb_init(p.small);
    arf_init(p.me);
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
        if (f->compound)
            compound_parts(&p, x, f->n, wp);
        else
            log_parts(&p, f->b, x, f->one_plus, wp);
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
    arb_clear(p.main);
    arb_clear(p.small);
    arf_clear(p.me);
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
    fmpq_t zq;
    fmpq_init(zq);
    arf_get_fmpq(zq, z);
    int c = fmpq_cmp(zq, v);
    fmpq_clear(zq);
    return c;
}

/* The class of ln(v): -2 for NaN, and otherwise 3 * sign + (infinite ? 1 : 0) encoded as (s, i). */
typedef struct
{
    int nan;
    int sign;
    int infinite;
} class_t;

static class_t
ln_class_value(const value_t * v, int one_plus)
{
    class_t c = {0, 0, 0};
    if (v->is_arf)
    {
        if (arf_is_nan(v->a))
        {
            c.nan = 1;
            return c;
        }
        if (arf_is_pos_inf(v->a))
        {
            c.sign = 1;
            c.infinite = 1;
            return c;
        }
        if (arf_is_neg_inf(v->a))
        {
            c.nan = 1;
            return c;
        }
    }
    slong edge = one_plus ? -1 : 0;
    int s = value_cmp_si(v, edge);
    if (s < 0)
    {
        c.nan = 1;
        return c;
    }
    if (s == 0)
    {
        c.sign = -1;
        c.infinite = 1;
        return c;
    }
    int t = one_plus ? value_sgn(v) : value_cmp_si(v, 1);
    c.sign = t;
    return c;
}

/* A representative of the class c: NaN, +-inf, 0, or +-1. */
static void
class_representative(arf_t r, class_t c)
{
    if (c.nan)
        arf_nan(r);
    else if (c.infinite)
    {
        if (c.sign > 0)
            arf_pos_inf(r);
        else
            arf_neg_inf(r);
    }
    else
        arf_set_si(r, c.sign);
}

/* The special value of ln(x) / ln(b), or 0 if none (both logarithms finite and nonzero). The
   special values are those of arf_div, applied to representatives of the two classes. */
static int
quotient_special(arf_t z, class_t lx, class_t lb)
{
    if (!lx.nan && !lb.nan && !lx.infinite && !lb.infinite && lx.sign != 0 && lb.sign != 0)
        return 0;
    arf_t a, b;
    arf_init(a);
    arf_init(b);
    class_representative(a, lx);
    class_representative(b, lb);
    arf_div(z, a, b, 30, ARF_RND_DOWN);
    arf_clear(a);
    arf_clear(b);
    return 1;
}

static int
expected_log(arf_t z, const base_t * b, const value_t * x, int one_plus, slong prec, arf_rnd_t rnd)
{
    class_t lb = b->kind == 0 ? (class_t){0, 1, 0} : ln_class_value(&b->v, 0);
    if (quotient_special(z, ln_class_value(x, one_plus), lb))
        return 0;
    /* exact: log_b(v) for v = x, or v = 1 + x when that has a moderate size */
    value_t v;
    value_init(&v);
    int have = 1;
    if (!one_plus)
    {
        v.is_arf = x->is_arf;
        arf_set(v.a, x->a);
        fmpq_set(v.q, x->q);
    }
    else
    {
        fmpq_t xq;
        fmpq_init(xq);
        have = value_moderate_fmpq(xq, x);
        if (!have && x->is_arf && arf_is_int(x->a) &&
            fmpz_cmp_ui(ARF_EXPREF(x->a), arf_bits(x->a) + 64) <= 0 && arf_bits(x->a) < MODERATE_BITS)
            have = 1, arf_get_fmpq(xq, x->a);
        if (have)
        {
            fmpq_add_si(v.q, xq, 1);
            v.is_arf = 0;
        }
        fmpq_clear(xq);
    }
    fmpq_t ex;
    fmpq_init(ex);
    int found = have && value_cmp_si(&v, 1) != 0 && exact_log(ex, b, &v);
    int o = found ? round_fmpq(z, ex, prec, rnd) : 2;
    fmpq_clear(ex);
    value_clear(&v);
    if (found)
        return o;
    fn_t f = {0, one_plus, b, 0};
    return round_by_bracketing(z, &f, x, prec, rnd);
}

static int
expected_compound(arf_t z, const value_t * x, slong n, slong prec, arf_rnd_t rnd)
{
    /* x is a BigExponentFloat */
    const arf_struct * a = x->a;
    if (arf_is_nan(a))
    {
        if (n == 0)
            arf_one(z);
        else
            arf_nan(z);
        return 0;
    }
    if (arf_is_neg_inf(a) || arf_cmp_si(a, -1) < 0)
    {
        arf_nan(z);
        return 0;
    }
    if (n == 0 || arf_is_zero(a))
    {
        arf_one(z);
        return 0;
    }
    if (arf_is_pos_inf(a))
    {
        if (n > 0)
            arf_pos_inf(z);
        else
            arf_zero(z);
        return 0;
    }
    if (arf_cmp_si(a, -1) == 0)
    {
        if (n > 0)
            arf_zero(z);
        else
            arf_pos_inf(z);
        return 0;
    }
    /* exact when 1 + x and its power are of moderate size */
    fmpq_t xq;
    fmpq_init(xq);
    int o = 2;
    if (arf_moderate_fmpq(xq, a))
    {
        fmpq_add_si(xq, xq, 1);
        ulong bits = fmpz_bits(fmpq_numref(xq)) + fmpz_bits(fmpq_denref(xq));
        ulong an = n < 0 ? -(ulong) n : (ulong) n;
        if (bits * an <= MODERATE_BITS)
        {
            fmpq_t r;
            fmpq_init(r);
            fmpq_pow_si(r, xq, n);
            o = round_fmpq(z, r, prec, rnd);
            fmpq_clear(r);
        }
    }
    fmpq_clear(xq);
    if (o != 2)
        return o;
    fn_t f = {1, 0, NULL, n};
    return round_by_bracketing(z, &f, x, prec, rnd);
}

typedef struct
{
    const char * name;
    int x_rational;
    int base_kind; /* 3: u64, 4: BigExponentFloat, 5: Rational, 6: compound exponent */
    int one_plus;
} op_t;

static const op_t OPS[] = {
    {"log_base", 0, 3, 0},
    {"log_base_1_plus_x", 0, 3, 1},
    {"log_base_big_exponent_float_base", 0, 4, 0},
    {"log_base_big_exponent_float_base_1_plus_x", 0, 4, 1},
    {"log_base_rational_base", 0, 5, 0},
    {"log_base_rational_base_1_plus_x", 0, 5, 1},
    {"log_base_rational", 1, 3, 0},
    {"log_base_rational_big_exponent_float_base", 1, 4, 0},
    {"log_base_rational_rational_base", 1, 5, 0},
    {"compound", 0, 6, 0},
};

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
    if (op == NULL || (xtext == NULL) != op->x_rational)
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
    int first = op->x_rational;
    if (n_items != first + 2 + with_rm)
        return report(line_number, "wrong number of arguments");
    if (op->x_rational)
        xtext = items[0];
    char * btext = items[first];
    char * end = NULL;
    long long prec = strtoll(items[first + 1], &end, 10);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    value_t x;
    base_t b;
    value_init(&x);
    value_init(&b.v);
    b.kind = op->base_kind;
    arf_t z, got;
    arf_init(z);
    arf_init(got);
    int res = 0;
    slong n = 0;
    if (op->x_rational ? parse_rational(x.q, xtext) != 0 : arf_parse_base(xtext, 4, x.a) != 0)
        res = report(line_number, "unreadable argument");
    x.is_arf = !op->x_rational;
    if (res == 0)
    {
        char * bend = NULL;
        if (op->base_kind == 3 || op->base_kind == 6)
        {
            long long bv = strtoll(btext, &bend, 10);
            if (*bend != '\0' || (op->base_kind == 3 && bv < 2))
                res = report(line_number, "unreadable base");
            n = (slong) bv;
            fmpq_set_si(b.v.q, (slong) bv, 1);
            b.v.is_arf = 0;
        }
        else if (op->base_kind == 4)
        {
            if (arf_parse_base(btext, 4, b.v.a) != 0)
                res = report(line_number, "unreadable base");
            b.v.is_arf = 1;
        }
        else
        {
            if (parse_rational(b.v.q, btext) != 0 || fmpq_cmp_ui(b.v.q, 1) <= 0)
                res = report(line_number, "unreadable base");
            b.v.is_arf = 0;
        }
    }
    if (res == 0 && (*end != '\0' || prec <= 0))
        res = report(line_number, "unreadable precision");
    else if (res == 0 && with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        int o = op->base_kind == 6 ? expected_compound(z, &x, n, prec, rnd)
                                   : expected_log(z, &b, &x, op->one_plus, prec, rnd);
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
    value_clear(&x);
    value_clear(&b.v);
    arf_clear(z);
    arf_clear(got);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_logarithm_general(const char * arg)
{
    checked = 0;
    int result = arf_for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_logarithm_general", checked);
}
