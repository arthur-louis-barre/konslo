DELETE
FROM demo_users d USING users u
WHERE d.user_id = u.user_id AND u.created_at < $1;