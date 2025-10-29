use serde::Serialize;
use serde_json::Value;

/// Converts a record into JSON and appends a display label.
/// If the provided label is None or empty, a fallback is generated using the record ID (if present).
pub fn add_display_label<T>(record: &T, display_label: Option<impl Into<String>>) -> Value
where
    T: Serialize,
{
    // Extract optional label
    let label = display_label
        .map(|l| l.into())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| {
            // Try to read `id` field from record
            let value = serde_json::to_value(record).unwrap();
            let fallback = value
                .get("id")
                .and_then(|v| v.as_i64())
                .map(|id| format!("Record ID: {}", id))
                .unwrap_or_else(|| "Unnamed record".to_string());
            fallback
        });

    let mut data = serde_json::to_value(record).unwrap();

    if let Value::Object(ref mut obj) = data {
        obj.insert("displayLabel".to_string(), Value::String(label));
    }

    data
}
