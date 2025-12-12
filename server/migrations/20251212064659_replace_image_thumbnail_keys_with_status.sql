ALTER TABLE image
    DROP COLUMN small_thumbnail_key,
    DROP COLUMN medium_thumbnail_key,
    DROP COLUMN large_thumbnail_key;

ALTER TABLE image
    ADD COLUMN has_small_thumbnail BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN has_medium_thumbnail BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN has_large_thumbnail BOOLEAN NOT NULL DEFAULT FALSE;
