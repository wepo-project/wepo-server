INSERT INTO main.posts(id, sender, content, extends)
VALUES ($1, $2, $3, $4)
RETURNING id;