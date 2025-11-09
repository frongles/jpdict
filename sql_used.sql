-- search main english
EXPLAIN QUERY PLAN
SELECT DISTINCT gloss
FROM sense_eng
WHERE gloss LIKE ?
ORDER BY gloss COLLATE NOCASE
LIMIT 30;


-- search main reading
EXPLAIN QUERY PLAN
SELECT
    (SELECT k.keb
     FROM kanji k
     WHERE k.ent_seq = r.ent_seq
     ORDER BY k.pri_rank
     LIMIT 1) AS keb,
    r.reb AS reb,
    r.ent_seq AS ent_seq
FROM readings r
WHERE r.reb LIKE ?
ORDER BY r.reb, r.pri_rank
LIMIT 30;

-- search main kanji
EXPLAIN QUERY PLAN
SELECT
    (SELECT r.reb
     FROM readings r
     WHERE r.ent_seq = k.ent_seq
     ORDER BY r.pri_rank
     LIMIT 1) AS reb,
    k.keb AS keb,
    k.ent_seq AS ent_seq
FROM kanji k
WHERE k.keb LIKE ?
ORDER BY k.keb, k.pri_rank
LIMIT 30;

-- get by gloss
EXPLAIN QUERY PLAN
SELECT
    s.id AS sense_id,
    s.ent_seq AS ent_seq,
    
    -- First kanji by highest priority
    (SELECT kr.keb
     FROM kanji kr
     WHERE kr.ent_seq = s.ent_seq
     ORDER BY kr.pri_rank
     LIMIT 1) AS kanji,
    (SELECT kr.pri_rank
     FROM kanji kr
     WHERE kr.ent_seq = s.ent_seq
     ORDER BY kr.pri_rank
     LIMIT 1) AS pri,

    -- First reading by highest priority
    (SELECT rr.reb
     FROM readings rr
     WHERE rr.ent_seq = s.ent_seq
     ORDER BY rr.pri_rank
     LIMIT 1) AS reading,

    GROUP_CONCAT(DISTINCT se.gloss) AS gloss_list

FROM sense s
LEFT JOIN sense_eng se ON s.id = se.sense_id
WHERE s.id IN (
SELECT sense_id 
FROM sense_eng
WHERE gloss = ? COLLATE NOCASE
)
GROUP BY s.id
ORDER BY pri;

-- get by ent seq
EXPLAIN QUERY PLAN
SELECT
    se.sense_id AS sense_id,
    se.gloss AS gloss
FROM sense_eng se
LEFT JOIN sense s ON s.id = se.sense_id
WHERE s.ent_seq = ?;

