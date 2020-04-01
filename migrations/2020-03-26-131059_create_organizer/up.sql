CREATE TABLE organizers
(
    id      SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name    VARCHAR(255)              NOT NULL,
    website TEXT
);
