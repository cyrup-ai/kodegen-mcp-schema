//! Serde helper functions for flexible deserialization patterns
//!
//! This module provides zero-allocation deserializers that accept multiple
//! input formats for better user experience.

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::Deserialize;

/// Zero-allocation deserializer: string ∪ array ∪ null → Vec<T>
///
/// Accepts multiple JSON input formats and normalizes to Vec<T>:
/// • String: `"value"` → `vec![T::from_str("value")]`
/// • Array: `[item1, item2]` → `vec![item1, item2]`
/// • Null/Unit: `null` → `vec![]`
///
/// Performance characteristics:
/// • Static dispatch via visitor pattern
/// • Zero allocation visitor (zero-sized type)
/// • Inlined for maximum speed
///
/// # Examples
///
/// Use with serde's `deserialize_with` attribute:
///
/// ```rust
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config {
///     #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
///     values: Vec<String>,
/// }
///
/// // All these formats work:
/// // JSON: {"values": "single"}        → vec!["single"]
/// // JSON: {"values": ["a", "b"]}      → vec!["a", "b"]
/// // JSON: {"values": null}            → vec![]
/// // JSON: {}                          → vec![] (via #[serde(default)])
/// ```
#[inline]
pub fn string_or_vec<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: Deserialize<'de> + FromStr,
    T::Err: fmt::Display,
    D: Deserializer<'de>,
{
    /// Zero-sized visitor for maximum performance
    struct VisitorImpl<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for VisitorImpl<T>
    where
        T: Deserialize<'de> + FromStr,
        T::Err: fmt::Display,
    {
        type Value = Vec<T>;

        #[inline]
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("string, sequence, null, or unit")
        }

        /// Handle string input: parse single value into Vec
        #[inline]
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Single allocation for Vec with capacity 1
            Ok(vec![v.parse().map_err(E::custom)?])
        }

        /// Handle array input: deserialize sequence directly
        #[inline]
        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            // Direct deserialization - serde handles allocation efficiently
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }

        /// Handle null/unit input: return empty Vec
        #[inline]
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Zero allocation: empty Vec
            Ok(Vec::new())
        }

        /// Handle null input: return empty Vec
        #[inline]
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Zero allocation: empty Vec
            Ok(Vec::new())
        }
    }

    // Zero-sized visitor construction
    deserializer.deserialize_any(VisitorImpl(PhantomData))
}
