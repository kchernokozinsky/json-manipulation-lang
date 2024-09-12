use crate::{
    context::Context,
    errors::{EvalError, TypeError, TypeErrorKind},
    jml_type::JmlType,
    value::{list::JmlList, object::JmlObject, JmlValue},
};
use indexmap::indexmap;

pub fn pluck<'source>(
    span: miette::SourceSpan,
    args: Vec<JmlValue<'source>>,
    _: &mut Context<'source>,
) -> Result<JmlValue<'source>, EvalError> {
    // First argument should be an object (JmlValue::Object)
    let obj = match &args[0] {
        JmlValue::Object(JmlObject(o)) => o,
        _ => {
            let type_error_kind = TypeErrorKind::MismatchedTypes {
                expected: vec![JmlType::Object],
                found: args[0].type_of(),
            };

            return Err(TypeError {
                kind: type_error_kind,
                span,
            }
            .into());
        }
    };

    // Second argument should be a list of keys (JmlValue::List(JmlString))
    let keys = obj.keys();

    let mut plucked_values: Vec<JmlValue> = vec![];

    // Iterate over each key, fetch value from the object, and add it to the list
    for key in keys {
        let value = obj.get(key).unwrap();
        let pair = JmlValue::object(JmlObject(indexmap! {
            "key".to_string() => JmlValue::string(key),
            "value".to_string() => value.clone(),
        }));
        plucked_values.push(pair);
    }

    // Return the list of plucked values
    Ok(JmlValue::List(JmlList(plucked_values)))
}
