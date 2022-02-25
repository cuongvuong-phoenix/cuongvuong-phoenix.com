-- ----------------------------------------------------------------
-- post
-- ----------------------------------------------------------------
-- Mocks
INSERT INTO post(title, slug, reading_time, visible, content)
SELECT
	'Post ' || seq AS title,
	'post-' || seq AS slug,
	round((random() * 30)::NUMERIC, 0) AS reading_time,
	random() > 0.5 AS visible,
	'Content ' || seq AS content
FROM GENERATE_SERIES(1, 64) seq;

-- ----------------------------------------------------------------
-- tag
-- ----------------------------------------------------------------
-- Mocks
INSERT INTO tag(name, icon)
SELECT
	'Tag ' || seq AS name,
	'mdi:vuejs' AS icon
FROM generate_series(1, 16) seq;

-- ----------------------------------------------------------------
-- post_has_tag
-- ----------------------------------------------------------------
-- Mocks
WITH
	expanded AS (
		SELECT random(), seq, p.id AS post_id, t.id AS tag_id
		FROM generate_series(1, 64) seq, post p, tag t
	),
	shuffled AS (
		SELECT e.post_id, e.tag_id
		FROM
			expanded e
			JOIN (
				SELECT seq, min(random) FROM expanded GROUP BY seq
			) e_grouped
			ON (e.seq = e_grouped.seq AND e.random = e_grouped.min)
			ORDER BY e.seq
	)
INSERT INTO post_has_tag(post_id, tag_id)
SELECT post_id, tag_id
FROM shuffled;