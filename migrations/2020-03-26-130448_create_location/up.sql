CREATE TABLE locations
(
    id            SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name          TEXT                      NOT NULL,
    website       TEXT,
    street        TEXT                      NOT NULL,
    street_number INTEGER                   NOT NULL,
    city          TEXT                      NOT NULL,
    postal_code   INTEGER                   NOT NULL,
    building      TEXT,
    maps_link     TEXT                      NOT NULL,
    UNIQUE (name, street, street_number),
    UNIQUE (name, maps_link)
);
