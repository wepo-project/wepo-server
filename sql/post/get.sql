SELECT
    r1.id, 
    r1.content, 
    r1.create_time, 
    r1.likes, 
    r1.comments, 
    r1.sender_nick, 
    r1.sender_id,
    p1.id AS origin_id,
    p1.content AS origin_content,
    p1.create_time AS origin_create_time,
    u1.nick AS origin_sender_nick,
    u1.id AS origin_sender_id
FROM (
    SELECT p.id, p.content, p.create_time, p.likes, p.comments, p.extends, u.nick AS sender_nick, u.id AS sender_id
    FROM main.posts AS p, main.users AS u 
    WHERE p.id = $1 AND p.sender = u.id
) AS r1
LEFT JOIN main.posts p1 ON p1.id = r1.extends
LEFT JOIN main.users u1 ON p1.sender = u1.id