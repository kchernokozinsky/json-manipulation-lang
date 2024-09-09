use std::{
    collections::HashMap,
    fmt::{self},
};

use bool::JmlBool;
use float::JmlFloat;
use indexmap::IndexMap;
use integer::JmlInt;
use lambda::JmlLambda;
use list::JmlList;
use object::JmlObject;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::Value;
use string::JmlString;

use crate::{errors::TypeErrorKind, jml_type::JmlType};

use derive_more::{derive::Display, From};

pub mod bool;
pub mod float;
pub mod integer;
pub mod lambda;
pub mod list;
pub mod object;
pub mod string;

#[derive(Default, Debug, Clone, PartialEq, Eq, Display, From)]
pub enum JmlValue<'source> {
    #[default]
    #[display("null")]
    Null,
    #[from]
    Bool(JmlBool),
    #[from]
    Float(JmlFloat),
    #[from]
    Int(JmlInt),
    #[from]
    List(JmlList<'source>),
    #[from]
    String(JmlString),
    #[from]
    Object(JmlObject<'source>),
    #[from]
    Lambda(JmlLambda<'source>),
}

impl From<Value> for JmlValue<'_> {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => JmlValue::Null,
            Value::Bool(b) => JmlValue::bool(b),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    JmlValue::Int(JmlInt(i))
                } else if let Some(f) = n.as_f64() {
                    JmlValue::Float(JmlFloat(f))
                } else {
                    JmlValue::Null
                }
            }

            Value::String(s) => JmlValue::string(s),

            Value::Array(arr) => {
                let list = arr.into_iter().map(JmlValue::from).collect();
                JmlValue::List(JmlList(list))
            }

            Value::Object(obj) => {
                let object = obj
                    .into_iter()
                    .map(|(k, v)| (k, JmlValue::from(v)))
                    .collect();
                JmlValue::Object(JmlObject(object))
            }
        }
    }
}

impl<'source> TryFrom<JmlValue<'source>> for i64 {
    type Error = TypeErrorKind;

    fn try_from(value: JmlValue) -> Result<Self, Self::Error> {
        match value {
            JmlValue::Int(v) => Ok(v.0),
            _ => Err(TypeErrorKind::MismatchedTypes {
                expected: vec![JmlType::Int],
                found: value.type_of(),
            }),
        }
    }
}

impl<'source> JmlValue<'source> {
    pub fn null() -> JmlValue<'source> {
        JmlValue::Null
    }

    pub fn float(value: impl Into<JmlFloat>) -> JmlValue<'source> {
        Self::Float(value.into())
    }

    pub fn object(value: impl Into<JmlObject<'source>>) -> JmlValue<'source> {
        Self::Object(value.into())
    }

    pub fn list(value: impl Into<JmlList<'source>>) -> JmlValue<'source> {
        Self::List(value.into())
    }

    pub fn int(value: impl Into<JmlInt>) -> JmlValue<'source> {
        Self::Int(value.into())
    }

    pub fn bool(value: impl Into<JmlBool>) -> JmlValue<'source> {
        Self::Bool(value.into())
    }

    pub fn string(value: impl Into<JmlString>) -> JmlValue<'source> {
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
            Self::Object(_) => JmlType::Object,
            Self::Lambda(JmlLambda { params, .. }) => JmlType::Lambda {
                arity: params.len(),
            },
        }
    }

    pub fn is_number(&self) -> bool {
        self.type_of().is_number()
    }

    pub fn is_bool(&self) -> bool {
        self.type_of().is_bool()
    }

    pub fn is_comparable(&self) -> bool {
        self.type_of().is_comparable()
    }

    pub fn is_ord(&self) -> bool {
        self.type_of().is_ord()
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            JmlValue::Bool(val) => val.is_truthy(),
            _ => false,
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            JmlValue::Float(v) => v.0 == 0.0,
            JmlValue::Int(v) => v.0 == 0,
            _ => false,
        }
    }
}

impl<'a> Serialize for JmlValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            JmlValue::Null => serializer.serialize_none(),
            JmlValue::Bool(JmlBool(b)) => serializer.serialize_bool(*b),
            JmlValue::Float(JmlFloat(f)) => serializer.serialize_f64(*f),
            JmlValue::Int(JmlInt(i)) => serializer.serialize_i64(*i),
            JmlValue::String(JmlString(s)) => serializer.serialize_str(s),
            JmlValue::List(JmlList(list)) => list.serialize(serializer),
            JmlValue::Object(JmlObject(map)) => map.serialize(serializer),
            JmlValue::Lambda(_) => todo!(),
        }
    }
}

impl<'de> Deserialize<'de> for JmlValue<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JmlValueVisitor;

        impl<'de> Visitor<'de> for JmlValueVisitor {
            type Value = JmlValue<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid JSON value")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(JmlValue::Bool(JmlBool(v)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(JmlValue::Int(JmlInt(v)))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
                Ok(JmlValue::Float(JmlFloat(v)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JmlValue::String(JmlString(v.to_owned())))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JmlValue::String(JmlString(v)))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JmlValue::Null)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(JmlValue::Null)
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let values = Vec::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
                Ok(JmlValue::List(JmlList(values)))
            }

            fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let map = IndexMap::deserialize(de::value::MapAccessDeserializer::new(map))?;

                Ok(JmlValue::Object(JmlObject(map)))
            }
        }

        deserializer.deserialize_any(JmlValueVisitor)
    }
}
