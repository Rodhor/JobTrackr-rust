use chrono::Utc;

/// Builds a dynamic UPDATE SQL statement and parameter list for partial updates.
///
/// Returns:
/// `(sql, Vec<String>)` where `sql` is the full UPDATE query,
/// and the Vec contains bound values in correct order.
///
/// Example:
/// ```
/// use jobtrackr_lib::utils::sql_utils::build_update_sql;
///
/// let fields = vec![("first_name", Some("Chris")), ("email", None)];
/// let (sql, binds) = build_update_sql("user", "id", 42, fields);
/// ```
pub fn build_update_sql(
    table: &str,
    id_field: &str,
    id: i64,
    fields: Vec<(&str, Option<&str>)>,
) -> (String, Vec<String>) {
    let mut set_clauses = Vec::new();
    let mut binds = Vec::new();

    for (col, val) in fields {
        if let Some(v) = val {
            set_clauses.push(format!("{} = ?", col));
            binds.push(v.to_string());
        }
    }

    // Always update timestamp
    set_clauses.push("updated_at = ?".to_string());
    let now = Utc::now().to_rfc3339();
    binds.push(now);

    // Final SQL
    let sql = format!(
        "UPDATE {} SET {} WHERE {} = ? RETURNING *",
        table,
        set_clauses.join(", "),
        id_field
    );

    // Add ID as last parameter
    binds.push(id.to_string());

    (sql, binds)
}
