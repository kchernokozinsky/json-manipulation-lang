use eval::value::JmlValue;

#[test]
fn test_unary_negation() {
    let source = "-5";
    let source_str = source.trim();

    let jml = parser::parse(source_str).expect("should successfully parse");

    let result = eval::eval_with_source(jml, source_str).expect("should successfully eval");

    assert_eq!(result, JmlValue::int(-5));
}

#[test]
fn test_logical_not() {
    let source = "!true";
    let source_str = source.trim();

    let jml = parser::parse(source_str).expect("should successfully parse");

    let result = eval::eval_with_source(jml, source_str).expect("should successfully eval");

    assert_eq!(result, JmlValue::bool(false));
}
