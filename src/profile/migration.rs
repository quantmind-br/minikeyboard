//! Schema version dispatcher. Only v1 accepted until a second schema exists.

use super::ProfileError;
use crate::domain::config::{DeviceConfig, SCHEMA_VERSION};

pub fn migrate_schema(value: serde_json::Value) -> Result<DeviceConfig, ProfileError> {
    let version = value
        .get("schema_version")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| ProfileError::Invalid("missing schema_version".into()))?;

    if version != SCHEMA_VERSION as u64 {
        return Err(ProfileError::UnsupportedSchema(version as u32));
    }

    let config: DeviceConfig = serde_json::from_value(value)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_schema() {
        let v = serde_json::json!({"schema_version": 2});
        assert!(matches!(
            migrate_schema(v),
            Err(ProfileError::UnsupportedSchema(2))
        ));
    }
}
