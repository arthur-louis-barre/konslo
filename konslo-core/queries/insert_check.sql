INSERT INTO checks(habit_id, value, checked_at)
VALUES ($1, $2, $3)
RETURNING check_id as id, habit_id, value, checked_at;
