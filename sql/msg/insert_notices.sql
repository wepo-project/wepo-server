INSERT INTO main.notices(addressee_id, notice_type, args)
VALUES ($1, $2, $3)
RETURNING addressee_id