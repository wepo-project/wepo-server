INSERT INTO wepo.users(nick, pwd, _salt)
VALUES ($1, $2, $3)
RETURNING $table_fields;