#![allow(clippy::module_name_repetitions)]

use serde::ser::Error;
use serde::Serialize;

/// Represents the potential absence of a value beyond `None`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UndefinedOr<T> {
    /// The value is present (T).
    Value(T),

    /// The value is not present (null).
    Null,

    /// The value is not present (undefined).
    Undefined,
}

impl<T> UndefinedOr<T> {
    /// True if this variant contains a T value.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UndefinedOr;
    /// let val = UndefinedOr::Value(0);
    ///
    /// assert!(val.is_some());
    /// ````
    pub fn is_some(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    /// True if this variant contains no value.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UndefinedOr;
    /// let val = UndefinedOr::<u8>::Undefined;
    ///
    /// assert!(val.is_undefined());
    /// ````
    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }

    /// True if this variant contains a null value.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UndefinedOr;
    /// let val = UndefinedOr::<u8>::Null;
    ///
    /// assert!(val.is_null());
    /// ````
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Gets an optional reference to the inner value.
    ///
    /// # Example
    /// ```
    /// # use unkey::models::UndefinedOr;
    /// let val = UndefinedOr::Value(420);
    ///
    /// assert_eq!(val.inner(), Some(&420));
    ///
    /// let val = UndefinedOr::<u8>::Null;
    ///
    /// assert_eq!(val.inner(), None);
    ///
    /// let val = UndefinedOr::<u8>::Undefined;
    ///
    /// assert_eq!(val.inner(), None);
    /// ```
    pub fn inner(&self) -> Option<&T> {
        match self {
            Self::Value(v) => Some(v),
            _ => None,
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

#[cfg(test)]
mod test {
    use serde::Serialize;

    use crate::models::UndefinedOr;

    #[derive(Serialize)]
    struct TestStruct {
        #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
        a: UndefinedOr<u32>,
        #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
        b: UndefinedOr<u32>,
        #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
        c: UndefinedOr<u32>,
        #[serde(skip_serializing_if = "UndefinedOr::is_undefined")]
        d: UndefinedOr<u32>,
    }

    #[test]
    fn default() {
        let d: UndefinedOr<u32> = Default::default();
        assert_eq!(d, UndefinedOr::Undefined);
    }

    #[test]
    fn serialize_null() {
        let t = TestStruct {
            a: UndefinedOr::Null,
            b: UndefinedOr::Null,
            c: UndefinedOr::Null,
            d: UndefinedOr::Null,
        };

        let res = serde_json::to_string(&t).unwrap();
        assert_eq!(res.as_str(), r#"{"a":null,"b":null,"c":null,"d":null}"#)
    }

    #[test]
    fn serialize_undefined() {
        let t = TestStruct {
            a: UndefinedOr::Undefined,
            b: UndefinedOr::Undefined,
            c: UndefinedOr::Undefined,
            d: UndefinedOr::Undefined,
        };

        let res = serde_json::to_string(&t).unwrap();
        assert_eq!(res.as_str(), "{}");
    }

    #[test]
    fn serialize_value() {
        let t = TestStruct {
            a: UndefinedOr::Value(69),
            b: UndefinedOr::Value(420),
            c: UndefinedOr::Value(42),
            d: UndefinedOr::Value(0),
        };

        let res = serde_json::to_string(&t).unwrap();
        assert_eq!(res.as_str(), r#"{"a":69,"b":420,"c":42,"d":0}"#)
    }

    #[test]
    fn serialize_mixed() {
        let t = TestStruct {
            a: UndefinedOr::Value(69),
            b: UndefinedOr::Value(420),
            c: UndefinedOr::Null,
            d: UndefinedOr::Undefined,
        };

        let res = serde_json::to_string(&t).unwrap();
        assert_eq!(res.as_str(), r#"{"a":69,"b":420,"c":null}"#)
    }

    #[test]
    fn from_some() {
        let o = Some(69);
        let res = UndefinedOr::from(o);
        assert_eq!(res, UndefinedOr::Value(69));
    }

    #[test]
    fn from_none() {
        let o = None::<u8>;
        let res = UndefinedOr::from(o);
        assert_eq!(res, UndefinedOr::Null);
    }
}
