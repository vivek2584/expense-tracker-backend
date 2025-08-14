CREATE TYPE category_type AS ENUM ('expense', 'income');

ALTER TABLE categories
ADD COLUMN type category_type NOT NULL DEFAULT 'expense';
