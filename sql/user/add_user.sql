INSERT INTO main.users(nick, pwd, _salt)
VALUES ($1, $2, $3)
RETURNING id, nick;