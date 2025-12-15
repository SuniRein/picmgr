CREATE TABLE image_storage(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    storage_key          TEXT UNIQUE NOT NULL,
    has_small_thumbnail  BOOLEAN NOT NULL DEFAULT FALSE,
    has_medium_thumbnail BOOLEAN NOT NULL DEFAULT FALSE,
    has_large_thumbnail  BOOLEAN NOT NULL DEFAULT FALSE,

    size_bytes BIGINT NOT NULL CHECK (size_bytes >= 0),
    width      INT NOT NULL CHECK (width > 0),
    height     INT NOT NULL CHECK (height > 0),
    mime_type  TEXT NOT NULL,
    exif       JSONB NOT NULL DEFAULT '{}'::JSON
);

-- Add storage_id column to image table
ALTER TABLE image
ADD COLUMN storage_id INT;

-- Migrate existing data to the new image_storage table
INSERT INTO image_storage(
    storage_key,
    size_bytes,
    width,
    height,
    mime_type,
    exif,
    has_small_thumbnail,
    has_medium_thumbnail,
    has_large_thumbnail
)
SELECT
    storage_key,
    size_bytes,
    width,
    height,
    mime_type,
    COALESCE(exif, '{}'::JSONB),
    BOOL_OR(has_small_thumbnail),
    BOOL_OR(has_medium_thumbnail),
    BOOL_OR(has_large_thumbnail)
FROM image
GROUP BY
    storage_key,
    size_bytes,
    width,
    height,
    mime_type,
    COALESCE(exif, '{}'::JSONB);

-- Update image table to reference image_storage
UPDATE image i
SET storage_id = s.id
FROM image_storage s
WHERE i.storage_key = s.storage_key;

-- Set storage_id as NOT NULL
ALTER TABLE image
ALTER COLUMN storage_id SET NOT NULL;

-- Add foreign key constraint
ALTER TABLE image
ADD CONSTRAINT fk_image_storage
FOREIGN KEY (storage_id)
REFERENCES image_storage(id)
ON DELETE RESTRICT;

CREATE INDEX idx_image_storage_id ON image(storage_id);

-- Remove old storage columns from image table
ALTER TABLE image
DROP COLUMN storage_key,
DROP COLUMN has_small_thumbnail,
DROP COLUMN has_medium_thumbnail,
DROP COLUMN has_large_thumbnail,
DROP COLUMN size_bytes,
DROP COLUMN width,
DROP COLUMN height,
DROP COLUMN mime_type,
DROP COLUMN exif;
