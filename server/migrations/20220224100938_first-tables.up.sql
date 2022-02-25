CREATE TABLE post (
	id				UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	title 			VARCHAR(256) NOT NULL,
	slug 			VARCHAR(256) UNIQUE NOT NULL,
	reading_time 	INTEGER NOT NULL,
	visible 		BOOLEAN NOT NULL,
	created_at 		TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at 		TIMESTAMP
);

CREATE TABLE tag (
	id			UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	name 		VARCHAR(256) UNIQUE NOT NULL,
	icon 		VARCHAR(128),
	created_at 	TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at 	TIMESTAMP
);

CREATE TABLE post_has_tag (
	post_id		UUID REFERENCES post (id) ON DELETE CASCADE ON UPDATE CASCADE,
	tag_id		UUID REFERENCES tag (id) ON DELETE CASCADE ON UPDATE CASCADE,
	created_at 	TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at 	TIMESTAMP,

	PRIMARY KEY (post_id, tag_id)
);