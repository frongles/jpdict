SELECT e.ent_seq,
       k.kanji_list,
       r.reading_list,
       g.gloss_list
FROM entries e
LEFT JOIN (
    SELECT ek.ent_seq, GROUP_CONCAT(kr.keb) AS kanji_list
    FROM entries_kanji ek
    JOIN kanji kr ON ek.keb = kr.keb
    GROUP BY ek.ent_seq
) k ON e.ent_seq = k.ent_seq
LEFT JOIN (
    SELECT er.ent_seq, GROUP_CONCAT(rr.reb) AS reading_list
    FROM entries_readings er
    JOIN japanese_readings rr ON er.reb = rr.reb
    GROUP BY er.ent_seq
) r ON e.ent_seq = r.ent_seq
LEFT JOIN (
    SELECT s.ent_seq, GROUP_CONCAT(g.gloss) AS gloss_list
    FROM sense s
    JOIN sense_eng se ON s.id = se.sense_id
    JOIN eng g ON g.gloss = se.gloss
    GROUP BY s.ent_seq
) g ON e.ent_seq = g.ent_seq
WHERE e.ent_seq = '1007820';
