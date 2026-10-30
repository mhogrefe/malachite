/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the IntegerPolynomial division demos against FLINT. Mode
    fmpz_poly_divrem reads `div_rem`, `checked_div_rem`, `div_assign_rem`, `checked_div`, and
    `checked_rem`, and the operators `/`, `/=`, `%`, and `%=`; a result must match
    fmpz_poly_divrem, fmpz_poly_div, or fmpz_poly_rem, and `None` must be where the exact
    _fmpz_poly_divrem fails or the divisor is 0. Mode fmpz_poly_divexact reads `div_exact`,
    `div_exact_assign`, `checked_div_exact`, and `divisible_by`, checked against fmpz_poly_divides
    and fmpz_poly_divexact. Mode fmpz_poly_remove reads `remove_power` and `remove_power_assign`,
    checked against fmpz_poly_remove, except for a constant divisor when the dividend's value at 1
    is 0, where FLINT divides by zero; and mode fmpz_poly_div_root reads `div_root` and
    `div_root_assign`, checked against fmpz_poly_div_root_fmpz and fmpz_poly_evaluate_fmpz.
    Arguments may be written `&(...)`. Any other nonempty line is an error, as is an input with no
    lines at all.
*/

#include <stdlib.h>
#include <string.h>
#include <flint/fmpz.h>
#include <flint/fmpz_poly.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

static long checked;
static int mode;
static const char * mode_name;

enum { MODE_DIVREM, MODE_DIVEXACT, MODE_REMOVE, MODE_DIV_ROOT };

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
read_poly(fmpz_poly_t p, const char * s)
{
    return s != NULL && fmpz_poly_set_str_malachite(p, s);
}

static int
check_divrem(struct parsed * l, const fmpz_poly_t a, const fmpz_poly_t b, int line_number)
{
    /* What the line claims: the quotient, the remainder, or both. */
    int want_q = 0, want_r = 0;
    const char * q_str = NULL;
    const char * r_str = NULL;
    const char * m = l->method;
    if (strcmp(m, "div_rem") == 0 || strcmp(m, "checked_div_rem") == 0)
    {
        want_q = want_r = 1;
        q_str = l->x;
        r_str = l->y;
    }
    else if (strcmp(m, "div_assign_rem") == 0)
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
    int result = 0;
    fmpz_poly_t q, r, eq, er;
    fmpz_poly_init(q);
    fmpz_poly_init(r);
    fmpz_poly_init(eq);
    fmpz_poly_init(er);
    int exact = 0;
    if (!fmpz_poly_is_zero(b))
    {
        if (a->length < b->length)
            exact = 1;
        else
        {
            fmpz * qq = _fmpz_vec_init(a->length - b->length + 1);
            fmpz * rr = _fmpz_vec_init(a->length);
            exact = _fmpz_poly_divrem(qq, rr, a->coeffs, a->length, b->coeffs, b->length, 1);
            _fmpz_vec_clear(qq, a->length - b->length + 1);
            _fmpz_vec_clear(rr, a->length);
        }
    }
    if (!l->has_result)
    {
        if (exact)
            result = fail("None, but the exact division succeeds", line_number);
        goto cleanup;
    }
    if (!exact)
    {
        result = fail("a result, but the exact division fails", line_number);
        goto cleanup;
    }
    fmpz_poly_divrem(q, r, a, b);
    if ((want_q && (!read_poly(eq, q_str) || !fmpz_poly_equal(q, eq)))
        || (want_r && (!read_poly(er, r_str) || !fmpz_poly_equal(r, er))))
    {
        result = fail("disagrees with fmpz_poly_divrem", line_number);
        goto cleanup;
    }
    if (want_q)
    {
        fmpz_poly_div(q, a, b);
        if (!fmpz_poly_equal(q, eq))
            result = fail("disagrees with fmpz_poly_div", line_number);
    }
    if (want_r)
    {
        fmpz_poly_rem(r, a, b);
        if (!fmpz_poly_equal(r, er))
            result = fail("disagrees with fmpz_poly_rem", line_number);
    }
cleanup:
    fmpz_poly_clear(q);
    fmpz_poly_clear(r);
    fmpz_poly_clear(eq);
    fmpz_poly_clear(er);
    return result;
}

static int
check_divexact(struct parsed * l, const fmpz_poly_t a, const fmpz_poly_t b, int line_number)
{
    const char * m = l->method;
    int result = 0;
    fmpz_poly_t q, e;
    fmpz_poly_init(q);
    fmpz_poly_init(e);
    int divides = 0;
    if (fmpz_poly_is_zero(b))
        divides = fmpz_poly_is_zero(a);
    else
        divides = fmpz_poly_divides(q, a, b);
    if (strcmp(m, "divisible_by") == 0)
    {
        if ((strcmp(l->x, "true") == 0) != divides)
            result = fail("disagrees with fmpz_poly_divides", line_number);
    }
    else if (strcmp(m, "checked_div_exact") == 0 && !l->has_result)
    {
        if (divides && !fmpz_poly_is_zero(b))
            result = fail("None, but fmpz_poly_divides finds a quotient", line_number);
    }
    else if (strcmp(m, "checked_div_exact") == 0 || strcmp(m, "div_exact") == 0
             || strcmp(m, "div_exact_assign") == 0)
    {
        if (!divides || !read_poly(e, l->x) || !fmpz_poly_equal(q, e))
            result = fail("disagrees with fmpz_poly_divides", line_number);
        else if (strcmp(m, "checked_div_exact") != 0)
        {
            fmpz_poly_divexact(q, a, b);
            if (!fmpz_poly_equal(q, e))
                result = fail("disagrees with fmpz_poly_divexact", line_number);
        }
    }
    else
        result = fail("unrecognized method", line_number);
    fmpz_poly_clear(q);
    fmpz_poly_clear(e);
    return result;
}

static int
check_remove(struct parsed * l, const fmpz_poly_t a, const fmpz_poly_t b, int line_number)
{
    if (strcmp(l->method, "remove_power") != 0 && strcmp(l->method, "remove_power_assign") != 0)
        return fail("unrecognized method", line_number);
    int result = 0;
    fmpz_poly_t s, e;
    fmpz_t sum;
    fmpz_poly_init(s);
    fmpz_poly_init(e);
    fmpz_init(sum);
    ulong expected = strtoul(l->y, NULL, 10);
    if (!read_poly(e, l->x))
    {
        result = fail("unreadable input", line_number);
        goto cleanup;
    }
    {
        fmpz_t one;
        fmpz_init_set_ui(one, 1);
        fmpz_poly_evaluate_fmpz(sum, a, one);
        fmpz_clear(one);
    }
    if (b->length == 1 && fmpz_is_zero(sum))
    {
        /* FLINT divides by zero here; check q^e * s = a instead. */
        fmpz_poly_t t;
        fmpz_poly_init(t);
        fmpz_poly_pow(t, b, expected);
        fmpz_poly_mul(t, t, e);
        if (!fmpz_poly_equal(t, a))
            result = fail("b^e * s is not a", line_number);
        fmpz_poly_clear(t);
        goto cleanup;
    }
    slong k = fmpz_poly_remove(s, a, b);
    if ((ulong) k != expected || !fmpz_poly_equal(s, e))
        result = fail("disagrees with fmpz_poly_remove", line_number);
cleanup:
    fmpz_poly_clear(s);
    fmpz_poly_clear(e);
    fmpz_clear(sum);
    return result;
}

static int
check_div_root(struct parsed * l, const fmpz_poly_t a, const fmpz_t c, int line_number)
{
    if (strcmp(l->method, "div_root") != 0 && strcmp(l->method, "div_root_assign") != 0)
        return fail("unrecognized method", line_number);
    int result = 0;
    fmpz_poly_t q, e;
    fmpz_t r, er;
    fmpz_poly_init(q);
    fmpz_poly_init(e);
    fmpz_init(r);
    fmpz_init(er);
    if (!read_poly(e, l->x) || fmpz_set_str(er, l->y, 10) != 0)
    {
        result = fail("unreadable input", line_number);
        goto cleanup;
    }
    fmpz_poly_div_root_fmpz(q, a, c);
    fmpz_poly_evaluate_fmpz(r, a, c);
    if (!fmpz_poly_equal(q, e) || !fmpz_equal(r, er))
        result = fail("disagrees with fmpz_poly_div_root_fmpz", line_number);
cleanup:
    fmpz_poly_clear(q);
    fmpz_poly_clear(e);
    fmpz_clear(r);
    fmpz_clear(er);
    return result;
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
    checked++;
    int result;
    fmpz_poly_t a, b;
    fmpz_t c;
    fmpz_poly_init(a);
    fmpz_poly_init(b);
    fmpz_init(c);
    if (!read_poly(a, l.a)
        || (mode == MODE_DIV_ROOT ? fmpz_set_str(c, l.b, 10) != 0 : !read_poly(b, l.b)))
    {
        result = fail("unreadable input", line_number);
    }
    else if (mode == MODE_DIVREM)
        result = check_divrem(&l, a, b, line_number);
    else if (mode == MODE_DIVEXACT)
        result = check_divexact(&l, a, b, line_number);
    else if (mode == MODE_REMOVE)
        result = check_remove(&l, a, b, line_number);
    else
        result = check_div_root(&l, a, c, line_number);
    fmpz_poly_clear(a);
    fmpz_poly_clear(b);
    fmpz_clear(c);
    return result;
}

static int
run(const char * arg, int m, const char * name)
{
    checked = 0;
    mode = m;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    if (result == 0)
        flint_printf("%s: %ld lines agree with FLINT\n", name, checked);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_poly_divrem(const char * arg)
{
    return run(arg, MODE_DIVREM, "fmpz_poly_divrem");
}

int
run_fmpz_poly_divexact(const char * arg)
{
    return run(arg, MODE_DIVEXACT, "fmpz_poly_divexact");
}

int
run_fmpz_poly_remove(const char * arg)
{
    return run(arg, MODE_REMOVE, "fmpz_poly_remove");
}

int
run_fmpz_poly_div_root(const char * arg)
{
    return run(arg, MODE_DIV_ROOT, "fmpz_poly_div_root");
}
