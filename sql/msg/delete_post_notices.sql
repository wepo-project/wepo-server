DELETE FROM main.notices
WHERE addressee_id = $1 and notice_type in ($2, $3);