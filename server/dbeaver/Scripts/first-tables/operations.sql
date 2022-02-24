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

PREPARE read_many_posts(INTEGER, INTEGER) AS
	SELECT id, title, slug, reading_time, visible, created_at, updated_at
	FROM post
	ORDER BY
		coalesce(updated_at, created_at) DESC,
		reading_time DESC,
		title ASC
	LIMIT $1
	OFFSET $2;

EXECUTE read_many_posts(8, 0);
DEALLOCATE read_many_posts;

-- UPDATE
PREPARE update_post(VARCHAR, VARCHAR, INTEGER, BOOLEAN, UUID) AS
	UPDATE post
	SET
		title = coalesce($1, title),
		slug = coalesce($2, slug),
		reading_time = coalesce($3, reading_time),
		visible = coalesce($4, visible)
	WHERE id = $5
	RETURNING id, title, slug, reading_time, visible, created_at, updated_at;

EXECUTE update_post('Post updated', 'post-updated', 15, TRUE, 'bf9f6b3e-d0cb-485c-b62f-dfe01c75b9fc');
DEALLOCATE update_post;

-- DELETE
PREPARE delete_post(UUID) AS
	DELETE FROM post
	WHERE id = $1
	RETURNING id, title, slug, reading_time, visible, created_at, updated_at;

EXECUTE delete_post('bf9f6b3e-d0cb-485c-b62f-dfe01c75b9fc');
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

PREPARE read_tag(UUID) AS
	SELECT id, name, icon, created_at, updated_at
	FROM tag
	WHERE id = $1;

EXECUTE read_tag('f65b838a-b01e-4849-8e59-46f2e4460600');
DEALLOCATE read_tag;

-- UPDATE
PREPARE update_tag(UUID, VARCHAR, VARCHAR) AS
	UPDATE tag
	SET
		name = coalesce($2, name),
		icon = coalesce($3, icon),
		updated_at = NULL
	WHERE id = $1
	RETURNING id, name, icon, created_at, updated_at;

EXECUTE update_tag('f65b838a-b01e-4849-8e59-46f2e4460600', 'Tag update', 'icon-update');
DEALLOCATE update_tag;

-- DELETE
PREPARE delete_tag(UUID) AS
	DELETE FROM tag
	WHERE id = $1
	RETURNING id, name, icon, created_at, updated_at;

EXECUTE delete_tag('f65b838a-b01e-4849-8e59-46f2e4460600');
DEALLOCATE delete_tag;

-- ----------------------------------------------------------------
-- post_has_tag
-- ----------------------------------------------------------------
-- CREATE
PREPARE create_post_has_tag(UUID, UUID) AS
	INSERT INTO post_has_tag(post_id, tag_id)
	VALUES ($1, $2);

EXECUTE create_post_has_tag('27596ba5-0435-4bf8-9cc3-2f2b2486fdc6', 'c3f902d6-e141-4fff-b044-5f4aa972d39b');
DEALLOCATE create_post_has_tag;

-- READ
PREPARE read_tags_of_posts(UUID[]) AS
	SELECT pht.post_id, t.id, t.name, t.icon, t.created_at, t.updated_at
	FROM post_has_tag pht JOIN tag t ON pht.tag_id = t.id
	WHERE pht.post_id = ANY($1)
	ORDER BY
		coalesce(pht.updated_at, pht.created_at) DESC,
		t.name ASC;

EXECUTE read_tags_of_posts(ARRAY['af802536-732f-485b-ba68-845c05d0b2dd', '34c7445e-3026-4299-8eee-0e1d5920c8df']::UUID[]);
DEALLOCATE read_tags_of_posts;

PREPARE read_posts_has_tags(UUID[]) AS
	SELECT p.id, p.title, p.slug, p.reading_time, p.visible, p.created_at, p.updated_at
	FROM post_has_tag pht JOIN post p ON pht.post_id = p.id
	WHERE pht.tag_id = ANY($1) 
	ORDER BY
		coalesce(pht.updated_at, pht.created_at) DESC,
		p.reading_time DESC,
		p.title ASC;

EXECUTE read_posts_has_tags(ARRAY['c3f902d6-e141-4fff-b044-5f4aa972d39b', '2b6cf970-b1e8-4974-9aa3-ca78f45b0ac1']::UUID[]);
DEALLOCATE read_posts_has_tags;

-- UPDATE
PREPARE update_tag_of_post(UUID, UUID) AS
	UPDATE post_has_tag
	SET
		tag_id = $2,
		updated_at = NULL
	WHERE post_id = $1;

EXECUTE update_tag_of_post('773d4404-046b-4081-bea8-6e5e2e0d98f1', '2b358236-e176-42d6-9aa5-3e69281043c8');
DEALLOCATE update_tag_of_post;