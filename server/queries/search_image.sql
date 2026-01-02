-- Search for images based on various criteria
-- Parameters:
-- $1: limit::int (required)
-- $2: offset::int (required)
--
-- $3: min_width::int
-- $4: max_width::int
-- $5: min_height::int
-- $6: max_height::int
-- $7: mime_type::text
-- $8: created_before::timestamp
-- $9: created_after::timestamp
-- $10: updated_before::timestamp
-- $11: updated_after::timestamp
-- $12: is_public::boolean
-- $13: owner_id::int
--
-- $14: album_id::int
-- $15: tags::text[]

SELECT
  i.id, i.owner_id,
  s.size_bytes, s.width, s.height, s.mime_type, s.exif,
  i.is_public, i.created_at, i.updated_at,
  coalesce(tl.tags, '{}'::text[]) AS "tags!"
FROM image i
 JOIN image_storage s ON i.storage_id = s.id
LEFT JOIN LATERAL (
  SELECT array_agg(t.name ORDER BY t.name) AS tags
  FROM image_tag it
  JOIN tag t ON it.tag_id = t.id
  WHERE it.image_id = i.id
) tl ON TRUE
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
  AND ($8::timestamp IS NULL OR i.created_at <= $8::timestamp)
  AND ($9::timestamp IS NULL OR i.created_at >= $9::timestamp)
  -- Filter by updated_at
  AND ($10::timestamp IS NULL OR i.updated_at <= $10::timestamp)
  AND ($11::timestamp IS NULL OR i.updated_at >= $11::timestamp)
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
ORDER BY i.id DESC
LIMIT $1 OFFSET $2;
