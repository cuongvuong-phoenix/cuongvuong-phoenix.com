CREATE TABLE post (
	id				SERIAL PRIMARY KEY,
	title 			VARCHAR(256) NOT NULL,
	slug 			VARCHAR(256) UNIQUE NOT NULL,
	reading_time 	INTEGER NOT NULL,
	visible 		BOOLEAN NOT NULL,
	content			TEXT NOT NULL,
	created_at 		TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at 		TIMESTAMP WITH TIME ZONE
);

CREATE TABLE tag (
	id			SERIAL PRIMARY KEY,
	name 		VARCHAR(256) UNIQUE NOT NULL,
	icon 		VARCHAR(128),
	created_at 	TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at 	TIMESTAMP WITH TIME ZONE
);

CREATE TABLE post_has_tag (
	post_id		INTEGER REFERENCES post (id) ON DELETE CASCADE ON UPDATE CASCADE,
	tag_id		INTEGER REFERENCES tag (id) ON DELETE CASCADE ON UPDATE CASCADE,
	created_at 	TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at 	TIMESTAMP WITH TIME ZONE,

	PRIMARY KEY (post_id, tag_id)
);