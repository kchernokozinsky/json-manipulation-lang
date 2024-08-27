use std::{
    fmt::{self},
    ops,
};

use bool::JmlBool;
use float::JmlFloat;
use integer::JmlInt;
use list::JmlList;
use string::JmlString;

use crate::{error::TypeErrorKind, jml_type::JmlType};

pub mod bool;
pub mod float;
pub mod integer;
pub mod list;
pub mod string;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum JmlValue {
    #[default]
    Null,
    Bool(JmlBool),
    Float(JmlFloat),
    Int(JmlInt),
    List(JmlList),
    String(JmlString),
}

impl From<JmlBool> for JmlValue {
    fn from(value: JmlBool) -> Self {
        JmlValue::Bool(value)
    }
}

impl From<JmlFloat> for JmlValue {
    fn from(value: JmlFloat) -> Self {
        JmlValue::Float(value)
    }
}

impl From<JmlInt> for JmlValue {
    fn from(value: JmlInt) -> Self {
        JmlValue::Int(value)
    }
}

impl From<JmlList> for JmlValue {
    fn from(value: JmlList) -> Self {
        JmlValue::List(value)
    }
}

impl From<JmlString> for JmlValue {
    fn from(value: JmlString) -> Self {
        JmlValue::String(value)
    }
}

impl fmt::Display for JmlValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JmlValue::Bool(v) => v.fmt(f),
            JmlValue::Float(v) => v.fmt(f),
            JmlValue::Int(v) => v.fmt(f),
            JmlValue::List(v) => v.fmt(f),
            JmlValue::String(v) => v.fmt(f),
            JmlValue::Null => write!(f, "null"),
        }
    }
}

impl JmlValue {
    pub fn null() -> JmlValue {
        JmlValue::Null
    }

    pub fn float(value: impl Into<JmlFloat>) -> JmlValue {
        Self::Float(value.into())
    }

    pub fn int(value: impl Into<JmlInt>) -> JmlValue {
        Self::Int(value.into())
    }

    pub fn bool(value: impl Into<JmlBool>) -> JmlValue {
        Self::Bool(value.into())
    }

    pub fn string(value: impl Into<JmlString>) -> JmlValue {
        Self::String(value.into())
    }

    pub fn type_of(&self) -> JmlType {
        match self {
            Self::Null => JmlType::Null,
            Self::Int(_) => JmlType::Int,
            Self::Float(_) => JmlType::Float,
            Self::Bool(_) => JmlType::Bool,
            Self::List(_) => JmlType::List,
            Self::String(_) => JmlType::String,
            // Self::String(_) => JmlType::String,
            // Self::Function(_) => JmlType::Function,
            // Self::NativeFunction(_) => JmlType::Function,
        }
    }

    pub fn is_number(&self) -> bool {
        self.type_of().is_number()
    }

    pub fn is_comparable(&self) -> bool {
        self.type_of().is_comparable()
    }
}

impl ops::Add<JmlValue> for JmlValue {
    type Output = Result<JmlValue, TypeErrorKind>;

    fn add(self, rhs: JmlValue) -> Result<JmlValue, TypeErrorKind> {
        let lhs_type = self.type_of();
        let rhs_type = rhs.type_of();
        match (self, rhs) {
            (JmlValue::Float(a), JmlValue::Float(b)) => Ok((a + b).into()),
            (JmlValue::Float(a), JmlValue::Int(b)) => Ok((a + b).into()),
            (JmlValue::Int(a), JmlValue::Int(b)) => Ok((a + b).into()),
            (JmlValue::Int(a), JmlValue::Float(b)) => Ok((a + b).into()),
            _ => Err(TypeErrorKind::MismatchedTypes {
                expected: vec![JmlType::Float, JmlType::Int],
                found: if lhs_type.is_number() {
                    rhs_type
                } else {
                    lhs_type
                },
            }),
        }
    }
}
