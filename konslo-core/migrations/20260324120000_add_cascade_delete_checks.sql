-- add ON DELETE CASCADE to checks.habit_id foreign key
ALTER TABLE checks DROP CONSTRAINT checks_habit_id_fkey;
ALTER TABLE checks ADD CONSTRAINT checks_habit_id_fkey
    FOREIGN KEY (habit_id) REFERENCES habits(habit_id) ON DELETE CASCADE;
