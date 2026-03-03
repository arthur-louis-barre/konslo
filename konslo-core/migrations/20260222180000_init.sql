-- habits table
CREATE TABLE habits (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- checks table
CREATE TABLE checks (
    id SERIAL PRIMARY KEY,
    habit_id INTEGER NOT NULL REFERENCES habits(id) ON DELETE CASCADE,
    checked_date DATE NOT NULL DEFAULT CURRENT_DATE,
    UNIQUE(habit_id, checked_date)
);