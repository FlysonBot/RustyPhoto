//! Reading command parameters safely. Every getter validates its input and
//! returns an error instead of panicking, so hostile parameters can't crash.

use crate::cmd::EngineError;
use serde_json::Value;

/// A command's JSON parameters (an object, or anything else treated as empty).
pub struct Params<'a> {
    value: &'a Value,
}

impl<'a> Params<'a> {
    pub fn new(value: &'a Value) -> Self {
        Params { value }
    }

    fn get(&self, key: &str) -> Option<&'a Value> {
        self.value.as_object().and_then(|o| o.get(key))
    }

    /// A layer index: the `layer` field if present, otherwise `default`.
    pub fn layer(&self, default: usize) -> Result<usize, EngineError> {
        match self.get("layer") {
            None | Some(Value::Null) => Ok(default),
            Some(v) => v
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .ok_or_else(|| EngineError::BadParam {
                    name: "layer".into(),
                    reason: format!("expected a layer index, got {v}"),
                }),
        }
    }

    /// An optional string field.
    pub fn string(&self, key: &str) -> Result<Option<&'a str>, EngineError> {
        match self.get(key) {
            None | Some(Value::Null) => Ok(None),
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(v) => Err(EngineError::BadParam {
                name: key.into(),
                reason: format!("expected a string, got {v}"),
            }),
        }
    }

    /// A number field clamped to `min..=max`, or `default` when absent.
    /// NaN and infinities are rejected.
    pub fn number(&self, key: &str, default: f64, min: f64, max: f64) -> Result<f64, EngineError> {
        match self.get(key) {
            None | Some(Value::Null) => Ok(default),
            Some(v) => match v.as_f64() {
                Some(n) if n.is_finite() => Ok(n.clamp(min, max)),
                _ => Err(EngineError::BadParam {
                    name: key.into(),
                    reason: format!("expected a finite number, got {v}"),
                }),
            },
        }
    }
}
