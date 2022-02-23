-- Add up migration script here
CREATE TABLE post (
	id				UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	title 			VARCHAR(256) NOT NULL,
	slug 			VARCHAR(256) UNIQUE NOT NULL,
	reading_time 	INTEGER,
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
