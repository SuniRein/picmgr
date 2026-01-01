-- when the metadata of an album is updated
CREATE OR REPLACE FUNCTION album_set_updated_at()
RETURNS trigger AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER album_before_update_set_updated_at
BEFORE UPDATE
ON album
FOR EACH ROW
EXECUTE FUNCTION album_set_updated_at();


-- when images in an album are added, removed, or updated
CREATE OR REPLACE FUNCTION touch_album_updated_at()
RETURNS trigger AS $$
BEGIN
  UPDATE album
  SET updated_at = NOW()
  WHERE id = ANY(ARRAY[OLD.album_id, NEW.album_id]::int[]);

  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER image_album_touch_album_updated_at
AFTER INSERT OR UPDATE OR DELETE
ON image_album
FOR EACH ROW
EXECUTE FUNCTION touch_album_updated_at();
