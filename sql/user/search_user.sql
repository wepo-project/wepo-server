SELECT id, nick, avatar_url
FROM main.users
WHERE nick LIKE $1
LIMIT $2 OFFSET $3;