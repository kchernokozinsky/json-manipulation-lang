use std::{
    collections::HashMap,
    fmt::{self},
};

use bool::JmlBool;
use float::JmlFloat;
use integer::JmlInt;
use list::JmlList;
use object::JmlObject;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use string::JmlString;

use crate::{error::TypeErrorKind, jml_type::JmlType};

pub mod bool;
pub mod float;
pub mod integer;
pub mod list;
pub mod object;
pub mod string;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum JmlValue {
    #[default]
    Null,
    Bool(JmlBool),
    Float(JmlFloat),
    Int(JmlInt),
    List(JmlList),
    String(JmlString),
    Object(JmlObject),
}

impl From<JmlBool> for JmlValue {
    fn from(value: JmlBool) -> Self {
        JmlValue::Bool(value)
    }
}

impl TryFrom<JmlValue> for i64 {
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
            JmlValue::Object(v) => v.fmt(f),
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

    pub fn object(value: impl Into<JmlObject>) -> JmlValue {
        Self::Object(value.into())
    }

    pub fn list(value: impl Into<JmlList>) -> JmlValue {
        Self::List(value.into())
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
            Self::Object(_) => JmlType::Object,
            // Self::Function(_) => JmlType::Function,
            // Self::NativeFunction(_) => JmlType::Function,
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

    pub fn is_truthy(&self) -> bool {
        match self {
            JmlValue::Bool(val) => val.is_truthy(),
            _ => false,
        }
    }
}

impl Serialize for JmlValue {
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
        }
    }
}

impl<'de> Deserialize<'de> for JmlValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JmlValueVisitor;

        impl<'de> Visitor<'de> for JmlValueVisitor {
            type Value = JmlValue;

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
                let map = HashMap::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(JmlValue::Object(JmlObject(map)))
            }
        }

        deserializer.deserialize_any(JmlValueVisitor)
    }
}
