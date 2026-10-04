/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the demos of Malachite's port of FLINT's Schönhage–Strassen code
    (`natural::arithmetic::mul::schonhage_strassen`) against FLINT's `fft` module. Each line has
    the form `name(a, b, ...) = result`, where the arguments and the result are written as Rust's
    `Debug` prints them: numbers, `[x, y, ...]` limb slices, `[[...], ...]` arrays of residues,
    `Some(...)`/`None`, and `(...)` tuples. A `_` argument stands for an output or scratch buffer.
    Each mode parses the inputs, calls the FLINT function on them, prints what it computed in the
    same notation, and compares that text with the result on the line. Residues are compared limb
    for limb, not modulo anything, so the two implementations must agree exactly, including on
    unnormalized residues.

    Where Malachite's function works in place and FLINT's takes separate input and output
    pointers, FLINT is called with them aliased, as its own callers do. In a result, `_` in place
    of a value matches anything; the lines generated from unit tests use it for outputs that the
    tests do not check. Each mode treats any
    other nonempty line as an error, and an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/fft.h>
#include <flint/fmpz_vec.h>
#include <flint/mpn_extras.h>

#include "oracle.h"

/* -- parsing -- */

typedef struct
{
    ulong * d;
    slong len;
} limbs_t;

typedef struct
{
    limbs_t * v;
    slong len;
} limbs_vec_t;

static int
p_lit(const char ** s, const char * lit)
{
    size_t n = strlen(lit);
    if (strncmp(*s, lit, n) != 0)
    {
        return 0;
    }
    *s += n;
    return 1;
}

static int
p_ulong(const char ** s, ulong * x)
{
    const char * p = *s;
    if (*p < '0' || *p > '9')
    {
        return 0;
    }
    ulong r = 0;
    while (*p >= '0' && *p <= '9')
    {
        r = r * 10 + (ulong) (*p - '0');
        p++;
    }
    *x = r;
    *s = p;
    return 1;
}

static int
p_slong(const char ** s, slong * x)
{
    ulong u;
    if (!p_ulong(s, &u))
    {
        return 0;
    }
    *x = (slong) u;
    return 1;
}

static int
p_limbs(const char ** s, limbs_t * x)
{
    if (!p_lit(s, "["))
    {
        return 0;
    }
    slong cap = 4;
    x->d = flint_malloc(cap * sizeof(ulong));
    x->len = 0;
    if (p_lit(s, "]"))
    {
        return 1;
    }
    for (;;)
    {
        if (x->len == cap)
        {
            cap <<= 1;
            x->d = flint_realloc(x->d, cap * sizeof(ulong));
        }
        if (!p_ulong(s, x->d + x->len))
        {
            return 0;
        }
        x->len++;
        if (p_lit(s, "]"))
        {
            return 1;
        }
        if (!p_lit(s, ", "))
        {
            return 0;
        }
    }
}

static int
p_limbs_vec(const char ** s, limbs_vec_t * x)
{
    if (!p_lit(s, "["))
    {
        return 0;
    }
    slong cap = 4;
    x->v = flint_malloc(cap * sizeof(limbs_t));
    x->len = 0;
    if (p_lit(s, "]"))
    {
        return 1;
    }
    for (;;)
    {
        if (x->len == cap)
        {
            cap <<= 1;
            x->v = flint_realloc(x->v, cap * sizeof(limbs_t));
        }
        if (!p_limbs(s, x->v + x->len))
        {
            return 0;
        }
        x->len++;
        if (p_lit(s, "]"))
        {
            return 1;
        }
        if (!p_lit(s, ", "))
        {
            return 0;
        }
    }
}

/* Parses `Some(...)` or `None`, setting `*some`. */
static int
p_opt_limbs(const char ** s, int * some, limbs_t * x)
{
    if (p_lit(s, "None"))
    {
        *some = 0;
        return 1;
    }
    *some = 1;
    return p_lit(s, "Some(") && p_limbs(s, x) && p_lit(s, ")");
}

static int
p_opt_limbs_vec(const char ** s, int * some, limbs_vec_t * x)
{
    if (p_lit(s, "None"))
    {
        *some = 0;
        return 1;
    }
    *some = 1;
    return p_lit(s, "Some(") && p_limbs_vec(s, x) && p_lit(s, ")");
}

static void
limbs_clear(limbs_t * x)
{
    flint_free(x->d);
}

static void
limbs_vec_clear(limbs_vec_t * x)
{
    for (slong i = 0; i < x->len; i++)
    {
        limbs_clear(x->v + i);
    }
    flint_free(x->v);
}

/* An array of pointers to separately allocated copies of the residues, as FLINT's transforms
   take them; they may swap the pointers with those of their scratch buffers. */
static ulong **
residue_pointers(const limbs_vec_t * x)
{
    ulong ** p = flint_malloc(FLINT_MAX(x->len, 1) * sizeof(ulong *));
    for (slong i = 0; i < x->len; i++)
    {
        p[i] = flint_malloc(FLINT_MAX(x->v[i].len, 1) * sizeof(ulong));
        memcpy(p[i], x->v[i].d, x->v[i].len * sizeof(ulong));
    }
    return p;
}

static void
residue_pointers_clear(ulong ** p, slong len)
{
    for (slong i = 0; i < len; i++)
    {
        flint_free(p[i]);
    }
    flint_free(p);
}

/* -- printing -- */

typedef struct
{
    char * s;
    size_t len;
    size_t cap;
} sb_t;

static void
sb_init(sb_t * b)
{
    b->cap = 256;
    b->s = flint_malloc(b->cap);
    b->s[0] = '\0';
    b->len = 0;
}

static void
sb_str(sb_t * b, const char * t)
{
    size_t n = strlen(t);
    while (b->len + n + 1 > b->cap)
    {
        b->cap <<= 1;
        b->s = flint_realloc(b->s, b->cap);
    }
    memcpy(b->s + b->len, t, n + 1);
    b->len += n;
}

static void
sb_ulong(sb_t * b, ulong x)
{
    char t[32];
    flint_sprintf(t, "%wu", x);
    sb_str(b, t);
}

static void
sb_limbs(sb_t * b, const ulong * d, slong len)
{
    sb_str(b, "[");
    for (slong i = 0; i < len; i++)
    {
        if (i != 0)
        {
            sb_str(b, ", ");
        }
        sb_ulong(b, d[i]);
    }
    sb_str(b, "]");
}

static void
sb_residues(sb_t * b, ulong * const * p, slong count, slong size)
{
    sb_str(b, "[");
    for (slong i = 0; i < count; i++)
    {
        if (i != 0)
        {
            sb_str(b, ", ");
        }
        sb_limbs(b, p[i], size);
    }
    sb_str(b, "]");
}

static void
sb_fmpz_vec(sb_t * b, const fmpz * v, slong len)
{
    sb_str(b, "[");
    for (slong i = 0; i < len; i++)
    {
        if (i != 0)
        {
            sb_str(b, ", ");
        }
        char * t = fmpz_get_str(NULL, 10, v + i);
        sb_str(b, t);
        flint_free(t);
    }
    sb_str(b, "]");
}

/* -- comparing -- */

/* Returns the end of the value that starts at `s`: a number, `[...]`, `(...)`, `Some(...)`,
   `None`, or `_`. */
static const char *
skip_value(const char * s)
{
    int depth = 0;
    for (;; s++)
    {
        char c = *s;
        if (c == '\0')
        {
            return s;
        }
        if (c == '[' || c == '(')
        {
            depth++;
        }
        else if (c == ']' || c == ')')
        {
            if (depth == 0)
            {
                return s;
            }
            depth--;
        }
        else if (c == ',' && depth == 0)
        {
            return s;
        }
    }
}

/* Whether `actual` matches `expected`, in which a `_` in place of a value matches any value. The
   unit-test lines use `_` for outputs that the tests do not pin down, such as the inputs that an
   inverse butterfly overwrites. */
static int
wildcard_equal(const char * expected, const char * actual)
{
    const char * e = expected;
    const char * a = actual;
    while (*e != '\0')
    {
        int value_start = e == expected || e[-1] == ' ' || e[-1] == '(' || e[-1] == '[';
        if (*e == '_' && value_start
            && (e[1] == ',' || e[1] == ')' || e[1] == ']' || e[1] == '\0'))
        {
            e++;
            a = skip_value(a);
            continue;
        }
        if (*e != *a)
        {
            return 0;
        }
        e++;
        a++;
    }
    return *a == '\0';
}

/* -- the modes -- */

typedef int (* check_t)(const char * args, sb_t * actual);

static const char * current_prefix;
static check_t current_check;
static const char * current_mode_name;
static long checked;

static int
handle_line(char * line, int line_number)
{
    size_t n = strlen(line);
    while (n > 0 && (line[n - 1] == '\n' || line[n - 1] == '\r'))
    {
        line[--n] = '\0';
    }
    if (n == 0)
    {
        return 0;
    }
    size_t prefix_len = strlen(current_prefix);
    if (strncmp(line, current_prefix, prefix_len) != 0 || line[prefix_len] != '(')
    {
        flint_printf("%s: line %d has an unexpected shape: %s\n", current_mode_name, line_number,
                     line);
        return 1;
    }
    char * eq = strstr(line, ") = ");
    /* The arguments contain no ") = ", but a tuple result may contain ")"; find the separator
       that ends the argument list, which is the first one. */
    if (eq == NULL)
    {
        flint_printf("%s: line %d has no result: %s\n", current_mode_name, line_number, line);
        return 1;
    }
    *eq = '\0';
    const char * args = line + prefix_len + 1;
    const char * expected = eq + 4;
    sb_t actual;
    sb_init(&actual);
    if (!current_check(args, &actual))
    {
        flint_printf("%s: cannot parse line %d: %s\n", current_mode_name, line_number, line);
        flint_free(actual.s);
        return 1;
    }
    if (!wildcard_equal(expected, actual.s))
    {
        flint_printf("%s: mismatch on line %d\n  line:  %s) = %s\n  FLINT: %s\n",
                     current_mode_name, line_number, line, expected, actual.s);
        flint_free(actual.s);
        return 1;
    }
    flint_free(actual.s);
    checked++;
    return 0;
}

static int
run_fft_mode(const char * path, const char * mode_name, const char * prefix, check_t check)
{
    current_prefix = prefix;
    current_check = check;
    current_mode_name = mode_name;
    checked = 0;
    int result = for_each_line(path, handle_line);
    if (result != 0)
    {
        return result;
    }
    return require_some_lines(mode_name, checked);
}

/* Several Malachite functions map to one FLINT function; this runs a mode over lines of any of
   them. */
typedef struct
{
    const char * prefix;
    check_t check;
} prefix_check_t;

static const prefix_check_t * current_alternatives;
static slong current_alternative_count;

static int
handle_line_alternatives(char * line, int line_number)
{
    for (slong i = 0; i < current_alternative_count; i++)
    {
        size_t prefix_len = strlen(current_alternatives[i].prefix);
        if (strncmp(line, current_alternatives[i].prefix, prefix_len) == 0
            && line[prefix_len] == '(')
        {
            current_prefix = current_alternatives[i].prefix;
            current_check = current_alternatives[i].check;
            return handle_line(line, line_number);
        }
    }
    if (line[0] == '\n' || line[0] == '\0')
    {
        return 0;
    }
    flint_printf("%s: line %d has an unexpected shape: %s", current_mode_name, line_number, line);
    return 1;
}

static int
run_fft_mode_alternatives(const char * path, const char * mode_name,
                          const prefix_check_t * alternatives, slong count)
{
    current_alternatives = alternatives;
    current_alternative_count = count;
    current_mode_name = mode_name;
    checked = 0;
    int result = for_each_line(path, handle_line_alternatives);
    if (result != 0)
    {
        return result;
    }
    return require_some_lines(mode_name, checked);
}

/* limbs_add_signed_limb_mod_2expp1(r, limbs, c) = r */
static int
check_addmod_2expp1_1(const char * s, sb_t * b)
{
    limbs_t r;
    slong limbs;
    ulong c;
    if (!(p_limbs(&s, &r) && p_lit(&s, ", ") && p_slong(&s, &limbs) && p_lit(&s, ", ")
          && p_ulong(&s, &c) && *s == '\0'))
    {
        return 0;
    }
    mpn_addmod_2expp1_1(r.d, limbs, (slong) c);
    sb_limbs(b, r.d, r.len);
    limbs_clear(&r);
    return 1;
}

/* limbs_sum_diff(_, _, x, y, n) = (carry, s, d) */
static int
check_sumdiff(const char * s, sb_t * b)
{
    limbs_t x, y;
    slong n;
    if (!(p_lit(&s, "_, _, ") && p_limbs(&s, &x) && p_lit(&s, ", ") && p_limbs(&s, &y)
          && p_lit(&s, ", ") && p_slong(&s, &n) && *s == '\0'))
    {
        return 0;
    }
    ulong * sum = flint_malloc(FLINT_MAX(n, 1) * sizeof(ulong));
    ulong * diff = flint_malloc(FLINT_MAX(n, 1) * sizeof(ulong));
    ulong carry = flint_mpn_sumdiff_n(sum, diff, x.d, y.d, n);
    sb_str(b, "(");
    sb_ulong(b, carry);
    sb_str(b, ", ");
    sb_limbs(b, sum, n);
    sb_str(b, ", ");
    sb_limbs(b, diff, n);
    sb_str(b, ")");
    flint_free(sum);
    flint_free(diff);
    limbs_clear(&x);
    limbs_clear(&y);
    return 1;
}

/* limbs_norm_mod_2expp1(t, limbs) = t */
static int
check_normmod(const char * s, sb_t * b)
{
    limbs_t t;
    slong limbs;
    if (!(p_limbs(&s, &t) && p_lit(&s, ", ") && p_slong(&s, &limbs) && *s == '\0'))
    {
        return 0;
    }
    mpn_normmod_2expp1(t.d, limbs);
    sb_limbs(b, t.d, t.len);
    limbs_clear(&t);
    return 1;
}

/* limbs_neg_mod_2expp1_to_out(_, a, limbs) = z */
static int
check_negmod(const char * s, sb_t * b)
{
    limbs_t a;
    slong limbs;
    if (!(p_lit(&s, "_, ") && p_limbs(&s, &a) && p_lit(&s, ", ") && p_slong(&s, &limbs)
          && *s == '\0'))
    {
        return 0;
    }
    ulong * z = flint_malloc((limbs + 1) * sizeof(ulong));
    mpn_negmod_2expp1(z, a.d, limbs);
    sb_limbs(b, z, limbs + 1);
    flint_free(z);
    limbs_clear(&a);
    return 1;
}

typedef void (* shift_fn_t)(mp_limb_t * t, mp_limb_t * i1, mp_size_t limbs, flint_bitcnt_t d);

/* limbs_*_2exp_mod_2expp1_in_place(t, limbs, d) = t, and
   limbs_*_2exp_mod_2expp1_to_out(_, i1, limbs, d) = t */
static int
check_shift(const char * s, sb_t * b, shift_fn_t f)
{
    int to_out = p_lit(&s, "_, ");
    limbs_t i1;
    slong limbs;
    ulong d;
    if (!(p_limbs(&s, &i1) && p_lit(&s, ", ") && p_slong(&s, &limbs) && p_lit(&s, ", ")
          && p_ulong(&s, &d) && *s == '\0'))
    {
        return 0;
    }
    if (to_out)
    {
        ulong * t = flint_malloc((limbs + 1) * sizeof(ulong));
        f(t, i1.d, limbs, d);
        sb_limbs(b, t, limbs + 1);
        flint_free(t);
    }
    else
    {
        f(i1.d, i1.d, limbs, d);
        sb_limbs(b, i1.d, limbs + 1);
    }
    limbs_clear(&i1);
    return 1;
}

static int
check_mul_2expmod(const char * s, sb_t * b)
{
    return check_shift(s, b, mpn_mul_2expmod_2expp1);
}

static int
check_div_2expmod(const char * s, sb_t * b)
{
    return check_shift(s, b, mpn_div_2expmod_2expp1);
}

/* limbs_fft_adjust(_, i1, i, limbs, w) = r, and
   limbs_fft_adjust_sqrt2(_, i1, i, limbs, w, _) = r */
static int
check_adjust_common(const char * s, sb_t * b, int sqrt2)
{
    limbs_t i1;
    slong i, limbs;
    ulong w;
    if (!(p_lit(&s, "_, ") && p_limbs(&s, &i1) && p_lit(&s, ", ") && p_slong(&s, &i)
          && p_lit(&s, ", ") && p_slong(&s, &limbs) && p_lit(&s, ", ") && p_ulong(&s, &w)))
    {
        return 0;
    }
    if (sqrt2 ? !p_lit(&s, ", _") : 0)
    {
        return 0;
    }
    if (*s != '\0')
    {
        return 0;
    }
    ulong * r = flint_malloc((limbs + 1) * sizeof(ulong));
    if (sqrt2)
    {
        ulong * temp = flint_malloc((limbs + 1) * sizeof(ulong));
        fft_adjust_sqrt2(r, i1.d, i, limbs, w, temp);
        flint_free(temp);
    }
    else
    {
        fft_adjust(r, i1.d, i, limbs, w);
    }
    sb_limbs(b, r, limbs + 1);
    flint_free(r);
    limbs_clear(&i1);
    return 1;
}

static int
check_adjust(const char * s, sb_t * b)
{
    return check_adjust_common(s, b, 0);
}

static int
check_adjust_sqrt2(const char * s, sb_t * b)
{
    return check_adjust_common(s, b, 1);
}

/* Parses `_, _, i1, i2, ` at the start of a butterfly's arguments. */
static int
p_butterfly_inputs(const char ** s, limbs_t * i1, limbs_t * i2)
{
    return p_lit(s, "_, _, ") && p_limbs(s, i1) && p_lit(s, ", ") && p_limbs(s, i2)
        && p_lit(s, ", ");
}

static void
sb_pair_or_quadruple(sb_t * b, const ulong * t, const ulong * u, const ulong * i1,
                     const ulong * i2, slong size)
{
    sb_str(b, "(");
    sb_limbs(b, t, size);
    sb_str(b, ", ");
    sb_limbs(b, u, size);
    if (i1 != NULL)
    {
        sb_str(b, ", ");
        sb_limbs(b, i1, size);
        sb_str(b, ", ");
        sb_limbs(b, i2, size);
    }
    sb_str(b, ")");
}

/* limbs_butterfly_lsh_b(_, _, i1, i2, limbs, x, y) = (t, u), and
   limbs_butterfly_rsh_b(_, _, i1, i2, limbs, x, y) = (t, u, i1, i2) */
static int
check_butterfly_b(const char * s, sb_t * b, int rsh)
{
    limbs_t i1, i2;
    slong limbs, x, y;
    if (!(p_butterfly_inputs(&s, &i1, &i2) && p_slong(&s, &limbs) && p_lit(&s, ", ")
          && p_slong(&s, &x) && p_lit(&s, ", ") && p_slong(&s, &y) && *s == '\0'))
    {
        return 0;
    }
    ulong * t = flint_malloc((limbs + 1) * sizeof(ulong));
    ulong * u = flint_malloc((limbs + 1) * sizeof(ulong));
    if (rsh)
    {
        butterfly_rshB(t, u, i1.d, i2.d, limbs, x, y);
        sb_pair_or_quadruple(b, t, u, i1.d, i2.d, limbs + 1);
    }
    else
    {
        butterfly_lshB(t, u, i1.d, i2.d, limbs, x, y);
        sb_pair_or_quadruple(b, t, u, NULL, NULL, limbs + 1);
    }
    flint_free(t);
    flint_free(u);
    limbs_clear(&i1);
    limbs_clear(&i2);
    return 1;
}

static int
check_butterfly_lshB(const char * s, sb_t * b)
{
    return check_butterfly_b(s, b, 0);
}

static int
check_butterfly_rshB(const char * s, sb_t * b)
{
    return check_butterfly_b(s, b, 1);
}

/* The butterflies with an index and a twiddle exponent:
   limbs_[i]fft_butterfly[_sqrt2](_, _, i1, i2, i, limbs, w[, _]) = (s, t[, i1, i2]) */
static int
check_butterfly_iw(const char * s, sb_t * b, int inverse, int sqrt2)
{
    limbs_t i1, i2;
    slong i, limbs;
    ulong w;
    if (!(p_butterfly_inputs(&s, &i1, &i2) && p_slong(&s, &i) && p_lit(&s, ", ")
          && p_slong(&s, &limbs) && p_lit(&s, ", ") && p_ulong(&s, &w)))
    {
        return 0;
    }
    if (sqrt2 && !p_lit(&s, ", _"))
    {
        return 0;
    }
    if (*s != '\0')
    {
        return 0;
    }
    ulong * out1 = flint_malloc((limbs + 1) * sizeof(ulong));
    ulong * out2 = flint_malloc((limbs + 1) * sizeof(ulong));
    ulong * temp = flint_malloc((limbs + 1) * sizeof(ulong));
    if (inverse)
    {
        if (sqrt2)
        {
            ifft_butterfly_sqrt2(out1, out2, i1.d, i2.d, i, limbs, w, temp);
        }
        else
        {
            ifft_butterfly(out1, out2, i1.d, i2.d, i, limbs, w);
        }
        sb_pair_or_quadruple(b, out1, out2, i1.d, i2.d, limbs + 1);
    }
    else
    {
        if (sqrt2)
        {
            fft_butterfly_sqrt2(out1, out2, i1.d, i2.d, i, limbs, w, temp);
        }
        else
        {
            fft_butterfly(out1, out2, i1.d, i2.d, i, limbs, w);
        }
        sb_pair_or_quadruple(b, out1, out2, NULL, NULL, limbs + 1);
    }
    flint_free(out1);
    flint_free(out2);
    flint_free(temp);
    limbs_clear(&i1);
    limbs_clear(&i2);
    return 1;
}

static int
check_fft_butterfly(const char * s, sb_t * b)
{
    return check_butterfly_iw(s, b, 0, 0);
}

static int
check_ifft_butterfly(const char * s, sb_t * b)
{
    return check_butterfly_iw(s, b, 1, 0);
}

static int
check_fft_butterfly_sqrt2(const char * s, sb_t * b)
{
    return check_butterfly_iw(s, b, 0, 1);
}

static int
check_ifft_butterfly_sqrt2(const char * s, sb_t * b)
{
    return check_butterfly_iw(s, b, 1, 1);
}

/* limbs_[i]fft_butterfly_twiddle(_, _, s, t, limbs, b1, b2) = (u, v[, s, t]) */
static int
check_butterfly_twiddle(const char * s, sb_t * b, int inverse)
{
    limbs_t x, y;
    slong limbs;
    ulong b1, b2;
    if (!(p_butterfly_inputs(&s, &x, &y) && p_slong(&s, &limbs) && p_lit(&s, ", ")
          && p_ulong(&s, &b1) && p_lit(&s, ", ") && p_ulong(&s, &b2) && *s == '\0'))
    {
        return 0;
    }
    ulong * u = flint_malloc((limbs + 1) * sizeof(ulong));
    ulong * v = flint_malloc((limbs + 1) * sizeof(ulong));
    if (inverse)
    {
        ifft_butterfly_twiddle(u, v, x.d, y.d, limbs, b1, b2);
        sb_pair_or_quadruple(b, u, v, x.d, y.d, limbs + 1);
    }
    else
    {
        fft_butterfly_twiddle(u, v, x.d, y.d, limbs, b1, b2);
        sb_pair_or_quadruple(b, u, v, NULL, NULL, limbs + 1);
    }
    flint_free(u);
    flint_free(v);
    limbs_clear(&x);
    limbs_clear(&y);
    return 1;
}

static int
check_fft_butterfly_twiddle(const char * s, sb_t * b)
{
    return check_butterfly_twiddle(s, b, 0);
}

static int
check_ifft_butterfly_twiddle(const char * s, sb_t * b)
{
    return check_butterfly_twiddle(s, b, 1);
}

/* The transforms on an array of residues. Which extra arguments follow `ii, n, w` depends on the
   transform. */
typedef enum
{
    T_FFT_RADIX2,
    T_IFFT_RADIX2,
    T_FFT_TRUNCATE1,
    T_FFT_TRUNCATE,
    T_IFFT_TRUNCATE1,
    T_IFFT_TRUNCATE,
    T_FFT_TRUNCATE_SQRT2,
    T_IFFT_TRUNCATE_SQRT2,
    T_FFT_NEGACYCLIC,
    T_IFFT_NEGACYCLIC,
    T_FFT_MFA_OUTER,
    T_IFFT_MFA_OUTER
} transform_t;

static transform_t current_transform;

/* fft_radix2(ii, n, w, _, _) = ii, fft_truncate(ii, n, w, _, _, trunc) = ii,
   fft_truncate_sqrt2(ii, n, w, _, _, _, trunc) = ii, fft_negacyclic(ii, n, w, _, _, _) = ii,
   fft_mfa_truncate_sqrt2_outer(ii, n, w, _, _, _, n1, trunc) = ii, and their inverses */
static int
check_transform(const char * s, sb_t * b)
{
    limbs_vec_t ii;
    slong n, trunc = 0, n1 = 0;
    ulong w;
    if (!(p_limbs_vec(&s, &ii) && p_lit(&s, ", ") && p_slong(&s, &n) && p_lit(&s, ", ")
          && p_ulong(&s, &w) && p_lit(&s, ", _, _")))
    {
        return 0;
    }
    transform_t t = current_transform;
    int three_temps = t == T_FFT_TRUNCATE_SQRT2 || t == T_IFFT_TRUNCATE_SQRT2
        || t == T_FFT_NEGACYCLIC || t == T_IFFT_NEGACYCLIC || t == T_FFT_MFA_OUTER
        || t == T_IFFT_MFA_OUTER;
    if (three_temps && !p_lit(&s, ", _"))
    {
        return 0;
    }
    if ((t == T_FFT_MFA_OUTER || t == T_IFFT_MFA_OUTER)
        && !(p_lit(&s, ", ") && p_slong(&s, &n1)))
    {
        return 0;
    }
    int truncated = t != T_FFT_RADIX2 && t != T_IFFT_RADIX2 && t != T_FFT_NEGACYCLIC
        && t != T_IFFT_NEGACYCLIC;
    if (truncated && !(p_lit(&s, ", ") && p_slong(&s, &trunc)))
    {
        return 0;
    }
    if (*s != '\0')
    {
        return 0;
    }
    slong size = ii.v[0].len;
    ulong ** p = residue_pointers(&ii);
    ulong * t1 = flint_malloc(size * sizeof(ulong));
    ulong * t2 = flint_malloc(size * sizeof(ulong));
    ulong * temp = flint_malloc(size * sizeof(ulong));
    switch (t)
    {
        case T_FFT_RADIX2:
            fft_radix2(p, n, w, &t1, &t2);
            break;
        case T_IFFT_RADIX2:
            ifft_radix2(p, n, w, &t1, &t2);
            break;
        case T_FFT_TRUNCATE1:
            fft_truncate1(p, n, w, &t1, &t2, trunc);
            break;
        case T_FFT_TRUNCATE:
            fft_truncate(p, n, w, &t1, &t2, trunc);
            break;
        case T_IFFT_TRUNCATE1:
            ifft_truncate1(p, n, w, &t1, &t2, trunc);
            break;
        case T_IFFT_TRUNCATE:
            ifft_truncate(p, n, w, &t1, &t2, trunc);
            break;
        case T_FFT_TRUNCATE_SQRT2:
            fft_truncate_sqrt2(p, n, w, &t1, &t2, &temp, trunc);
            break;
        case T_IFFT_TRUNCATE_SQRT2:
            ifft_truncate_sqrt2(p, n, w, &t1, &t2, &temp, trunc);
            break;
        case T_FFT_NEGACYCLIC:
            fft_negacyclic(p, n, w, &t1, &t2, &temp);
            break;
        case T_IFFT_NEGACYCLIC:
            ifft_negacyclic(p, n, w, &t1, &t2, &temp);
            break;
        case T_FFT_MFA_OUTER:
            fft_mfa_truncate_sqrt2_outer(p, n, w, &t1, &t2, &temp, n1, trunc);
            break;
        case T_IFFT_MFA_OUTER:
            ifft_mfa_truncate_sqrt2_outer(p, n, w, &t1, &t2, &temp, n1, trunc);
            break;
    }
    sb_residues(b, p, ii.len, size);
    residue_pointers_clear(p, ii.len);
    flint_free(t1);
    flint_free(t2);
    flint_free(temp);
    limbs_vec_clear(&ii);
    return 1;
}

typedef enum
{
    TW_FFT_RADIX2,
    TW_IFFT_RADIX2,
    TW_FFT_TRUNCATE1,
    TW_IFFT_TRUNCATE1
} twiddle_t;

static twiddle_t current_twiddle;

/* fft_radix2_twiddle(ii, offset, is, n, w, _, _, ws, r, c, rs) = ii, and the others with a
   trailing `trunc`; the transform is applied to `ii + offset`. */
static int
check_twiddle(const char * s, sb_t * b)
{
    limbs_vec_t ii;
    slong offset, is, n, r, c, rs, trunc = 0;
    ulong w, ws;
    if (!(p_limbs_vec(&s, &ii) && p_lit(&s, ", ") && p_slong(&s, &offset) && p_lit(&s, ", ")
          && p_slong(&s, &is) && p_lit(&s, ", ") && p_slong(&s, &n) && p_lit(&s, ", ")
          && p_ulong(&s, &w) && p_lit(&s, ", _, _, ") && p_ulong(&s, &ws) && p_lit(&s, ", ")
          && p_slong(&s, &r) && p_lit(&s, ", ") && p_slong(&s, &c) && p_lit(&s, ", ")
          && p_slong(&s, &rs)))
    {
        return 0;
    }
    twiddle_t t = current_twiddle;
    if ((t == TW_FFT_TRUNCATE1 || t == TW_IFFT_TRUNCATE1)
        && !(p_lit(&s, ", ") && p_slong(&s, &trunc)))
    {
        return 0;
    }
    if (*s != '\0')
    {
        return 0;
    }
    slong size = ii.v[0].len;
    ulong ** p = residue_pointers(&ii);
    ulong * t1 = flint_malloc(size * sizeof(ulong));
    ulong * t2 = flint_malloc(size * sizeof(ulong));
    switch (t)
    {
        case TW_FFT_RADIX2:
            fft_radix2_twiddle(p + offset, is, n, w, &t1, &t2, ws, r, c, rs);
            break;
        case TW_IFFT_RADIX2:
            ifft_radix2_twiddle(p + offset, is, n, w, &t1, &t2, ws, r, c, rs);
            break;
        case TW_FFT_TRUNCATE1:
            fft_truncate1_twiddle(p + offset, is, n, w, &t1, &t2, ws, r, c, rs, trunc);
            break;
        case TW_IFFT_TRUNCATE1:
            ifft_truncate1_twiddle(p + offset, is, n, w, &t1, &t2, ws, r, c, rs, trunc);
            break;
    }
    sb_residues(b, p, ii.len, size);
    residue_pointers_clear(p, ii.len);
    flint_free(t1);
    flint_free(t2);
    limbs_vec_clear(&ii);
    return 1;
}

static void
sb_option_residues(sb_t * b, ulong * const * p, slong count, slong size, int some)
{
    if (some)
    {
        sb_str(b, "Some(");
        sb_residues(b, p, count, size);
        sb_str(b, ")");
    }
    else
    {
        sb_str(b, "None");
    }
}

/* fft_mfa_truncate_sqrt2_inner(ii, jj, n, w, _, _, _, n1, trunc, _) = (ii, jj) */
static int
check_mfa_inner(const char * s, sb_t * b)
{
    limbs_vec_t ii, jj;
    int some;
    slong n, n1, trunc;
    ulong w;
    if (!(p_limbs_vec(&s, &ii) && p_lit(&s, ", ") && p_opt_limbs_vec(&s, &some, &jj)
          && p_lit(&s, ", ") && p_slong(&s, &n) && p_lit(&s, ", ") && p_ulong(&s, &w)
          && p_lit(&s, ", _, _, _, ") && p_slong(&s, &n1) && p_lit(&s, ", ")
          && p_slong(&s, &trunc) && p_lit(&s, ", _") && *s == '\0'))
    {
        return 0;
    }
    slong size = ii.v[0].len;
    ulong ** p = residue_pointers(&ii);
    ulong ** q = some ? residue_pointers(&jj) : p;
    ulong * t1 = flint_malloc(size * sizeof(ulong));
    ulong * t2 = flint_malloc(size * sizeof(ulong));
    ulong * temp = flint_malloc(size * sizeof(ulong));
    ulong * tt = flint_malloc(2 * size * sizeof(ulong));
    fft_mfa_truncate_sqrt2_inner(p, q, n, w, &t1, &t2, &temp, n1, trunc, &tt);
    sb_str(b, "(");
    sb_residues(b, p, ii.len, size);
    sb_str(b, ", ");
    sb_option_residues(b, q, some ? jj.len : 0, size, some);
    sb_str(b, ")");
    if (some)
    {
        residue_pointers_clear(q, jj.len);
        limbs_vec_clear(&jj);
    }
    residue_pointers_clear(p, ii.len);
    flint_free(t1);
    flint_free(t2);
    flint_free(temp);
    flint_free(tt);
    limbs_vec_clear(&ii);
    return 1;
}

/* limbs_mul_mod_2expp1_basecase(xs, ys, c, b, _) = (carry, xs) */
static int
check_mulmod_basecase(const char * s, sb_t * b)
{
    limbs_t xs, ys;
    int some;
    ulong c, bits;
    if (!(p_limbs(&s, &xs) && p_lit(&s, ", ") && p_opt_limbs(&s, &some, &ys) && p_lit(&s, ", ")
          && p_ulong(&s, &c) && p_lit(&s, ", ") && p_ulong(&s, &bits) && p_lit(&s, ", _")
          && *s == '\0'))
    {
        return 0;
    }
    ulong * tp = flint_malloc(2 * FLINT_MAX(xs.len, 1) * sizeof(ulong));
    int carry = flint_mpn_mulmod_2expp1_basecase(xs.d, xs.d, some ? ys.d : xs.d, (int) c, bits,
                                                 tp);
    sb_str(b, "(");
    sb_ulong(b, (ulong) carry);
    sb_str(b, ", ");
    sb_limbs(b, xs.d, xs.len);
    sb_str(b, ")");
    flint_free(tp);
    if (some)
    {
        limbs_clear(&ys);
    }
    limbs_clear(&xs);
    return 1;
}

/* fft_naive_convolution_1(_, ii, jj, m) = r */
static int
check_naive_convolution_1(const char * s, sb_t * b)
{
    limbs_t ii, jj;
    slong m;
    if (!(p_lit(&s, "_, ") && p_limbs(&s, &ii) && p_lit(&s, ", ") && p_limbs(&s, &jj)
          && p_lit(&s, ", ") && p_slong(&s, &m) && *s == '\0'))
    {
        return 0;
    }
    ulong * r = flint_malloc(FLINT_MAX(m, 1) * sizeof(ulong));
    fft_naive_convolution_1(r, ii.d, jj.d, m);
    sb_limbs(b, r, m);
    flint_free(r);
    limbs_clear(&ii);
    limbs_clear(&jj);
    return 1;
}

/* fft_mulmod_2expp1_negacyclic(r1, i2, r_limbs, depth, w) = r1 */
static int
check_mulmod_negacyclic(const char * s, sb_t * b)
{
    limbs_t r1, i2;
    int some;
    slong r_limbs;
    ulong depth, w;
    if (!(p_limbs(&s, &r1) && p_lit(&s, ", ") && p_opt_limbs(&s, &some, &i2) && p_lit(&s, ", ")
          && p_slong(&s, &r_limbs) && p_lit(&s, ", ") && p_ulong(&s, &depth) && p_lit(&s, ", ")
          && p_ulong(&s, &w) && *s == '\0'))
    {
        return 0;
    }
    _fft_mulmod_2expp1(r1.d, r1.d, some ? i2.d : r1.d, r_limbs, depth, w);
    sb_limbs(b, r1.d, r1.len);
    if (some)
    {
        limbs_clear(&i2);
    }
    limbs_clear(&r1);
    return 1;
}

/* fft_mulmod_2expp1(r, i2, n, w, _) = r */
static int
check_mulmod(const char * s, sb_t * b)
{
    limbs_t r, i2;
    int some;
    slong n;
    ulong w;
    if (!(p_limbs(&s, &r) && p_lit(&s, ", ") && p_opt_limbs(&s, &some, &i2) && p_lit(&s, ", ")
          && p_slong(&s, &n) && p_lit(&s, ", ") && p_ulong(&s, &w) && p_lit(&s, ", _")
          && *s == '\0'))
    {
        return 0;
    }
    ulong * tt = flint_malloc(2 * r.len * sizeof(ulong));
    fft_mulmod_2expp1(r.d, r.d, some ? i2.d : r.d, n, w, tt);
    sb_limbs(b, r.d, r.len);
    flint_free(tt);
    if (some)
    {
        limbs_clear(&i2);
    }
    limbs_clear(&r);
    return 1;
}

/* fft_adjust_limbs(limbs) = limbs */
static int
check_adjust_limbs(const char * s, sb_t * b)
{
    slong limbs;
    if (!(p_slong(&s, &limbs) && *s == '\0'))
    {
        return 0;
    }
    sb_ulong(b, (ulong) fft_adjust_limbs(limbs));
    return 1;
}

/* fft_split_limbs(_, limbs, total_limbs, coeff_limbs, output_limbs) = (length, poly), and
   fft_split_bits(_, limbs, total_limbs, bits, output_limbs) = (length, poly) */
static int
check_split(const char * s, sb_t * b, int bits_mode)
{
    limbs_t limbs;
    slong total, output;
    ulong x;
    if (!(p_lit(&s, "_, ") && p_limbs(&s, &limbs) && p_lit(&s, ", ") && p_slong(&s, &total)
          && p_lit(&s, ", ") && p_ulong(&s, &x) && p_lit(&s, ", ") && p_slong(&s, &output)
          && *s == '\0'))
    {
        return 0;
    }
    slong length = bits_mode ? (FLINT_BITS * total - 1) / (slong) x + 1
                             : (total - 1) / (slong) x + 1;
    ulong ** poly = flint_malloc(length * sizeof(ulong *));
    for (slong i = 0; i < length; i++)
    {
        poly[i] = flint_malloc((output + 1) * sizeof(ulong));
    }
    slong result = bits_mode ? fft_split_bits(poly, limbs.d, total, x, output)
                             : fft_split_limbs(poly, limbs.d, total, x, output);
    sb_str(b, "(");
    sb_ulong(b, (ulong) result);
    sb_str(b, ", ");
    sb_residues(b, poly, length, output + 1);
    sb_str(b, ")");
    residue_pointers_clear(poly, length);
    limbs_clear(&limbs);
    return 1;
}

static int
check_split_limbs(const char * s, sb_t * b)
{
    return check_split(s, b, 0);
}

static int
check_split_bits(const char * s, sb_t * b)
{
    return check_split(s, b, 1);
}

/* fft_combine_limbs(res, poly, length, coeff_limbs, output_limbs, total_limbs) = res, and
   fft_combine_bits(res, poly, length, bits, output_limbs, total_limbs) = res */
static int
check_combine(const char * s, sb_t * b, int bits_mode)
{
    limbs_t res;
    limbs_vec_t poly;
    slong length, output, total;
    ulong x;
    if (!(p_limbs(&s, &res) && p_lit(&s, ", ") && p_limbs_vec(&s, &poly) && p_lit(&s, ", ")
          && p_slong(&s, &length) && p_lit(&s, ", ") && p_ulong(&s, &x) && p_lit(&s, ", ")
          && p_slong(&s, &output) && p_lit(&s, ", ") && p_slong(&s, &total) && *s == '\0'))
    {
        return 0;
    }
    ulong ** p = residue_pointers(&poly);
    if (bits_mode)
    {
        fft_combine_bits(res.d, p, length, x, output, total);
    }
    else
    {
        fft_combine_limbs(res.d, p, length, x, output, total);
    }
    sb_limbs(b, res.d, res.len);
    residue_pointers_clear(p, poly.len);
    limbs_vec_clear(&poly);
    limbs_clear(&res);
    return 1;
}

static int
check_combine_limbs(const char * s, sb_t * b)
{
    return check_combine(s, b, 0);
}

static int
check_combine_bits(const char * s, sb_t * b)
{
    return check_combine(s, b, 1);
}

/* fft_convolution(ii, jj, depth, limbs, trunc, _, _, _, _) = (ii, jj) */
static int
check_convolution(const char * s, sb_t * b)
{
    limbs_vec_t ii, jj;
    int some;
    slong depth, limbs, trunc;
    if (!(p_limbs_vec(&s, &ii) && p_lit(&s, ", ") && p_opt_limbs_vec(&s, &some, &jj)
          && p_lit(&s, ", ") && p_slong(&s, &depth) && p_lit(&s, ", ") && p_slong(&s, &limbs)
          && p_lit(&s, ", ") && p_slong(&s, &trunc) && p_lit(&s, ", _, _, _, _")
          && *s == '\0'))
    {
        return 0;
    }
    slong size = ii.v[0].len;
    ulong ** p = residue_pointers(&ii);
    ulong ** q = some ? residue_pointers(&jj) : p;
    ulong * t1 = flint_malloc(size * sizeof(ulong));
    ulong * t2 = flint_malloc(size * sizeof(ulong));
    ulong * s1 = flint_malloc(size * sizeof(ulong));
    ulong * tt = flint_malloc(2 * size * sizeof(ulong));
    fft_convolution(p, q, depth, limbs, trunc, &t1, &t2, &s1, &tt);
    sb_str(b, "(");
    sb_residues(b, p, ii.len, size);
    sb_str(b, ", ");
    sb_option_residues(b, q, some ? jj.len : 0, size, some);
    sb_str(b, ")");
    if (some)
    {
        residue_pointers_clear(q, jj.len);
        limbs_vec_clear(&jj);
    }
    residue_pointers_clear(p, ii.len);
    flint_free(t1);
    flint_free(t2);
    flint_free(s1);
    flint_free(tt);
    limbs_vec_clear(&ii);
    return 1;
}

/* Parses `[x, y, ...]` of decimal integers into a new fmpz vector. */
static int
p_fmpz_vec(const char ** s, fmpz ** v, slong * len)
{
    if (!p_lit(s, "["))
    {
        return 0;
    }
    slong cap = 4;
    *v = _fmpz_vec_init(cap);
    *len = 0;
    if (p_lit(s, "]"))
    {
        return 1;
    }
    for (;;)
    {
        if (*len == cap)
        {
            fmpz * w = _fmpz_vec_init(cap << 1);
            _fmpz_vec_swap(w, *v, cap);
            _fmpz_vec_clear(*v, cap);
            *v = w;
            cap <<= 1;
        }
        const char * start = *s;
        const char * p = start;
        if (*p == '-')
        {
            p++;
        }
        while (*p >= '0' && *p <= '9')
        {
            p++;
        }
        if (p == start)
        {
            return 0;
        }
        char * digits = flint_malloc(p - start + 1);
        memcpy(digits, start, p - start);
        digits[p - start] = '\0';
        int bad = fmpz_set_str(*v + *len, digits, 10);
        flint_free(digits);
        if (bad)
        {
            return 0;
        }
        *s = p;
        (*len)++;
        if (p_lit(s, "]"))
        {
            return 1;
        }
        if (!p_lit(s, ", "))
        {
            return 0;
        }
    }
}

/* integers_to_fermat_residues(_, xs, limbs) = coeffs_f */
static int
check_get_fft(const char * s, sb_t * b)
{
    fmpz * xs;
    slong len, limbs;
    if (!(p_lit(&s, "_, ") && p_fmpz_vec(&s, &xs, &len) && p_lit(&s, ", ")
          && p_slong(&s, &limbs) && *s == '\0'))
    {
        return 0;
    }
    ulong ** p = flint_malloc(FLINT_MAX(len, 1) * sizeof(ulong *));
    for (slong i = 0; i < len; i++)
    {
        p[i] = flint_malloc((limbs + 1) * sizeof(ulong));
    }
    _fmpz_vec_get_fft(p, xs, limbs, len);
    sb_residues(b, p, len, limbs + 1);
    residue_pointers_clear(p, len);
    _fmpz_vec_clear(xs, FLINT_MAX(len, 4));
    return 1;
}

/* integers_from_fermat_residues(_, coeffs_f, limbs, sign) = xs */
static int
check_set_fft(const char * s, sb_t * b)
{
    limbs_vec_t f;
    slong limbs;
    int sign;
    if (!(p_lit(&s, "_, ") && p_limbs_vec(&s, &f) && p_lit(&s, ", ") && p_slong(&s, &limbs)
          && p_lit(&s, ", ")))
    {
        return 0;
    }
    if (p_lit(&s, "true"))
    {
        sign = 1;
    }
    else if (p_lit(&s, "false"))
    {
        sign = 0;
    }
    else
    {
        return 0;
    }
    if (*s != '\0')
    {
        return 0;
    }
    ulong ** p = residue_pointers(&f);
    fmpz * xs = _fmpz_vec_init(FLINT_MAX(f.len, 1));
    _fmpz_vec_set_fft(xs, f.len, (const nn_ptr *) p, limbs, sign);
    /* FLINT 3.6.0 tests only `coeffs_f[i][limbs - 1] > halflimb`, which reads a residue whose top
       limb is `halflimb` and whose lower limbs are not all zero, the representation of a negative
       coefficient whose absolute value is within 2^(N - FLINT_BITS) of 2^(N - 1), as positive.
       FLINT fixed this after 3.6.0, in commit 7ad753d51c, and Malachite tests as the fix does;
       subtract p = 2^N + 1 from what FLINT computed for those residues. */
    if (sign)
    {
        for (slong i = 0; i < f.len; i++)
        {
            if (p[i][limbs] == 0 && p[i][limbs - 1] == UWORD(1) << (FLINT_BITS - 1)
                && !flint_mpn_zero_p(p[i], limbs - 1))
            {
                fmpz_t modulus;
                fmpz_init(modulus);
                fmpz_one(modulus);
                fmpz_mul_2exp(modulus, modulus, limbs * FLINT_BITS);
                fmpz_add_ui(modulus, modulus, 1);
                fmpz_sub(xs + i, xs + i, modulus);
                fmpz_clear(modulus);
            }
        }
    }
    sb_fmpz_vec(b, xs, f.len);
    _fmpz_vec_clear(xs, FLINT_MAX(f.len, 1));
    residue_pointers_clear(p, f.len);
    limbs_vec_clear(&f);
    return 1;
}

/* -- entry points -- */

#define SIMPLE_MODE(mode, prefix, check) \
    int run_##mode(const char * arg) { return run_fft_mode(arg, #mode, prefix, check); }

SIMPLE_MODE(mpn_addmod_2expp1_1, "limbs_add_signed_limb_mod_2expp1", check_addmod_2expp1_1)
SIMPLE_MODE(flint_mpn_sumdiff_n, "limbs_sum_diff", check_sumdiff)
SIMPLE_MODE(mpn_normmod_2expp1, "limbs_norm_mod_2expp1", check_normmod)
SIMPLE_MODE(mpn_negmod_2expp1, "limbs_neg_mod_2expp1_to_out", check_negmod)
SIMPLE_MODE(fft_adjust, "limbs_fft_adjust", check_adjust)
SIMPLE_MODE(fft_adjust_sqrt2, "limbs_fft_adjust_sqrt2", check_adjust_sqrt2)
SIMPLE_MODE(butterfly_lshB, "limbs_butterfly_lsh_b", check_butterfly_lshB)
SIMPLE_MODE(butterfly_rshB, "limbs_butterfly_rsh_b", check_butterfly_rshB)
SIMPLE_MODE(fft_butterfly, "limbs_fft_butterfly", check_fft_butterfly)
SIMPLE_MODE(ifft_butterfly, "limbs_ifft_butterfly", check_ifft_butterfly)
SIMPLE_MODE(fft_butterfly_sqrt2, "limbs_fft_butterfly_sqrt2", check_fft_butterfly_sqrt2)
SIMPLE_MODE(ifft_butterfly_sqrt2, "limbs_ifft_butterfly_sqrt2", check_ifft_butterfly_sqrt2)
SIMPLE_MODE(fft_butterfly_twiddle, "limbs_fft_butterfly_twiddle", check_fft_butterfly_twiddle)
SIMPLE_MODE(ifft_butterfly_twiddle, "limbs_ifft_butterfly_twiddle",
            check_ifft_butterfly_twiddle)
SIMPLE_MODE(fft_mfa_truncate_sqrt2_inner, "fft_mfa_truncate_sqrt2_inner", check_mfa_inner)
SIMPLE_MODE(flint_mpn_mulmod_2expp1_basecase, "limbs_mul_mod_2expp1_basecase",
            check_mulmod_basecase)
SIMPLE_MODE(fft_naive_convolution_1, "fft_naive_convolution_1", check_naive_convolution_1)
SIMPLE_MODE(_fft_mulmod_2expp1, "fft_mulmod_2expp1_negacyclic", check_mulmod_negacyclic)
SIMPLE_MODE(fft_mulmod_2expp1, "fft_mulmod_2expp1", check_mulmod)
SIMPLE_MODE(fft_adjust_limbs, "fft_adjust_limbs", check_adjust_limbs)
SIMPLE_MODE(fft_split_limbs, "fft_split_limbs", check_split_limbs)
SIMPLE_MODE(fft_split_bits, "fft_split_bits", check_split_bits)
SIMPLE_MODE(fft_combine_limbs, "fft_combine_limbs", check_combine_limbs)
SIMPLE_MODE(fft_combine_bits, "fft_combine_bits", check_combine_bits)
SIMPLE_MODE(fft_convolution, "fft_convolution", check_convolution)
SIMPLE_MODE(_fmpz_vec_get_fft, "integers_to_fermat_residues", check_get_fft)
SIMPLE_MODE(_fmpz_vec_set_fft, "integers_from_fermat_residues", check_set_fft)

static const prefix_check_t mul_2expmod_alternatives[] = {
    {"limbs_mul_2exp_mod_2expp1_in_place", check_mul_2expmod},
    {"limbs_mul_2exp_mod_2expp1_to_out", check_mul_2expmod},
};

int
run_mpn_mul_2expmod_2expp1(const char * arg)
{
    return run_fft_mode_alternatives(arg, "mpn_mul_2expmod_2expp1", mul_2expmod_alternatives, 2);
}

static const prefix_check_t div_2expmod_alternatives[] = {
    {"limbs_div_2exp_mod_2expp1_in_place", check_div_2expmod},
    {"limbs_div_2exp_mod_2expp1_to_out", check_div_2expmod},
};

int
run_mpn_div_2expmod_2expp1(const char * arg)
{
    return run_fft_mode_alternatives(arg, "mpn_div_2expmod_2expp1", div_2expmod_alternatives, 2);
}

#define TRANSFORM_MODE(mode, t) \
    int run_##mode(const char * arg) \
    { \
        current_transform = t; \
        return run_fft_mode(arg, #mode, #mode, check_transform); \
    }

TRANSFORM_MODE(fft_radix2, T_FFT_RADIX2)
TRANSFORM_MODE(ifft_radix2, T_IFFT_RADIX2)
TRANSFORM_MODE(fft_truncate1, T_FFT_TRUNCATE1)
TRANSFORM_MODE(fft_truncate, T_FFT_TRUNCATE)
TRANSFORM_MODE(ifft_truncate1, T_IFFT_TRUNCATE1)
TRANSFORM_MODE(ifft_truncate, T_IFFT_TRUNCATE)
TRANSFORM_MODE(fft_truncate_sqrt2, T_FFT_TRUNCATE_SQRT2)
TRANSFORM_MODE(ifft_truncate_sqrt2, T_IFFT_TRUNCATE_SQRT2)
TRANSFORM_MODE(fft_negacyclic, T_FFT_NEGACYCLIC)
TRANSFORM_MODE(ifft_negacyclic, T_IFFT_NEGACYCLIC)
TRANSFORM_MODE(fft_mfa_truncate_sqrt2_outer, T_FFT_MFA_OUTER)
TRANSFORM_MODE(ifft_mfa_truncate_sqrt2_outer, T_IFFT_MFA_OUTER)

#define TWIDDLE_MODE(mode, t) \
    int run_##mode(const char * arg) \
    { \
        current_twiddle = t; \
        return run_fft_mode(arg, #mode, #mode, check_twiddle); \
    }

TWIDDLE_MODE(fft_radix2_twiddle, TW_FFT_RADIX2)
TWIDDLE_MODE(ifft_radix2_twiddle, TW_IFFT_RADIX2)
TWIDDLE_MODE(fft_truncate1_twiddle, TW_FFT_TRUNCATE1)
TWIDDLE_MODE(ifft_truncate1_twiddle, TW_IFFT_TRUNCATE1)
