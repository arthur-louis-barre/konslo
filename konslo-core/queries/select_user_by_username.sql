SELECT
    user_id as id,
    username,
    password_hash,
    created_at
FROM users
WHERE username = $1