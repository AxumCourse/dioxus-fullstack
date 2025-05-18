use crate::CFG;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::OnceCell;

static DB: OnceCell<PgPool> = OnceCell::const_new();

async fn init_db() -> PgPool {
    PgPoolOptions::new()
        .max_connections(CFG.database_max_conns)
        .connect(&CFG.database_url)
        .await
        .unwrap()
}

pub async fn get_db() -> &'static PgPool {
    DB.get_or_init(init_db).await
}
