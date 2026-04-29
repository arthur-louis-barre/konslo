SELECT DISTINCT checked_at::date AS date
FROM checks c JOIN habits h ON c.habit_id = h.habit_id
WHERE h.user_id = $1
  AND c.checked_at::date >= $2
  AND c.checked_at::date <= $3
ORDER BY date