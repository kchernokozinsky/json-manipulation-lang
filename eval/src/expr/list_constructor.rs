use parser::ast::Expression;

use crate::{context::Context, errors::EvalError, value::JmlValue};

use super::eval_expr;

pub(crate) fn eval_list<'source>(
    elems: Vec<Expression<'source>>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    let mut list: Vec<JmlValue> = vec![];
    for expr in elems {
        let val = eval_expr(expr, ctx)?;
        list.push(val);
    }
    Ok(JmlValue::list(list))
}
