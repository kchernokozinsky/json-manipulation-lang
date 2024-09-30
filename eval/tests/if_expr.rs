use eval::value::JmlValue;

#[test]
fn test_simple_if_expression() {
    let source = r#"
        if 1 < 2 then "Yes" else "No"
    "#;
    let source_str = source.trim();

    let jml = parser::parse(source_str).expect("should successfully parse");

    let result = eval::eval_with_source(jml, source_str).expect("should successfully eval");

    assert_eq!(result, JmlValue::string("Yes".to_string()));
}

#[test]
fn test_nested_if_expression() {
    let source = r#"
        x = 10
        ---
        if x > 5 then
            if x < 15 then
                "Between 5 and 15"
             else
                "Greater than or equal to 15"
        else 
            "5 or less"
    "#;

    let source_str = source.trim();

    let jml = parser::parse(source_str).expect("should successfully parse");

    let result = eval::eval_with_source(jml, source_str).expect("should successfully eval");

    assert_eq!(result, JmlValue::string("Between 5 and 15".to_string()));
}
