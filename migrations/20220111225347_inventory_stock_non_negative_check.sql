ALTER TABLE inventory ADD CONSTRAINT non_negative_stock CHECK (stock >= 0);
