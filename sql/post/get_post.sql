SELECT p.id, p.content, p.create_time, p.likes, p.comments, u.nick,
    CASE 
        WHEN p.extends is NULL THEN NULL
        ELSE (SELECT p1.content FROM main.posts as p1 WHERE p1.id = p.extends)
    END 
    as origin
FROM main.posts as p, main.users as u WHERE p.id = $1 and p.sender = u.id;

-- SELECT
--     r1.id, r1.content, r1.create_time, r1.likes, r1.comments, r1.nick,
--     p1.content as origin_content, u1.nick as origin_sender_nick
-- FROM (
--     SELECT p.id, p.content, p.create_time, p.likes, p.comments, u.nick, p.extends 
--     FROM main.posts as p, main.users as u 
--     WHERE p.id = 6952559375257051130 and p.sender = u.id
-- ) as r1, main.posts as p1, main.users as u1 where p1.id = r1.extends and p1.sender = u1.id;