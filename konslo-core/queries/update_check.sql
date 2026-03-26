UPDATE checks SET value = $1
WHERE check_id = $2
RETURNING check_id as id, habit_id, value, checked_at;
