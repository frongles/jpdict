.timer on

SELECT
    (SELECT k.keb
     FROM kanji k
     WHERE k.ent_seq = r.ent_seq
     ORDER BY k.pri_rank
     LIMIT 1) AS top_kanji,
    r.reb,
    r.ent_seq
FROM readings r
WHERE r.reb LIKE 'いく%'
ORDER BY r.reb, r.pri_rank
LIMIT 30;


