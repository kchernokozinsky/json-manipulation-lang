
fn main() {
    let source = r#"
    a = 1
    d = "some_string"
    // c = \x y. x + y
    ---
    
    {
        "key1": -a, 
        "key2": d,
        "key3": d."as" + d
    }"#;
    let ast = parser::parse(source).unwrap();
    println!("{:?}", ast);
}