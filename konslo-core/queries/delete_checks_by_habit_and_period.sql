DELETE
FROM checks
WHERE habit_id = $1
  AND checked_at::date >= $2::date
  AND checked_at::date <= $3::date;