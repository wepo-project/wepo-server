INSERT INTO wepo.posts(sender, content)
VALUES ($1, $2)
RETURNING $table_fields;