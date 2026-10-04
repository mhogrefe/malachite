/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the RationalPolynomial addition and subtraction demos against
    fmpq_poly_add and fmpq_poly_sub. The shapes are `(P) op (Q) = R`, `(P) op &(Q) = R`,
    `&(P) op (Q) = R`, `&(P) op &(Q) = R`, `p := P; p op= Q; p = R`, and
    `p := P; p op= &Q; p = R`, with `op` either `+` or `-`. When the two operands are written
    identically, FLINT is given the same polynomial for both, so that its `poly1 == poly2` path
    runs too. Each mode treats any other nonempty line as an error, and an input with no lines at
    all.
*/

#include <stdio.h>
#include <string.h>

#include <flint/fmpq_poly.h>

#include "oracle.h"

static long checked;
static const char * current_name;
/* '+' for fmpq_poly_add, '-' for fmpq_poly_sub. */
static char current_op;

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

/* Strips a leading `&(` or `(` and the matching trailing `)` from an operand written as
   `(P)` or `&(P)`, in place. Returns NULL if it has neither form. */
static char *
unwrap(char * operand)
{
    if (operand[0] == '&')
    {
        operand++;
    }
    size_t len = strlen(operand);
    if (len < 2 || operand[0] != '(' || operand[len - 1] != ')')
    {
        return NULL;
    }
    operand[len - 1] = '\0';
    return operand + 1;
}

/* Splits a line into its two operands and its result, in place. Returns 1 on success. */
static int
split_line(char * line, char ** p, char ** q, char ** r)
{
    char needle[16];
    line[strcspn(line, "\r\n")] = '\0';
    if (strncmp(line, "p := ", 5) == 0)
    {
        snprintf(needle, sizeof(needle), "; p %c= ", current_op);
        *p = line + 5;
        *q = cut(*p, needle);
        if (*q == NULL)
        {
            return 0;
        }
        *r = cut(*q, "; p = ");
        if (**q == '&')
        {
            (*q)++;
        }
        return *r != NULL;
    }
    /* Polynomials are displayed without spaces, so `) op ` and ` = ` are unambiguous. */
    snprintf(needle, sizeof(needle), ") %c ", current_op);
    char * rest = cut(line, needle);
    if (rest == NULL)
    {
        return 0;
    }
    *r = cut(rest, " = ");
    if (*r == NULL)
    {
        return 0;
    }
    /* `cut` removed the operand's closing parenthesis along with the operator. */
    size_t len = strlen(line);
    line[len] = ')';
    line[len + 1] = '\0';
    *p = unwrap(line);
    *q = unwrap(rest);
    return *p != NULL && *q != NULL;
}

static int
check_line(char * line, int line_number)
{
    char * p_str;
    char * q_str;
    char * r_str;
    if (!split_line(line, &p_str, &q_str, &r_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name,
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    int same = strcmp(p_str, q_str) == 0;
    fmpq_poly_t p, q, expected, r;
    fmpq_poly_init(p);
    fmpq_poly_init(q);
    fmpq_poly_init(expected);
    fmpq_poly_init(r);
    if (!fmpq_poly_set_str_malachite(p, p_str) || !fmpq_poly_set_str_malachite(q, q_str)
        || !fmpq_poly_set_str_malachite(expected, r_str))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        result = 1;
    }
    else
    {
        if (current_op == '+')
        {
            fmpq_poly_add(r, p, same ? p : q);
        }
        else
        {
            fmpq_poly_sub(r, p, same ? p : q);
        }
        if (!fmpq_poly_equal(r, expected))
        {
            flint_printf("error in %s test, line %d. FLINT: ", current_name, line_number);
            fmpq_poly_print_pretty(r, "x");
            flint_printf("\n");
            result = 1;
        }
    }
    fmpq_poly_clear(p);
    fmpq_poly_clear(q);
    fmpq_poly_clear(expected);
    fmpq_poly_clear(r);
    return result;
}

static int
run(const char * arg, char op, const char * name)
{
    checked = 0;
    current_op = op;
    current_name = name;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpq_poly_add(const char * arg)
{
    return run(arg, '+', "fmpq_poly_add");
}

int
run_fmpq_poly_sub(const char * arg)
{
    return run(arg, '-', "fmpq_poly_sub");
}
