use std::{cell::RefCell, collections::HashMap};

use parser::ast::Expression;

use crate::value::JmlValue;

struct Context<'a> {
    bindings: HashMap<String, RefCell<Binding<'a>>>,
}

// planning for lazy evaluation here
enum Binding<'a> {
    Expression(Expression<'a>),
    Value(JmlValue),
}

impl<'a> Binding<'a> {
    fn new(expr: Expression<'a>) -> Self {
        Binding::Expression(expr)
    }

    fn eval(&mut self) -> &JmlValue {
        match self {
            Binding::Expression(expr) => todo!(), // expression evaluation should be done here
            Binding::Value(val) => val,
        }
    }
}

impl<'a> Context<'a> {
    fn new() -> Self {
        Context {
            bindings: HashMap::new(),
        }
    }

    fn bind(&mut self, name: String, expr: Expression<'a>) {
        self.bindings.insert(name, RefCell::new(Binding::new(expr)));
    }

    fn get(&self, name: impl AsRef<str>) -> Option<JmlValue> {
        self.bindings
            .get(name.as_ref())
            .map(|binding| binding.borrow_mut().eval().clone())
    }
}
