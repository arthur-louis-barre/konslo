-- create an enum
CREATE TYPE goal_period_enum AS ENUM ('day', 'week', 'month');

-- add columns to habits table
ALTER TABLE habits ADD COLUMN goal_value INT NOT NULL;
ALTER TABLE habits ADD COLUMN goal_unit VARCHAR(20) NOT NULL;
ALTER TABLE habits ADD COLUMN goal_period goal_period_enum NOT NULL;

-- add constraints to habits table
ALTER TABLE habits ADD CONSTRAINT habits_goal_value_positive CHECK(goal_value > 0);
ALTER TABLE habits ALTER COLUMN created_at SET NOT NULL;

-- create checks table
CREATE TABLE checks (
    check_id SERIAL PRIMARY KEY,
    habit_id INT REFERENCES habits(id) NOT NULL,
    value INT NOT NULL,
    checked_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- add constraints to checks
ALTER TABLE checks ADD CONSTRAINT checks_value_positive CHECK(value > 0);