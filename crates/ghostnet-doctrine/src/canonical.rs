//! Restricted RFC 8785 canonical JSON encoding.

use serde_json::Value;
use sha2::{Digest, Sha256};
use thiserror::Error;

const MAX_INTEROPERABLE_INTEGER: u64 = 9_007_199_254_740_991;
const SIGNATURE_DOMAIN: &[u8] = b"ghostnet-artifact-v1\0";

/// A failure to encode a value in `GhostNet`'s restricted canonical profile.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CanonicalError {
    #[error("floating-point values are not permitted")]
    Float,
    #[error("null values are not permitted")]
    Null,
    #[error("integer is outside the interoperable signed 53-bit range")]
    IntegerRange,
    #[error("schema identifier must not be empty or contain NUL")]
    InvalidSchema,
}

/// Encodes JSON using `GhostNet`'s restricted RFC 8785 profile.
///
/// # Errors
///
/// Returns [`CanonicalError`] for floats, nulls, or integers outside the
/// interoperable signed 53-bit range.
///
/// Object names are ordered lexicographically by UTF-16 code units, as required
/// by JCS. Strings use `serde_json`'s compact JSON escaping. Floats, nulls, and
/// integers outside the interoperable signed 53-bit range are rejected.
pub fn canonicalize(value: &Value) -> Result<Vec<u8>, CanonicalError> {
    let mut output = Vec::new();
    encode(value, &mut output)?;
    Ok(output)
}

/// Returns a contract-safe SHA-256 reference for canonical body bytes.
#[must_use]
pub fn body_hash(canonical_body: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(canonical_body))
}

/// Constructs the bytes submitted to detached signing.
///
/// # Errors
///
/// Returns [`CanonicalError::InvalidSchema`] when the schema is empty or
/// contains a NUL byte, which would make domain separation ambiguous.
pub fn signature_input(schema: &str, canonical_body: &[u8]) -> Result<Vec<u8>, CanonicalError> {
    if schema.is_empty() || schema.as_bytes().contains(&0) {
        return Err(CanonicalError::InvalidSchema);
    }

    let mut output =
        Vec::with_capacity(SIGNATURE_DOMAIN.len() + schema.len() + 1 + canonical_body.len());
    output.extend_from_slice(SIGNATURE_DOMAIN);
    output.extend_from_slice(schema.as_bytes());
    output.push(0);
    output.extend_from_slice(canonical_body);
    Ok(output)
}

fn encode(value: &Value, output: &mut Vec<u8>) -> Result<(), CanonicalError> {
    match value {
        Value::Null => Err(CanonicalError::Null),
        Value::Bool(value) => {
            output.extend_from_slice(if *value { b"true" } else { b"false" });
            Ok(())
        }
        Value::Number(value) => encode_number(value, output),
        Value::String(value) => {
            output.extend_from_slice(
                serde_json::to_string(value)
                    .expect("serializing a JSON string cannot fail")
                    .as_bytes(),
            );
            Ok(())
        }
        Value::Array(values) => {
            output.push(b'[');
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    output.push(b',');
                }
                encode(value, output)?;
            }
            output.push(b']');
            Ok(())
        }
        Value::Object(values) => {
            let mut entries: Vec<_> = values.iter().collect();
            entries.sort_by(|(left, _), (right, _)| utf16_cmp(left, right));
            output.push(b'{');
            for (index, (name, value)) in entries.into_iter().enumerate() {
                if index != 0 {
                    output.push(b',');
                }
                output.extend_from_slice(
                    serde_json::to_string(name)
                        .expect("serializing a JSON object name cannot fail")
                        .as_bytes(),
                );
                output.push(b':');
                encode(value, output)?;
            }
            output.push(b'}');
            Ok(())
        }
    }
}

fn encode_number(value: &serde_json::Number, output: &mut Vec<u8>) -> Result<(), CanonicalError> {
    if let Some(value) = value.as_i64() {
        if value.unsigned_abs() > MAX_INTEROPERABLE_INTEGER {
            return Err(CanonicalError::IntegerRange);
        }
        output.extend_from_slice(value.to_string().as_bytes());
        return Ok(());
    }
    if let Some(value) = value.as_u64() {
        if value > MAX_INTEROPERABLE_INTEGER {
            return Err(CanonicalError::IntegerRange);
        }
        output.extend_from_slice(value.to_string().as_bytes());
        return Ok(());
    }
    Err(CanonicalError::Float)
}

fn utf16_cmp(left: &str, right: &str) -> std::cmp::Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}
