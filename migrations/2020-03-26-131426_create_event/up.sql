CREATE TABLE events
(
    id            SERIAL UNIQUE PRIMARY KEY NOT NULL,
    name          VARCHAR(255)              NOT NULL,
    description   TEXT                      NOT NULL,
    timestamp     TIMESTAMPTZ               NOT NULL,
    timestamp_end TIMESTAMPTZ               NOT NULL,
    price         INTEGER    DEFAULT 0,
    currency      VARCHAR(3) DEFAULT 'CHF',
    location_id   INTEGER                   NOT NULL,
    organizer_id  INTEGER,
    CONSTRAINT check_timestamp CHECK (timestamp > '1970-01-01 00:00:00+0'),
    CONSTRAINT ends_before_start CHECK (timestamp < timestamp_end),
    CONSTRAINT check_price CHECK ((price IS NOT NULL AND currency IS NOT NULL) OR
                                  (price IS NULL AND currency IS NULL)),
    FOREIGN KEY (location_id) REFERENCES locations (id),
    FOREIGN KEY (organizer_id) REFERENCES organizers (id)
);
