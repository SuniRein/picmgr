ALTER TABLE app_user
  ALTER COLUMN created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC',
  ALTER COLUMN created_at SET DEFAULT now();

ALTER TABLE image
  ALTER COLUMN created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC',
  ALTER COLUMN created_at SET DEFAULT now(),
  ALTER COLUMN updated_at TYPE timestamptz USING updated_at AT TIME ZONE 'UTC',
  ALTER COLUMN updated_at SET DEFAULT now();

ALTER TABLE album
  ALTER COLUMN created_at TYPE timestamptz USING created_at AT TIME ZONE 'UTC',
  ALTER COLUMN created_at SET DEFAULT now(),
  ALTER COLUMN updated_at TYPE timestamptz USING updated_at AT TIME ZONE 'UTC',
  ALTER COLUMN updated_at SET DEFAULT now();
