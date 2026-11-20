// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::get_parent_expr;
use clippy_utils::source::snippet;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags building a power of 2 as a [`Float`] or a [`BigExponentFloat`] the long way:
    ///
    /// - converting a power-of-2 literal, as in `Float::from_unsigned_prec(1u32, p).0` (or
    ///   `from_signed_prec`, or the `_round` variants);
    /// - shifting such a value, or `one_prec(p)`, `two_prec(p)`, `one_half_prec(p)`, or the
    ///   constants `ONE`, `TWO`, and `ONE_HALF`, as in `Float::one_prec(p) >> 1u32`.
    ///
    /// For a `Float` the value is `Float::one_prec(p)`, `Float::two_prec(p)`, or
    /// `Float::one_half_prec(p)` when it is 1, 2, or 1/2, and `Float::power_of_2_prec(k, p).0`
    /// otherwise; at precision 1 it is `Float::ONE`, `Float::TWO`, `Float::ONE_HALF`, or
    /// `Float::power_of_2(k)`. A `BigExponentFloat` has no precision, so the value is
    /// `BigExponentFloat::ONE`, `TWO`, `ONE_HALF`, or `BigExponentFloat::power_of_2(k)`. The bodies
    /// of the power-of-2 constructors and of `one_prec`, `two_prec`, and `one_half_prec`, which may
    /// define them this way, and test code that checks them against each other, are exempt.
    ///
    /// ### Why is this bad?
    ///
    /// The conversion builds a value and an `Ordering` that is always `Equal`, and the shift then
    /// builds a second value; the dedicated constructor builds the power of 2 directly, and says
    /// what it is.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let half = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// let one = Float::from_unsigned_prec(1u32, prec).0;
    /// let quarter = BigExponentFloat::ONE >> 2u32;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let half = Float::one_half_prec(100);
    /// let one = Float::one_prec(prec);
    /// let quarter = BigExponentFloat::power_of_2(-2i64);
    /// ```
    pub MANUAL_FLOAT_POWER_OF_2,
    Deny,
    "a power of 2 built by converting a literal or by shifting 1, instead of `one_prec` or \
    `power_of_2_prec`"
}

declare_lint_pass!(ManualFloatPowerOf2 => [MANUAL_FLOAT_POWER_OF_2]);

// How a power of 2 is spelled.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Spelling {
    // A conversion of a power-of-2 literal, which is flagged even without a shift.
    Conversion,
    // A dedicated constructor (`one_prec` and the like) or a named constant, which is fine alone.
    Dedicated,
}

// A power of 2, 2^`exponent`, as a value of the type named `ty` (`"Float"` or
// `"BigExponentFloat"`), with the precision `prec` if it was given one (`None` for a constant,
// whose precision is 1).
struct PowerOf2<'tcx> {
    ty: &'static str,
    exponent: i128,
    prec: Option<&'tcx Expr<'tcx>>,
    spelling: Spelling,
}

// The type of `e`, if it is `Float` or `BigExponentFloat`.
fn float_type<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<&'static str> {
    match crate::bignum_name(cx, cx.typeck_results().expr_ty(e).peel_refs())? {
        name @ ("Float" | "BigExponentFloat") => Some(name),
        _ => None,
    }
}

// The name of the associated function (or, if `constant` is set, the associated constant) that the
// path `e` resolves to.
fn assoc_name<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>, constant: bool) -> Option<String> {
    let ExprKind::Path(qpath) = &e.kind else {
        return None;
    };
    match cx.qpath_res(qpath, e.hir_id) {
        Res::Def(DefKind::AssocFn, did) if !constant => Some(cx.tcx.item_name(did).to_string()),
        Res::Def(DefKind::AssocConst { .. }, did) if constant => {
            Some(cx.tcx.item_name(did).to_string())
        }
        _ => None,
    }
}

// If `e` is a power of 2 spelled in one of the forms the lint knows, returns it.
fn power_of_2<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<PowerOf2<'tcx>> {
    let ty = float_type(cx, e)?;
    match e.kind {
        // `T::from_unsigned_prec(2^m, p).0` and the like.
        ExprKind::Field(base, field) if field.name.as_str() == "0" => {
            let ExprKind::Call(callee, [x, prec] | [x, prec, _]) = base.kind else {
                return None;
            };
            let name = assoc_name(cx, callee, false)?;
            if !matches!(
                name.as_str(),
                "from_unsigned_prec"
                    | "from_signed_prec"
                    | "from_unsigned_prec_round"
                    | "from_signed_prec_round"
            ) {
                return None;
            }
            let v = crate::literal_value(x)?;
            if v <= 0 || v.count_ones() != 1 {
                return None;
            }
            Some(PowerOf2 {
                ty,
                exponent: i128::from(v.trailing_zeros()),
                prec: Some(prec),
                spelling: Spelling::Conversion,
            })
        }
        // `T::one_prec(p)`, `T::two_prec(p)`, and `T::one_half_prec(p)`.
        ExprKind::Call(callee, [prec]) => {
            let exponent = match assoc_name(cx, callee, false)?.as_str() {
                "one_prec" => 0,
                "two_prec" => 1,
                "one_half_prec" => -1,
                _ => return None,
            };
            Some(PowerOf2 {
                ty,
                exponent,
                prec: Some(prec),
                spelling: Spelling::Dedicated,
            })
        }
        // `T::ONE`, `T::TWO`, and `T::ONE_HALF`.
        ExprKind::Path(_) => {
            let exponent = match assoc_name(cx, e, true)?.as_str() {
                "ONE" => 0,
                "TWO" => 1,
                "ONE_HALF" => -1,
                _ => return None,
            };
            Some(PowerOf2 {
                ty,
                exponent,
                prec: None,
                spelling: Spelling::Dedicated,
            })
        }
        _ => None,
    }
}

// The exponent of a power of 2, as source text of type `i64` when `literal` is not set: either a
// known number, or a base exponent plus or minus a shift amount.
enum Exponent {
    Known(i128),
    Shifted {
        base: i128,
        plus: bool,
        amount: String,
    },
}

// `text` without the parentheses around it, if it is a parenthesized expression; and whether what
// remains needs parentheses as an operand of `-` or of a binary operator.
fn bare_operand(text: &str) -> (&str, bool) {
    let mut t = text.trim();
    while t.starts_with('(') && t.ends_with(')') && balanced(&t[1..t.len() - 1]) {
        t = t[1..t.len() - 1].trim();
    }
    (t, has_top_level_space(t))
}

// Whether `text` has a space outside all parentheses, as a binary operation does and a call or a
// path does not.
fn has_top_level_space(text: &str) -> bool {
    let mut depth = 0i32;
    for c in text.chars() {
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ' ' if depth == 0 => return true,
            _ => {}
        }
    }
    false
}

// Whether the parentheses in `text` are balanced, so that it is a single parenthesized expression
// when wrapped in a pair.
fn balanced(text: &str) -> bool {
    let mut depth = 0i32;
    for c in text.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}

impl Exponent {
    // The exponent as an argument: `literal` adds an `i64` suffix to a known exponent, which a
    // `power_of_2` call needs to choose between its `u64` and `i64` implementations.
    fn text(&self, literal: bool) -> String {
        match self {
            Self::Known(k) if literal => format!("{k}i64"),
            Self::Known(k) => k.to_string(),
            Self::Shifted { base, plus, amount } => {
                let (bare, compound) = bare_operand(amount);
                let operand = if compound {
                    format!("({bare})")
                } else {
                    bare.to_string()
                };
                match (base, plus) {
                    (0, true) => bare.to_string(),
                    (0, false) => format!("-{operand}"),
                    (_, true) => format!("{base} + {operand}"),
                    (_, false) => format!("{base} - {operand}"),
                }
            }
        }
    }
}

// The shift amount `n` as source text of type `i64`.
fn amount_as_i64<'tcx>(cx: &LateContext<'tcx>, n: &'tcx Expr<'tcx>) -> String {
    let snippet = snippet(cx, n.span, "..");
    let text = bare_operand(&snippet).0;
    let ty = cx.typeck_results().expr_ty(n).peel_refs().to_string();
    match ty.as_str() {
        "i64" => text.to_string(),
        "u8" | "u16" | "u32" | "i8" | "i16" | "i32" => format!("i64::from({text})"),
        _ => format!("i64::exact_from({text})"),
    }
}

// The name to call the type by: `Self` inside an `impl` of it, where clippy's `use_self` prefers
// it, and its name elsewhere.
fn type_name(cx: &LateContext<'_>, e: &Expr<'_>, name: &'static str) -> &'static str {
    let owner = cx.tcx.hir_get_parent_item(e.hir_id).to_def_id();
    let parent = cx.tcx.parent(owner);
    if matches!(cx.tcx.def_kind(parent), DefKind::Impl { .. })
        && crate::bignum_name(
            cx,
            cx.tcx.type_of(parent).instantiate_identity().peel_refs(),
        ) == Some(name)
    {
        "Self"
    } else {
        name
    }
}

// The dedicated spelling of 2^`exponent` as a value of the type `ty`, with the precision `prec`.
fn suggestion(
    cx: &LateContext<'_>,
    ty_name: &str,
    float: bool,
    exponent: &Exponent,
    prec: Option<&Expr<'_>>,
) -> String {
    // A `Float` at precision 1 and a `BigExponentFloat` take the precision-free forms.
    let prec = prec.filter(|p| float && crate::literal_value(p) != Some(1));
    match (exponent, prec) {
        (Exponent::Known(0), None) => format!("{ty_name}::ONE"),
        (Exponent::Known(1), None) => format!("{ty_name}::TWO"),
        (Exponent::Known(-1), None) => format!("{ty_name}::ONE_HALF"),
        (_, None) => format!("{ty_name}::power_of_2({})", exponent.text(true)),
        (Exponent::Known(0), Some(p)) => {
            format!("{ty_name}::one_prec({})", snippet(cx, p.span, ".."))
        }
        (Exponent::Known(1), Some(p)) => {
            format!("{ty_name}::two_prec({})", snippet(cx, p.span, ".."))
        }
        (Exponent::Known(-1), Some(p)) => {
            format!("{ty_name}::one_half_prec({})", snippet(cx, p.span, ".."))
        }
        (_, Some(p)) => format!(
            "{ty_name}::power_of_2_prec({}, {}).0",
            exponent.text(false),
            snippet(cx, p.span, "..")
        ),
    }
}

// Whether `e` is in the definition of a constructor that the lint suggests, which may build its
// value this way.
fn in_constructor_definition(cx: &LateContext<'_>, e: &Expr<'_>) -> bool {
    ["power_of_2", "one_prec", "two_prec", "one_half_prec"]
        .iter()
        .any(|name| crate::in_fn_named_for(cx, e, name))
}

impl<'tcx> LateLintPass<'tcx> for ManualFloatPowerOf2 {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion()
            || crate::in_cross_check_code(cx, expr.span)
            || in_constructor_definition(cx, expr)
        {
            return;
        }
        // A shift of a power of 2.
        if let ExprKind::Binary(op, lhs, rhs) = expr.kind
            && matches!(op.node, BinOpKind::Shl | BinOpKind::Shr)
            && let Some(p) = power_of_2(cx, lhs)
            && float_type(cx, expr) == Some(p.ty)
        {
            let plus = op.node == BinOpKind::Shl;
            let exponent = match crate::literal_value(rhs) {
                Some(k) => Exponent::Known(if plus { p.exponent + k } else { p.exponent - k }),
                None => Exponent::Shifted {
                    base: p.exponent,
                    plus,
                    amount: amount_as_i64(cx, rhs),
                },
            };
            let ty_name = type_name(cx, expr, p.ty);
            span_lint_and_help(
                cx,
                MANUAL_FLOAT_POWER_OF_2,
                expr.span,
                "this shift builds a power of 2",
                None,
                format!(
                    "build it directly: `{}`",
                    suggestion(cx, ty_name, p.ty == "Float", &exponent, p.prec)
                ),
            );
            return;
        }
        // A conversion of a power-of-2 literal, unless a shift of it is reported instead.
        if let Some(p) = power_of_2(cx, expr)
            && p.spelling == Spelling::Conversion
            && !get_parent_expr(cx, expr).is_some_and(|parent| {
                matches!(
                    parent.kind,
                    ExprKind::Binary(op, lhs, _)
                        if matches!(op.node, BinOpKind::Shl | BinOpKind::Shr)
                            && lhs.hir_id == expr.hir_id
                )
            })
        {
            let ty_name = type_name(cx, expr, p.ty);
            span_lint_and_help(
                cx,
                MANUAL_FLOAT_POWER_OF_2,
                expr.span,
                "this conversion builds a power of 2",
                None,
                format!(
                    "build it directly: `{}`",
                    suggestion(
                        cx,
                        ty_name,
                        p.ty == "Float",
                        &Exponent::Known(p.exponent),
                        p.prec
                    )
                ),
            );
        }
    }
}
