use std::collections::HashMap;

use parser::ast::Expression;

use crate::{context::Context, error::EvalError, value::JmlValue};

use super::eval_expr;

pub(crate) fn eval_object(
    data: HashMap<&str, Expression>,
    ctx: &Context<'_>,
) -> Result<JmlValue, EvalError> {
    let mut result_map = HashMap::new();

    for (k, expr) in data {
        let evaluated_value = eval_expr(expr, ctx)?;
        result_map.insert(k.to_owned(), evaluated_value);
    }

    Ok(JmlValue::Object(result_map.into()))
}
