-- when the metadata of an image is updated
CREATE OR REPLACE FUNCTION image_set_updated_at()
RETURNS trigger AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER image_before_update_set_updated_at
BEFORE UPDATE
ON image
FOR EACH ROW
EXECUTE FUNCTION image_set_updated_at();


--- when the tags of an image is updated
CREATE OR REPLACE FUNCTION update_image_timestamp_on_tag_change()
RETURNS trigger AS $$
BEGIN
  UPDATE image
  SET updated_at = NOW()
  WHERE id = ANY(ARRAY[OLD.image_id, NEW.image_id]::int[]);

  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER image_tags_touch_image_updated_at
AFTER INSERT OR UPDATE OR DELETE
ON image_tag
FOR EACH ROW
EXECUTE FUNCTION update_image_timestamp_on_tag_change();
