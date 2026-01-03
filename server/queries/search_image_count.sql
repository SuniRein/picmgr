-- Get total count of images matching search criteria
-- Parameters: see search_image.sql

SELECT
  COUNT(*) AS "total_count!"
FROM image i
JOIN image_storage s ON i.storage_id = s.id
WHERE
  -- Filter by owner_id
  ($13::int IS NULL OR i.owner_id = $13::int)
  -- Filter by width
  AND ($3::int IS NULL OR s.width >= $3::int)
  AND ($4::int IS NULL OR s.width <= $4::int)
  -- Filter by height
  AND ($5::int IS NULL OR s.height >= $5::int)
  AND ($6::int IS NULL OR s.height <= $6::int)
  -- Filter by mime_type
  AND ($7::text IS NULL OR s.mime_type = $7::text)
  -- Filter by created_at
  AND ($8::timestamptz IS NULL OR i.created_at <= $8::timestamptz)
  AND ($9::timestamptz IS NULL OR i.created_at >= $9::timestamptz)
  -- Filter by updated_at
  AND ($10::timestamptz IS NULL OR i.updated_at <= $10::timestamptz)
  AND ($11::timestamptz IS NULL OR i.updated_at >= $11::timestamptz)
  -- Filter by is_public
  AND ($12::boolean IS NULL OR i.is_public = $12::boolean)
  -- Filter by album_id
  AND (
    $14::int IS NULL OR EXISTS (
      SELECT 1
      FROM image_album ia
      WHERE ia.image_id = i.id
        AND ia.album_id = $14::int
    )
  )
  -- Filter by tags
  AND (
    cardinality($15::text[]) = 0 OR NOT EXISTS (
      SELECT 1
      FROM unnest($15::text[]) AS want_tag(name)
      WHERE NOT EXISTS (
        SELECT 1
        FROM image_tag it
        JOIN tag t ON t.id = it.tag_id
        WHERE it.image_id = i.id AND t.name = want_tag.name
      )
    )
  )
  -- Ignore pagination parameters
  AND ($1::int IS NOT DISTINCT FROM $1)
  AND ($2::int IS NOT DISTINCT FROM $2);
