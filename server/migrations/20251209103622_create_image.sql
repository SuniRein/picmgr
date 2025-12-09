CREATE TABLE category(
    id        INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    owner_id  INT REFERENCES app_user(id) ON DELETE SET NULL,
    parent_id INT REFERENCES category(id) ON DELETE RESTRICT,

    name        TEXT NOT NULL,
    description TEXT,

    is_public BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT category_name_unique UNIQUE (owner_id, parent_id, name)
);

CREATE TABLE image(
    id          INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    owner_id    INT REFERENCES app_user(id) ON DELETE SET NULL,
    category_id INT REFERENCES category(id) ON DELETE SET NULL,

    storage_key TEXT NOT NULL,
    size_bytes  BIGINT NOT NULL CHECK (size_bytes >= 0),
    width       INT NOT NULL CHECK (width > 0),
    height      INT NOT NULL CHECK (height > 0),
    mime_type   TEXT NOT NULL,
    exif        JSONB,

    small_thumbnail_key  TEXT,
    medium_thumbnail_key TEXT,
    large_thumbnail_key  TEXT,

    is_public BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE tag(
    id   INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE image_tag(
    image_id INT REFERENCES image(id) ON DELETE CASCADE,
    tag_id   INT REFERENCES tag(id) ON DELETE CASCADE,
    is_ai_generated BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (image_id, tag_id)
);

CREATE INDEX idx_image_owner_id    ON image(owner_id);
CREATE INDEX idx_image_category_id ON image(category_id);
CREATE INDEX idx_image_is_public   ON image(is_public);
CREATE INDEX idx_image_created_at  ON image(created_at);

CREATE INDEX idx_category_owner_id  ON category(owner_id);
CREATE INDEX idx_category_parent_id ON category(parent_id);
CREATE INDEX idx_category_is_public ON category(is_public);

CREATE INDEX idx_image_tag_image_id ON image_tag(image_id);
CREATE INDEX idx_image_tag_tag_id   ON image_tag(tag_id);
