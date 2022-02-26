CREATE TABLE home (
    biography           VARCHAR(256) NOT NULL,
    contact             VARCHAR(256) NOT NULL,
    years_of_experience INTEGER NOT NULL,
    num_projects        INTEGER NOT NULL,
    created_at          TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP WITH TIME ZONE
);