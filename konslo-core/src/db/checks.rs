use chrono::NaiveDate;
use sqlx::PgPool;

pub async fn check_habit(pool: &PgPool, habit_id: i64, date: NaiveDate) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO checks (habit_id, checked_date) VALUES ($1, $2)")
        .bind(habit_id)
        .bind(date)
        .execute(pool)
        .await?;

    Ok(())
}



