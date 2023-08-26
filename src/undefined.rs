use serde::ser::Error;
use serde::Serialize;
use std::ops::Deref;

/// Represents the potential absence of a value beyond `None`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UndefinedOr<T> {
    /// The value is present (T).
    Value(T),

    /// The value is present (null).
    Null,

    /// The value is not present (undefined).
    Undefined,
}

impl<T> UndefinedOr<T> {
    pub fn is_some(&self) -> bool {
        match self {
            Self::Value(_) => true,
            _ => false,
        }
    }

    pub fn is_undefined(&self) -> bool {
        match self {
            Self::Undefined => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
}

impl<T> Default for UndefinedOr<T> {
    fn default() -> Self {
        Self::Undefined
    }
}

impl<T: Serialize> Serialize for UndefinedOr<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Value(v) => v.serialize(serializer),
            Self::Undefined => Err(Error::custom("Undefined should never be serialized.")),
        }
    }
}

impl<T> From<Option<T>> for UndefinedOr<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => Self::Value(v),
            None => Self::Null,
        }
    }
}

impl<T> Deref for UndefinedOr<Option<T>> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Value(v) => &v,
            Self::Null => &None::<T>,
            Self::Undefined => &None::<T>,
        }
    }
}
