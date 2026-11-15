/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's forward trigonometric functions against FLINT's arb ball arithmetic.
    The mode `arf_trig` accepts lines of the forms

    - `X.f_prec_round(p, rm) = R`, `X.f_pi_prec_round(p, rm) = R`, and
      `X.f_with_period_prec_round(u, p, rm) = R`;
    - `BigExponentFloat::f_rational_prec_round(Q, p, rm) = R`,
      `BigExponentFloat::f_pi_rational_prec_round(Q, p, rm) = R`, and
      `BigExponentFloat::f_with_period_rational_prec_round(Q, u, p, rm) = R`;

    and their `_prec` forms (without the rounding mode), for f one of sin, cos, tan, cot, sec, csc,
    and sin_cos; R is `(Z, O)`, or `(S, C, O_S, O_C)` for sin_cos. X and Z are BigExponentFloats in
    Malachite's exact hexadecimal format, and Q is a Rational written as `a/b` or `a`.

    The special cases follow Malachite's documentation: NaN and the infinities give NaN, as does a
    period u of 0; the functions at the angle 0 are 0, 1, 0, NaN, 1, and NaN; and, as a
    BigExponentFloat has no negative zero, every pole gives NaN. With a period, the angle in turns,
    t = (x mod u)/u, is formed exactly (for a huge x, from 2^k mod u by modular exponentiation),
    and the values that are rational, at multiples of 1/24 of a turn by Niven's theorem, are exact.

    Otherwise f is written as main + small and rounded by bracketing it between consecutive numbers
    with one more bit than the target precision, each comparison with such a number w being the
    sign of (main - w) + small, with the working precision doubling while it is undecided. For an
    argument y with |y| < 2^(-10) and |y|^4 below 2^(-wp - 10) (and, with a period, 2πx/u for a
    tiny x), main is the leading term
    of f's series (y, 1, or 1/y) and small the rest, enclosed by its next term and a bound on the
    remainder, so that f can be separated from a leading term that is itself a breakpoint;
    otherwise main is arb's value (arb_sin, arb_cos, arb_tan, arb_cot, arb_sec, arb_csc, or, in
    half-turns, arb_sin_pi_fmpq, arb_cos_pi_fmpq, arb_tan_pi, arb_cot_pi, and reciprocals) and
    small is 0.

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

typedef enum { F_SIN, F_COS, F_TAN, F_COT, F_SEC, F_CSC } fn_t;

static const char * FN_NAMES[] = {"sin", "cos", "tan", "cot", "sec", "csc"};

/* The value at the angle 0, as a code: 0, 1, or NaN (2). */
static int
at_zero(fn_t f)
{
    return (f == F_SIN || f == F_TAN) ? 0 : (f == F_COS || f == F_SEC) ? 1 : 2;
}

/* sin(2πj/24) in halves (0, ±1, ±2), or 99 if irrational. */
static int
sin_halves(long j)
{
    switch (j)
    {
        case 0: case 12: return 0;
        case 2: case 10: return 1;
        case 6: return 2;
        case 14: case 22: return -1;
        case 18: return -2;
        default: return 99;
    }
}

/* Sets z to f(2πj/24) and returns 1 if it is rational or a pole (NaN); returns 0 if irrational. */
static int
at_24th(arf_t z, fn_t f, long j)
{
    if (f == F_SIN || f == F_COS || f == F_SEC || f == F_CSC)
    {
        int h = (f == F_SIN || f == F_CSC) ? sin_halves(j) : sin_halves((j + 6) % 24);
        if (h == 99)
            return 0;
        if (f == F_SIN || f == F_COS)
        {
            arf_set_si(z, h);
            arf_mul_2exp_si(z, z, -1);
        }
        else if (h == 0)
            arf_nan(z);
        else
        {
            /* 2/h */
            arf_set_si(z, (h == 1 || h == -1) ? 2 * h : h / 2);
        }
        return 1;
    }
    if (j % 3 != 0)
        return 0;
    long k = (j / 3) % 4;
    if (f == F_COT)
        k = (k + 2) % 4;
    if (k == 0)
        arf_zero(z);
    else if (k == 2)
        arf_nan(z);
    else
        arf_set_si(z, (k == 1) == (f == F_TAN) ? 1 : -1);
    return 1;
}

/* The value being rounded, main + small, at working precision wp. */
typedef struct
{
    fn_t f;
    int turns;        /* the argument is a fraction of a turn t (exact), or 2πx/u for a tiny x */
    const fmpq * t;   /* turns: t in [0, 1); NULL for the tiny-x case */
    const arf_struct * xa;   /* the argument (non-turns), or x (tiny turns case) */
    const fmpq * xq;  /* a Rational argument (non-turns) */
    ulong u;          /* the period, for the tiny turns case */
} value_t;

/* Sets r to the argument y: x itself, a Rational x rounded to wp bits, or 2πx/u. */
static void
argument(arb_t r, const value_t * v, slong wp)
{
    if (v->xa != NULL)
        arb_set_arf(r, v->xa);
    else
        arb_set_fmpq(r, v->xq, wp);
    if (v->turns)
    {
        arb_t p;
        arb_init(p);
        arb_const_pi(p, wp);
        arb_mul_2exp_si(p, p, 1);
        arb_mul(r, r, p, wp);
        arb_div_ui(r, r, v->u, wp);
        arb_clear(p);
    }
}

/* main = leading term of f at the tiny y, small = the rest, from the series: sin y = y - y^3/6 + ...,
   tan y = y + y^3/3 + (2/15)y^5 + ..., cos y = 1 - y^2/2 + ..., sec y = 1 + y^2/2 + (5/24)y^4 + ...,
   cot y = 1/y - y/3 - y^3/45 - ..., csc y = 1/y + y/6 + (7/360)y^3 + .... For |y| <= 2^(-10) the
   remainder after the second term is below twice the next term, which is the radius used. */
static void
tiny_parts(arb_t main, arb_t small, fn_t f, const arb_t y, slong wp)
{
    arb_t y2, t;
    mag_t r;
    arb_init(y2);
    arb_init(t);
    mag_init(r);
    arb_sqr(y2, y, wp);
    switch (f)
    {
        case F_SIN:
        case F_TAN:
            arb_set(main, y);
            arb_mul(small, y2, y, wp);
            arb_div_ui(small, small, f == F_SIN ? 6 : 3, wp);
            if (f == F_SIN)
                arb_neg(small, small);
            /* next term: |y|^5/120 or (2/15)|y|^5; radius twice that */
            arb_mul(t, small, y2, wp);
            break;
        case F_COS:
        case F_SEC:
            arb_one(main);
            arb_mul_2exp_si(small, y2, -1);
            if (f == F_COS)
                arb_neg(small, small);
            arb_mul(t, small, y2, wp);
            break;
        default:
            arb_inv(main, y, wp);
            arb_div_ui(small, y, f == F_COT ? 3 : 6, wp);
            if (f == F_COT)
                arb_neg(small, small);
            arb_mul(t, small, y2, wp);
            break;
    }
    /* |next term| <= |second term| * y^2 (the ratios 1/20, 2/5, 1/12, 5/12, 1/15, 7/60 are all
       below 1), and the remainder is below twice the next term */
    arb_get_mag(r, t);
    mag_mul_2exp_si(r, r, 1);
    arb_add_error_mag(small, r);
    arb_clear(y2);
    arb_clear(t);
    mag_clear(r);
}

static void
eval(arb_t main, arb_t small, const value_t * v, slong wp)
{
    arb_zero(small);
    if (v->turns && v->t != NULL)
    {
        /* g = 2t half-turns */
        fmpq_t g;
        fmpq_init(g);
        fmpq_mul_ui(g, v->t, 2);
        switch (v->f)
        {
            case F_SIN: arb_sin_pi_fmpq(main, g, wp); break;
            case F_COS: arb_cos_pi_fmpq(main, g, wp); break;
            case F_CSC: arb_sin_pi_fmpq(main, g, wp); arb_inv(main, main, wp); break;
            case F_SEC: arb_cos_pi_fmpq(main, g, wp); arb_inv(main, main, wp); break;
            default:
            {
                arb_t a;
                arb_init(a);
                arb_set_fmpq(a, g, wp + 64);
                if (v->f == F_TAN)
                    arb_tan_pi(main, a, wp);
                else
                    arb_cot_pi(main, a, wp);
                arb_clear(a);
            }
        }
        fmpq_clear(g);
        return;
    }
    arb_t y;
    arb_init(y);
    argument(y, v, wp + 64);
    /* the series form when its remainder, about |y|^4 relative to main, is below 2^(-wp - 10) */
    if (arf_cmpabs_2exp_si(arb_midref(y), -10) < 0
        && arf_cmpabs_2exp_si(arb_midref(y), -((wp + 74) / 4) - 1) < 0)
        tiny_parts(main, small, v->f, y, wp + 64);
    else
    {
        switch (v->f)
        {
            case F_SIN: arb_sin(main, y, wp); break;
            case F_COS: arb_cos(main, y, wp); break;
            case F_TAN: arb_tan(main, y, wp); break;
            case F_COT: arb_cot(main, y, wp); break;
            case F_SEC: arb_sec(main, y, wp); break;
            case F_CSC: arb_csc(main, y, wp); break;
        }
    }
    arb_clear(y);
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

/* Rounds the irrational main + small to `prec` bits by bracketing; returns the ordering, or 2. */
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

/* x mod u, exactly, as a Rational in [0, u), for a finite nonzero x; returns 0 on success, or -1
   if x has a huge negative exponent (whose exact value would be expensive). */
static int
arf_mod_ui(fmpq_t r, const arf_t x, ulong u)
{
    fmpz_t m, e, uu, p;
    fmpz_init(m);
    fmpz_init(e);
    fmpz_init(uu);
    fmpz_init(p);
    arf_get_fmpz_2exp(m, e, x);
    fmpz_set_ui(uu, u);
    int res = 0;
    if (fmpz_sgn(e) >= 0)
    {
        fmpz_set_ui(p, 2);
        fmpz_powm(p, p, e, uu);
        fmpz_mul(p, p, m);
        fmpz_mod(p, p, uu);
        fmpz_set(fmpq_numref(r), p);
        fmpz_one(fmpq_denref(r));
    }
    else if (fmpz_cmp_si(e, -(1L << 20)) < 0)
        res = -1;
    else
    {
        /* m 2^e mod u = (m mod u 2^-e) / 2^-e */
        fmpz_neg(e, e);
        fmpz_mul_2exp(uu, uu, fmpz_get_ui(e));
        fmpz_mod(p, m, uu);
        fmpq_set_fmpz_frac(r, p, uu);
        fmpq_mul_ui(r, r, u);
    }
    fmpz_clear(m);
    fmpz_clear(e);
    fmpz_clear(uu);
    fmpz_clear(p);
    return res;
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

/* Rounds an exact value code (0, 1, NaN) or arf to the result. */
static int
round_exact(arf_t z, const arf_t v, slong prec, arf_rnd_t rnd)
{
    if (arf_is_nan(v))
    {
        arf_nan(z);
        return 0;
    }
    arf_set_round(z, v, prec, rnd);
    return arf_cmp(z, v);
}

static int
set_code(arf_t z, int code)
{
    if (code == 2)
        arf_nan(z);
    else
        arf_set_si(z, code);
    return 0;
}

/* The expected f, for a turns argument given by the Rational q (if `rational`) or the arf x, with
   period u. */
static int
expected_turns(arf_t z, fn_t f, const arf_t x, const fmpq_t q, int rational, ulong u, slong prec,
               arf_rnd_t rnd)
{
    if (!rational && (arf_is_nan(x) || arf_is_inf(x)))
        return set_code(z, 2);
    if (u == 0)
        return set_code(z, 2);
    if (rational ? fmpq_is_zero(q) : arf_is_zero(x))
        return set_code(z, at_zero(f));
    fmpq_t t;
    fmpq_init(t);
    int res;
    int tiny = 0;
    if (rational)
    {
        /* t = (q mod u)/u */
        fmpq_set(t, q);
        fmpq_div_ulong(t, t, u);
        fmpz_t fl;
        fmpz_init(fl);
        fmpz_fdiv_q(fl, fmpq_numref(t), fmpq_denref(t));
        fmpq_sub_fmpz(t, t, fl);
        fmpz_clear(fl);
    }
    else if (arf_mod_ui(t, x, u) == 0)
        fmpq_div_ulong(t, t, u);
    else
        tiny = 1;
    if (!tiny)
    {
        fmpq_t j;
        fmpq_init(j);
        fmpq_mul_ui(j, t, 24);
        if (fmpz_is_one(fmpq_denref(j)))
        {
            arf_t v;
            arf_init(v);
            int ok = at_24th(v, f, fmpz_get_si(fmpq_numref(j)) % 24);
            if (ok)
            {
                res = round_exact(z, v, prec, rnd);
                arf_clear(v);
                fmpq_clear(j);
                fmpq_clear(t);
                return res;
            }
            arf_clear(v);
        }
        fmpq_clear(j);
    }
    value_t v;
    v.f = f;
    v.turns = 1;
    v.t = tiny ? NULL : t;
    v.xa = tiny ? x : NULL;
    v.xq = NULL;
    v.u = u;
    res = round_value(z, &v, prec, rnd);
    fmpq_clear(t);
    return res;
}

static int
expected_plain(arf_t z, fn_t f, const arf_t x, const fmpq_t q, int rational, slong prec,
               arf_rnd_t rnd)
{
    if (!rational && (arf_is_nan(x) || arf_is_inf(x)))
        return set_code(z, 2);
    if (rational ? fmpq_is_zero(q) : arf_is_zero(x))
        return set_code(z, at_zero(f));
    value_t v;
    v.f = f;
    v.turns = 0;
    v.t = NULL;
    v.xa = rational ? NULL : x;
    v.xq = rational ? q : NULL;
    v.u = 0;
    return round_value(z, &v, prec, rnd);
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
    /* name = f + family */
    int sincos = 0;
    fn_t f = F_SIN;
    const char * fam = NULL;
    if (strncmp(name, "sin_cos", 7) == 0)
    {
        sincos = 1;
        fam = name + 7;
    }
    else
    {
        for (int i = 0; i < 6 && fam == NULL; i++)
            if (strncmp(name, FN_NAMES[i], 3) == 0)
            {
                f = (fn_t) i;
                fam = name + 3;
            }
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
    int base = rational ? 1 : 0;
    if (n_items != base + period + 1 + with_rm)
        return report(line_number, "wrong number of arguments");
    if (rational)
        xtext = items[0];
    char * end = NULL;
    ulong u = 2;
    if (period)
    {
        u = strtoull(items[base], &end, 10);
        if (*end != '\0')
            return report(line_number, "unreadable period");
    }
    long long prec = strtoll(items[base + period], &end, 10);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    arf_t x, z1, z2;
    fmpq_t q;
    arf_init(x);
    arf_init(z1);
    arf_init(z2);
    fmpq_init(q);
    int res = 0;
    if (rational ? parse_rational(q, xtext) != 0 : arf_parse_base(xtext, 4, x) != 0)
        res = report(line_number, "unreadable argument");
    else if (*end != '\0' || prec <= 0)
        res = report(line_number, "unreadable precision");
    else if (with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        int o1, o2 = 0;
        fn_t f1 = sincos ? F_SIN : f;
        if (period || pi)
            o1 = expected_turns(z1, f1, x, q, rational, u, prec, rnd);
        else
            o1 = expected_plain(z1, f1, x, q, rational, prec, rnd);
        if (sincos)
        {
            if (period || pi)
                o2 = expected_turns(z2, F_COS, x, q, rational, u, prec, rnd);
            else
                o2 = expected_plain(z2, F_COS, x, q, rational, prec, rnd);
        }
        char * value = result;
        size_t rl = strlen(value);
        if (rl < 2 || value[0] != '(' || value[rl - 1] != ')')
            res = report(line_number, "unrecognized result");
        else
        {
            value[rl - 1] = '\0';
            value++;
            if (sincos)
            {
                char * c = split(value, ", ");
                char * os = c == NULL ? NULL : split(c, ", ");
                char * oc = os == NULL ? NULL : split(os, ", ");
                if (oc == NULL)
                    res = report(line_number, "unrecognized result");
                else
                {
                    res = compare(line_number, value, os, z1, o1, exact);
                    if (res == 0)
                        res = compare(line_number, c, oc, z2, o2, exact);
                }
            }
            else
            {
                char * ordering = split(value, ", ");
                res = compare(line_number, value, ordering, z1, o1, exact);
            }
        }
    }
    arf_clear(x);
    arf_clear(z1);
    arf_clear(z2);
    fmpq_clear(q);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_trig(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_trig", checked);
}
