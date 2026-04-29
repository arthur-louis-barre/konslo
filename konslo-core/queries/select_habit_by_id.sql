SELECT
    habit_id as id,
    user_id,
    name,
    goal_value,
    goal_unit,
    goal_period as "goal_period: GoalPeriod",
    created_at
FROM habits
WHERE habit_id = $1 AND user_id = $2