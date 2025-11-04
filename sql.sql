.timer on

-- need to turn k.pri into an integer when parsing for this to work well

SELECT
    (SELECT k.keb
     FROM kanji k
     WHERE k.ent_seq = r.ent_seq
     ORDER BY k.pri_rank, k.keb
     LIMIT 1) AS top_kanji,
    r.reb
FROM readings r
WHERE r.reb LIKE 'ちてき%'
ORDER BY r.reb
LIMIT 30;

