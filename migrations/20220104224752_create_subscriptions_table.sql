-- Add migration script here
-- Constraints have been added but this comes at a cost of write-throughput
CREATE TABLE subscriptions(
--     field should not be empty
    id uuid NOT NULL,
--     id is the primary key for this table
    PRIMARY KEY (id),
--     enforcing email uniqueness & field should not be empty
    email TEXT NOT NULL UNIQUE,
--     field should not be empty
    name TEXT NOT NULL,
--     timestamptz is a timezone aware date and time type
    subscribed_at timestamptz NOT NULL
);