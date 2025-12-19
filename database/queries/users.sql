--! insert_user (username)
INSERT INTO users (username)
VALUES (:username)
RETURNING id, username;

--! get_all_users
SELECT id, username FROM users;