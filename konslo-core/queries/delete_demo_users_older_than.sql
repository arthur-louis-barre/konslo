DELETE
FROM users u USING demo_users d
WHERE u.user_id = d.user_id AND u.created_at < $1;