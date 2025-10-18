SELECT
    g.gloss,
    MIN(kr.keb) AS first_kanji,       -- pick first kanji alphabetically
    GROUP_CONCAT(DISTINCT rr.reb) AS reading_list
FROM eng g
LEFT JOIN sense_eng se ON se.gloss = g.gloss
LEFT JOIN sense s ON s.id = se.sense_id
LEFT JOIN entries e ON s.ent_seq = e.ent_seq
LEFT JOIN entries_kanji ek ON e.ent_seq = ek.ent_seq
LEFT JOIN kanji kr ON ek.keb = kr.keb
LEFT JOIN entries_readings er ON e.ent_seq = er.ent_seq
LEFT JOIN japanese_readings rr ON er.reb = rr.reb
GROUP BY g.gloss;

