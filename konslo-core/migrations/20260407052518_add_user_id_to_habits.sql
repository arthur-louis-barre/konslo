ALTER TABLE habits ADD COLUMN user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE;
ALTER TABLE habits ADD CONSTRAINT habits_user_id_name_unique UNIQUE (user_id, name);