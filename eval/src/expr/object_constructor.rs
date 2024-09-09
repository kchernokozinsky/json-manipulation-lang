use indexmap::IndexMap;
use parser::ast::Expression;

use crate::{context::Context, errors::EvalError, value::JmlValue};

use super::eval_expr;

pub(crate) fn eval_object<'source>(
    data: IndexMap<&str, Expression<'source>>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    let mut result_map = IndexMap::new();

    for (k, expr) in data {
        let evaluated_value = eval_expr(expr, ctx)?;
        result_map.insert(k.to_owned(), evaluated_value);
    }

    Ok(JmlValue::Object(result_map.into()))
}
