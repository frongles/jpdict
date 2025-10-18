CREATE TABLE gloss_entry AS
SELECT g.gloss,
    GROUP_CONCAT(DISTINCT kr.keb) AS kanji_list,
    GROUP_CONCAT(DISTINCT rr.reb) AS reading_list,
    GROUP_CONCAT(DISTINCT related.gloss) AS related_gloss
FROM eng g
LEFT JOIN sense_eng se ON se.gloss = g.gloss
LEFT JOIN sense s ON se.sense_id = s.id
LEFT JOIN sense_eng related ON se.sense_id = related.sense_id
LEFT JOIN entries_kanji kr ON s.ent_seq = kr.ent_seq
LEFT JOIN entries_readings rr ON s.ent_seq = rr.ent_seq
GROUP BY se.sense_id;

