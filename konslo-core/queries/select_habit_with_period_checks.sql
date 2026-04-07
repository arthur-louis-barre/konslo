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
    h.goal_period = 'day' AND date_trunc('day', c.checked_at) = date_trunc('day', $3::timestamptz) OR
    h.goal_period = 'week' AND date_trunc('week', c.checked_at) = date_trunc('week', $3::timestamptz) OR
    h.goal_period = 'month' AND date_trunc('month', c.checked_at) = date_trunc('month', $3::timestamptz)
)
WHERE h.habit_id = $1 AND h.user_id = $2;
