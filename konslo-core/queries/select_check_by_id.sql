SELECT check_id as id, habit_id, value, checked_at
FROM checks
WHERE check_id = $1
