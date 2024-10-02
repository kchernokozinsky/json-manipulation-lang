use indexmap::IndexMap;
use parser::ast::{Expression, Key};

use crate::{
    context::Context,
    errors::{EvalError, TypeError, TypeErrorKind},
    jml_type::JmlType,
    value::{float::JmlFloat, integer::JmlInt, string::JmlString, JmlValue},
};

use super::eval_expr;

pub(crate) fn eval_object<'source>(
    data: Vec<(Key<'source>, Expression<'source>)>,
    ctx: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    let mut result_map: IndexMap<String, JmlValue<'source>> = IndexMap::new();

    for (k, expr) in data {
        let evaluated_value = eval_expr(expr, ctx)?;
        match k {
            Key::Ident(key) => {
                result_map.insert(key.node.to_string(), evaluated_value);
            }
            Key::Expression(expr) => {
                let key = eval_expr(expr.clone(), ctx)?;
                match key {
                    JmlValue::String(JmlString(str)) => {
                        result_map.insert(str, evaluated_value);
                    }
                    JmlValue::Int(JmlInt(i)) => {
                        result_map.insert( i.to_string(), evaluated_value);
                    }
                    JmlValue::Float(JmlFloat(f)) => {
                        result_map.insert(f.to_string(), evaluated_value);
                    }
                    _ => {
                        let type_error_kind = TypeErrorKind::MismatchedTypes {
                            expected: vec![JmlType::String],
                            found: key.type_of(),
                        };

                        return Err(TypeError {
                            kind: type_error_kind,
                            span: (expr.l, expr.r - expr.l).into(),
                        }
                        .into());
                    }
                }
            }
        }
    }

    Ok(JmlValue::Object(result_map.into()))
}
