SELECT check_id as id, habit_id, value, checked_at
FROM checks
WHERE habit_id = $1
  AND checked_at::date = $2::date;