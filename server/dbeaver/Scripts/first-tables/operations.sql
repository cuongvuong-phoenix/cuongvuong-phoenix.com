-- ----------------------------------------------------------------
-- post
-- ----------------------------------------------------------------
-- CREATE
PREPARE create_post(title, slug, reading_time, visible) AS
	INSERT INTO post(title, slug, reading_time, visible)
	VALUES ($1, $2, $3, $4)
	RETURNING id, title, slug, reading_time, visible, created_at, updated_at;

EXECUTE create_post('Post create', 'post-create', 30, true);
DEALLOCATE create_post;

-- READ
SELECT COUNT(*) FROM post;

PREPARE read_posts(INTEGER, INTEGER) AS
	SELECT id, title, slug, reading_time, visible, created_at, updated_at
	FROM post
	ORDER BY
		coalesce(updated_at, created_at) DESC,
		reading_time DESC,
		title ASC
	LIMIT $1
	OFFSET $2;

EXECUTE read_posts(8, 0);
DEALLOCATE read_posts;

PREPARE read_posts_content(INTEGER[]) AS
	SELECT content
	FROM post
	WHERE id = ANY($1);

EXECUTE read_posts_content(ARRAY[1, 2]);
DEALLOCATE read_posts_content;

-- READ by `tag`s.
PREPARE count_posts_by_tags(INTEGER[]) AS
	SELECT COUNT(*)
	FROM
		post p
		JOIN post_has_tag pht ON p.id = pht.post_id
	WHERE pht.tag_id = ANY($1);

EXECUTE count_posts_by_tags(ARRAY[1, 2]);
DEALLOCATE count_posts_by_tags;

PREPARE read_posts_by_tags(INTEGER[], INTEGER, INTEGER) AS
	SELECT p.id, p.title, p.slug, p.reading_time, p.visible, p.created_at, p.updated_at
	FROM
		post p
		JOIN post_has_tag pht ON p.id = pht.post_id
	WHERE pht.tag_id = ANY($1)
	ORDER BY
		coalesce(p.updated_at, p.created_at) DESC,
		p.reading_time DESC,
		p.title ASC
	LIMIT $2
	OFFSET $3;

EXECUTE read_posts_by_tags(ARRAY[1, 2], 8, 0);
DEALLOCATE read_posts_by_tags;

-- UPDATE
PREPARE update_post(INTEGER, VARCHAR, VARCHAR, INTEGER, BOOLEAN) AS
	UPDATE post
	SET
		title = coalesce($2, title),
		slug = coalesce($3, slug),
		reading_time = coalesce($4, reading_time),
		visible = coalesce($5, visible)
	WHERE id = $1
	RETURNING id, title, slug, reading_time, visible, created_at, updated_at;

EXECUTE update_post(1, 'Post updated', 'post-updated', 15, TRUE);
DEALLOCATE update_post;

-- DELETE
PREPARE delete_post(INTEGER) AS
	DELETE FROM post
	WHERE id = $1
	RETURNING id, title, slug, reading_time, visible, created_at, updated_at;

EXECUTE delete_post(1);
DEALLOCATE delete_post;

-- ----------------------------------------------------------------
-- tag
-- ----------------------------------------------------------------
-- CREATE
PREPARE create_tag(VARCHAR, VARCHAR) AS
	INSERT INTO tag(name, icon)
	VALUES ($1, $2)
	RETURNING id, name, icon, created_at, updated_at;

EXECUTE create_tag('Tag create', 'icon-create');
DEALLOCATE create_tag;

-- READ
PREPARE read_tags(INTEGER, INTEGER) AS
	SELECT id, name, icon, created_at, updated_at
	FROM tag
	ORDER BY
		coalesce(updated_at, created_at) DESC,
		name ASC
	LIMIT $1
	OFFSET $2;

EXECUTE read_tags(8, 0);
DEALLOCATE read_tags;

PREPARE read_tag(INTEGER) AS
	SELECT id, name, icon, created_at, updated_at
	FROM tag
	WHERE id = $1;

EXECUTE read_tag(1);
DEALLOCATE read_tag;

-- UPDATE
PREPARE update_tag(INTEGER, VARCHAR, VARCHAR) AS
	UPDATE tag
	SET
		name = coalesce($2, name),
		icon = coalesce($3, icon),
		updated_at = NULL
	WHERE id = $1
	RETURNING id, name, icon, created_at, updated_at;

EXECUTE update_tag(1, 'Tag update', 'icon-update');
DEALLOCATE update_tag;

-- DELETE
PREPARE delete_tag(INTEGER) AS
	DELETE FROM tag
	WHERE id = $1
	RETURNING id, name, icon, created_at, updated_at;

EXECUTE delete_tag(1);
DEALLOCATE delete_tag;

-- ----------------------------------------------------------------
-- post_has_tag
-- ----------------------------------------------------------------
-- CREATE
PREPARE create_tag_of_post(INTEGER, INTEGER) AS
	INSERT INTO post_has_tag(post_id, tag_id)
	VALUES ($1, $2);

EXECUTE create_tag_of_post(1, 1);
DEALLOCATE create_tag_of_post;

PREPARE create_tags_of_post(INTEGER, INTEGER[]) AS
	INSERT INTO post_has_tag(post_id, tag_id)
	SELECT $1 AS post_id, tag_id
	FROM unnest($2::INTEGER[]) tag_id;

EXECUTE create_tags_of_post(1, ARRAY[1, 2]);
DEALLOCATE create_tags_of_post;

-- READ
PREPARE read_tags_of_posts(INTEGER[]) AS
	SELECT pht.post_id, t.id, t.name, t.icon, t.created_at, t.updated_at
	FROM post_has_tag pht JOIN tag t ON pht.tag_id = t.id
	WHERE pht.post_id = ANY($1)
	ORDER BY
		coalesce(pht.updated_at, pht.created_at) DESC,
		t.name ASC;

EXECUTE read_tags_of_posts(ARRAY[1, 2]);
DEALLOCATE read_tags_of_posts;

PREPARE read_posts_has_tags(INTEGER[]) AS
	SELECT p.id, p.title, p.slug, p.reading_time, p.visible, p.created_at, p.updated_at
	FROM post_has_tag pht JOIN post p ON pht.post_id = p.id
	WHERE pht.tag_id = ANY($1) 
	ORDER BY
		coalesce(pht.updated_at, pht.created_at) DESC,
		p.reading_time DESC,
		p.title ASC;

EXECUTE read_posts_has_tags(ARRAY[1, 2]);
DEALLOCATE read_posts_has_tags;

-- UPDATE
PREPARE update_tag_of_post(INTEGER, INTEGER, INTEGER) AS
	UPDATE post_has_tag
	SET
		tag_id = $3,
		updated_at = NULL
	WHERE post_id = $1 AND tag_id = $2;

EXECUTE update_tag_of_post(1, 1, 2);
DEALLOCATE update_tag_of_post;

CREATE OR REPLACE FUNCTION update_tags_of_post(INTEGER, INTEGER[])
RETURNS VOID AS
$$
	DELETE FROM post_has_tag
	WHERE post_id = $1;

	EXECUTE create_tags_of_post($1, $2);
$$ LANGUAGE sql;

SELECT update_tags_of_post(1, ARRAY[1, 2]);

-- DELETE
PREPARE delete_tag_of_post(INTEGER, INTEGER) AS
	DELETE FROM post_has_tag
	WHERE post_id = $1 AND tag_id = $2;

EXECUTE delete_tag_of_post(1, 1);
DEALLOCATE delete_tag_of_post;