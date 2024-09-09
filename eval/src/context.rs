use std::{cell::RefCell, collections::HashMap};

use parser::ast::Expression;

use crate::{error::RuntimeErrorKind, value::JmlValue};

#[derive(Debug, Default, Clone)]
pub struct Context<'source> {
    bindings: HashMap<String, RefCell<Binding<'source>>>,
    locals: Vec<Context<'source>>,
}

// planning for lazy evaluation here
#[derive(Debug, Clone)]
pub enum Binding<'source> {
    Expression(Expression<'source>),
    Value(JmlValue<'source>),
}

impl<'source> Binding<'source> {
    fn new(expr: Expression<'source>) -> Self {
        Binding::Expression(expr)
    }

    fn new_with_value(value: JmlValue<'source>) -> Self {
        Binding::Value(value)
    }
}

impl<'source> Context<'source> {
    pub fn new() -> Self {
        Context::default()
    }

    pub fn bind_with_expr(&mut self, name: String, expr: Expression<'source>) {
        self.bindings.insert(name, RefCell::new(Binding::new(expr)));
    }

    pub fn bind_with_value(&mut self, name: String, value: JmlValue<'source>) {
        self.bindings
            .entry(name)
            .and_modify(|e| *e.get_mut() = Binding::new_with_value(value.clone()))
            .or_insert(RefCell::new(Binding::new_with_value(value)));
    }

    pub fn add_local_ctx(&mut self, ctx: Context<'source>) {
        self.locals.push(ctx)
    }

    pub fn pop_local_ctx(&mut self) {
        self.locals.pop();
    }

    pub fn lookup_variable<'context>(
        &'context self,
        name: impl AsRef<str>,
    ) -> Result<Binding<'source>, RuntimeErrorKind> {
        let local = self.locals.last();

        match local {
            Some(local) => match local._lookup_variable(name.as_ref()) {
                Ok(binding) => Ok(binding),
                Err(_) => self._lookup_variable(name.as_ref()),
            },
            None => self._lookup_variable(name),
        }
    }

    fn _lookup_variable<'context>(
        &'context self,
        name: impl AsRef<str>,
    ) -> Result<Binding<'source>, RuntimeErrorKind> {
        self.bindings
            .get(name.as_ref())
            .map(|b| b.borrow().clone())
            .ok_or(RuntimeErrorKind::UndefinedVariable {
                name: name.as_ref().to_owned(),
            })
    }
}
