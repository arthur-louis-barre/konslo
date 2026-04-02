SELECT DISTINCT checked_at::date AS date
FROM checks
WHERE checked_at::date >= $1 AND checked_at::date <= $2
ORDER BY date;