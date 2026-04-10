CREATE TYPE goal_period_enum AS ENUM ('day', 'week', 'month');

ALTER TABLE habits ADD COLUMN goal_value INT NOT NULL;
ALTER TABLE habits ADD COLUMN goal_unit TEXT NOT NULL;
ALTER TABLE habits ADD COLUMN goal_period goal_period_enum NOT NULL;

ALTER TABLE habits
ADD CONSTRAINT habits_goal_value_positive CHECK (goal_value > 0);

CREATE TABLE checks (
    check_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    habit_id UUID NOT NULL REFERENCES habits(habit_id) ON DELETE CASCADE,
    value INT NOT NULL,
    checked_at TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT checks_value_positive CHECK (value > 0),
    CONSTRAINT checks_checked_at_not_future CHECK (checked_at <= CURRENT_TIMESTAMP + INTERVAL '30 seconds')
);

CREATE UNIQUE INDEX uq_idx_checks_habit_date
ON checks (habit_id, ((checked_at AT TIME ZONE 'UTC')::date));