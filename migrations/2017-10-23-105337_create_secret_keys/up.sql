CREATE TABLE secrets (
	id INTEGER PRIMARY KEY NOT NULL,
	key_id UNSIGNED BIG INT NOT NULL UNIQUE,
	threshold INTEGER NOT NULL
)
