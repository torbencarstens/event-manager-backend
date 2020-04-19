CREATE TABLE event_tags
(
    tag_id   INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    PRIMARY KEY (tag_id, event_id),
    FOREIGN KEY (tag_id) REFERENCES tags (id),
    FOREIGN KEY (event_id) REFERENCES events (id)
);
