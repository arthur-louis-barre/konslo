INSERT INTO habits (user_id, name, goal_value, goal_unit, goal_period)
VALUES ($1, $2, $3, $4, $5)
RETURNING
    habit_id as id,
    user_id,
    name,
    goal_value,
    goal_unit,
    goal_period as "goal_period: GoalPeriod",
    created_at;