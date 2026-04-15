-- Drop the foreign key constraint
ALTER TABLE user_category
DROP FOREIGN KEY user_category_ibfk_2;

-- Modify the column type
ALTER TABLE user_category
    MODIFY category_id BINARY(16);

-- Recreate the foreign key
ALTER TABLE user_category
    ADD CONSTRAINT user_category_category_id_fk
        FOREIGN KEY (category_id) REFERENCES categories(id)
            ON DELETE CASCADE;