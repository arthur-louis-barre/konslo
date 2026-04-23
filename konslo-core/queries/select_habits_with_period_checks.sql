SELECT
    h.habit_id as id,
    h.name,
    h.goal_value,
    h.goal_unit,
    h.goal_period as "goal_period: GoalPeriod",
    h.created_at,
    c.check_id as "check_id?",
    c.value as "value?",
    c.checked_at as "checked_at?"
FROM habits h LEFT JOIN checks c ON h.habit_id = c.habit_id AND (
    h.goal_period = 'day' AND date_trunc('day', c.checked_at) = date_trunc('day', $2::timestamptz) OR
    h.goal_period = 'week' AND date_trunc('week', c.checked_at) = date_trunc('week', $2::timestamptz) OR
    h.goal_period = 'month' AND date_trunc('month', c.checked_at) = date_trunc('month', $2::timestamptz)
)
WHERE h.user_id = $1
ORDER BY h.created_at;