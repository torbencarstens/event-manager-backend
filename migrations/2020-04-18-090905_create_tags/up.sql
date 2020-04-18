CREATE TABLE tags
(
    id          SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name        VARCHAR(255)              NOT NULL,
    description TEXT
);
