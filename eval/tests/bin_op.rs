use eval::value::JmlValue;
use proptest::prelude::*;

#[test]
fn test_subtraction_operations() {
    proptest!(|(a in -1000i64..1000, b in -1000i64..1000)| {
        let source = format!("{} - {}", a, b).leak();

        let jml = parser::parse(source).expect("should successfully parse");

        let result: JmlValue = eval::eval_with_source(jml, source).expect("should successfully eval");

        assert_eq!(result, JmlValue::int(a - b));
    });
}

#[test]
fn test_multiplication_operations() {
    proptest!(|(a in -100i64..100, b in -100i64..100)| {
        let source = format!("{} * {}", a, b).leak();

        let jml = parser::parse(source).expect("should successfully parse");

        let result: JmlValue = eval::eval_with_source(jml, source).expect("should successfully eval");

        assert_eq!(result, JmlValue::int(a * b));
    });
}

#[test]
fn test_division_operations() {
    proptest!(|(a in -1000i64..1000, b in -1000i64..1000)| {
        prop_assume!(b != 0);

        let source = format!("{} / {}", a, b).leak();

        let jml = parser::parse(source).expect("should successfully parse");

        let result: JmlValue = eval::eval_with_source(jml, source).expect("should successfully eval");

        let expected = a / b;
        assert_eq!(result, JmlValue::int(expected));
    });
}

#[test]
fn test_modulo_operations() {
    proptest!(|(a in -1000i64..1000, b in 1i64..1000)| {
        prop_assume!(b != 0);

        let source = format!("{} % {}", a, b).leak();

        let jml = parser::parse(source).expect("should successfully parse");

        let result: JmlValue = eval::eval_with_source(jml, source).expect("should successfully eval");

        let expected = a % b;
        assert_eq!(result, JmlValue::int(expected));
    });
}

#[test]
fn test_exponentiation_operations() {
    proptest!(|(a in -10i64..10, b in 0u32..5)| {
        let source = format!("{} ^ {}", a, b).leak();

        let jml = parser::parse(source).expect("should successfully parse");

        let result: JmlValue = eval::eval_with_source(jml, source).expect("should successfully eval");

        let expected = a.pow(b);
        assert_eq!(result, JmlValue::int(expected));
    });
}

#[test]
fn test_combined_arithmetic_operations() {
    proptest!(|(a in -100i64..100, b in -100i64..100, c in -100i64..100)| {
        prop_assume!(b != 0 && c != 0);

        let source = format!("({} + {}) * {} / {}", a, b, c, b).leak();

        let jml = parser::parse(source).expect("should successfully parse");

        let result: JmlValue = eval::eval_with_source(jml, source).expect("should successfully eval");

        let expected = ((a + b) * c) / b;
        assert_eq!(result, JmlValue::int(expected));
    });
}

#[test]
fn test_addition_operations() {
    proptest!(|(a in -1000i64..1000, b in -1000i64..1000)| {
        let source = format!("{} + {}", a, b).leak();

        let jml = parser::parse(source).expect("should successfully parse");

        let result: JmlValue = eval::eval_with_source(jml, source).expect("should successfully eval");

        assert_eq!(result, JmlValue::int(a + b));
    });
}

// #[test]
// fn test_string_concatenation() {
//     proptest!(|(s1 in ".*", s2 in ".*")| {
//         let source = format!(r#""{}" ++ "{}""#, s1, s2).leak();
//         let jml = parser::parse(source).expect("should successfully parse");

//         let result: JmlValue = eval::eval_with_source(jml, source).expect("should successfully eval");
//         assert_eq!(result, JmlValue::string(format!("{}{}", s1, s2)));
//     });
// }
