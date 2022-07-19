INSERT INTO main.notices
    (sender, notice_type, sender_obj_id, addressee_id)
VALUES ($1, $2, $3, $4)
RETURNING *