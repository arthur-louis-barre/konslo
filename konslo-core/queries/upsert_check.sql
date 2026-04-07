INSERT INTO checks (habit_id, value, checked_at)
VALUES ($1, $2, $3)
ON CONFLICT (habit_id, ((checked_at AT TIME ZONE 'UTC')::date))
DO UPDATE SET value = checks.value + EXCLUDED.value
RETURNING
    check_id as id,
    habit_id, value,
    checked_at;