use std::{cell::RefCell, collections::HashMap, rc::Rc};

use parser::ast::Expression;

use crate::{errors::RuntimeErrorKind, value::JmlValue};

#[derive(Debug, Default, Clone)]
pub struct Context<'source> {
    bindings: HashMap<String, RefCell<Binding<'source>>>,
    parent: Option<Rc<Context<'source>>>,
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

    pub fn new_with_parent(parent: Rc<Context<'source>>) -> Self {
        Context {
            bindings: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn bind_with_expr<N>(&mut self, name: N, expr: Expression<'source>)
    where
        N: Into<String>,
    {
        self.bindings
            .insert(name.into(), RefCell::new(Binding::new(expr)));
    }

    pub fn bind_with_value<N>(&mut self, name: N, value: impl Into<JmlValue<'source>>)
    where
        N: Into<String>,
    {
        let value = value.into();
        self.bindings
            .entry(name.into())
            .and_modify(|e| *e.get_mut() = Binding::new_with_value(value.clone()))
            .or_insert(RefCell::new(Binding::new_with_value(value)));
    }

    pub fn lookup_variable<N>(&self, name: N) -> Result<Binding<'source>, RuntimeErrorKind>
    where
        N: AsRef<str>,
    {
        if let Some(binding) = self.bindings.get(name.as_ref()) {
            Ok(binding.borrow().clone())
        } else if let Some(parent) = &self.parent {
            // Recursively search in parent contexts
            parent.lookup_variable(name)
        } else {
            // Variable not found
            Err(RuntimeErrorKind::UndefinedVariable {
                name: name.as_ref().to_owned(),
            })
        }
    }
}
