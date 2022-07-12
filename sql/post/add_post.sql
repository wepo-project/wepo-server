INSERT INTO main.posts(id, sender, content)
VALUES ($1, $2, $3)
RETURNING $table_fields;