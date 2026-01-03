-- add trashed_at column
ALTER TABLE image
  ADD COLUMN trashed_at TIMESTAMPTZ;

CREATE INDEX idx_image_trashed_at ON image (owner_id, trashed_at DESC) WHERE trashed_at IS NOT NULL;

--- recreate indexes to exclude trashed images
CREATE INDEX idx_image_owner_id_not_trashed   ON image (owner_id)   WHERE trashed_at IS NULL;
CREATE INDEX idx_image_created_at_not_trashed ON image (created_at) WHERE trashed_at IS NULL;
CREATE INDEX idx_image_updated_at_not_trashed ON image (updated_at) WHERE trashed_at IS NULL;
CREATE INDEX idx_image_is_public_not_trashed  ON image (is_public)  WHERE trashed_at IS NULL;

-- remove old indexes
DROP INDEX idx_image_owner_id;
DROP INDEX idx_image_created_at;
DROP INDEX idx_image_is_public;
