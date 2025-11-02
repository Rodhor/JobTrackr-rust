use chrono::NaiveDate;

/// Parse a required date string (YYYY-MM-DD).
pub fn parse_required_date(s: String) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|_| {
        serde_json::json!({
            "status": "error",
            "message": "Invalid date format. Expected YYYY-MM-DD."
        })
        .to_string()
    })
}

/// Parse an optional date string (YYYY-MM-DD).
pub fn parse_optional_date(s: Option<String>) -> Result<Option<NaiveDate>, String> {
    match s {
        Some(v) => Ok(Some(parse_required_date(v)?)),
        None => Ok(None),
    }
}
