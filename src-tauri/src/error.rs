// `tokio_postgres::Error`'s `Display` just writes "db error" for any DB-kind
// error (e.g. auth or query failures) - the real message lives on the
// wrapped `DbError`, reachable only via `as_db_error()`/`source()`.
pub fn describe_postgres_error(error: &tokio_postgres::Error) -> String {
    match error.as_db_error() {
        Some(db_error) => db_error.to_string(),
        None => error.to_string(),
    }
}
