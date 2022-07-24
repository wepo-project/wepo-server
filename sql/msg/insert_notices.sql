INSERT INTO main.notices
    (sender, notice_type, sender_object, addressee_id)
VALUES ($1, $2, $3, $4)
RETURNING *