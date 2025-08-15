use sqlx::PgPool;

pub async fn init_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}
