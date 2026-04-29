CREATE TABLE demo_users(
    user_id UUID PRIMARY KEY REFERENCES users(user_id) ON DELETE CASCADE
);
