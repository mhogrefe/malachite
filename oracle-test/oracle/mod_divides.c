/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `b.mod_div(c, m) = Some(q)|None` lines against fmpz_mod_divides, and
    `b.mod_div_list(c, m) = Some((s, t, l))|None` lines against fmpz_divides_mod_list. Both the
    Natural and the primitive-integer demos print these shapes, so each mode serves both.
*/

#include <string.h>

#include <flint/fmpz_mod.h>

#include "oracle.h"

static int
check_fmpz_mod_divides_line(char * line, int line_number)
{
    char * pieces[3];
    char * rest;
    if (!split_method_call(line, ".mod_div(", pieces, 3, &rest))
    {
        return 0;
    }
    int result = 0;
    fmpz_t b, c, n, a, expected;
    fmpz_init(b);
    fmpz_init(c);
    fmpz_init(n);
    fmpz_init(a);
    fmpz_init(expected);
    fmpz_set_str(b, pieces[0], 10);
    fmpz_set_str(c, pieces[1], 10);
    fmpz_set_str(n, pieces[2], 10);
    int has_expected = parse_option_fmpz(rest, expected);
    fmpz_mod_ctx_t ctx;
    fmpz_mod_ctx_init(ctx, n);
    int ok = fmpz_mod_divides(a, b, c, ctx);
    fmpz_mod_ctx_clear(ctx);
    if (ok != has_expected || (ok && !fmpz_equal(a, expected)))
    {
        flint_printf("error in fmpz_mod_divides test, line %d. FLINT: ok=%d a=", line_number, ok);
        fmpz_print(a);
        flint_printf("\n");
        result = 1;
    }
    fmpz_clear(b);
    fmpz_clear(c);
    fmpz_clear(n);
    fmpz_clear(a);
    fmpz_clear(expected);
    return result;
}

int
run_fmpz_mod_divides(const char * arg)
{
    return for_each_line(arg, check_fmpz_mod_divides_line);
}

static int
check_fmpz_divides_mod_list_line(char * line, int line_number)
{
    char * pieces[3];
    char * rest;
    if (!split_method_call(line, ".mod_div_list(", pieces, 3, &rest))
    {
        return 0;
    }
    int result = 0;
    fmpz_t b, c, n, s, t, l, es, et, el;
    fmpz_init(b);
    fmpz_init(c);
    fmpz_init(n);
    fmpz_init(s);
    fmpz_init(t);
    fmpz_init(l);
    fmpz_init(es);
    fmpz_init(et);
    fmpz_init(el);
    fmpz_set_str(b, pieces[0], 10);
    fmpz_set_str(c, pieces[1], 10);
    fmpz_set_str(n, pieces[2], 10);
    /* parse " = Some((s, t, l))" or " = None" */
    int has_expected = 0;
    char * some = strstr(rest, "Some((");
    if (some != NULL)
    {
        char * expected_pieces[3];
        char * p = some + strlen("Some((");
        expected_pieces[0] = p;
        char * sep = strstr(p, ", ");
        *sep = '\0';
        p = sep + 2;
        expected_pieces[1] = p;
        sep = strstr(p, ", ");
        *sep = '\0';
        p = sep + 2;
        expected_pieces[2] = p;
        sep = strchr(p, ')');
        *sep = '\0';
        fmpz_set_str(es, expected_pieces[0], 10);
        fmpz_set_str(et, expected_pieces[1], 10);
        fmpz_set_str(el, expected_pieces[2], 10);
        has_expected = 1;
    }
    /* the argument order is (dividend, divisor, modulus): solutions x of divisor * x = dividend */
    fmpz_divides_mod_list(s, t, l, b, c, n);
    int ok = !fmpz_is_zero(l);
    if (ok != has_expected
        || (ok && !(fmpz_equal(s, es) && fmpz_equal(t, et) && fmpz_equal(l, el))))
    {
        flint_printf("error in fmpz_divides_mod_list test, line %d. FLINT: s=", line_number);
        fmpz_print(s);
        flint_printf(" t=");
        fmpz_print(t);
        flint_printf(" l=");
        fmpz_print(l);
        flint_printf("\n");
        result = 1;
    }
    fmpz_clear(b);
    fmpz_clear(c);
    fmpz_clear(n);
    fmpz_clear(s);
    fmpz_clear(t);
    fmpz_clear(l);
    fmpz_clear(es);
    fmpz_clear(et);
    fmpz_clear(el);
    return result;
}

int
run_fmpz_divides_mod_list(const char * arg)
{
    return for_each_line(arg, check_fmpz_divides_mod_list_line);
}
