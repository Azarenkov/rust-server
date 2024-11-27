pub enum DbErrors {
    NotFound(),
    DbError(mongodb::error::Error)
}