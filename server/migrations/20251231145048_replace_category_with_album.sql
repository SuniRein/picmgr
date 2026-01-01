ALTER TABLE image
    DROP COLUMN category_id;

DROP TABLE category;

CREATE TABLE album(
    id        INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    owner_id  INT NOT NULL REFERENCES app_user(id) ON DELETE CASCADE,

    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',

    is_public BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT album_name_unique UNIQUE (owner_id, name)
);

CREATE TABLE image_album(
    image_id INT REFERENCES image(id) ON DELETE CASCADE,
    album_id INT REFERENCES album(id) ON DELETE CASCADE,
    position BIGINT NOT NULL,
    PRIMARY KEY (image_id, album_id),
    CONSTRAINT position_unique_per_album UNIQUE (album_id, position)
);

CREATE INDEX idx_album_owner_id ON album(owner_id);

CREATE INDEX idx_image_album_image_id ON image_album(image_id);
CREATE INDEX idx_image_album_album_id ON image_album(album_id);
