DELETE
FROM habits
WHERE habit_id = $1 AND user_id = $2;