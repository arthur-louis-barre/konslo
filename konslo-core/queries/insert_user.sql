INSERT INTO users (username, password_hash)
VALUES ($1, $2)
RETURNING
    user_id as id,
    username,
    password_hash,
    created_at;
