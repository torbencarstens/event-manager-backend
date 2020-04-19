table! {
    event_tags (tag_id, event_id) {
        tag_id -> Int4,
        event_id -> Int4,
    }
}

table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        timestamp -> Timestamptz,
        timestamp_end -> Timestamptz,
        price -> Nullable<Int4>,
        currency -> Nullable<Varchar>,
        location_id -> Int4,
        organizer_id -> Nullable<Int4>,
    }
}

table! {
    locations (id) {
        id -> Int4,
        name -> Text,
        website -> Nullable<Text>,
        street -> Text,
        street_number -> Int4,
        city -> Text,
        postal_code -> Int4,
        country -> Text,
        building -> Nullable<Text>,
        maps_link -> Text,
    }
}

table! {
    organizers (id) {
        id -> Int4,
        name -> Varchar,
        website -> Nullable<Text>,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

joinable!(event_tags -> events (event_id));
joinable!(event_tags -> tags (tag_id));
joinable!(events -> locations (location_id));
joinable!(events -> organizers (organizer_id));

allow_tables_to_appear_in_same_query!(
    event_tags,
    events,
    locations,
    organizers,
    tags,
);
