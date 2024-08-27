use std::{cell::RefCell, collections::HashMap};

use parser::ast::Expression;

use crate::{error::RuntimeErrorKind, value::JmlValue};

#[derive(Debug, Default)]
pub struct Context<'a> {
    bindings: HashMap<String, RefCell<Binding<'a>>>,
}

// planning for lazy evaluation here
#[derive(Debug, Clone)]
pub enum Binding<'a> {
    Expression(Expression<'a>),
    Value(JmlValue),
}

impl<'a> Binding<'a> {
    fn new(expr: Expression<'a>) -> Self {
        Binding::Expression(expr)
    }

    fn new_with_value(value: JmlValue) -> Self {
        Binding::Value(value)
    }
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context::default()
    }

    pub fn bind_with_expr(&mut self, name: String, expr: Expression<'a>) {
        self.bindings.insert(name, RefCell::new(Binding::new(expr)));
    }

    pub fn bind_with_value(&mut self, name: String, value: JmlValue) {
        self.bindings
            .entry(name)
            .and_modify(|e| *e.get_mut() = Binding::new_with_value(value.clone()))
            .or_insert(RefCell::new(Binding::new_with_value(value)));
    }

    pub fn lookup_variable(&self, name: impl AsRef<str>) -> Result<Binding, RuntimeErrorKind> {
        self.bindings
            .get(name.as_ref())
            .map(|b| b.borrow().clone())
            .ok_or(RuntimeErrorKind::UndefinedVariable {
                name: name.as_ref().to_owned(),
            })
    }
}
