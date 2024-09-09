use parser::ast::Expression;

use crate::{
    context::Context,
    error::{EvalError, TypeError, TypeErrorKind},
    jml_type::JmlType,
    value::JmlValue,
};

use super::eval_expr;
pub fn eval_if_expr<'source>(
    condition: Expression<'source>,
    then_branch: Expression<'source>,
    else_branch: Expression<'source>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    let cond_l = condition.l;
    let cond_r = condition.r;
    let cond = eval_expr(condition, ctx)?;
    if !cond.is_bool() {
        let type_error = TypeError {
            span: (cond_l, cond_r - cond_l).into(),
            kind: TypeErrorKind::MismatchedTypes {
                expected: vec![JmlType::Bool],
                found: cond.type_of(),
            },
        };
        Err(type_error)?
    }

    if cond.is_truthy() {
        eval_expr(then_branch, ctx)
    } else {
        eval_expr(else_branch, ctx)
    }
}
