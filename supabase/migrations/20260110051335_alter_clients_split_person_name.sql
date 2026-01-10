-- Add first_name and last_name columns
ALTER TABLE clients
ADD COLUMN first_name VARCHAR(255),
ADD COLUMN last_name VARCHAR(255);

-- Migrate existing person_name data to first_name and last_name
-- Split on first space: everything before = first_name, everything after = last_name
UPDATE clients
SET
    first_name = SPLIT_PART(person_name, ' ', 1),
    last_name = SUBSTRING(person_name FROM POSITION(' ' IN person_name) + 1)
WHERE client_type = 'person' AND person_name IS NOT NULL;

-- Handle cases where there's no space (single name)
UPDATE clients
SET
    first_name = person_name,
    last_name = ''
WHERE client_type = 'person'
    AND person_name IS NOT NULL
    AND POSITION(' ' IN person_name) = 0;

-- Drop the old person_name column
ALTER TABLE clients DROP COLUMN IF EXISTS person_name;

-- Update the constraint to use first_name and last_name instead of person_name
ALTER TABLE clients DROP CONSTRAINT IF EXISTS client_name_check;

ALTER TABLE clients ADD CONSTRAINT client_name_check CHECK (
    (client_type = 'company' AND company_name IS NOT NULL) OR
    (client_type = 'person' AND first_name IS NOT NULL)
);
