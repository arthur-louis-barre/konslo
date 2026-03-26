INSERT INTO habits (name, goal_value, goal_unit, goal_period)
VALUES ($1, $2, $3, $4)
RETURNING
    habit_id as id,
    name,
    goal_value,
    goal_unit,
    goal_period as "goal_period: GoalPeriod",
    created_at;