DELETE FROM main.friendship
WHERE requester_id = $1 and addressee_id = $2