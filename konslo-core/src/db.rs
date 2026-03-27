use sqlx::{Error, PgPool, migrate};

pub async fn run_migrations(pool: &PgPool) -> Result<(), Error> {
    migrate!().run(pool).await?;
    Ok(())
}
