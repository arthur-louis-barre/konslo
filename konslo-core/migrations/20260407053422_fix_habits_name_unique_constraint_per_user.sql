ALTER TABLE habits DROP CONSTRAINT habits_name_key;
ALTER TABLE habits ADD CONSTRAINT habits_user_id_name_key UNIQUE(user_id, name);
