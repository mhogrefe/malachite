/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the basic functions of Malachite's BigExponentFloat against FLINT's arf_t, its
    counterpart. A BigExponentFloat is written in Malachite's exact hexadecimal format (`0x1.8`,
    `-0xf.fcE+274877915135`, `NaN`, `Infinity`, `-Infinity`; the `0x` is optional), which is parsed
    exactly into an arf_t, whatever the size of its exponent. The modes:

    - `arf_classification`: `x.f() = v` for f among the classification predicates, `classify`,
      `to_non_nan`, `into_non_nan`, `to_finite`, and `into_finite`, against arf_is_nan, arf_is_inf,
      arf_is_zero, arf_is_finite, arf_is_normal, and arf_sgn. Zero counts as sign-positive.
    - `arf_get`: `x.get_exponent()`, `x.get_prec()`, `x.to_significand()` (and `into_significand`,
      `significand_ref`), `x.complexity()`, and `x.significant_bits()`, against ARF_EXP, arf_bits,
      and arf_get_fmpz_2exp. A significand is its odd mantissa shifted to fill 64-bit limbs.
    - `arf_set_round`: the `round_to_prec_round` and `round_to_prec` forms (by value, `_ref`, and
      `_assign`), against arf_set_round, with the direction of rounding from arf_cmp. `Exact`
      requires arf_set_round to be exact.
    - `arf_hex`: `to_lower_hex_string(v) = s`, `to_upper_hex_string(v) = s`, and
      `format!("{:?}", v) = s`, where v is written independently as `m*2^k`; s must parse to
      exactly that value, in the right case and with the right prefix.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <ctype.h>
#include <stdlib.h>
#include <string.h>

#include <flint/arf.h>
#include <flint/fmpz.h>

#include "oracle.h"

typedef enum
{
    KIND_CLASSIFICATION,
    KIND_GET,
    KIND_SET_ROUND,
    KIND_HEX
} arf_kind;

static long checked;
static arf_kind kind;
static const char * mode_name;

/* Parses Malachite's BigExponentFloat format in base 2^`log_base`, for `log_base` from 1 to 5, into
   `x`, independently of Malachite: `NaN`, `Infinity`, `-Infinity`, or an optional `-`, an optional
   prefix (`0b` in base 2, `0o` in base 8, `0x` in base 16), digits in either case with an optional
   `.`, and an optional exponent of the base, written as `E` or `e` and a decimal number. The
   exponent's sign is optional in bases below 15, where `E` is not a digit, and mandatory in the
   others. Returns 0 on success. */
int
arf_parse_base(const char * s, int log_base, arf_t x)
{
    int base = 1 << log_base;
    if (strcmp(s, "NaN") == 0)
    {
        arf_nan(x);
        return 0;
    }
    int negative = 0;
    if (*s == '-')
    {
        negative = 1;
        s++;
    }
    if (strcmp(s, "Infinity") == 0)
    {
        if (negative)
            arf_neg_inf(x);
        else
            arf_pos_inf(x);
        return 0;
    }
    const char * prefix = base == 2 ? "0b" : base == 8 ? "0o" : base == 16 ? "0x" : NULL;
    if (prefix != NULL && strncmp(s, prefix, 2) == 0)
    {
        s += 2;
    }
    size_t n = strlen(s);
    char * digits = flint_malloc(n + 1);
    size_t d = 0;
    slong frac = -1;
    const char * e = NULL;
    for (const char * p = s; *p != '\0'; p++)
    {
        if ((*p == 'E' || *p == 'e')
            && (base < 15 || p[1] == '+' || p[1] == '-'))
        {
            e = p + 1;
            break;
        }
        if (*p == '.')
        {
            if (frac >= 0)
            {
                flint_free(digits);
                return 1;
            }
            frac = 0;
            continue;
        }
        int v = isdigit((unsigned char) *p) ? *p - '0'
                : isalpha((unsigned char) *p) ? tolower((unsigned char) *p) - 'a' + 10 : 99;
        if (v >= base)
        {
            flint_free(digits);
            return 1;
        }
        digits[d++] = *p;
        if (frac >= 0)
            frac++;
    }
    digits[d] = '\0';
    if (frac < 0)
        frac = 0;
    if (d == 0)
    {
        flint_free(digits);
        return 1;
    }
    fmpz_t m, exp;
    fmpz_init(m);
    fmpz_init(exp);
    int result = fmpz_set_str(m, digits, base);
    if (result == 0 && e != NULL)
    {
        const char * t = *e == '+' ? e + 1 : e;
        /* the exponent must be a nonempty decimal number with an optional minus sign */
        const char * u = *t == '-' ? t + 1 : t;
        if (*u == '\0')
            result = 1;
        for (const char * q = u; *q != '\0' && result == 0; q++)
            if (!isdigit((unsigned char) *q))
                result = 1;
        if (result == 0)
            result = fmpz_set_str(exp, t, 10);
    }
    if (result == 0)
    {
        /* the value is m * base^(exp - frac) */
        fmpz_sub_si(exp, exp, frac);
        fmpz_mul_ui(exp, exp, log_base);
        arf_set_fmpz_2exp(x, m, exp);
        if (negative)
            arf_neg(x, x);
    }
    fmpz_clear(m);
    fmpz_clear(exp);
    flint_free(digits);
    return result == 0 ? 0 : 1;
}

/* Parses Malachite's hexadecimal BigExponentFloat format into `x`. Returns 0 on success. */
static int
parse_hex(const char * s, arf_t x)
{
    return arf_parse_base(s, 4, x);
}

/* Parses `m*2^k` (or a special value, in the hexadecimal format) into `x`. */
int
arf_parse_exact(const char * s, arf_t x)
{
    const char * star = strstr(s, "*2^");
    if (star == NULL)
    {
        return parse_hex(s, x);
    }
    char * m_str = flint_malloc(star - s + 1);
    memcpy(m_str, s, star - s);
    m_str[star - s] = '\0';
    fmpz_t m, k;
    fmpz_init(m);
    fmpz_init(k);
    int result = fmpz_set_str(m, m_str, 10) || fmpz_set_str(k, star + 3, 10);
    if (result == 0)
        arf_set_fmpz_2exp(x, m, k);
    fmpz_clear(m);
    fmpz_clear(k);
    flint_free(m_str);
    return result;
}

static int
report(int line_number, const char * what)
{
    flint_printf("error in %s test, line %d: %s\n", mode_name, line_number, what);
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

static int
check_bool(int line_number, const char * expected, int actual)
{
    if (strcmp(expected, actual ? "true" : "false") != 0)
        return report(line_number, "wrong result");
    return 0;
}

/* Checks `expected`, `None` or `Some(x)`, against x when `present` holds. */
static int
check_option_same(int line_number, const char * expected, int present, const arf_t x)
{
    if (!present)
        return strcmp(expected, "None") == 0 ? 0 : report(line_number, "expected None");
    size_t n = strlen(expected);
    if (strncmp(expected, "Some(", 5) != 0 || expected[n - 1] != ')')
        return report(line_number, "expected Some");
    char * inner = flint_malloc(n);
    memcpy(inner, expected + 5, n - 6);
    inner[n - 6] = '\0';
    arf_t y;
    arf_init(y);
    int result = parse_hex(inner, y) ? report(line_number, "unreadable value")
                                     : (arf_equal(x, y) ? 0 : report(line_number, "wrong value"));
    arf_clear(y);
    flint_free(inner);
    return result;
}

/* Checks `expected`, `None` or `Some(n)`, against the integer `n` when `present` holds. */
static int
check_option_fmpz(int line_number, const char * expected, int present, const fmpz_t n)
{
    if (!present)
        return strcmp(expected, "None") == 0 ? 0 : report(line_number, "expected None");
    char * s = fmpz_get_str(NULL, 10, n);
    size_t len = strlen(s);
    int ok = strncmp(expected, "Some(", 5) == 0 && strncmp(expected + 5, s, len) == 0
             && strcmp(expected + 5 + len, ")") == 0;
    flint_free(s);
    return ok ? 0 : report(line_number, "wrong value");
}

static const char * const classification_names[] = {
    "is_nan", "is_finite", "is_infinite", "is_zero", "is_normal", "is_sign_positive",
    "is_sign_negative", "classify", "into_non_nan", "to_non_nan", "into_finite", "to_finite"};

static int
check_classification(char * line, int line_number)
{
    for (size_t i = 0; i < sizeof(classification_names) / sizeof(classification_names[0]); i++)
    {
        char sep[64];
        snprintf(sep, sizeof(sep), ".%s() = ", classification_names[i]);
        char * rest = strstr(line, sep);
        if (rest == NULL)
            continue;
        *rest = '\0';
        rest += strlen(sep);
        arf_t x;
        arf_init(x);
        int result;
        if (parse_hex(line, x))
        {
            result = report(line_number, "unreadable value");
        }
        else
        {
            int nan = arf_is_nan(x);
            int finite = arf_is_finite(x);
            int zero = arf_is_zero(x);
            int positive = !nan && (zero || arf_sgn(x) > 0);
            int negative = !nan && !zero && arf_sgn(x) < 0;
            const char * category = nan ? "Nan" : arf_is_inf(x) ? "Infinite" : zero ? "Zero"
                                                                                    : "Normal";
            switch (i)
            {
                case 0: result = check_bool(line_number, rest, nan); break;
                case 1: result = check_bool(line_number, rest, finite); break;
                case 2: result = check_bool(line_number, rest, arf_is_inf(x)); break;
                case 3: result = check_bool(line_number, rest, zero); break;
                case 4: result = check_bool(line_number, rest, arf_is_normal(x)); break;
                case 5: result = check_bool(line_number, rest, positive); break;
                case 6: result = check_bool(line_number, rest, negative); break;
                case 7:
                    result = strcmp(rest, category) == 0 ? 0 : report(line_number, "wrong class");
                    break;
                case 8:
                case 9: result = check_option_same(line_number, rest, !nan, x); break;
                default: result = check_option_same(line_number, rest, finite, x); break;
            }
        }
        arf_clear(x);
        return result;
    }
    return report(line_number, "unrecognized line");
}

static int
check_get(char * line, int line_number)
{
    static const char * const names[] = {"get_exponent", "get_prec", "to_significand",
                                         "into_significand", "significand_ref", "complexity",
                                         "significant_bits"};
    for (size_t i = 0; i < sizeof(names) / sizeof(names[0]); i++)
    {
        char sep[64];
        snprintf(sep, sizeof(sep), ".%s() = ", names[i]);
        char * rest = strstr(line, sep);
        if (rest == NULL)
            continue;
        *rest = '\0';
        rest += strlen(sep);
        arf_t x;
        arf_init(x);
        fmpz_t n, man, exp;
        fmpz_init(n);
        fmpz_init(man);
        fmpz_init(exp);
        int result;
        if (parse_hex(line, x))
        {
            result = report(line_number, "unreadable value");
        }
        else
        {
            int normal = arf_is_normal(x);
            ulong bits = normal ? arf_bits(x) : 0;
            if (i == 0)
            {
                if (normal)
                    fmpz_set(n, ARF_EXPREF(x));
                result = check_option_fmpz(line_number, rest, normal, n);
            }
            else if (i == 1)
            {
                fmpz_set_ui(n, bits);
                result = check_option_fmpz(line_number, rest, normal, n);
            }
            else if (i <= 4)
            {
                if (normal)
                {
                    /* the odd mantissa, shifted to fill whole 64-bit limbs */
                    arf_get_fmpz_2exp(man, exp, x);
                    fmpz_abs(man, man);
                    fmpz_mul_2exp(n, man, (64 - bits % 64) % 64);
                }
                result = check_option_fmpz(line_number, rest, normal, n);
            }
            else if (i == 5)
            {
                /* max(|exponent - 1|, saturated at 2^64 - 1, precision), or 1 */
                if (normal)
                {
                    fmpz_sub_ui(n, ARF_EXPREF(x), 1);
                    fmpz_abs(n, n);
                    if (!fmpz_abs_fits_ui(n))
                        fmpz_set_ui(n, UWORD_MAX);
                    if (fmpz_cmp_ui(n, bits) < 0)
                        fmpz_set_ui(n, bits);
                }
                else
                {
                    fmpz_one(n);
                }
                char * s = fmpz_get_str(NULL, 10, n);
                result = strcmp(s, rest) == 0 ? 0 : report(line_number, "wrong complexity");
                flint_free(s);
            }
            else
            {
                char s[32];
                snprintf(s, sizeof(s), "%lu", (unsigned long) (normal ? bits : 1));
                result = strcmp(s, rest) == 0 ? 0 : report(line_number, "wrong significant bits");
            }
        }
        arf_clear(x);
        fmpz_clear(n);
        fmpz_clear(man);
        fmpz_clear(exp);
        return result;
    }
    return report(line_number, "unrecognized line");
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

static int
check_set_round(char * line, int line_number)
{
    char * x_str;
    char * method;
    char * args;
    char * y_str;
    char * o_str;
    if (strncmp(line, "x := ", 5) == 0)
    {
        /* `x := X; x.method(args) = O; x = Y` */
        x_str = line + 5;
        method = split(x_str, "; x.");
        args = method == NULL ? NULL : split(method, "(");
        o_str = args == NULL ? NULL : split(args, ") = ");
        y_str = o_str == NULL ? NULL : split(o_str, "; x = ");
    }
    else
    {
        /* `(X).method(args) = (Y, O)`, with `&X` for the `_ref` forms */
        if (line[0] != '(')
            return report(line_number, "unrecognized line");
        x_str = line + 1;
        if (*x_str == '&')
            x_str++;
        method = split(x_str, ").");
        args = method == NULL ? NULL : split(method, "(");
        y_str = args == NULL ? NULL : split(args, ") = (");
        o_str = y_str == NULL ? NULL : strrchr(y_str, ',');
        if (o_str != NULL)
        {
            *o_str = '\0';
            o_str += 2;
            size_t n = strlen(o_str);
            if (n == 0 || o_str[n - 1] != ')')
                o_str = NULL;
            else
                o_str[n - 1] = '\0';
        }
    }
    if (y_str == NULL || o_str == NULL)
        return report(line_number, "unrecognized line");
    int with_rm = strncmp(method, "round_to_prec_round", 19) == 0;
    if (!with_rm && strncmp(method, "round_to_prec", 13) != 0)
        return report(line_number, "unrecognized method");
    char * rm_str = with_rm ? split(args, ", ") : NULL;
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    char * end;
    unsigned long long prec = strtoull(args, &end, 10);
    if (*end != '\0' || prec == 0 || (with_rm && (rm_str == NULL || parse_rm(rm_str, &rnd, &exact))))
        return report(line_number, "unreadable arguments");
    arf_t x, y, z;
    arf_init(x);
    arf_init(y);
    arf_init(z);
    int result;
    if (parse_hex(x_str, x) || parse_hex(y_str, y))
    {
        result = report(line_number, "unreadable value");
    }
    else
    {
        int inexact = arf_set_round(z, x, (slong) prec, rnd);
        int c = (arf_is_nan(x) || arf_is_nan(z)) ? 0 : arf_cmp(z, x);
        const char * o = c < 0 ? "Less" : c > 0 ? "Greater" : "Equal";
        if (exact && inexact)
            result = report(line_number, "Exact rounding of an inexact value");
        else if (!arf_equal(z, y))
            result = report(line_number, "wrong value");
        else if (strcmp(o, o_str) != 0)
            result = report(line_number, "wrong ordering");
        else
            result = 0;
    }
    arf_clear(x);
    arf_clear(y);
    arf_clear(z);
    return result;
}

static int
check_hex(char * line, int line_number)
{
    int upper = 0;
    int debug = 0;
    char * v;
    if (strncmp(line, "to_lower_hex_string(", 20) == 0)
        v = line + 20;
    else if (strncmp(line, "to_upper_hex_string(", 20) == 0)
    {
        v = line + 20;
        upper = 1;
    }
    else if (strncmp(line, "format!(\"{:?}\", ", 16) == 0)
    {
        v = line + 16;
        debug = 1;
    }
    else
        return report(line_number, "unrecognized line");
    char * s = split(v, ") = ");
    if (s == NULL)
        return report(line_number, "unrecognized line");
    arf_t x, y;
    arf_init(x);
    arf_init(y);
    int result;
    if (arf_parse_exact(v, x) || parse_hex(s, y))
    {
        result = report(line_number, "unreadable value");
    }
    else if (!arf_equal(x, y))
    {
        result = report(line_number, "wrong value");
    }
    else
    {
        result = 0;
        int finite = arf_is_finite(x);
        const char * body = s[0] == '-' ? s + 1 : s;
        int prefixed = strncmp(body, "0x", 2) == 0;
        if (finite && prefixed != debug)
            result = report(line_number, "wrong prefix");
        /* the digits, apart from the exponent marker `E`, have the requested case */
        for (const char * p = body + (prefixed ? 2 : 0); finite && *p != '\0' && result == 0; p++)
        {
            if (*p == 'E' && (p[1] == '+' || p[1] == '-'))
                break;
            if (upper ? islower((unsigned char) *p) : isupper((unsigned char) *p))
                result = report(line_number, "wrong case");
        }
    }
    arf_clear(x);
    arf_clear(y);
    return result;
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
    switch (kind)
    {
        case KIND_CLASSIFICATION: result = check_classification(line, line_number); break;
        case KIND_GET: result = check_get(line, line_number); break;
        case KIND_SET_ROUND: result = check_set_round(line, line_number); break;
        default: result = check_hex(line, line_number); break;
    }
    if (result == 0)
        checked++;
    return result;
}

static int
run(const char * arg, const char * name, arf_kind k)
{
    checked = 0;
    kind = k;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_arf_classification(const char * arg)
{
    return run(arg, "arf_classification", KIND_CLASSIFICATION);
}

int
run_arf_get(const char * arg)
{
    return run(arg, "arf_get", KIND_GET);
}

int
run_arf_set_round(const char * arg)
{
    return run(arg, "arf_set_round", KIND_SET_ROUND);
}

int
run_arf_hex(const char * arg)
{
    return run(arg, "arf_hex", KIND_HEX);
}
