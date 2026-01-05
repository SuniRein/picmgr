-- Adds an image to an album if the user has permission and the image is not already in the album.
-- Parameters:
-- $1: album_id
-- $2: user_id
-- $3: image_id

WITH
-- Check if the album exists
alb AS (
  SELECT id, owner_id
  FROM album
  WHERE id = $1
),
-- Check if the image exists
img AS (
  SELECT id, owner_id, is_public
  FROM image
  WHERE id = $3
),
-- Check if the user has permission
perm AS (
  SELECT 1 AS ok
  FROM alb a, img i
  WHERE a.owner_id = $2
    AND (i.owner_id = $2 OR i.is_public = TRUE)
),
-- Attempt to insert the new image into the album
ins AS (
  INSERT INTO image_album(album_id, image_id, position)
  SELECT
      $1,
      $3,
      nextval('image_album_position_seq') * 1000
  FROM perm
  ON CONFLICT (album_id, image_id) DO NOTHING
  RETURNING 1 AS ok
)
SELECT
  CASE
    WHEN NOT EXISTS (SELECT 1 FROM alb) THEN 'album_not_found'
    WHEN NOT EXISTS (SELECT 1 FROM img) THEN 'image_not_found'
    WHEN NOT EXISTS (SELECT 1 FROM perm) THEN 'no_permission'
    WHEN EXISTS (SELECT 1 FROM ins) THEN 'ok'
    WHEN EXISTS (
      SELECT 1 FROM image_album
      WHERE album_id = $1 AND image_id = $3
    ) THEN 'duplicate'
    ELSE 'unknown'
  END AS "status!"
