/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the RationalPolynomial division demos against FLINT. Mode
    fmpq_poly_divrem reads `div_rem`, `checked_div_rem`, `div_assign_rem`, `checked_div`,
    `checked_rem`, and the operators `/`, `/=`, `%`, and `%=`, checked against fmpq_poly_divrem,
    fmpq_poly_div, and fmpq_poly_rem; `None` must be exactly where the divisor is 0. Mode
    fmpq_poly_divides reads `div_exact`, `div_exact_assign`, `checked_div_exact`, and
    `divisible_by`, checked against fmpq_poly_divides; a zero divisor divides only zero, and
    `checked_div_exact` by zero is `None`. Mode fmpq_poly_remove reads `remove_power` and
    `remove_power_assign`, checked against fmpq_poly_remove. Mode fmpq_poly_inv_series reads
    `inverse_truncated`, checked against fmpq_poly_inv_series and fmpq_poly_inv_series_newton;
    `None` must be exactly where the constant coefficient is 0. Mode fmpq_poly_div_series reads
    `div_truncated`, `div_truncated_assign`, and `checked_div_truncated`, checked against
    fmpq_poly_div_series; `None` must be exactly where the divisor's constant coefficient is 0.
    In the series modes, length 0 must give 0, where FLINT throws. Arguments may be written
    `&(...)`, and results `Some(...)`. Any other nonempty line is an error, as is an input with no
    lines at all.
*/

#include <stdlib.h>
#include <string.h>
#include <flint/fmpz.h>
#include <flint/fmpq.h>
#include <flint/fmpq_poly.h>

#include "oracle.h"

static long checked;
static int mode;
static const char * mode_name;

enum { MODE_DIVREM, MODE_DIVIDES, MODE_REMOVE, MODE_INV_SERIES, MODE_DIV_SERIES };

/* Strips `&(` and the matching `)` from an argument written `&(X)`, in place. */
static char *
strip_ref(char * argument)
{
    size_t len = strlen(argument);
    if (len >= 3 && argument[0] == '&' && argument[1] == '(' && argument[len - 1] == ')')
    {
        argument[len - 1] = '\0';
        return argument + 2;
    }
    return argument;
}

/* Cuts `line` at `needle`, returning the text after it, or NULL if the needle is absent. */
static char *
cut(char * line, const char * needle)
{
    char * at = strstr(line, needle);
    if (at == NULL)
    {
        return NULL;
    }
    *at = '\0';
    return at + strlen(needle);
}

/* Strips a leading `&` from an operand written `&(X)` or `(X)`, and the parentheses. */
static char *
strip_operand(char * s)
{
    if (s[0] == '&')
        s++;
    size_t len = strlen(s);
    if (len >= 2 && s[0] == '(' && s[len - 1] == ')')
    {
        s[len - 1] = '\0';
        return s + 1;
    }
    return s;
}

/* The parsed line: the operands, the method or operator, and the result, split into up to two
   parts; `has_result` is 0 for `None`. */
struct parsed
{
    char * a;
    char * b;
    char * method;
    char * x;
    char * y;
    int has_result;
};

/* Splits `(A).METHOD(B) = R`, `A op B = R`, `p := A; p.METHOD(B) = R; p = X`,
   `p := A; p.METHOD(B); p = X`, or `p := A; p op= B; p = X`. For the in-place forms, `x` is the
   new value of `p` and `y` the returned value, if any. */
static int
split_line(char * line, struct parsed * out)
{
    line[strcspn(line, "\r\n")] = '\0';
    out->x = NULL;
    out->y = NULL;
    out->has_result = 1;
    if (strncmp(line, "p := ", 5) == 0)
    {
        out->a = line + 5;
        char * rest = cut(out->a, "; p");
        if (rest == NULL)
            return 0;
        out->x = cut(rest, "; p = ");
        if (out->x == NULL)
            return 0;
        if (rest[0] == '.')
        {
            out->method = rest + 1;
            char * args = cut(out->method, "(");
            if (args == NULL)
                return 0;
            char * ret = cut(args, ") = ");
            if (ret == NULL)
            {
                size_t len = strlen(args);
                if (len == 0 || args[len - 1] != ')')
                    return 0;
                args[len - 1] = '\0';
            }
            out->y = ret;
            out->b = strip_ref(args);
        }
        else
        {
            /* " /= B" or " %= B" */
            if (strncmp(rest, " /= ", 4) == 0)
                out->method = "/=";
            else if (strncmp(rest, " %= ", 4) == 0)
                out->method = "%=";
            else
                return 0;
            out->b = rest + 4;
        }
        return 1;
    }
    char * result = cut(line, " = ");
    if (result == NULL)
        return 0;
    if (strcmp(result, "None") == 0)
        out->has_result = 0;
    char * op;
    if ((op = strstr(line, " / ")) != NULL || (op = strstr(line, " % ")) != NULL)
    {
        out->method = op[1] == '/' ? "/" : "%";
        *op = '\0';
        out->a = strip_operand(line);
        out->b = strip_operand(op + 3);
    }
    else
    {
        out->method = cut(line, ").");
        if (out->method == NULL || line[0] != '(')
            return 0;
        out->a = strip_ref(line + 1);
        char * args = cut(out->method, "(");
        if (args == NULL)
            return 0;
        size_t len = strlen(args);
        if (len == 0 || args[len - 1] != ')')
            return 0;
        args[len - 1] = '\0';
        out->b = strip_ref(args);
    }
    if (out->has_result)
    {
        size_t len = strlen(result);
        if (len >= 2 && result[0] == '(' && result[len - 1] == ')')
        {
            result[len - 1] = '\0';
            out->x = result + 1;
            out->y = cut(out->x, ", ");
        }
        else
            out->x = result;
    }
    return 1;
}

static int
fail(const char * what, int line_number)
{
    flint_printf("error in %s test, line %d: %s\n", mode_name, line_number, what);
    return 1;
}

static int
read_poly(fmpq_poly_t p, const char * s)
{
    return s != NULL && fmpq_poly_set_str_malachite(p, s);
}

/* Strips `Some(` and the matching `)` from a result, in place, if present. */
static void
strip_some(struct parsed * l)
{
    char * s = l->x;
    if (s == NULL)
        return;
    size_t len = strlen(s);
    if (len >= 6 && strncmp(s, "Some(", 5) == 0 && s[len - 1] == ')')
    {
        s[len - 1] = '\0';
        s += 5;
        len = strlen(s);
        /* A tuple inside: `(A, B)`. */
        if (len >= 2 && s[0] == '(' && s[len - 1] == ')' && strstr(s, ", ") != NULL)
        {
            s[len - 1] = '\0';
            s += 1;
            l->y = cut(s, ", ");
        }
        l->x = s;
    }
}

static int
check_divrem(struct parsed * l, const fmpq_poly_t a, const fmpq_poly_t b, int line_number)
{
    int want_q = 0, want_r = 0;
    const char * q_str = NULL;
    const char * r_str = NULL;
    const char * m = l->method;
    if (strcmp(m, "div_rem") == 0 || strcmp(m, "checked_div_rem") == 0
        || strcmp(m, "div_assign_rem") == 0)
    {
        want_q = want_r = 1;
        q_str = l->x;
        r_str = l->y;
    }
    else if (strcmp(m, "/") == 0 || strcmp(m, "/=") == 0 || strcmp(m, "checked_div") == 0)
    {
        want_q = 1;
        q_str = l->x;
    }
    else if (strcmp(m, "%") == 0 || strcmp(m, "%=") == 0 || strcmp(m, "checked_rem") == 0)
    {
        want_r = 1;
        r_str = l->x;
    }
    else
        return fail("unrecognized method", line_number);
    if (!l->has_result)
        return fmpq_poly_is_zero(b) ? 0 : fail("None, but the divisor is nonzero", line_number);
    if (fmpq_poly_is_zero(b))
        return fail("a result, but the divisor is 0", line_number);
    int result = 0;
    fmpq_poly_t q, r, eq, er;
    fmpq_poly_init(q);
    fmpq_poly_init(r);
    fmpq_poly_init(eq);
    fmpq_poly_init(er);
    if ((want_q && !read_poly(eq, q_str)) || (want_r && !read_poly(er, r_str)))
    {
        result = fail("unreadable result", line_number);
        goto cleanup;
    }
    fmpq_poly_divrem(q, r, a, b);
    if ((want_q && !fmpq_poly_equal(q, eq)) || (want_r && !fmpq_poly_equal(r, er)))
    {
        result = fail("disagrees with fmpq_poly_divrem", line_number);
        goto cleanup;
    }
    if (want_q)
    {
        fmpq_poly_div(q, a, b);
        if (!fmpq_poly_equal(q, eq))
            result = fail("disagrees with fmpq_poly_div", line_number);
    }
    if (want_r)
    {
        fmpq_poly_rem(r, a, b);
        if (!fmpq_poly_equal(r, er))
            result = fail("disagrees with fmpq_poly_rem", line_number);
    }
cleanup:
    fmpq_poly_clear(q);
    fmpq_poly_clear(r);
    fmpq_poly_clear(eq);
    fmpq_poly_clear(er);
    return result;
}

static int
check_divides(struct parsed * l, const fmpq_poly_t a, const fmpq_poly_t b, int line_number)
{
    const char * m = l->method;
    int result = 0;
    fmpq_poly_t q, e;
    fmpq_poly_init(q);
    fmpq_poly_init(e);
    int divides = fmpq_poly_is_zero(b) ? fmpq_poly_is_zero(a) : fmpq_poly_divides(q, a, b);
    if (strcmp(m, "divisible_by") == 0)
    {
        if ((strcmp(l->x, "true") == 0) != divides)
            result = fail("disagrees with fmpq_poly_divides", line_number);
    }
    else if (strcmp(m, "checked_div_exact") == 0 && !l->has_result)
    {
        if (divides && !fmpq_poly_is_zero(b))
            result = fail("None, but fmpq_poly_divides finds a quotient", line_number);
    }
    else if (strcmp(m, "checked_div_exact") == 0 || strcmp(m, "div_exact") == 0
             || strcmp(m, "div_exact_assign") == 0)
    {
        if (fmpq_poly_is_zero(b))
            result = fail("a quotient by 0", line_number);
        else if (!divides || !read_poly(e, l->x) || !fmpq_poly_equal(q, e))
            result = fail("disagrees with fmpq_poly_divides", line_number);
    }
    else
        result = fail("unrecognized method", line_number);
    fmpq_poly_clear(q);
    fmpq_poly_clear(e);
    return result;
}

static int
check_remove(struct parsed * l, const fmpq_poly_t a, const fmpq_poly_t b, int line_number)
{
    if (strcmp(l->method, "remove_power") != 0 && strcmp(l->method, "remove_power_assign") != 0)
        return fail("unrecognized method", line_number);
    int result = 0;
    fmpq_poly_t s, e;
    fmpq_poly_init(s);
    fmpq_poly_init(e);
    if (l->y == NULL || !read_poly(e, l->x))
    {
        result = fail("unreadable input", line_number);
        goto cleanup;
    }
    ulong expected = strtoul(l->y, NULL, 10);
    slong k = fmpq_poly_remove(s, a, b);
    if ((ulong) k != expected || !fmpq_poly_equal(s, e))
        result = fail("disagrees with fmpq_poly_remove", line_number);
cleanup:
    fmpq_poly_clear(s);
    fmpq_poly_clear(e);
    return result;
}

static int
constant_coefficient_is_zero(const fmpq_poly_t p)
{
    return p->length == 0 || fmpz_is_zero(p->coeffs + 0);
}

static int
check_inv_series(struct parsed * l, const fmpq_poly_t a, ulong n, int line_number)
{
    if (strcmp(l->method, "inverse_truncated") != 0)
        return fail("unrecognized method", line_number);
    fmpq_poly_t r, e;
    fmpq_poly_init(r);
    fmpq_poly_init(e);
    int result = 0;
    if (n == 0)
    {
        if (!l->has_result || !read_poly(e, l->x) || !fmpq_poly_is_zero(e))
            result = fail("expected Some(0)", line_number);
    }
    else if (constant_coefficient_is_zero(a))
    {
        if (l->has_result)
            result = fail("expected None", line_number);
    }
    else if (!l->has_result || !read_poly(e, l->x))
        result = fail("expected an inverse", line_number);
    else
    {
        fmpq_poly_inv_series(r, a, (slong) n);
        if (!fmpq_poly_equal(r, e))
            result = fail("disagrees with fmpq_poly_inv_series", line_number);
        fmpq_poly_inv_series_newton(r, a, (slong) n);
        if (!result && !fmpq_poly_equal(r, e))
            result = fail("disagrees with fmpq_poly_inv_series_newton", line_number);
    }
    fmpq_poly_clear(r);
    fmpq_poly_clear(e);
    return result;
}

static int
check_div_series(struct parsed * l, const fmpq_poly_t a, const fmpq_poly_t b, ulong n,
                 int line_number)
{
    const char * m = l->method;
    if (strcmp(m, "div_truncated") != 0 && strcmp(m, "div_truncated_assign") != 0
        && strcmp(m, "checked_div_truncated") != 0)
        return fail("unrecognized method", line_number);
    fmpq_poly_t r, e;
    fmpq_poly_init(r);
    fmpq_poly_init(e);
    int result = 0;
    if (n == 0)
    {
        if (!l->has_result || !read_poly(e, l->x) || !fmpq_poly_is_zero(e))
            result = fail("expected 0", line_number);
    }
    else if (constant_coefficient_is_zero(b))
    {
        if (l->has_result)
            result = fail("expected no quotient", line_number);
    }
    else if (!l->has_result || !read_poly(e, l->x))
        result = fail("expected a quotient", line_number);
    else
    {
        fmpq_poly_div_series(r, a, b, (slong) n);
        if (!fmpq_poly_equal(r, e))
            result = fail("disagrees with fmpq_poly_div_series", line_number);
    }
    fmpq_poly_clear(r);
    fmpq_poly_clear(e);
    return result;
}

/* Splits `Q, n` or `&(Q), n` in place at the last ", ", returning the length and setting `*q` to
   Q, or returning -1 if the argument has another form. */
static long
split_series_argument(char * arg, char ** q)
{
    char * comma = strrchr(arg, ',');
    if (comma == NULL || comma[1] != ' ')
        return -1;
    *comma = '\0';
    char * end;
    long n = strtol(comma + 2, &end, 10);
    if (*end != '\0' || n < 0)
        return -1;
    *q = strip_ref(arg);
    return n;
}

static int
check_line(char * line, int line_number)
{
    struct parsed l;
    if (!split_line(line, &l))
    {
        if (line[0] == '\0')
            return 0;
        return fail("unrecognized line", line_number);
    }
    strip_some(&l);
    checked++;
    int result;
    fmpq_poly_t a, b;
    fmpq_poly_init(a);
    fmpq_poly_init(b);
    if (mode == MODE_INV_SERIES)
    {
        char * end;
        long n = strtol(l.b, &end, 10);
        if (*end != '\0' || n < 0 || !read_poly(a, l.a))
            result = fail("unreadable input", line_number);
        else
            result = check_inv_series(&l, a, (ulong) n, line_number);
    }
    else if (mode == MODE_DIV_SERIES)
    {
        char * q_str;
        long n = split_series_argument(l.b, &q_str);
        if (n < 0 || !read_poly(a, l.a) || !read_poly(b, q_str))
            result = fail("unreadable input", line_number);
        else
            result = check_div_series(&l, a, b, (ulong) n, line_number);
    }
    else if (!read_poly(a, l.a) || !read_poly(b, l.b))
        result = fail("unreadable input", line_number);
    else if (mode == MODE_DIVREM)
        result = check_divrem(&l, a, b, line_number);
    else if (mode == MODE_DIVIDES)
        result = check_divides(&l, a, b, line_number);
    else
        result = check_remove(&l, a, b, line_number);
    fmpq_poly_clear(a);
    fmpq_poly_clear(b);
    return result;
}

static int
run(const char * arg, int m, const char * name)
{
    checked = 0;
    mode = m;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpq_poly_divrem(const char * arg)
{
    return run(arg, MODE_DIVREM, "fmpq_poly_divrem");
}

int
run_fmpq_poly_divides(const char * arg)
{
    return run(arg, MODE_DIVIDES, "fmpq_poly_divides");
}

int
run_fmpq_poly_remove(const char * arg)
{
    return run(arg, MODE_REMOVE, "fmpq_poly_remove");
}

int
run_fmpq_poly_inv_series(const char * arg)
{
    return run(arg, MODE_INV_SERIES, "fmpq_poly_inv_series");
}

int
run_fmpq_poly_div_series(const char * arg)
{
    return run(arg, MODE_DIV_SERIES, "fmpq_poly_div_series");
}
