use eval::value::JmlValue;
use proptest::prelude::*;

#[test]
fn test_list_concatenation() {
    proptest!(|(list1 in proptest::collection::vec(any::<i64>(), 0..100), list2 in proptest::collection::vec(any::<i64>(), 0..100))| {
        let list1_str = format!("[{}]", list1.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
        let list2_str = format!("[{}]", list2.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
        let source = format!("{} ++ {}", list1_str, list2_str).leak();

        let jml = parser::parse(source).expect("should successfully parse");

        let result = eval::eval_with_source(jml, source).expect("should successfully eval");

        let expected_list = list1.iter().chain(list2.iter()).map(|&v| JmlValue::int(v)).collect::<Vec<_>>();
        assert_eq!(result, JmlValue::list(expected_list));
    });
}

#[test]
fn test_list_indexing() {
    proptest!(|(list in proptest::collection::vec(any::<i64>(), 1..100), idx in 0usize..100)| {

        let list_str = format!("[{}]", list.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
        let source = format!("{}[{}]", list_str, idx).leak();

        let jml = parser::parse(source).expect("should successfully parse");

        let result = eval::eval_with_source(jml, source).expect("should successfully eval");

        if idx < list.len() {

        assert_eq!(result, JmlValue::int(list[idx]));
        } else {
            assert_eq!(result, JmlValue::null());
        }
    });
}

#[test]
fn test_list_creation() {
    let source = r#"
        myList = [1, 2, 3, 4, 5]
        ---
        myList
    "#;
    let source_str = source.trim();

    let expected_list = JmlValue::list(vec![
        JmlValue::int(1),
        JmlValue::int(2),
        JmlValue::int(3),
        JmlValue::int(4),
        JmlValue::int(5),
    ]);

    let jml = parser::parse(source_str).expect("should successfully parse");

    let result = eval::eval_with_source(jml, source_str).expect("should successfully eval");

    assert_eq!(result, expected_list);
}

#[test]
fn test_list_operations() {
    let source = r#"
        numbers = [1, 2, 3]
        total = numbers[0] + numbers[1] + numbers[2]
        ---
        total
    "#;
    let source_str = source.trim();

    let jml = parser::parse(source_str).expect("should successfully parse");

    let result = eval::eval_with_source(jml, source_str).expect("should successfully eval");

    assert_eq!(result, JmlValue::int(6));
}

#[test]
fn test_list_of_objects() {
    let source = r#"
        users = [
            { "id": 1, "name": "Alice" },
            { "id": 2, "name": "Bob" }
        ]
        ---
        users[1].name
    "#;
    let source_str = source.trim();

    let jml = parser::parse(source_str).expect("should successfully parse");

    let result = eval::eval_with_source(jml, source_str).expect("should successfully eval");

    assert_eq!(result, JmlValue::string("Bob".to_string()));
}
