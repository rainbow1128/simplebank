pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;

pub type SQLResult<T> = std::result::Result<T, sqlx::Error>;
