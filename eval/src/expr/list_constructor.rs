use parser::ast::Expression;

use crate::{context::Context, error::EvalError, value::JmlValue};

use super::eval_expr;

pub(crate) fn eval_list(elems: Vec<Expression>, ctx: &Context<'_>) -> Result<JmlValue, EvalError> {
    let mut list: Vec<JmlValue> = vec![];
    for expr in elems {
        let val = eval_expr(expr, ctx)?;
        list.push(val);
    }
    Ok(JmlValue::list(list))
}
