/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the UnsignedPolynomial and NaturalPolynomial modular composition
    demos against fmpz_mod_poly_compose with modulus m and, when m is at least 2 and fits in a word,
    against nmod_poly_compose as well. The shapes recognized are `(P).mod_compose(Q, m) = R`, with
    either operand written `&(P)` or `&(Q)` instead, and `p := P; p.mod_compose_assign(Q, m); p =
    R`, also with `&(Q)`. Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <string.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_poly.h>
#include <flint/nmod_poly.h>

#include "oracle.h"

static long checked;
static long nmod_lines;

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

/* Strips `&(` and the matching `)` from an operand written `&(Q)`, in place. */
static char *
strip_ref(char * operand)
{
    size_t len = strlen(operand);
    if (len >= 3 && operand[0] == '&' && operand[1] == '(' && operand[len - 1] == ')')
    {
        operand[len - 1] = '\0';
        return operand + 2;
    }
    return operand;
}

/* Splits a line into its two operands, the exponent, and the result, in place. Polynomials are
   displayed without spaces or parentheses, so the separators are unambiguous. */
static int
split_line(char * line, char ** p, char ** q, char ** k, char ** r)
{
    line[strcspn(line, "\r\n")] = '\0';
    char * args;
    if (strncmp(line, "p := ", 5) == 0)
    {
        *p = line + 5;
        args = cut(*p, "; p.mod_compose_assign(");
        if (args == NULL)
        {
            return 0;
        }
        *r = cut(args, "); p = ");
    }
    else
    {
        args = cut(line, ").mod_compose(");
        if (args == NULL || line[0] != '(')
        {
            return 0;
        }
        *p = line + 1;
        if ((*p)[0] == '&')
        {
            size_t len = strlen(*p);
            if (len < 3 || (*p)[1] != '(' || (*p)[len - 1] != ')')
            {
                return 0;
            }
            (*p)[len - 1] = '\0';
            *p += 2;
        }
        *r = cut(args, ") = ");
    }
    if (*r == NULL)
    {
        return 0;
    }
    *k = cut(args, ", ");
    if (*k == NULL)
    {
        return 0;
    }
    *q = strip_ref(args);
    return 1;
}

static int
check_line(char * line, int line_number)
{
    char * p_str;
    char * q_str;
    char * k_str;
    char * r_str;
    if (!split_line(line, &p_str, &q_str, &k_str, &r_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in mod_compose test, line %d: unrecognized line\n",
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    fmpz_poly_t p, q, expected, r;
    fmpz_t m;
    fmpz_poly_init(p);
    fmpz_poly_init(q);
    fmpz_poly_init(expected);
    fmpz_poly_init(r);
    fmpz_init(m);
    if (!fmpz_poly_set_str_malachite(p, p_str) || !fmpz_poly_set_str_malachite(q, q_str)
        || !fmpz_poly_set_str_malachite(expected, r_str) || fmpz_set_str(m, k_str, 10) != 0
        || fmpz_sgn(m) <= 0)
    {
        flint_printf("error in mod_compose test, line %d: unreadable input\n",
                     line_number);
        result = 1;
        goto cleanup;
    }
    {
        fmpz_mod_ctx_t ctx;
        fmpz_mod_ctx_init(ctx, m);
        fmpz_mod_poly_t a, b, c;
        fmpz_mod_poly_init(a, ctx);
        fmpz_mod_poly_init(b, ctx);
        fmpz_mod_poly_init(c, ctx);
        fmpz_mod_poly_set_fmpz_poly(a, p, ctx);
        fmpz_mod_poly_set_fmpz_poly(b, q, ctx);
        fmpz_mod_poly_compose(c, a, b, ctx);
        fmpz_mod_poly_get_fmpz_poly(r, c, ctx);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in mod_compose test, line %d, by "
                         "fmpz_mod_poly_compose. FLINT: ",
                         line_number);
            fmpz_poly_print(r);
            flint_printf("\n");
            result = 1;
        }
        fmpz_mod_poly_clear(a, ctx);
        fmpz_mod_poly_clear(b, ctx);
        fmpz_mod_poly_clear(c, ctx);
        fmpz_mod_ctx_clear(ctx);
    }
    if (result == 0 && fmpz_abs_fits_ui(m) && fmpz_cmp_ui(m, 2) >= 0)
    {
        nmod_lines++;
        nmod_poly_t a, b, c;
        nmod_poly_init(a, fmpz_get_ui(m));
        nmod_poly_init(b, fmpz_get_ui(m));
        nmod_poly_init(c, fmpz_get_ui(m));
        fmpz_poly_get_nmod_poly(a, p);
        fmpz_poly_get_nmod_poly(b, q);
        nmod_poly_compose(c, a, b);
        fmpz_poly_set_nmod_poly_unsigned(r, c);
        if (!fmpz_poly_equal(r, expected))
        {
            flint_printf("error in mod_compose test, line %d, by nmod_poly_compose. "
                         "FLINT: ",
                         line_number);
            fmpz_poly_print(r);
            flint_printf("\n");
            result = 1;
        }
        nmod_poly_clear(a);
        nmod_poly_clear(b);
        nmod_poly_clear(c);
    }
cleanup:
    fmpz_poly_clear(p);
    fmpz_poly_clear(q);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(r);
    fmpz_clear(m);
    return result;
}

int
run_mod_compose(const char * arg)
{
    checked = 0;
    nmod_lines = 0;
    int result = for_each_line(arg, check_line);
    if (result == 0)
    {
        flint_printf("mod_compose: %ld lines by fmpz_mod_poly_compose, %ld of them also "
                     "by nmod_poly_compose\n",
                     checked, nmod_lines);
    }
    return result != 0 ? result : require_some_lines("mod_compose", checked);
}
