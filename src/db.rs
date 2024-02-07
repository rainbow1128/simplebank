use crate::prelude::*;
use futures::future::BoxFuture;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub mod account_sql;
pub mod entry_sql;
pub mod store;
pub mod transfer_sql;

async fn create_connection_pool(max_conn: Option<u32>) -> Result<PgPool> {
    let max_conn = max_conn.unwrap_or(5);
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

mod tests {
    use super::*;
    use crate::{
        db::{
            create_connection_pool,
            entry_sql::{create_entry, CreateEntryParams},
        },
        util::*,
    };
}
