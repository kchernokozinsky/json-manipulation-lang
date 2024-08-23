use std::{cell::RefCell, collections::HashMap};

use parser::ast::Expression;

use crate::{error::EvalError, expr::eval_expr, value::JmlValue};

pub struct Context<'a> {
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

    fn new_with_value(value: JmlValue) -> Self {
        Binding::Value(value)
    }

    fn eval(&mut self, ctx: &Context) -> Result<JmlValue, EvalError> {
        match self {
            Binding::Expression(expr) => {
                let value = eval_expr(expr.clone(), ctx)?;
                *self = Binding::new_with_value(value.clone());
                Ok(value)
            } // expression evaluation should be done here
            Binding::Value(val) => Ok(val.clone()),
        }
    }
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            bindings: HashMap::new(),
        }
    }

    pub fn bind(&mut self, name: String, expr: Expression<'a>) {
        self.bindings.insert(name, RefCell::new(Binding::new(expr)));
    }

    pub fn get(&self, name: impl AsRef<str>) -> Result<JmlValue, EvalError> {
        let mut binding = self
            .bindings
            .get(name.as_ref())
            .ok_or(EvalError::UndefinedVariable {
                name: name.as_ref().to_owned(),
            })?
            .borrow_mut();

        binding.eval(self)
    }
}
