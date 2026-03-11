-- create an enum
CREATE TYPE period_enum AS ENUM ('day', 'week', 'month');

-- add columns to habits table
ALTER TABLE habits
ADD COLUMN goal_value INT,
ADD COLUMN goal_unit VARCHAR(20),
ADD COLUMN goal_period period_enum;

-- add constraints to habits table
ALTER TABLE habits
ADD CONSTRAINT habits_goal_value_positive CHECK(goal_value > 0);

-- create checks table
CREATE TABLE checks (
    check_id SERIAL PRIMARY KEY,
    habit_id INT REFERENCES habits(id),
    value INT,
    checked_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- add constraints to checks
ALTER TABLE checks
ADD CONSTRAINT checks_value_positive CHECK(value > 0);